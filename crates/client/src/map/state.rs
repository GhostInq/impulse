//! What the user set on a map, kept per map on disk: found pins, filters, layers, the view.

use std::path::{Path, PathBuf};

use model::{Map, Ref, Region, TileSet};
// Looked up per drawn pin each frame.
use rustc_hash::FxHashSet as HashSet;
use serde::{Deserialize, Serialize};

use crate::storage;

/// Colors of the user's own pins, new ones taking the next.
pub const PALETTE: [u32; 6] = [0x10BDDB, 0xF5C542, 0xFF6B6B, 0x7BD88F, 0xB28DFF, 0xF2F5EE];

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Saved {
    /// Pin ids.
    pub found: HashSet<String>,
    /// Group ids.
    pub hidden: HashSet<String>,
    /// Region ids.
    #[serde(rename = "hidden_region_ids")]
    pub hidden_regions: HashSet<String>,
    /// Region titles older builds saved under this key, until a copy of the map names them (`resolve`).
    #[serde(rename = "hidden_regions")]
    pub hidden_labels: HashSet<String>,
    pub unfound_only: bool,
    pub teleport_only: bool,
    pub labels: bool,
    pub regions: bool,
    pub clusters: bool,
    /// The minimap groups close pins too, while `clusters`.
    pub mini_clusters: bool,
    /// How far above or below the player near pins are.
    pub rises: bool,
    /// Camera center and span.
    pub view: Option<(f64, f64, f64)>,
    pub marks: Vec<Mark>,
    pub marks_shown: bool,
    /// The chosen tile set's id; `None` or one the map no longer has: its first.
    pub tile_set: Option<String>,
}

/// A pin of the user's own.
#[derive(Clone, Serialize, Deserialize)]
pub struct Mark {
    /// A plane point.
    pub at: (f64, f64),
    /// The game position of the player it was put at; `None`: a point on the map, of no height.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<[f64; 3]>,
    pub name: String,
    #[serde(default)]
    pub note: String,
    pub color: u32,
}

impl Default for Saved {
    fn default() -> Self {
        Self {
            found: HashSet::default(),
            hidden: HashSet::default(),
            hidden_regions: HashSet::default(),
            hidden_labels: HashSet::default(),
            unfound_only: false,
            teleport_only: false,
            labels: true,
            regions: true,
            clusters: true,
            mini_clusters: false,
            rises: true,
            view: None,
            marks: Vec::new(),
            marks_shown: true,
            tile_set: None,
        }
    }
}

impl Saved {
    /// A map seen for the first time hides what its source hides by default.
    pub fn first(map: &Map) -> Self {
        let hidden = map.categories.iter().filter(|category| !category.shown);
        Self { hidden: hidden.flat_map(|category| category.groups.iter().map(|group| group.id.clone())).collect(), ..Self::default() }
    }

    /// Uses the first tile set if there is no saved choice or it no longer exists.
    pub fn tile_set_of<'a>(&self, map: &'a Map) -> (usize, &'a TileSet) {
        map.tile_set(self.tile_set.as_deref())
    }

    /// A title an older build saved hides the regions of that name; one no region has yet waits for another copy.
    pub fn resolve(&mut self, regions: &[Region]) {
        let hidden = &mut self.hidden_regions;
        self.hidden_labels.retain(|label| {
            let mut named = false;
            for region in regions.iter().filter(|region| region.name == *label) {
                hidden.insert(region.id.clone());
                named = true;
            }
            !named
        });
    }
}

/// The copy on disk, else the v1 one, which `storage::migrate` put under the source that wrote it (blocking).
pub fn load(dir: &Path, map: &Ref) -> Option<Saved> {
    storage::read(&file(dir, map)).or_else(|| storage::read(&v1(dir, map)?))
}

/// Blocking.
pub fn store(dir: &Path, map: &Ref, saved: &Saved) {
    storage::write(&file(dir, map), saved);
}

fn file(dir: &Path, map: &Ref) -> PathBuf {
    storage::maps_of(dir, map.source(), false).join(format!("{}.json", storage::name(map.id())))
}

// v1 named a file by the id's letters and digits only: another id's file could have the same name.
fn v1(dir: &Path, map: &Ref) -> Option<PathBuf> {
    let id = map.id();
    id.bytes().all(|byte| byte.is_ascii_alphanumeric()).then(|| storage::maps_of(dir, map.source(), true).join(format!("{id}.json")))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn a_saved_region_title_hides_every_region_of_that_name() {
        let region = |id: &str, name: &str| Region {
            id: id.to_owned(),
            name: name.to_owned(),
            outlines: Vec::new(),
            color: 0,
            thickness: 1.,
            zoom: (0., 20.),
        };
        let regions = [region("a", "Camp"), region("b", "Camp"), region("c", "Keep")];
        let hidden = |ids: &[&str]| ids.iter().map(|id| (*id).to_owned()).collect::<HashSet<String>>();
        // An older build's file: titles under `hidden_regions`.
        let mut saved: Saved =
            serde_json::from_str(r#"{"hidden_regions": ["Camp", "Gone", "Keep"], "hidden_region_ids": ["Camp"]}"#).unwrap();
        saved.resolve(&regions);
        assert_eq!((saved.hidden_regions, saved.hidden_labels), (hidden(&["a", "b", "c", "Camp"]), hidden(&["Gone"])));
    }

    #[test]
    fn state_survives_a_round_trip_and_older_files() {
        let dir = std::env::temp_dir().join(format!("impulse-map-state-{}", std::process::id()));
        let mark = Mark { at: (0.5, 0.5), position: Some([1., 2., 3.]), name: "Camp".to_owned(), note: String::new(), color: 0xFF0000 };
        let saved = Saved {
            found: HashSet::from_iter(["p1".to_owned()]),
            clusters: false,
            view: Some((0.5, 0.25, 4096.)),
            marks: vec![mark],
            tile_set: Some("drawn".to_owned()),
            ..Saved::default()
        };
        let map = |id: &str| Ref::parse(&format!("s:{id}")).unwrap();
        store(&dir, &map("../aB"), &saved);
        assert!(load(&dir, &map("../ab")).is_none());
        let back = load(&dir, &map("../aB")).unwrap();
        assert_eq!((back.found, back.clusters, back.view, back.labels), (saved.found, false, saved.view, true));
        assert_eq!((back.marks[0].name.as_str(), back.marks[0].color, back.marks[0].position), ("Camp", 0xFF0000, Some([1., 2., 3.])));
        assert_eq!(back.tile_set.as_deref(), Some("drawn"));
        fs::create_dir_all(dir.join("s").join("v1")).unwrap();
        fs::write(dir.join("s").join("v1").join("OldMap.json"), br#"{"found":["x"]}"#).unwrap();
        let old = load(&dir, &map("OldMap")).unwrap();
        assert!(
            old.found.contains("x") && old.clusters && old.regions && old.marks_shown && old.tile_set.is_none() && old.marks.is_empty()
        );
        // v1 kept letters and digits only: the file may be another map's.
        assert!(load(&dir, &map("Old-Map")).is_none());
        store(&dir, &map("OldMap"), &Saved::default());
        assert!(load(&dir, &map("OldMap")).unwrap().found.is_empty());
        let _ = fs::remove_dir_all(&dir);
    }
}
