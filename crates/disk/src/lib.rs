//! Files the app keeps between runs.

mod stored;

use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process,
    sync::atomic::{AtomicU32, Ordering},
};

use anyhow::{Context, Result};

pub use stored::{Stored, VERSION, read};

/// Writes through a temporary file, so a crash never leaves a half-written one.
pub fn replace(path: &Path, data: &[u8]) -> Result<()> {
    let mut staged = stage(path.parent().unwrap_or(Path::new("")))?;
    staged.write_all(data).with_context(|| format!("write {}", staged.path.display()))?;
    staged.publish(path)
}

/// A file written in `dir` under a name no other writer takes, for one whose name is known only once written; it is
/// removed unless published.
pub fn stage(dir: &Path) -> Result<Staged> {
    // Two writers, in this process or another instance, must not share the temporary file.
    static NEXT: AtomicU32 = AtomicU32::new(0);
    fs::create_dir_all(dir).with_context(|| format!("create {}", dir.display()))?;
    let path = dir.join(format!("{}.{}.tmp", process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
    let file = File::create(&path).with_context(|| format!("create {}", path.display()))?;
    Ok(Staged { file: Some(file), path })
}

pub struct Staged {
    file: Option<File>,
    path: PathBuf,
}

impl Staged {
    /// Puts it at `path`, over a file there; on a failure it is removed.
    pub fn publish(mut self, path: &Path) -> Result<()> {
        drop(self.file.take());
        let moved = fs::rename(&self.path, path);
        if moved.is_err() {
            let _ = fs::remove_file(&self.path);
        }
        moved.with_context(|| format!("replace {}", path.display()))
    }
}

impl Write for Staged {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.file.as_mut().map_or(Ok(0), |file| file.write(bytes))
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.as_mut().map_or(Ok(()), File::flush)
    }
}

impl Drop for Staged {
    fn drop(&mut self) {
        if self.file.take().is_some() {
            let _ = fs::remove_file(&self.path);
        }
    }
}

/// Bytes under `path`, a file or a folder (blocking); what cannot be read counts as nothing.
pub fn size(path: &Path) -> u64 {
    let Ok(meta) = fs::symlink_metadata(path) else { return 0 };
    if !meta.is_dir() {
        return meta.len();
    }
    fs::read_dir(path).into_iter().flatten().flatten().map(|entry| size(&entry.path())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_staged_file_is_there_once_published_and_gone_when_dropped() {
        let dir = std::env::temp_dir().join(format!("impulse-disk-stage-{}", process::id()));
        let _ = fs::remove_dir_all(&dir);
        let mut kept = stage(&dir).unwrap();
        kept.write_all(b"kept").unwrap();
        kept.publish(&dir.join("kept")).unwrap();
        stage(&dir).unwrap().write_all(b"dropped").unwrap();
        let names: Vec<_> = fs::read_dir(&dir).unwrap().map(|entry| entry.unwrap().file_name()).collect();
        assert_eq!((names, fs::read(dir.join("kept")).unwrap()), (vec!["kept".into()], b"kept".to_vec()));
        fs::remove_dir_all(dir).unwrap();
    }
}
