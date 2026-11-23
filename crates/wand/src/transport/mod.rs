mod webview;

use std::{
    fs,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
};

use anyhow::{Context, Result, anyhow};
use futures::channel::oneshot;
use model::{Failure, FailureKind};
use serde::Serialize;
use tao::event_loop::EventLoopProxy;

const BRIDGE: &str = include_str!("bridge.html");
// The browser's profile; WebView2 holds it while it runs, so it is cleared before the next start.
pub(super) const PROFILE: &str = "webview";
const CLEAR: &str = "webview.clear";

/// The browser profile's size, and whether it is to be cleared at the next start (blocking).
pub fn profile(data_dir: &Path) -> (u64, bool) {
    (disk::size(&data_dir.join(PROFILE)), data_dir.join(CLEAR).exists())
}

pub fn clear_profile_on_start(data_dir: &Path) {
    if let Err(e) = fs::write(data_dir.join(CLEAR), []) {
        log::warn!("browser profile not marked for clearing: {e}");
    }
}

#[derive(Serialize)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

pub struct Response {
    pub status: u16,
    pub body: String,
}

enum Cmd {
    Fetch(Request, oneshot::Sender<Result<Response>>),
    Ready,
}

#[derive(Clone)]
pub struct Transport {
    proxy: EventLoopProxy<Cmd>,
}

impl Transport {
    pub fn spawn(data_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&data_dir).context("create data dir")?;
        if data_dir.join(CLEAR).exists() {
            let profile = data_dir.join(PROFILE);
            // The marker stays while the profile does: after a hard kill WebView2 can hold it a while.
            match fs::remove_dir_all(&profile) {
                Err(e) if profile.exists() => log::warn!("bridge profile not cleared, tried again next start: {e}"),
                _ => drop(fs::remove_file(data_dir.join(CLEAR))),
            }
        }
        let (started, wait) = mpsc::sync_channel(0);
        thread::Builder::new()
            .name("impulse-transport".into())
            .spawn(move || webview::run(&data_dir, started))
            .context("start transport thread")?;
        let proxy = wait.recv().context("transport thread exited")?;
        Ok(Self { proxy })
    }

    pub async fn fetch(&self, request: Request) -> Result<Response> {
        let (tx, rx) = oneshot::channel();
        self.proxy.send_event(Cmd::Fetch(request, tx)).map_err(|_| offline("transport is closed"))?;
        rx.await.map_err(|_| offline("transport dropped the request"))?
    }
}

/// A request the bridge could not make: the UI words it as no connection.
fn offline(why: &str) -> anyhow::Error {
    anyhow!("{why}").context(Failure::new(FailureKind::Offline))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "reaches api.wemod.com"]
    async fn the_token_endpoint_answers_the_bridge() {
        let dir = std::env::temp_dir().join(format!("impulse-bridge-test-{}", std::process::id()));
        let request = Request {
            method: "POST".into(),
            url: "https://api.wemod.com/auth/token".into(),
            headers: vec![("Content-Type".into(), "application/x-www-form-urlencoded".into())],
            body: Some("grant_type=tiny_code&code=fake&client_id=overlay".into()),
        };
        let response = Transport::spawn(dir).unwrap().fetch(request).await.unwrap();
        assert_eq!(response.status, 400);
        assert!(response.body.contains("`code`"), "the 400 is not about the code");
    }

    /// The whole bridge without the network: WebView2 starts, loads the page, fetches from a server on this
    /// machine and hands the answer back. Needs a WebView2 runtime (under Wine: `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER`).
    #[tokio::test]
    #[ignore = "starts WebView2"]
    async fn a_local_server_answers_through_the_bridge() {
        use std::{
            io::{Read, Write},
            net::TcpListener,
        };

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://127.0.0.1:{}/ping", listener.local_addr().unwrap().port());
        thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            let mut head = [0u8; 4096];
            let _ = socket.read(&mut head);
            let body = r#"{"pong":true}"#;
            let answer = format!(
                "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            socket.write_all(answer.as_bytes()).unwrap();
        });
        let dir = std::env::temp_dir().join(format!("impulse-bridge-local-{}", std::process::id()));
        let request = Request { method: "GET".into(), url, headers: vec![("Accept".into(), "application/json".into())], body: None };
        let response = Transport::spawn(dir).unwrap().fetch(request).await.unwrap();
        assert_eq!((response.status, response.body.as_str()), (200, r#"{"pong":true}"#));
    }
}
