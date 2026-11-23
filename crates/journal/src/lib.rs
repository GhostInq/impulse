//! The application log. Slices write through the `log` macros; this keeps the
//! records in memory for the log view, appends them to a file and mirrors them to stderr.
//! Never log tokens, auth codes, request headers or bodies.

mod file;

use std::{
    collections::VecDeque,
    io::{self, Write},
    panic,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex, MutexGuard, OnceLock, PoisonError,
        atomic::{AtomicBool, Ordering},
    },
};

use chrono::{DateTime, Local};
use futures::channel::mpsc::{self, Receiver, Sender};
pub use log::Level;
use log::{LevelFilter, Log, Metadata};

pub const CAPACITY: usize = 5000;
const FILE_LIMIT: u64 = 2 << 20;
const APP: &str = "impulse";
/// The workspace's crates: their records are kept from Debug up, the others' from Warn.
const OURS: [&str; 24] = [
    APP,
    "agent",
    "checks",
    "client",
    "disk",
    "fetch",
    "hotkeys",
    "i18n",
    "journal",
    "launchers",
    "model",
    "overlay",
    "pe",
    "play",
    "plugins",
    "remote",
    "sdk",
    "shell",
    "sound",
    "tray",
    "updater",
    "video",
    "wand",
    "wine",
];

pub struct Record {
    /// Grows by one per record; unique within a run.
    pub seq: u64,
    pub time: DateTime<Local>,
    pub level: Level,
    /// The module that logged; the app's own without its crate name.
    pub source: String,
    pub message: String,
}

impl Record {
    pub fn line(&self) -> String {
        format!("{} {:<5} {} - {}", self.time.format("%Y-%m-%d %H:%M:%S%.3f"), self.level, self.source, self.message)
    }
}

struct Journal {
    dir: PathBuf,
    state: Mutex<State>,
}

struct State {
    records: VecDeque<Arc<Record>>,
    next_seq: u64,
    file: Option<file::Sink>,
    listener: Option<Sender<()>>,
}

static JOURNAL: OnceLock<Journal> = OnceLock::new();
static CLOSING: AtomicBool = AtomicBool::new(false);

/// Installs the logger and the panic hook. Call once, before anything logs.
pub fn init(data_dir: &Path) {
    let dir = data_dir.join("logs");
    let file = file::Sink::open(&dir, FILE_LIMIT).inspect_err(|e| mirror(&format!("log file unavailable: {e}"))).ok();
    let journal = JOURNAL.get_or_init(|| Journal {
        dir,
        state: Mutex::new(State { records: VecDeque::with_capacity(CAPACITY), next_seq: 0, file, listener: None }),
    });
    if log::set_logger(journal).is_ok() {
        log::set_max_level(LevelFilter::Debug);
    }
    let previous = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        log::error!(target: "impulse::panic", "{info}");
        previous(info);
    }));
    log::info!(
        target: APP,
        "Impulse {} ({}) on Windows {}",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::ARCH,
        windows_build()
    );
}

/// Buffered records with `seq >= from`, oldest first.
pub fn since(from: u64) -> Vec<Arc<Record>> {
    let Some(journal) = JOURNAL.get() else {
        return Vec::new();
    };
    let state = journal.lock();
    let skip = state.records.partition_point(|record| record.seq < from);
    state.records.iter().skip(skip).cloned().collect()
}

/// Pings when records arrived since the last ping was taken. A new subscriber replaces the previous one.
pub fn subscribe() -> Receiver<()> {
    // One slot coalesces a burst into a single wake-up.
    let (sender, receiver) = mpsc::channel(1);
    if let Some(journal) = JOURNAL.get() {
        journal.lock().listener = Some(sender);
    }
    receiver
}

/// Call when the last window closes: GPUI logs invalid-handle errors while it tears
/// the window down, and they would end every log with false alarms.
pub fn closing() {
    // The flag first: another thread may already be logging the teardown.
    CLOSING.store(true, Ordering::Relaxed);
    log::info!("closing");
}

pub fn dir() -> Option<&'static Path> {
    JOURNAL.get().map(|journal| journal.dir.as_path())
}

impl Journal {
    // A panic while logging must not silence the panic hook's own record.
    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl Log for Journal {
    // Dependencies are chatty below Warn.
    fn enabled(&self, metadata: &Metadata) -> bool {
        let ours = OURS.contains(&crate_of(metadata.target()));
        if !ours && CLOSING.load(Ordering::Relaxed) {
            return false;
        }
        metadata.level() <= if ours { Level::Debug } else { Level::Warn }
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        // Some dependencies log with an empty target; their file still says who it was.
        let located;
        let target = match record.target() {
            "" => {
                located = location(record);
                located.as_str()
            }
            target => target,
        };
        let source = match target.strip_prefix(APP) {
            Some("") => "app",
            Some(module) if module.starts_with("::") => &module[2..],
            _ => target,
        };
        // Formatted before the lock: a `Display` impl may log too.
        let message = scrub(&record.args().to_string());
        let time = Local::now();
        let mut state = self.lock();
        let record = Arc::new(Record { seq: state.next_seq, time, level: record.level(), source: source.to_owned(), message });
        state.next_seq += 1;
        let line = record.line();
        if let Some(file) = &mut state.file
            && let Err(e) = file.write(&line)
        {
            mirror(&format!("log file write failed: {e}"));
            state.file = None;
        }
        if state.records.len() == CAPACITY {
            state.records.pop_front();
        }
        state.records.push_back(record);
        // A full slot means a ping is already pending.
        if let Some(listener) = &mut state.listener
            && listener.try_send(()).is_err_and(|e| e.is_disconnected())
        {
            state.listener = None;
        }
        drop(state);
        mirror(&line);
    }

    fn flush(&self) {}
}

fn crate_of(target: &str) -> &str {
    target.split("::").next().unwrap_or_default()
}

// Not `eprintln!`: it panics when stderr is a closed pipe, and a panic here re-enters the logger.
fn mirror(line: &str) {
    let _ = writeln!(io::stderr(), "{line}");
}

// `crate/src/file.rs:line`, without the build machine's directories.
fn location(record: &log::Record) -> String {
    let file = record.file().unwrap_or("?");
    let mut tail: Vec<&str> = file.rsplit(['\\', '/']).take(3).collect();
    tail.reverse();
    format!("{}:{}", tail.join("/"), record.line().unwrap_or(0))
}

// One line per record, and no Windows user name from paths under the profile.
fn scrub(message: &str) -> String {
    static HOME: OnceLock<Option<String>> = OnceLock::new();
    let home = HOME.get_or_init(|| std::env::var("USERPROFILE").ok().filter(|home| !home.is_empty()));
    let line = message.replace(['\r', '\n'], " ");
    match home {
        Some(home) => line.replace(home.as_str(), "%USERPROFILE%"),
        None => line,
    }
}

fn windows_build() -> String {
    let version = windows_registry::LOCAL_MACHINE.open(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion");
    let value = |name| version.as_ref().ok().and_then(|key| key.get_string(name).ok()).unwrap_or_else(|| "?".to_owned());
    format!("{} build {}", value("DisplayVersion"), value("CurrentBuild"))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn every_workspace_crate_is_ours() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        for dir in fs::read_dir(root.join("crates")).unwrap() {
            let manifest = fs::read_to_string(dir.unwrap().path().join("Cargo.toml")).unwrap();
            let name = manifest.lines().find_map(|line| line.strip_prefix("name = \"")?.strip_suffix('"')).unwrap();
            assert!(OURS.contains(&name.replace('-', "_").as_str()), "{name} is missing from OURS");
        }
        assert_eq!(crate_of("wand::catalog"), "wand");
    }
}
