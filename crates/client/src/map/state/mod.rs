//! What the user did on a map, a folder per map: `view.json` (filters, layers, the camera: this device's),
//! `progress.json` (found pins) and `presets/<id>.json` (the user's own pins). Records carry ids and change stamps
//! and a delete leaves a tombstone, so two copies can merge record by record.

mod legacy;

use std::{
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use model::{Frame, Map, Pin, Point, Ref, Region, TileSet};
// Looked up per drawn pin each frame.
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use serde::{Deserialize, Serialize};

use self::legacy::Parts;
use crate::storage::{self, Paths, Stored};

/// Colors of the user's own pins, new ones taking the next.
pub const PALETTE: [u32; 6] = [0x10BDDB, 0xF5C542, 0xFF6B6B, 0x7BD88F, 0xB28DFF, 0xF2F5EE];
/// The preset new pins go to.
const MINE: &str = "mine";

/// A map's state as its board holds it; changes wait for `flush`. The default is kept in memory only.
#[derive(Clone, Default)]
pub struct Saved {
    view: Stored<View>,
    progress: Stored<Progress>,
    mine: Stored<Preset>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct View {
    /// Group ids.
    #[serde(rename = "hidden")]
    pub hidden_groups: HashSet<String>,
    /// Region ids.
    #[serde(rename = "hidden_region_ids")]
    pub hidden_regions: HashSet<String>,
    /// Region titles older builds saved under this key, until a copy of the map names them (`resolve`).
    #[serde(rename = "hidden_regions")]
    pub hidden_labels: HashSet<String>,
    /// Presets of the user's pins not shown, by id.
    pub hidden_presets: HashSet<String>,
    pub unfound_only: bool,
    pub teleport_only: bool,
    pub labels: bool,
    pub regions: bool,
    pub clusters: bool,
    /// The minimap groups close pins too, while `clusters`.
    pub mini_clusters: bool,
    /// How far above or below the player near pins are.
    pub rises: bool,
    /// Center and span.
    #[serde(rename = "view")]
    pub camera: Option<(f64, f64, f64)>,
    /// The chosen tile set's id; `None` or one the map no longer has: its first.
    pub tile_set: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct Progress {
    /// The map's pins by id.
    found: HashMap<String, Flag>,
    /// The map's pins the user hid (gone from the game, or moved to a copy), by id.
    hidden: HashMap<String, Flag>,
}

/// A yes or no that merges: the later `updated` wins.
#[derive(Clone, Copy, Default, Serialize, Deserialize)]
struct Flag {
    on: bool,
    updated: u64,
}

/// A set of the user's own pins; its file's name is its id.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct Preset {
    pins: Vec<UserPin>,
}

/// A change of a user's pin; its id, stamps and tombstone stay `Saved`'s.
pub enum PinEdit {
    Name(String),
    Description(String),
    Color(u32),
}

/// A pin of the user's own. One with a game `position` is there and `at` shows it; one without is at `at`.
#[derive(Clone, Serialize, Deserialize)]
pub struct UserPin {
    pub id: Rc<str>,
    /// A plane point.
    pub at: (f64, f64),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<[f64; 3]>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub color: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<Origin>,
    /// `stamp`s: the order pins show in, and which copy of a pin wins.
    #[serde(default)]
    pub created: u64,
    #[serde(default)]
    pub updated: u64,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub deleted: bool,
}

/// A group's pins as counted, and those found.
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Tally {
    pub found: usize,
    pub total: usize,
}

impl<'a> std::iter::Sum<&'a Tally> for Tally {
    fn sum<I: Iterator<Item = &'a Tally>>(tallies: I) -> Self {
        tallies.fold(Self::default(), |sum, tally| Self { found: sum.found + tally.found, total: sum.total + tally.total })
    }
}

/// Where a user's pin came from, when not made by hand.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Origin {
    /// A copy of the map's pin of this id, which it stands in for: that one is hidden and found through this one.
    Clone(String),
}

impl UserPin {
    fn new(at: Point, position: Option<[f64; 3]>, color: u32) -> Self {
        let now = stamp(0);
        let id = storage::random_hex::<16>().into();
        let (name, description) = (String::new(), String::new());
        Self { id, at: (at.x, at.y), position, name, description, color, origin: None, created: now, updated: now, deleted: false }
    }

    /// The map's pin this one stands in for.
    pub fn stands_for(&self) -> Option<&str> {
        match &self.origin {
            Some(Origin::Clone(id)) => Some(id),
            None => None,
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            hidden_groups: HashSet::default(),
            hidden_regions: HashSet::default(),
            hidden_labels: HashSet::default(),
            hidden_presets: HashSet::default(),
            unfound_only: false,
            teleport_only: false,
            labels: true,
            regions: true,
            clusters: true,
            mini_clusters: false,
            rises: true,
            camera: None,
            tile_set: None,
        }
    }
}

impl View {
    /// A map seen for the first time hides what its source hides by default.
    fn first(map: &Map) -> Self {
        let hidden = map.categories.iter().filter(|category| !category.shown);
        Self {
            hidden_groups: hidden.flat_map(|category| category.groups.iter().map(|group| group.id.clone())).collect(),
            ..Self::default()
        }
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

    pub fn pins_shown(&self) -> bool {
        !self.hidden_presets.contains(MINE)
    }

    pub fn toggle_pins(&mut self) {
        if !self.hidden_presets.remove(MINE) {
            self.hidden_presets.insert(MINE.to_owned());
        }
    }
}

impl Saved {
    /// Map `id`'s state; one without any hides what its source hides by default. The first load of a map takes over
    /// the file older builds kept (blocking, tiny).
    pub fn load(paths: &Paths, id: &Ref, map: &Map) -> Self {
        let files = paths.map_files(id);
        if !files.dir().is_dir() {
            let failed = match legacy::read(&paths.map_state(), id) {
                Ok(None) => None,
                Ok(Some(parts)) => legacy::publish(&parts, &files).err().map(|e| (e, parts)),
                Err(e) => Some((e.into(), Parts { view: View::first(map), ..Parts::default() })),
            };
            // Kept in memory only: the older file stays for the next load to try again, nothing written over it.
            if let Some((e, parts)) = failed {
                log::warn!("map state not taken over: {e:#}");
                return Self::memory(parts);
            }
        }
        let first = !files.view().exists();
        let mut view = Stored::<View>::load(files.view());
        if first {
            *view.get_mut() = View::first(map);
        }
        Self { view, progress: Stored::load(files.progress()), mine: Stored::load(files.preset(MINE)) }
    }

    fn memory(Parts { view, progress, mine }: Parts) -> Self {
        let mut saved = Self::default();
        *saved.view.get_mut() = view;
        *saved.progress.get_mut() = progress;
        *saved.mine.get_mut() = mine;
        saved
    }

    /// Writes what changed (blocking, tiny).
    pub fn flush(&mut self) {
        self.view.flush();
        self.progress.flush();
        self.mine.flush();
    }

    pub fn view(&self) -> &View {
        &self.view
    }

    pub fn view_mut(&mut self) -> &mut View {
        self.view.get_mut()
    }

    /// Whether the map's pin `id` is found.
    pub fn found(&self, id: &str) -> bool {
        self.progress.found.get(id).is_some_and(|flag| flag.on)
    }

    pub fn toggle_found(&mut self, id: &str) {
        flip(&mut self.progress.get_mut().found, id);
    }

    /// Whether the map's pin `id` is hidden.
    pub fn hidden(&self, id: &str) -> bool {
        self.progress.hidden.get(id).is_some_and(|flag| flag.on)
    }

    pub fn toggle_hidden(&mut self, id: &str) {
        flip(&mut self.progress.get_mut().hidden, id);
    }

    /// A copy's FOUND, its original's; `None` for a pin made by hand.
    pub fn found_of(&self, pin: &UserPin) -> Option<bool> {
        pin.stands_for().map(|id| self.found(id))
    }

    /// `map`'s tallies by category and group. A hidden pin counts only while a copy stands in for it: its place is
    /// gone otherwise.
    pub fn tally(&self, map: &Map) -> Vec<Vec<Tally>> {
        let copied: HashSet<&str> = self.pins().filter_map(UserPin::stands_for).collect();
        let mut tally: Vec<Vec<Tally>> = map.categories.iter().map(|category| vec![Tally::default(); category.groups.len()]).collect();
        for pin in map.pins.iter().filter(|pin| !self.hidden(&pin.id) || copied.contains(pin.id.as_str())) {
            let group = &mut tally[pin.group.0][pin.group.1];
            group.total += 1;
            group.found += usize::from(self.found(&pin.id));
        }
        tally
    }

    /// The user's pins, oldest first.
    pub fn pins(&self) -> impl DoubleEndedIterator<Item = &UserPin> {
        self.mine.pins.iter().filter(|pin| !pin.deleted)
    }

    pub fn pin(&self, id: &str) -> Option<&UserPin> {
        self.pins().find(|pin| *pin.id == *id)
    }

    /// A pin of the user's at `at`, of the player's game `position` if given, shown with the rest; its id.
    pub fn add_pin(&mut self, at: Point, position: Option<[f64; 3]>) -> Rc<str> {
        let color = PALETTE[self.pins().count() % PALETTE.len()];
        self.push(UserPin::new(at, position, color))
    }

    /// The map's `pin` as one of the user's, standing in for it: the original hides. Its id.
    pub fn clone_pin(&mut self, pin: &Pin, color: u32) -> Rc<str> {
        let (name, description, origin) = (pin.name.clone(), pin.description.clone(), Some(Origin::Clone(pin.id.clone())));
        if !self.hidden(&pin.id) {
            self.toggle_hidden(&pin.id);
        }
        self.push(UserPin { name, description, origin, ..UserPin::new(pin.at, pin.position, color) })
    }

    fn push(&mut self, pin: UserPin) -> Rc<str> {
        let id = pin.id.clone();
        self.mine.get_mut().pins.push(pin);
        if !self.view.pins_shown() {
            self.view.get_mut().toggle_pins();
        }
        id
    }

    /// Moves pin `id` to `at` on the plane: its game position keeps its height, and needs `frame` for the rest.
    pub fn move_pin(&mut self, id: &str, at: Point, frame: Option<&Frame>) {
        self.touch(id, |pin| {
            pin.at = (at.x, at.y);
            pin.position = pin.position.zip(frame).map(|(position, frame)| frame.to_game(at, frame.height(position)));
        });
    }

    pub fn edit_pin(&mut self, id: &str, edit: PinEdit) {
        self.touch(id, |pin| match edit {
            PinEdit::Name(name) => pin.name = name,
            PinEdit::Description(text) => pin.description = text,
            PinEdit::Color(color) => pin.color = color,
        });
    }

    pub fn delete_pin(&mut self, id: &str) {
        self.touch(id, |pin| pin.deleted = true);
    }

    fn touch(&mut self, id: &str, change: impl FnOnce(&mut UserPin)) {
        if let Some(pin) = self.mine.get_mut().pins.iter_mut().find(|pin| *pin.id == *id && !pin.deleted) {
            change(pin);
            pin.updated = stamp(pin.updated);
        }
    }
}

fn flip(flags: &mut HashMap<String, Flag>, id: &str) {
    let flag = flags.entry(id.to_owned()).or_default();
    (flag.on, flag.updated) = (!flag.on, stamp(flag.updated));
}

/// Now in milliseconds, and past `previous`: an edit wins over what it changed even when the clock went back.
fn stamp(previous: u64) -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |since| since.as_millis() as u64);
    now.max(previous + 1)
}

#[cfg(test)]
mod tests;
