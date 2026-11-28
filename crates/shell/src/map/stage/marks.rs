//! The user's own pins on the stage: their order, look and drawing over the map's.

use std::rc::Rc;

use client::map::state::{Saved, UserPin};
use gpui_kit::{App, Window};
use model::Point;

use super::{
    Moving, Stage,
    camera::{Camera, Pixel},
};
use crate::map::{
    board::plane,
    markers::{Look, Marker},
};

impl Stage {
    /// A copy of a map's pin shows that one's FOUND.
    pub(super) fn paint_marks(
        &mut self,
        saved: &Saved,
        open: Option<&str>,
        moving: Option<&Held>,
        camera: &Camera,
        window: &mut Window,
        cx: &App,
    ) {
        let area = self.size();
        for mark in mark_order(saved, open) {
            let (look, found) = (self.mark_look(&mark.id, open), saved.found_of(mark) == Some(true));
            let marker = Marker { group: None, fill: mark.color, look, found, level: None, picture: false };
            self.paint_look(marker, None, camera.to_view(drawn_at(mark, moving), area), window, cx);
        }
    }

    /// The user's pin on its way, and where it is now.
    pub(super) fn moving(&self, camera: &Camera) -> Option<Held> {
        let moving = self.drag.as_ref()?.moving.as_ref()?;
        Some((moving.id.clone(), camera.to_plane(self.tip(moving), self.size())))
    }

    pub(super) fn tip(&self, moving: &Moving) -> Pixel {
        let cursor = self.pixel(moving.cursor);
        (cursor.0 + moving.grab.0, cursor.1 + moving.grab.1)
    }

    pub(super) fn mark_look(&self, id: &str, open: Option<&str>) -> Look {
        match Some(id) {
            id if id == open => Look::Selected,
            id if id == self.shown.hovered_mark.as_deref() => Look::Hovered,
            _ => Look::Plain,
        }
    }
}

pub(super) type Held = (Rc<str>, Point);

/// Where `mark` is drawn: under the cursor while it moves.
pub(super) fn drawn_at(mark: &UserPin, moving: Option<&Held>) -> Point {
    match moving {
        Some((id, at)) if *id == mark.id => *at,
        _ => plane(mark.at),
    }
}

/// The user's own pins as drawn, bottom to top: the oldest first, the open one last.
pub(super) fn mark_order<'a>(saved: &'a Saved, open: Option<&'a str>) -> impl DoubleEndedIterator<Item = &'a UserPin> {
    let shown = saved.view().pins_shown();
    saved.pins().filter(move |pin| shown && Some(&*pin.id) != open).chain(open.and_then(|id| saved.pin(id)))
}
