//! Files the app keeps between runs.

mod stored;

use std::{
    fs,
    path::Path,
    process,
    sync::atomic::{AtomicU32, Ordering},
};

use anyhow::{Context, Result};

pub use stored::{Stored, VERSION, read};

/// Writes through a temporary file, so a crash never leaves a half-written one.
pub fn replace(path: &Path, data: &[u8]) -> Result<()> {
    // Two writers of one path, in this process or another instance, must not share the temporary file.
    static NEXT: AtomicU32 = AtomicU32::new(0);
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).with_context(|| format!("create {}", dir.display()))?;
    }
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    let tmp = path.with_extension(format!("{}.{n}.tmp", process::id()));
    fs::write(&tmp, data).with_context(|| format!("write {}", tmp.display()))?;
    fs::rename(&tmp, path).with_context(|| format!("replace {}", path.display()))
}

/// Bytes under `path`, a file or a folder (blocking); what cannot be read counts as nothing.
pub fn size(path: &Path) -> u64 {
    let Ok(meta) = fs::symlink_metadata(path) else { return 0 };
    if !meta.is_dir() {
        return meta.len();
    }
    fs::read_dir(path).into_iter().flatten().flatten().map(|entry| size(&entry.path())).sum()
}
