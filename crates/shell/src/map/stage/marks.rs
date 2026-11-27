//! The user's own pins on the stage: their order, look and drawing over the map's.

use client::map::state::{Saved, UserPin};
use gpui_kit::{App, Window};

use super::{Stage, camera::Camera};
use crate::map::{
    board::plane,
    markers::{Look, Marker},
};

impl Stage {
    pub(super) fn paint_marks(&mut self, saved: &Saved, open: Option<&str>, camera: &Camera, window: &mut Window, cx: &App) {
        let area = self.size();
        for mark in mark_order(saved, open) {
            let look = self.mark_look(&mark.id, open);
            let marker = Marker { group: None, fill: mark.color, look, found: false, level: None, picture: false };
            self.paint_look(marker, None, camera.to_view(plane(mark.at), area), window, cx);
        }
    }

    pub(super) fn mark_look(&self, id: &str, open: Option<&str>) -> Look {
        match Some(id) {
            id if id == open => Look::Selected,
            id if id == self.shown.hovered_mark.as_deref() => Look::Hovered,
            _ => Look::Plain,
        }
    }
}

/// The user's own pins as drawn, bottom to top: the oldest first, the open one last.
pub(super) fn mark_order<'a>(saved: &'a Saved, open: Option<&'a str>) -> impl DoubleEndedIterator<Item = &'a UserPin> {
    let shown = saved.view().pins_shown();
    saved.pins().filter(move |pin| shown && Some(&*pin.id) != open).chain(open.and_then(|id| saved.pin(id)))
}
