//! Public files over HTTPS: a blocking GET for CDNs (catalog, art, trainer DLLs).

use std::{
    sync::{LazyLock, OnceLock},
    time::Duration,
};

use anyhow::{Context, Result, bail, ensure};
use ureq::{Agent, RequestBuilder, config::AutoHeaderValue, typestate::WithoutBody};
use url::Url;

mod guarded;

pub use guarded::{Refused, guarded};

// Public CDNs send no CORS headers, so the null-origin bridge cannot read them.
static AGENT: LazyLock<Agent> = LazyLock::new(|| {
    Agent::config_builder()
        .http_status_as_error(false)
        .user_agent(USER_AGENT)
        // A server that stalls must not hold a worker, or a run, forever; a slow one that keeps sending is waited for.
        .timeout_resolve(Some(STALL))
        .timeout_connect(Some(STALL))
        .timeout_per_read(Some(STALL))
        // Map tiles load 12 at a time; a reopened connection costs a TLS handshake.
        .max_idle_connections(16)
        .max_idle_connections_per_host(12)
        .max_idle_age(Duration::from_secs(90))
        .build()
        .into()
});

// A domain and the User-Agent its requests carry instead (`name_as`).
static NAMED: OnceLock<(String, String)> = OnceLock::new();

const USER_AGENT: &str = concat!("Impulse/", env!("CARGO_PKG_VERSION"));
const MAX_BODY: u64 = 64 << 20;
const STALL: Duration = Duration::from_secs(30);

pub struct Response {
    pub status: u16,
    pub etag: Option<String>,
    pub body: Vec<u8>,
    /// Who answered an error (`server`, `cf-ray`, `cf-cache-status`), for the log; empty for a success.
    pub edge: String,
}

pub(crate) fn etag_of(headers: &ureq::http::HeaderMap) -> Option<String> {
    headers.get("etag").and_then(|v| v.to_str().ok()).map(|v| v.trim_start_matches("W/").trim_matches('"').to_owned())
}

pub(crate) fn edge_of(status: u16, headers: &ureq::http::HeaderMap) -> String {
    if status < 400 {
        return String::new();
    }
    let named = |name| headers.get(name).and_then(|v| v.to_str().ok()).map(|v| format!("{name} {v}"));
    ["server", "cf-ray", "cf-cache-status"].into_iter().filter_map(named).collect::<Vec<_>>().join(", ")
}

/// From now on [`get`] and [`range`] of `domain` or a host under it send `agent` as their User-Agent; the first call
/// holds.
pub fn name_as(domain: &str, agent: &str) {
    let _ = NAMED.set((domain.to_ascii_lowercase(), agent.to_owned()));
}

/// A GET of `url`, its User-Agent as [`name_as`] gave its host.
fn ask(url: &str) -> RequestBuilder<WithoutBody> {
    let request = AGENT.get(url);
    let host = Url::parse(url).ok().and_then(|url| url.host_str().map(str::to_owned));
    match (NAMED.get(), host) {
        (Some((domain, agent)), Some(host)) if under(&host, domain) => request.header("User-Agent", agent),
        _ => request,
    }
}

fn under(host: &str, domain: &str) -> bool {
    host.strip_suffix(domain).is_some_and(|rest| rest.is_empty() || rest.ends_with('.'))
}

/// Blocking GET; `etag` makes it conditional (304 has an empty body).
pub fn get(url: &str, etag: Option<&str>) -> Result<Response> {
    let mut request = ask(url);
    if let Some(tag) = etag {
        request = request.header("If-None-Match", format!("\"{tag}\""));
    }
    answer(url, request)
}

/// [`get`] with no header of its own: no User-Agent, Accept or Accept-Encoding.
pub fn get_bare(url: &str) -> Result<Response> {
    let none = || AutoHeaderValue::None;
    answer(url, AGENT.get(url).config().user_agent(none()).accept(none()).accept_encoding(none()).build())
}

fn answer(url: &str, request: RequestBuilder<WithoutBody>) -> Result<Response> {
    let mut response = request.call().with_context(|| format!("GET {url}"))?;
    let etag = etag_of(response.headers());
    let status = response.status().as_u16();
    let edge = edge_of(status, response.headers());
    // An error's body is never used: one past the limit must not hide its status. A 304 sends none, yet may name an
    // encoding (GitHub's does): reading it would fail in the decoder.
    let body = match status {
        304 | 400.. => Vec::new(),
        _ => response.body_mut().with_config().limit(MAX_BODY).read_to_vec().with_context(|| format!("read {url}"))?,
    };
    Ok(Response { status, etag, body, edge })
}

/// Blocking GET of `len` bytes from `from`: a 206 starting there (shorter only at the file's end). A server that
/// ignores the range is an error, its body unread; another status (a redirect too: none is followed) comes back with
/// no body.
pub fn range(url: &str, from: u64, len: u64) -> Result<Response> {
    ensure!(len > 0 && len <= MAX_BODY, "GET {url}: a range of {len} bytes");
    let last = from.checked_add(len - 1).with_context(|| format!("GET {url}: a range past the end"))?;
    // Offsets count the stored bytes: an encoded answer would shift them.
    let request = ask(url).config().max_redirects(0).build();
    let request = request.header("Range", format!("bytes={from}-{last}")).header("Accept-Encoding", "identity");
    let mut response = request.call().with_context(|| format!("GET {url}"))?;
    let (status, headers) = (response.status().as_u16(), response.headers());
    let (etag, edge) = (etag_of(headers), edge_of(status, headers));
    if status != 206 {
        ensure!(status >= 300, "GET {url}: the server ignores ranges");
        return Ok(Response { status, etag, body: Vec::new(), edge });
    }
    let span = headers.get("content-range").and_then(|v| v.to_str().ok()).and_then(content_range);
    // All that was asked, or less only up to the file's end.
    let whole = |&(first, end, size): &(u64, u64, Option<u64>)| {
        first == from && first <= end && (end == last || end < last && size == Some(end + 1))
    };
    let Some((first, end, _)) = span.filter(whole) else { bail!("GET {url}: a range other than {from}-{last}") };
    let body = response.body_mut().with_config().limit(len).read_to_vec().with_context(|| format!("read {url}"))?;
    ensure!(body.len() as u64 == end - first + 1, "GET {url}: {} bytes of {from}-{end}", body.len());
    Ok(Response { status, etag, body, edge })
}

/// `bytes <first>-<last>/<size>`, the size past `last`; `None` when the server does not know it (`*`).
fn content_range(value: &str) -> Option<(u64, u64, Option<u64>)> {
    let (span, size) = value.strip_prefix("bytes ")?.split_once('/')?;
    let (first, last) = span.split_once('-')?;
    let last = last.parse().ok()?;
    let size = match size {
        "*" => None,
        size => Some(size.parse().ok().filter(|size| *size > last)?),
    };
    Some((first.parse().ok()?, last, size))
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        net::TcpListener,
        thread,
    };

    use super::*;

    /// GitHub answers a conditional request with a 304 that names an encoding and sends no body.
    #[test]
    fn a_not_modified_answer_that_names_an_encoding_has_no_body() {
        let (url, server) = serve(b"HTTP/1.1 304 Not Modified\r\nETag: \"x\"\r\nContent-Encoding: gzip\r\nConnection: close\r\n\r\n");
        let response = get(&url, Some("x")).unwrap();
        server.join().unwrap();
        assert_eq!((response.status, response.etag.as_deref(), response.body.len()), (304, Some("x"), 0));
    }

    #[test]
    fn a_named_domain_is_asked_as_its_agent_and_a_bare_get_as_none() {
        const OK: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok";
        name_as("127.0.0.1", "Named/1");
        let (url, server) = serve(OK);
        get(&url, None).unwrap();
        assert!(server.join().unwrap().contains("user-agent: named/1"));

        let (url, server) = serve(OK);
        assert_eq!(get_bare(&url).unwrap().body, b"ok");
        let asked = server.join().unwrap();
        assert!(["user-agent", "accept"].iter().all(|name| !asked.contains(name)), "{asked}");
    }

    #[test]
    fn a_host_is_under_its_domain_and_its_subdomains_only() {
        assert!(under("wemod.com", "wemod.com") && under("storage-cdn.wemod.com", "wemod.com"));
        assert!(!under("notwemod.com", "wemod.com") && !under("wemod.com.evil.io", "wemod.com"));
    }

    /// Serves one request with `answer`, giving back what was asked.
    fn serve(answer: &'static [u8]) -> (String, thread::JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/file", listener.local_addr().unwrap());
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut asked = [0; 1024];
            let read = stream.read(&mut asked).unwrap();
            stream.write_all(answer).unwrap();
            String::from_utf8_lossy(&asked[..read]).to_lowercase()
        });
        (url, server)
    }

    #[test]
    fn a_range_is_its_bytes_and_a_server_that_ignores_it_an_error() {
        let (url, server) =
            serve(b"HTTP/1.1 206 Partial Content\r\nContent-Range: bytes 10-13/100\r\nContent-Length: 4\r\nConnection: close\r\n\r\nabcd");
        let response = range(&url, 10, 4).unwrap();
        let asked = server.join().unwrap();
        assert!(asked.contains("range: bytes=10-13") && asked.contains("accept-encoding: identity"), "{asked}");
        assert_eq!((response.status, response.body.as_slice()), (206, &b"abcd"[..]));

        let (url, server) = serve(b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\nConnection: close\r\n\r\nabcd");
        assert!(range(&url, 10, 4).is_err());
        server.join().unwrap();

        let (url, server) =
            serve(b"HTTP/1.1 206 Partial Content\r\nContent-Range: bytes 0-3/100\r\nContent-Length: 4\r\nConnection: close\r\n\r\nabcd");
        assert!(range(&url, 10, 4).is_err(), "another range than asked");
        server.join().unwrap();

        let (url, server) =
            serve(b"HTTP/1.1 206 Partial Content\r\nContent-Range: bytes 10-11/100\r\nContent-Length: 2\r\nConnection: close\r\n\r\nab");
        assert!(range(&url, 10, 4).is_err(), "short before the end");
        server.join().unwrap();

        let (url, server) =
            serve(b"HTTP/1.1 206 Partial Content\r\nContent-Range: bytes 10-13/12\r\nContent-Length: 4\r\nConnection: close\r\n\r\nabcd");
        assert!(range(&url, 10, 4).is_err(), "a size before its end");
        server.join().unwrap();

        let (url, server) =
            serve(b"HTTP/1.1 206 Partial Content\r\nContent-Range: bytes 10-11/12\r\nContent-Length: 2\r\nConnection: close\r\n\r\nab");
        assert_eq!(range(&url, 10, 4).unwrap().body, b"ab", "short at the end");
        server.join().unwrap();

        let (url, server) = serve(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
        assert_eq!(range(&url, 0, 4).unwrap().status, 404);
        server.join().unwrap();
    }
}
