//! Moving the player through the live trainer: to a pin, a plane point or a spot the player stood on.

use client::map::state::UserPin;
use gpui_kit::Context;
use model::{Map, Point};

use super::{Board, plane};
use crate::Load;

impl Board {
    pub fn lift(&self) -> f64 {
        self.lift
    }

    pub fn set_lift(&mut self, lift: f64, cx: &mut Context<Self>) {
        if self.lift != lift {
            self.lift = lift;
            cx.notify();
        }
    }

    pub fn can_teleport(&self, map: &Map) -> bool {
        map.teleport && self.run.as_ref().is_some_and(|run| run.teleport)
    }

    /// Whether the player can be moved to a plane point: the height comes from where the player is.
    pub fn can_teleport_to_plane(&self, map: &Map) -> bool {
        self.player.is_some() && self.can_teleport(map)
    }

    /// Moves the player to `at` on the plane, at the height the player is at now.
    pub fn teleport_to_plane(&mut self, at: Point, cx: &mut Context<Self>) {
        let (Load::Ready(map), Some(player)) = (&self.map, self.player) else { return };
        let Some(frame) = &map.frame else { return };
        let to = frame.to_game(at, frame.height(player));
        self.teleport(to, cx);
    }

    /// Whether the user's pin can be reached; one of the player's position needs no fix of the player.
    pub fn can_teleport_to_mark(&self, map: &Map, mark: &UserPin) -> bool {
        if mark.position.is_some() { self.can_teleport(map) } else { self.can_teleport_to_plane(map) }
    }

    /// A pin of the player's position goes back there exactly, another to its point at the player's height.
    pub fn teleport_to_mark(&mut self, id: &str, cx: &mut Context<Self>) {
        let Some(&UserPin { at, position, .. }) = self.saved.pin(id) else { return };
        match position {
            Some(position) => self.teleport_exact(position, cx),
            None => self.teleport_to_plane(plane(at), cx),
        }
    }

    /// Moves the player to a height the map gives, raised by the lift.
    pub fn teleport(&mut self, to: [f64; 3], cx: &mut Context<Self>) {
        // A map teleports only with a frame.
        let Load::Ready(map) = &self.map else { return };
        let Some(frame) = &map.frame else { return };
        self.teleport_exact(frame.raised(to, self.lift), cx);
    }

    /// Moves the player to where a player stood: no lift.
    fn teleport_exact(&mut self, to: [f64; 3], cx: &mut Context<Self>) {
        let Some(run) = &self.run else { return };
        let session = run.session.clone();
        self.note = None;
        cx.spawn(async move |board, cx| {
            let moved = session.teleport(to).await;
            if let Err(e) = &moved {
                log::warn!("teleport: {e:#}");
            }
            board
                .update(cx, |board, cx| {
                    board.note = moved.err();
                    cx.notify();
                })
                .ok();
        })
        .detach();
        cx.notify();
    }
}
