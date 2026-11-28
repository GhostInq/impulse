//! The app's own files in the data folder: `Paths` names them, `Stored` keeps one of them typed.
//! Blocking, in place and in order: the files are tiny and change on a click.

mod keys;
mod migrate;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub use disk::{Stored, VERSION, read};
use model::{Ref, SourceId};

pub use keys::{by_ref, name};
pub(crate) use keys::{hex, random_hex};
pub use migrate::migrate;

/// The data folder: the only place that names the app's own files and folders.
#[derive(Clone)]
pub struct Paths(Arc<Path>);

macro_rules! entries {
    ($($name:ident: $entry:literal),* $(,)?) => {
        impl Paths {
            $(pub fn $name(&self) -> PathBuf {
                self.0.join($entry)
            })*
        }
    };
}

entries! {
    settings: "settings.json",
    hotkeys: "hotkeys.json",
    games: "games.json",
    picked: "picked.json",
    installs: "installs.json",
    docks: "docks.json",
    overlay: "overlay.json",
    minimap: "minimap.json",
    resources: "resources.json",
    remote_devices: "remote.json",
    map_links: "map-links.json",
    trainer_links: "trainer-links.json",
    // Older builds' state per map, taken over by `map::state` into `map_data`.
    map_state: "map-state",
    map_data: "map-data",
    // The user's pins' pictures and clips, of every map (`map::media`).
    map_media: "map-media",
    pictures: "art",
    tiles: "tiles",
    videos: "videos",
    plugins: "plugins",
    plugin_lock: "plugins.lock",
    plugin_cache: "plugin-cache",
    update: "update.json",
    window: "window.json",
}

impl Paths {
    pub fn new(root: PathBuf) -> Self {
        Self(root.into())
    }

    /// Map `map`'s folder of the user's state, under `map_data`.
    pub fn map_files(&self, map: &Ref) -> MapFiles {
        MapFiles(self.map_data().join(name(map.source().as_str())).join(name(map.id())))
    }
}

/// A map's folder: `view.json`, `progress.json`, `presets/<id>.json`.
pub struct MapFiles(PathBuf);

impl MapFiles {
    pub fn dir(&self) -> &Path {
        &self.0
    }

    /// Its sibling, where the folder is made before it takes the name.
    pub fn staged(&self) -> Self {
        Self(self.0.with_extension("staged"))
    }

    pub fn view(&self) -> PathBuf {
        self.0.join("view.json")
    }

    pub fn progress(&self) -> PathBuf {
        self.0.join("progress.json")
    }

    pub fn preset(&self, id: &str) -> PathBuf {
        self.0.join("presets").join(format!("{}.json", name(id)))
    }
}

/// A source's folder of map states, under `map_state`; `v1`: the v1 files `migrate` moved there.
pub fn maps_of(map_state: &Path, source: &SourceId, v1: bool) -> PathBuf {
    let dir = map_state.join(name(source.as_str()));
    if v1 { dir.join("v1") } else { dir }
}
