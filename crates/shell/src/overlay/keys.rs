//! `overlay.json`: the overlay's and the extra hotkeys' keys.

use keyhook::{Key, Modifiers};
use serde::{Deserialize, Serialize};

use client::storage::{Paths, Stored};

const CTRL_SHIFT: Modifiers = Modifiers { control: true, shift: true, alt: false, win: false };
/// The overlay's own key when none is saved.
pub(super) const DEFAULT: Key = Key::new(CTRL_SHIFT, b'O');

/// The hotkeys beside the overlay's own.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Slot {
    Minimap,
    ZoomIn,
    ZoomOut,
    PinHere,
}

impl Slot {
    pub const ALL: [Slot; 4] = [Slot::Minimap, Slot::ZoomIn, Slot::ZoomOut, Slot::PinHere];

    pub fn default_key(self) -> Key {
        match self {
            Slot::Minimap => Key::new(CTRL_SHIFT, b'M'),
            // Plus and Minus of the main block.
            Slot::ZoomIn => Key::new(CTRL_SHIFT, 0xBB),
            Slot::ZoomOut => Key::new(CTRL_SHIFT, 0xBD),
            Slot::PinHere => Key::new(CTRL_SHIFT, b'P'),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
struct Saved {
    /// Virtual keys; missing or unusable means the default.
    hotkey: Option<Vec<u8>>,
    /// The same for each extra; empty means none.
    minimap: Option<Vec<u8>>,
    zoom_in: Option<Vec<u8>>,
    zoom_out: Option<Vec<u8>>,
    pin_here: Option<Vec<u8>>,
}

impl Saved {
    fn extra(&self, slot: Slot) -> &Option<Vec<u8>> {
        match slot {
            Slot::Minimap => &self.minimap,
            Slot::ZoomIn => &self.zoom_in,
            Slot::ZoomOut => &self.zoom_out,
            Slot::PinHere => &self.pin_here,
        }
    }

    fn extra_mut(&mut self, slot: Slot) -> &mut Option<Vec<u8>> {
        match slot {
            Slot::Minimap => &mut self.minimap,
            Slot::ZoomIn => &mut self.zoom_in,
            Slot::ZoomOut => &mut self.zoom_out,
            Slot::PinHere => &mut self.pin_here,
        }
    }

    fn overlay_key(&self) -> Key {
        self.hotkey.as_ref().and_then(|vks| Key::from_vks(vks.iter().copied())).unwrap_or(DEFAULT)
    }

    /// What the user saved for `slot`, when it is a usable key.
    fn saved_key(&self, slot: Slot) -> Option<Key> {
        self.extra(slot).as_ref().and_then(|vks| Key::from_vks(vks.iter().copied()))
    }

    fn key(&self, slot: Slot) -> Option<Key> {
        match self.extra(slot) {
            Some(vks) if vks.is_empty() => None,
            _ => self.saved_key(slot).or_else(|| self.default_key(slot)),
        }
    }

    /// A default never takes a key the user already holds: an upgrade must not bind one key twice.
    fn default_key(&self, slot: Slot) -> Option<Key> {
        let key = slot.default_key();
        let held = key == self.overlay_key() || Slot::ALL.into_iter().any(|other| other != slot && self.saved_key(other) == Some(key));
        (!held).then_some(key)
    }
}

pub(super) struct Keys {
    saved: Stored<Saved>,
}

impl Keys {
    pub fn load(paths: &Paths) -> Self {
        Self { saved: Stored::load(paths.overlay()) }
    }

    pub fn hotkey(&self) -> Key {
        self.saved.overlay_key()
    }

    pub fn set_hotkey(&mut self, key: Key) {
        self.saved.edit(|saved| saved.hotkey = Some(key.vks().collect()));
    }

    pub fn key(&self, slot: Slot) -> Option<Key> {
        self.saved.key(slot)
    }

    pub fn set_key(&mut self, slot: Slot, key: Option<Key>) {
        self.saved.edit(|saved| *saved.extra_mut(slot) = Some(key.map_or_else(Vec::new, |key| key.vks().collect())));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn saved(json: &str) -> Saved {
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn a_fresh_file_gives_each_slot_its_default() {
        let saved = saved("{}");
        assert_eq!(Slot::ALL.map(|slot| saved.key(slot)), Slot::ALL.map(|slot| Some(slot.default_key())));
    }

    #[test]
    fn an_empty_save_means_no_key() {
        assert_eq!(saved(r#"{"zoom_in":[]}"#).key(Slot::ZoomIn), None);
    }

    #[test]
    fn a_default_gives_way_to_a_key_the_user_already_holds() {
        // Ctrl, Shift and "=": the zoom-in default, saved earlier as the minimap's key.
        let held = saved(r#"{"minimap":[162,160,187]}"#);
        assert_eq!(held.key(Slot::Minimap), Some(Slot::ZoomIn.default_key()));
        assert_eq!(held.key(Slot::ZoomIn), None);
        assert_eq!(held.key(Slot::ZoomOut), Some(Slot::ZoomOut.default_key()));
        let overlay = saved(r#"{"hotkey":[162,160,189]}"#);
        assert_eq!(overlay.key(Slot::ZoomOut), None);
    }
}
