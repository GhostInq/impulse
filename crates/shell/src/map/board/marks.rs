//! The user's own pins on the board, by id: added, opened, changed, moved, deleted; and the map's pins they stand in for.

use std::rc::Rc;

use client::map::state::{PinEdit, UserPin};
use gpui_kit::Context;
use model::Point;

use super::{Board, BoardEvent, plane};
use crate::Load;

impl Board {
    /// The pin of the user's whose card is open.
    pub fn open_mark(&self) -> Option<&UserPin> {
        self.saved.pin(self.mark.as_deref()?)
    }

    /// Opens a mark's card; `bring` also centers the view on it.
    pub fn select_mark(&mut self, id: Rc<str>, bring: bool, cx: &mut Context<Self>) {
        let Some(mark) = self.saved.pin(&id) else { return };
        if bring {
            cx.emit(BoardEvent::Bring { to: plane(mark.at), close: false });
        }
        (self.mark, self.selected) = (Some(id), None);
        cx.emit(BoardEvent::Selected);
        cx.notify();
    }

    /// One of the user's pins where the player stands, which keeps the player's game position.
    pub fn mark_player(&mut self, cx: &mut Context<Self>) {
        let (Load::Ready(map), Some(player)) = (&self.map, self.player.filter(|_| self.here)) else { return };
        let Some(at) = map.frame.as_ref().map(|frame| frame.to_map(player)) else { return };
        self.add_mark(at, Some(player), cx);
    }

    pub fn add_mark(&mut self, at: Point, position: Option<[f64; 3]>, cx: &mut Context<Self>) {
        let id = self.saved_mut().add_pin(at, position);
        self.select_mark(id, false, cx);
        self.save();
        cx.emit(BoardEvent::MarkAdded);
    }

    /// A copy's original stays hidden, out of the counts.
    pub fn delete_mark(&mut self, id: &str, cx: &mut Context<Self>) {
        self.saved_mut().delete_pin(id);
        self.mark = None;
        self.recount();
        self.save();
        cx.emit(BoardEvent::Selected);
        cx.notify();
    }

    /// HIDE: the map's pin `pin` is gone from the game, and its card closes; again, it shows.
    pub fn toggle_hidden(&mut self, pin: usize, cx: &mut Context<Self>) {
        let Load::Ready(map) = &self.map else { return };
        let id = map.pins[pin].id.clone();
        self.saved_mut().toggle_hidden(&id);
        self.recount();
        if self.saved.hidden(&id) && self.selected == Some(pin) {
            self.deselect(cx);
        }
        self.filtered(cx);
    }

    /// MOVE: the map's pin `pin` as one of the user's, opened, which stands in for the original, now hidden.
    pub fn clone_pin(&mut self, pin: usize, cx: &mut Context<Self>) {
        let Load::Ready(map) = &self.map else { return };
        let map = map.clone();
        let pin = &map.pins[pin];
        let id = self.saved_mut().clone_pin(pin, map.categories[pin.group.0].color);
        self.recount();
        self.select_mark(id, false, cx);
        self.filtered(cx);
    }

    /// Moves the user's pin `id` to `at`; a game position keeps its height.
    pub fn move_mark(&mut self, id: &str, at: Point, cx: &mut Context<Self>) {
        let Load::Ready(map) = &self.map else { return };
        let map = map.clone();
        self.saved_mut().move_pin(id, at, map.frame.as_ref());
        self.save();
        cx.notify();
    }

    /// A copy's FOUND marks the map's pin it stands in for.
    pub fn toggle_mark_found(&mut self, id: &str, cx: &mut Context<Self>) {
        let Some(original) = self.saved.pin(id).and_then(UserPin::stands_for).map(str::to_owned) else { return };
        self.toggle_found_id(&original, cx);
    }

    /// Changes the open mark; the caller saves once the edit is done.
    pub fn edit_mark(&mut self, edit: PinEdit, cx: &mut Context<Self>) {
        let Some(id) = self.mark.clone() else { return };
        self.saved_mut().edit_pin(&id, edit);
        cx.notify();
    }
}
