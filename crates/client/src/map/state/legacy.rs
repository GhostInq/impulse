//! The one file per map older builds kept, taken over into the map's folder at its first load.

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use model::Ref;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{Flag, MINE, Preset, Progress, UserPin, View};
use crate::storage::{self, MapFiles, Stored};

/// An older file: `View`'s fields under their keys, then the rest.
#[derive(Deserialize)]
struct Old {
    #[serde(flatten)]
    view: View,
    #[serde(default)]
    found: Vec<String>,
    #[serde(default)]
    marks: Vec<Mark>,
    #[serde(default = "shown")]
    marks_shown: bool,
}

#[derive(Serialize, Deserialize)]
struct Mark {
    at: (f64, f64),
    #[serde(default)]
    position: Option<[f64; 3]>,
    #[serde(default)]
    name: String,
    #[serde(default)]
    note: String,
    color: u32,
}

fn shown() -> bool {
    true
}

/// What a map's folder holds.
#[derive(Default)]
pub(super) struct Parts {
    pub view: View,
    pub progress: Progress,
    pub mine: Preset,
}

/// Map `id`'s older file as a folder's parts: its own, else the v1 one, which `storage::migrate` put under the source
/// that wrote it; one that is not map state is passed over. An error: one that is there could not be read (blocking).
pub(super) fn read(map_state: &Path, id: &Ref) -> io::Result<Option<Parts>> {
    for file in [Some(file(map_state, id)), v1(map_state, id)].into_iter().flatten() {
        match fs::read(&file) {
            Ok(bytes) => {
                if let Ok(old) = serde_json::from_slice(&bytes) {
                    return Ok(Some(parts(old)));
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    Ok(None)
}

/// Writes `parts` as the map's folder, whole or not at all: a staged folder takes its name last.
pub(super) fn publish(parts: &Parts, files: &MapFiles) -> Result<()> {
    let staged = files.staged();
    // A crash may have left one: what stays in it would be published too.
    match fs::remove_dir_all(staged.dir()) {
        Err(e) if e.kind() != io::ErrorKind::NotFound => return Err(e).with_context(|| format!("clear {}", staged.dir().display())),
        _ => {}
    }
    Stored::<View>::load(staged.view()).try_edit(|view| *view = parts.view.clone())?;
    Stored::<Progress>::load(staged.progress()).try_edit(|progress| *progress = parts.progress.clone())?;
    if !parts.mine.pins.is_empty() {
        Stored::<Preset>::load(staged.preset(MINE)).try_edit(|mine| *mine = parts.mine.clone())?;
    }
    fs::rename(staged.dir(), files.dir()).with_context(|| format!("publish {}", files.dir().display()))
}

fn parts(Old { mut view, found, marks, marks_shown }: Old) -> Parts {
    if !marks_shown {
        view.toggle_pins();
    }
    // Stamped 0: any edit since wins over them.
    let found = found.into_iter().map(|id| (id, Flag { on: true, updated: 0 })).collect();
    Parts {
        view,
        progress: Progress { found, ..Progress::default() },
        mine: Preset { pins: marks.into_iter().enumerate().map(pin).collect() },
    }
}

/// Its id comes from what it holds, so one older file gives the same ids wherever it is taken over; `created`
/// keeps the list's order.
fn pin((index, mark): (usize, Mark)) -> UserPin {
    let digest = Sha256::digest(serde_json::to_vec(&(index, &mark)).unwrap_or_default());
    let Mark { at, position, name, note, color } = mark;
    let id = storage::hex(&digest[..16]).into();
    UserPin { id, at, position, name, description: note, color, origin: None, created: index as u64, updated: 0, deleted: false }
}

fn file(map_state: &Path, id: &Ref) -> PathBuf {
    storage::maps_of(map_state, id.source(), false).join(format!("{}.json", storage::name(id.id())))
}

// v1 named a file by the id's letters and digits only: another id's file could have the same name.
fn v1(map_state: &Path, map: &Ref) -> Option<PathBuf> {
    let id = map.id();
    id.bytes().all(|byte| byte.is_ascii_alphanumeric()).then(|| storage::maps_of(map_state, map.source(), true).join(format!("{id}.json")))
}
