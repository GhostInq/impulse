//! `guarded`: the HTTPS GET behind a plugin's `http_get`. Every hop is checked by hand and
//! only public addresses are dialled, so a plugin reaches its allowed hosts and nothing else.

use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    sync::LazyLock,
    time::{Duration, Instant},
};

use anyhow::{Context, Result, anyhow, bail};
use ureq::{
    Agent,
    http::Uri,
    unversioned::{
        resolver::{DefaultResolver, ResolvedSocketAddrs, Resolver},
        transport::{DefaultConnector, NextTimeout},
    },
};
use url::Url;

use crate::{Response, USER_AGENT, edge_of, etag_of};

const MAX_REDIRECTS: usize = 5;

/// Marker for a URL, redirect hop or host outside what the caller allowed. A caller detects it with
/// `error.downcast_ref::<fetch::Refused>().is_some()` (or `error.chain().any(|e| e.is::<fetch::Refused>())`).
#[derive(Debug)]
pub struct Refused;

impl std::fmt::Display for Refused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("outside the plugin's scope")
    }
}

impl std::error::Error for Refused {}

/// Drops every address that is not public. It runs on each hop, so redirects and IP-literal hosts are covered.
#[derive(Debug, Default)]
struct PublicOnly(DefaultResolver);

impl Resolver for PublicOnly {
    fn resolve(&self, uri: &Uri, config: &ureq::config::Config, timeout: NextTimeout) -> Result<ResolvedSocketAddrs, ureq::Error> {
        let all = self.0.resolve(uri, config, timeout)?;
        let mut kept = self.empty();
        for addr in all.iter().filter(|addr| public(addr.ip())) {
            kept.push(*addr);
        }
        if kept.is_empty() {
            // Not HostNotFound: the caller must tell "refused" from a DNS failure.
            return Err(ureq::Error::Other(Box::new(Refused)));
        }
        Ok(kept)
    }
}

// Its own agent: `get` follows redirects and reads the proxy settings, a plugin's request must do neither.
static AGENT: LazyLock<Agent> = LazyLock::new(|| {
    let config = Agent::config_builder()
        .http_status_as_error(false)
        .https_only(true)
        // A proxy would make the resolver check the proxy's address instead of the origin's.
        .proxy(None)
        .max_redirects(0)
        .user_agent(USER_AGENT)
        .build();
    Agent::with_parts(config, DefaultConnector::default(), PublicOnly::default())
});

/// Blocking GET for a plugin: https only, every hop's host passes `allowed`, public addresses only, at most
/// 5 redirects, `timeout` for the whole call. A 2xx body is read up to `limit`; other statuses return an empty body.
/// A refused URL, hop or host is an error carrying [`Refused`].
pub fn guarded(url: &str, allowed: &dyn Fn(&str) -> bool, limit: u64, timeout: Duration) -> Result<Response> {
    let deadline = Instant::now() + timeout;
    let mut current = Url::parse(url).map_err(|_| anyhow!("not a valid URL"))?;
    for _ in 0..=MAX_REDIRECTS {
        check(&current, allowed)?;
        let left = deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            bail!("request timed out");
        }
        let host = current.host_str().unwrap_or_default().to_owned();
        let mut response = AGENT
            .get(current.as_str())
            .config()
            .timeout_global(Some(left))
            .build()
            .call()
            .map_err(request_error)
            .with_context(|| format!("GET {host}"))?;
        let status = response.status().as_u16();
        let location = response.headers().get("location").and_then(|v| v.to_str().ok());
        if let Some(next) = redirect_target(&current, status, location)? {
            current = next;
            continue;
        }
        let etag = etag_of(response.headers());
        let edge = edge_of(status, response.headers());
        let body = match status {
            200..=299 => response.body_mut().with_config().limit(limit).read_to_vec().map_err(|e| match e {
                ureq::Error::BodyExceedsLimit(_) => anyhow::Error::new(Refused).context(format!("{host}: a body past {limit} bytes")),
                other => anyhow::Error::new(other).context(format!("read {host}")),
            })?,
            _ => Vec::new(),
        };
        return Ok(Response { status, etag, body, edge });
    }
    bail!("too many redirects")
}

/// Keeps the `Refused` marker the resolver put inside ureq's error.
fn request_error(error: ureq::Error) -> anyhow::Error {
    match error {
        ureq::Error::Other(inner) if inner.is::<Refused>() => anyhow::Error::new(Refused).context("host has no public address"),
        other => anyhow::Error::new(other),
    }
}

/// The scheme and host rule for the first URL and for every redirect hop.
fn check(url: &Url, allowed: &dyn Fn(&str) -> bool) -> Result<()> {
    if url.scheme() != "https" {
        return Err(anyhow::Error::new(Refused).context(format!("scheme {} is not https", url.scheme())));
    }
    match url.host_str() {
        Some(host) if allowed(host) => Ok(()),
        Some(host) => Err(anyhow::Error::new(Refused).context(format!("host {host} is not allowed"))),
        None => Err(anyhow::Error::new(Refused).context("URL has no host")),
    }
}

/// The next URL for a redirect response, `None` when the response is final.
fn redirect_target(base: &Url, status: u16, location: Option<&str>) -> Result<Option<Url>> {
    if !matches!(status, 301 | 302 | 303 | 307 | 308) {
        return Ok(None);
    }
    let Some(location) = location else { return Ok(None) };
    // Relative locations resolve against the hop that sent them.
    let next = base.join(location).map_err(|_| anyhow!("redirect to an invalid URL"))?;
    Ok(Some(next))
}

/// True for an address a plugin may reach: not loopback, private, link-local, shared, reserved or multicast.
fn public(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => public_v4(ip),
        IpAddr::V6(ip) => public_v6(ip),
    }
}

fn public_v4(ip: Ipv4Addr) -> bool {
    let [a, b, c, _] = ip.octets();
    !(ip.is_unspecified()
        || ip.is_loopback()
        || ip.is_private()
        || ip.is_link_local()
        || ip.is_documentation()
        || ip.is_multicast()
        || ip.is_broadcast()
        || a == 0
        || (a == 100 && b & 0xc0 == 64)
        || (a == 192 && b == 0 && c == 0)
        || (a == 198 && b & 0xfe == 18)
        || a >= 240)
}

fn public_v6(ip: Ipv6Addr) -> bool {
    let s = ip.segments();
    let embedded = || {
        let [.., a, b, c, d] = ip.octets();
        Ipv4Addr::new(a, b, c, d)
    };
    // Mapped and NAT64 addresses reach the IPv4 host inside them.
    if s[..5] == [0; 5] && s[5] == 0xffff || s[0] == 0x64 && s[1] == 0xff9b && s[2..6] == [0; 4] {
        return public_v4(embedded());
    }
    // Global unicast only: every local, compatible and reserved range lies outside 2000::/3, but documentation.
    let documentation = s[0] == 0x2001 && s[1] == 0x0db8 || s[0] == 0x3fff && s[1] < 0x1000;
    s[0] & 0xe000 == 0x2000 && !documentation
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v4(text: &str) -> bool {
        public(IpAddr::V4(text.parse().expect("test address")))
    }

    fn v6(text: &str) -> bool {
        public(IpAddr::V6(text.parse().expect("test address")))
    }

    fn url(text: &str) -> Url {
        Url::parse(text).expect("test url")
    }

    fn refused(error: &anyhow::Error) -> bool {
        error.downcast_ref::<Refused>().is_some() && error.chain().any(|e| e.is::<Refused>())
    }

    #[test]
    fn public_v4_addresses_pass() {
        for ip in [
            "8.8.8.8",
            "1.1.1.1",
            "172.15.255.255",
            "172.32.0.0",
            "100.63.255.255",
            "100.128.0.0",
            "198.17.255.255",
            "198.20.0.0",
            "223.255.255.255",
        ] {
            assert!(v4(ip), "{ip}");
        }
    }

    #[test]
    fn special_v4_ranges_are_refused() {
        for ip in [
            "0.0.0.0",
            "0.1.2.3",
            "127.0.0.1",
            "127.255.255.255",
            "10.0.0.1",
            "172.16.0.1",
            "172.31.255.255",
            "192.168.1.1",
            "100.64.0.1",
            "100.127.255.255",
            "169.254.169.254",
            "192.0.0.1",
            "192.0.2.1",
            "198.51.100.1",
            "203.0.113.1",
            "198.18.0.1",
            "198.19.255.255",
            "224.0.0.1",
            "239.255.255.255",
            "240.0.0.1",
            "255.255.255.255",
        ] {
            assert!(!v4(ip), "{ip}");
        }
    }

    #[test]
    fn public_v6_addresses_pass() {
        for ip in ["2606:4700:4700::1111", "2001:4860:4860::8888", "2001:db9::1", "3fff:1000::1", "::ffff:8.8.8.8", "64:ff9b::808:808"] {
            assert!(v6(ip), "{ip}");
        }
    }

    #[test]
    fn special_v6_ranges_are_refused() {
        let special = [
            "::",
            "::1",
            "::127.0.0.1",
            "::10.0.0.1",
            "100::1",
            "ff02::1",
            "fc00::1",
            "fe80::1",
            "fec0::1",
            "2001:db8::1",
            "3fff::1",
            "4000::1",
        ];
        for ip in special {
            assert!(!v6(ip), "{ip}");
        }
    }

    #[test]
    fn mapped_and_nat64_addresses_follow_their_ipv4() {
        for ip in ["::ffff:127.0.0.1", "::ffff:10.0.0.1", "::ffff:169.254.169.254", "64:ff9b::7f00:1", "64:ff9b::a00:1"] {
            assert!(!v6(ip), "{ip}");
        }
    }

    #[test]
    fn a_hop_needs_https_and_an_allowed_host() {
        let allowed = |host: &str| host == "cdn.example.com";
        assert!(check(&url("https://cdn.example.com/a"), &allowed).is_ok());
        for bad in
            ["http://cdn.example.com/a", "https://evil.example.com/a", "ftp://cdn.example.com/a", "https://cdn.example.com.evil.net/"]
        {
            let error = check(&url(bad), &allowed).expect_err(bad);
            assert!(refused(&error), "{bad}");
        }
    }

    #[test]
    fn the_host_is_given_in_lower_case() {
        assert!(check(&url("https://CDN.Example.com/"), &|host| host == "cdn.example.com").is_ok());
    }

    #[test]
    fn a_relative_location_resolves_against_the_current_url() {
        let base = url("https://cdn.example.com/a/b?q=1");
        let next = |location| redirect_target(&base, 302, Some(location)).expect("valid").expect("redirect");
        assert_eq!(next("/c").as_str(), "https://cdn.example.com/c");
        assert_eq!(next("d").as_str(), "https://cdn.example.com/a/d");
        assert_eq!(next("//other.example.com/e").as_str(), "https://other.example.com/e");
        assert_eq!(next("http://cdn.example.com/f").as_str(), "http://cdn.example.com/f");
    }

    #[test]
    fn a_redirect_to_http_or_another_host_fails_the_hop_check() {
        let base = url("https://cdn.example.com/a");
        let allowed = |host: &str| host == "cdn.example.com";
        for location in ["http://cdn.example.com/x", "https://evil.example.com/x", "//evil.example.com/x"] {
            let next = redirect_target(&base, 301, Some(location)).expect("valid").expect("redirect");
            assert!(check(&next, &allowed).is_err(), "{location}");
        }
        let same = redirect_target(&base, 301, Some("/x")).expect("valid").expect("redirect");
        assert!(check(&same, &allowed).is_ok());
    }

    #[test]
    fn only_redirect_statuses_with_a_location_are_followed() {
        let base = url("https://cdn.example.com/a");
        for status in [301, 302, 303, 307, 308] {
            assert!(redirect_target(&base, status, Some("/x")).expect("valid").is_some(), "{status}");
            assert!(redirect_target(&base, status, None).expect("valid").is_none(), "{status}");
        }
        for status in [200, 204, 300, 304, 305, 404, 500] {
            assert!(redirect_target(&base, status, Some("/x")).expect("valid").is_none(), "{status}");
        }
    }

    #[test]
    fn a_refused_first_url_fails_before_any_request() {
        let allowed = |host: &str| host == "cdn.example.com";
        let timeout = Duration::from_secs(5);
        for bad in ["http://cdn.example.com/a", "https://evil.example.com/a", "file:///c:/x"] {
            let error = guarded(bad, &allowed, 1024, timeout).err().expect(bad);
            assert!(refused(&error), "{bad}");
        }
        let nothing = |_: &str| false;
        assert!(refused(&guarded("https://cdn.example.com/a", &nothing, 1024, timeout).err().expect("nothing is allowed")));
    }

    #[test]
    fn a_host_with_only_private_addresses_is_refused() {
        // IP literals resolve without DNS, and the resolver refuses them before any connection.
        let any = |_: &str| true;
        for literal in ["https://127.0.0.1/", "https://10.1.2.3:8443/x", "https://[::1]/", "https://[::ffff:192.168.0.1]/"] {
            let error = guarded(literal, &any, 1024, Duration::from_secs(5)).err().expect(literal);
            assert!(refused(&error), "{literal}: {error:#}");
        }
    }

    #[test]
    fn a_bad_url_is_an_error_but_not_refused() {
        let error = guarded("not a url", &|_| true, 1024, Duration::from_secs(5)).err().expect("bad url");
        assert!(!refused(&error));
    }
}
