use std::{
    borrow::Cow,
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex, mpsc::SyncSender},
    thread,
    time::{Duration, Instant},
};

use anyhow::{Result, anyhow};
use futures::channel::oneshot;
use model::{Failure, FailureKind};
use serde::{Deserialize, Serialize};
use tao::{
    event::{Event, StartCause},
    event_loop::{ControlFlow, DeviceEventFilter, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget},
    platform::windows::{EventLoopBuilderExtWindows, WindowBuilderExtWindows},
    window::{Window, WindowBuilder},
};
use webview2_com::{Error::WindowsError, Microsoft::Web::WebView2::Win32::ICoreWebView2Settings2, take_pwstr};
use windows::core::{HSTRING, Interface, PWSTR};
use wry::{WebContext, WebView, WebViewBuilder, WebViewBuilderExtWindows, WebViewExtWindows, http::Response as HttpResponse};

use super::{BRIDGE, Cmd, Request, Response, offline};
use crate::{
    install,
    texts::{PROBE_FAILED, PROBE_OK, text},
};

type Pending = Arc<Mutex<HashMap<u64, oneshot::Sender<Result<Response>>>>>;
// Fields drop in order: the WebView closes before its window.
type Bridge = (WebView, WebContext, Window);

// A running WebView2 commits ~65 MB across its processes, so it runs only around requests.
const IDLE: Duration = Duration::from_secs(30);
// wry's defaults, then no GPU process and the network service inside the browser process.
const BROWSER_ARGS: &str = "--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection \
    --enable-features=NetworkServiceInProcess2 \
    --disable-gpu --disable-software-rasterizer --in-process-gpu";

#[derive(Serialize)]
struct Call<'a> {
    id: u64,
    #[serde(flatten)]
    request: &'a Request,
}

#[derive(Deserialize)]
struct Reply {
    id: u64,
    status: Option<u16>,
    body: Option<String>,
    error: Option<String>,
    probe_ok: Option<bool>,
}

pub(super) fn run(data_dir: &Path, started: SyncSender<EventLoopProxy<Cmd>>) {
    let event_loop = EventLoopBuilder::<Cmd>::with_user_event().with_any_thread(true).with_dpi_aware(false).build();
    let proxy = event_loop.create_proxy();
    let _ = started.send(proxy.clone());
    let pending = Pending::default();
    // tao registers raw input for the whole process, and then Windows skips the `keyhook` hook
    // while Impulse is in front.
    event_loop.set_device_event_filter(DeviceEventFilter::Always);
    let data_dir = data_dir.to_owned();

    let mut bridge: Option<Bridge> = None;
    let (mut ready, mut queue, mut next_id, mut last) = (false, Vec::new(), 0u64, Instant::now());
    event_loop.run(move |event, target, flow| {
        match event {
            Event::UserEvent(Cmd::Ready) => ready = true,
            Event::UserEvent(Cmd::Fetch(request, tx)) => {
                let new = bridge.is_none();
                let built = bridge.take().map_or_else(|| build_retrying(&data_dir, target, proxy.clone(), pending.clone()), Ok);
                // After the build, as its retries can outlast the idle time; a page still loading keeps its deadline.
                if new || ready {
                    last = Instant::now();
                }
                match built {
                    Ok(built) => {
                        bridge = Some(built);
                        queue.push((request, tx));
                    }
                    Err(e) => {
                        let _ = tx.send(Err(offline(&format!("{e:#}"))));
                    }
                }
            }
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                // A page that never loaded would hold its queue forever.
                if !ready {
                    for (_, tx) in queue.drain(..) {
                        let _ = tx.send(Err(offline("the bridge page did not load")));
                    }
                }
                if queue.is_empty() && pending.lock().unwrap().is_empty() {
                    (bridge, ready) = (None, false);
                    log::debug!("WebView2 closed while idle");
                } else {
                    last = Instant::now();
                }
            }
            _ => {}
        }
        *flow = match bridge {
            Some(_) => ControlFlow::WaitUntil(last + IDLE),
            None => ControlFlow::Wait,
        };
        let Some((webview, ..)) = bridge.as_ref().filter(|_| ready) else {
            return;
        };
        for (request, tx) in queue.drain(..) {
            next_id += 1;
            pending.lock().unwrap().insert(next_id, tx);
            let sent = serde_json::to_string(&Call { id: next_id, request: &request })
                .is_ok_and(|json| webview.evaluate_script(&format!("impulse({json})")).is_ok());
            if !sent && let Some(tx) = pending.lock().unwrap().remove(&next_id) {
                let _ = tx.send(Err(offline("webview dispatch failed")));
            }
        }
    });
}

// After some hard kills WebView2 keeps the profile busy for about 50 s; a clean exit frees it at once.
fn build_retrying(data_dir: &Path, target: &EventLoopWindowTarget<Cmd>, proxy: EventLoopProxy<Cmd>, pending: Pending) -> Result<Bridge> {
    const ERROR_BUSY: i32 = 0x800700AA_u32 as i32;
    let deadline = Instant::now() + Duration::from_secs(90);
    let mut busy = false;
    loop {
        match build(data_dir, target, proxy.clone(), pending.clone()) {
            Err(e)
                if Instant::now() < deadline
                    && matches!(e.downcast_ref(), Some(wry::Error::WebView2Error(WindowsError(w))) if w.code().0 == ERROR_BUSY) =>
            {
                if !busy {
                    log::warn!("WebView2 profile is busy, retrying for up to 90 s");
                    busy = true;
                }
                thread::sleep(Duration::from_secs(1));
            }
            built => {
                match &built {
                    Ok(_) => log::debug!("WebView2 started"),
                    Err(e) => log::error!("WebView2 failed to start: {e:#}"),
                }
                return built;
            }
        }
    }
}

// COM: WebViewBuilder::build runs CoInitializeEx(STA) on this thread, and tao skips OLE with drag and drop off.
fn build(data_dir: &Path, target: &EventLoopWindowTarget<Cmd>, proxy: EventLoopProxy<Cmd>, pending: Pending) -> Result<Bridge> {
    // Written for each build: a page removed since the last one would never load.
    std::fs::write(data_dir.join("bridge.html"), BRIDGE)?;
    let window = WindowBuilder::new().with_visible(false).with_drag_and_drop(false).build(target)?;
    let mut context = WebContext::new(Some(data_dir.join(super::PROFILE)));
    let url = url::Url::from_file_path(data_dir.join("bridge.html")).map_err(|()| anyhow!("data dir is not an absolute path"))?;
    let webview = WebViewBuilder::new_with_web_context(&mut context)
        .with_custom_protocol("impulse".into(), move |_, request| {
            match request.uri().path() {
                "/ready" => {
                    let _ = proxy.send_event(Cmd::Ready);
                }
                "/response" => reply(&pending, request.body()),
                _ => {}
            }
            HttpResponse::builder().status(204).header("Access-Control-Allow-Origin", "*").body(Cow::Borrowed(&[][..])).unwrap()
        })
        .with_additional_browser_args(BROWSER_ARGS)
        .build(&window)?;
    if let Err(e) = name_wand(&webview) {
        log::warn!("bridge User-Agent not set, signing in may fail: {e}");
    }
    webview.load_url(url.as_str())?;
    Ok((webview, context, window))
}

fn name_wand(webview: &WebView) -> windows::core::Result<()> {
    // SAFETY: the WebView was built on this thread and outlives the call.
    let settings = unsafe { webview.webview().Settings() }?.cast::<ICoreWebView2Settings2>()?;
    let mut agent = PWSTR::null();
    // SAFETY: WebView2 allocates the string, `take_pwstr` frees it.
    unsafe { settings.UserAgent(&mut agent) }?;
    let agent = HSTRING::from(format!("{} {}", take_pwstr(agent), install::user_agent()));
    // SAFETY: as for `Settings`.
    unsafe { settings.SetUserAgent(&agent) }
}

fn failed(error: &str, probe_ok: Option<bool>) -> anyhow::Error {
    let (result, words) = match probe_ok {
        Some(false) => ("origin probe failed (network or browser policy)", PROBE_FAILED),
        Some(true) => ("origin probe succeeded; cause of the original failure unknown", PROBE_OK),
        None => return offline(&format!("fetch failed: {error}")),
    };
    anyhow!("fetch failed: {error}; {result}").context(Failure::new(FailureKind::Offline).saying(text(words)))
}

fn reply(pending: &Pending, raw: &[u8]) {
    let Ok(reply) = serde_json::from_slice::<Reply>(raw) else {
        return;
    };
    if let Some(tx) = pending.lock().unwrap().remove(&reply.id) {
        let _ = tx.send(reply.result());
    }
}

impl Reply {
    fn result(self) -> Result<Response> {
        match (self.status, self.error.as_deref()) {
            (Some(status), None) => Ok(Response { status, body: self.body.unwrap_or_default() }),
            (Some(status), Some(error)) => Err(offline(&format!("HTTP {status}: could not read the response body ({error})"))),
            (None, error) => Err(failed(error.unwrap_or("browser request failed"), self.probe_ok)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn failure(json: &str) -> Failure {
        let reply: Reply = serde_json::from_str(json).unwrap();
        Failure::from(reply.result().err().expect("an error reply"))
    }

    #[test]
    fn a_body_read_error_keeps_the_status() {
        let got = failure(r#"{"id":1,"status":502,"error":"TypeError","body":"private"}"#);
        assert_eq!(got.kind, FailureKind::Offline);
        assert_eq!(got.detail, "Offline: HTTP 502: could not read the response body (TypeError)");
    }

    #[test]
    fn a_request_without_a_reply_is_worded_by_the_probe() {
        let not_reached = failure(r#"{"id":1,"error":"TypeError","probe_ok":false}"#);
        let reached = failure(r#"{"id":1,"error":"TypeError","probe_ok":true}"#);
        let unprobed = failure(r#"{"id":1,"error":"timed out"}"#);
        assert!([&not_reached, &reached, &unprobed].iter().all(|failure| failure.kind == FailureKind::Offline));
        assert_eq!((not_reached.message, reached.message, unprobed.message), (Some(text(PROBE_FAILED)), Some(text(PROBE_OK)), None));
        assert!(not_reached.detail.contains("origin probe failed"), "{}", not_reached.detail);
        assert!(reached.detail.contains("cause of the original failure unknown"), "{}", reached.detail);
    }
}
