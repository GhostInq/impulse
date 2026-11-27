//! What moves between frames: a zoom gliding to its end, the player walking.

use std::{ops::RangeInclusive, time::Instant};

use client::map::heading::{Bearing, JUMP, arc};
use gpui_kit::{Context, Window};
use model::Point;

use super::{
    Stage,
    camera::{Glide, Pixel},
};
use crate::map::markers::Look;

// Seconds the player takes to cover about two thirds of the way to a new fix: there in about 300 ms.
const WALK: f64 = 0.1;
// Closer than this (a fraction of a pixel at any zoom) or this turn is there.
const NEAR: f64 = 1e-7;
const TILT: f64 = 1e-3;

/// The player as drawn: where on the plane, and which way they head.
#[derive(Clone, Copy, PartialEq)]
pub(super) struct Walk {
    pub at: Point,
    pub bearing: Option<Bearing>,
}

impl Stage {
    /// Zooms by `factor` about `pixel`; a zoom on its way goes on from where it would end.
    pub(super) fn zoom(&mut self, factor: f64, pixel: Pixel, cx: &mut Context<Self>) {
        let (Some(map), Some(camera)) = (self.map(cx), self.camera) else { return };
        let spans = self.spans(&self.board.read(cx).saved.view().tile_set_of(&map).1.tiles);
        let from = self.glide.map_or(camera.span, |glide| glide.span);
        self.glide = Some(Glide { span: (from * factor).clamp(*spans.start(), *spans.end()), pixel });
        self.hover(None, cx);
    }

    /// Advances what moves by the time since the last frame; asks for the next frame while anything does.
    pub(super) fn animate(&mut self, spans: &RangeInclusive<f64>, player: Option<Walk>, window: &mut Window) {
        let now = Instant::now();
        // Long gaps (the first frame, a stall) do not jump.
        let seconds = self.clock.map_or(0., |last| (now - last).as_secs_f64().min(0.1));
        let size = self.size();
        if let (Some(glide), Some(camera)) = (self.glide, &mut self.camera)
            && glide.step(camera, seconds, size, spans)
        {
            self.glide = None;
        }
        let walked = self.walk(player, seconds);
        if self.follow {
            self.center_on_player();
        }
        self.clock = (self.glide.is_some() || !walked).then_some(now);
        if self.clock.is_some() {
            window.request_animation_frame();
        }
    }

    /// Eases the drawn player toward `target`; true once there. A teleport is not eased.
    fn walk(&mut self, target: Option<Walk>, seconds: f64) -> bool {
        let (Some(target), Some(shown)) = (target, &mut self.player) else {
            self.player = target;
            return true;
        };
        let part = 1. - (-seconds / WALK).exp();
        let (dx, dy) = (target.at.x - shown.at.x, target.at.y - shown.at.y);
        let length = dx.hypot(dy);
        shown.at = if length < NEAR || length >= JUMP { target.at } else { Point { x: shown.at.x + dx * part, y: shown.at.y + dy * part } };
        shown.bearing = match (shown.bearing, target.bearing) {
            (Some(from), Some(to)) if arc(from.angle, to.angle).abs() >= TILT => {
                Some(Bearing { angle: from.angle + arc(from.angle, to.angle) * part, ..to })
            }
            (_, to) => to,
        };
        *shown == target
    }

    pub(super) fn look(&self, pin: usize, selected: Option<usize>) -> Look {
        match Some(pin) {
            pin if pin == selected => Look::Selected,
            pin if pin == self.shown.hovered => Look::Hovered,
            _ => Look::Plain,
        }
    }
}
