use std::{rc::Rc, time::Duration};

use gpui_kit::{
    App, Context, Div, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ScrollEvent, Stateful, Task, Window, prelude::*, px,
};
use model::{Map, Point};

use super::{
    Drag, Moving, Stage,
    camera::{Camera, Pixel},
    marks::{drawn_at, mark_order},
    pins,
};
use crate::map::keys::{PanDown, PanLeft, PanRight, PanUp, ZoomIn, ZoomOut};

// Zoom levels per wheel line.
const WHEEL: f64 = 0.12;
// A press that moves less than this is a click.
const CLICK_SLACK: f32 = 4.;
// How much closer a click on a group of pins, a double click or a key brings the view.
const GROUP_ZOOM: f64 = 2.5;
pub(super) const KEY_ZOOM: f64 = 2.;
// View pixels an arrow key moves the map.
const KEY_PAN: f64 = 120.;
const CHIP_DELAY: Duration = Duration::from_millis(500);

impl Stage {
    pub(super) fn key_zoom(&mut self, factor: f64, cx: &mut Context<Self>) {
        let (width, height) = self.size();
        self.zoom(factor, (width / 2., height / 2.), cx);
    }

    fn key_pan(&mut self, by: Pixel, cx: &mut Context<Self>) {
        if let Some(camera) = &mut self.camera {
            camera.pan(by);
            self.follow = false;
            // The pins move away from a resting cursor; its next move finds the one under it.
            self.hover(None, cx);
        }
    }

    pub(super) fn bind_keys(&self, stage: Stateful<Div>, cx: &Context<Self>) -> Stateful<Div> {
        stage
            .on_action(cx.listener(|stage, _: &ZoomIn, _, cx| stage.key_zoom(KEY_ZOOM, cx)))
            .on_action(cx.listener(|stage, _: &ZoomOut, _, cx| stage.key_zoom(1. / KEY_ZOOM, cx)))
            .on_action(cx.listener(|stage, _: &PanLeft, _, cx| stage.key_pan((KEY_PAN, 0.), cx)))
            .on_action(cx.listener(|stage, _: &PanRight, _, cx| stage.key_pan((-KEY_PAN, 0.), cx)))
            .on_action(cx.listener(|stage, _: &PanUp, _, cx| stage.key_pan((0., KEY_PAN), cx)))
            .on_action(cx.listener(|stage, _: &PanDown, _, cx| stage.key_pan((0., -KEY_PAN), cx)))
            .on_scroll(cx.listener(|stage, event, _, cx| stage.wheel(event, cx)))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::pressed))
            .on_mouse_down(MouseButton::Right, cx.listener(Self::pressed_right))
    }

    fn wheel(&mut self, event: &ScrollEvent, cx: &mut Context<Self>) {
        self.zoom(2f64.powf(lines(event) * WHEEL), self.pixel(event.position), cx);
    }

    /// The minimap keeps its zoom with the live map, for both its views.
    pub(super) fn mini_wheel(&mut self, event: &ScrollEvent, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(live) = &self.live {
            live.update(cx, |live, cx| live.zoom_by(lines(event) * WHEEL, cx));
        }
    }

    fn pressed(&mut self, event: &MouseDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.focus.focus(window, cx);
        (self.shown.target, self.layers_open) = (None, false);
        let pixel = self.pixel(event.position);
        // Even right after a click on it: Alt on a user's pin moves it.
        let moving = self.camera.filter(|_| event.modifiers.alt).and_then(|camera| {
            let (id, tip) = self.hit_mark_at(&camera, pixel, cx)?;
            Some(Moving { id, cursor: event.position, grab: (tip.0 - pixel.0, tip.1 - pixel.1) })
        });
        if event.click_count == 2 && moving.is_none() {
            self.zoom(GROUP_ZOOM, pixel, cx);
            return;
        }
        self.drag = Some(Drag { last: event.position, moved: false, moving });
        cx.notify();
    }

    /// On a pin: found or not. On the user's own: its card. Elsewhere: a menu.
    fn pressed_right(&mut self, event: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let (Some(camera), Some(map)) = (self.camera, self.map(cx)) else { return };
        let pixel = self.pixel(event.position);
        if let Some(mark) = self.hit_mark(&camera, pixel, cx) {
            return self.board.update(cx, |board, cx| board.select_mark(mark, false, cx));
        }
        match self.hit(&map, &camera, pixel, cx) {
            Some(marker) if marker.count == 1 => self.board.update(cx, |board, cx| board.toggle_found(marker.pin, cx)),
            Some(_) => {}
            None => {
                self.shown.target = Some(camera.to_plane(pixel, self.size()));
                cx.notify();
            }
        }
    }

    /// `over`: the cursor is on the map itself, not on something over it.
    pub(super) fn moved(&mut self, event: &MouseMoveEvent, over: bool, cx: &mut Context<Self>) {
        let (Some(camera), Some(map)) = (self.camera, self.map(cx)) else { return };
        let pixel = self.pixel(event.position);
        if let Some(drag) = &mut self.drag {
            // Released outside the window: the up never came.
            if event.pressed_button != Some(MouseButton::Left) {
                self.drag = None;
                cx.notify();
                return;
            }
            let by = event.position - drag.last;
            drag.moved |= by.x.abs().max(by.y.abs()) > px(CLICK_SLACK);
            if drag.moved {
                drag.last = event.position;
                if let Some(moving) = &mut drag.moving {
                    moving.cursor = event.position;
                    cx.notify();
                    return;
                }
                self.follow = false;
                if let Some(camera) = &mut self.camera {
                    camera.pan((by.x.into(), by.y.into()));
                }
                cx.notify();
            }
            return;
        }
        let mark = self.hit_mark(&camera, pixel, cx).filter(|_| over);
        let hit = self.hit(&map, &camera, pixel, cx).filter(|_| mark.is_none());
        let hovered = hit.filter(|marker| over && marker.count == 1).map(|marker| marker.pin);
        if self.shown.hovered_mark != mark {
            self.shown.hovered_mark = mark.clone();
            cx.notify();
        }
        let saved = self.board.read(cx).saved.clone();
        let view = saved.view();
        let region = if over && hit.is_none() && mark.is_none() && view.regions {
            let at = camera.to_plane(pixel, self.size());
            map.regions
                .iter()
                .position(|region| !view.hidden_regions.contains(&region.id) && region.outlines.iter().any(|ring| inside(at, ring)))
        } else {
            None
        };
        if self.shown.hovered != hovered {
            self.hover(hovered, cx);
        }
        if self.shown.hovered_region != region {
            self.shown.hovered_region = region;
            cx.notify();
        }
    }

    /// The chip shows once the cursor rests on a pin, so passing over pins does not flash it.
    pub(super) fn hover(&mut self, pin: Option<usize>, cx: &mut Context<Self>) {
        (self.shown.hovered, self.shown.chip) = (pin, None);
        self._chip = match pin {
            Some(pin) => cx.spawn(async move |stage, cx| {
                cx.background_executor().timer(CHIP_DELAY).await;
                stage
                    .update(cx, |stage, cx| {
                        stage.shown.chip = stage.shown.hovered.filter(|hovered| *hovered == pin);
                        cx.notify();
                    })
                    .ok();
            }),
            None => Task::ready(()),
        };
        cx.notify();
    }

    pub(super) fn released(&mut self, event: &MouseUpEvent, cx: &mut Context<Self>) {
        if event.button != MouseButton::Left {
            return;
        }
        let Some(drag) = self.drag.take() else { return };
        let (Some(camera), Some(map)) = (self.camera, self.map(cx)) else { return };
        let (pixel, by) = (self.pixel(event.position), event.position - drag.last);
        // The last move may come short of the release.
        if drag.moved || by.x.abs().max(by.y.abs()) > px(CLICK_SLACK) {
            if let Some(mut moving) = drag.moving {
                moving.cursor = event.position;
                let at = camera.to_plane(self.tip(&moving), self.size());
                self.board.update(cx, |board, cx| board.move_mark(&moving.id, at, cx));
            }
            return;
        }
        let (selected, open) = (self.board.read(cx).selected, self.board.read(cx).mark.clone());
        // An Alt press that did not move is a click on its pin, wherever the camera went since.
        if let Some(mark) = drag.moving.map(|moving| moving.id).or_else(|| self.hit_mark(&camera, pixel, cx)) {
            // The open one again: closed.
            return self.board.update(cx, |board, cx| {
                if open.as_ref() == Some(&mark) {
                    board.deselect(cx);
                } else {
                    board.select_mark(mark, false, cx);
                }
            });
        }
        match self.hit(&map, &camera, pixel, cx) {
            Some(marker) if marker.count > 1 => self.zoom(GROUP_ZOOM, pixel, cx),
            Some(marker) if selected != Some(marker.pin) => self.board.update(cx, |board, cx| board.select(marker.pin, false, cx)),
            // The open pin again, or beside the pins: closed.
            _ => self.board.update(cx, |board, cx| {
                board.deselect(cx);
            }),
        }
    }

    /// The topmost marker under `pixel`: as drawn, the selected pin on top, then the hovered one, at their look's size.
    fn hit(&mut self, map: &Map, camera: &Camera, pixel: Pixel, cx: &App) -> Option<pins::Marker> {
        let markers = self.markers(map, camera, cx);
        let single = |pin: &usize| markers.iter().any(|marker| marker.pin == *pin && marker.count == 1);
        let selected = self.board.read(cx).selected;
        let lifted: Vec<_> = [selected, self.shown.hovered.filter(single)]
            .into_iter()
            .flatten()
            .map(|pin| (pin, map.pins[pin].at, self.look(pin, selected).scale()))
            .collect();
        pins::hit(&markers, &lifted, camera, self.size(), pixel)
    }

    fn hit_mark(&self, camera: &Camera, pixel: Pixel, cx: &App) -> Option<Rc<str>> {
        self.hit_mark_at(camera, pixel, cx).map(|(id, _)| id)
    }

    /// The topmost user's pin under `pixel` as drawn, and its tip on the view.
    fn hit_mark_at(&self, camera: &Camera, pixel: Pixel, cx: &App) -> Option<(Rc<str>, Pixel)> {
        let (board, size, moving) = (self.board.read(cx), self.size(), self.moving(camera));
        let open = board.mark.as_deref();
        mark_order(&board.saved, open).rev().find_map(|mark| {
            let tip = camera.to_view(drawn_at(mark, moving.as_ref()), size);
            pins::on_head(tip, self.mark_look(&mark.id, open).scale(), pixel).then(|| (mark.id.clone(), tip))
        })
    }
}

fn lines(event: &ScrollEvent) -> f64 {
    (event.delta.pixel_delta(px(20.)).y / px(20.)).into()
}

/// Even-odd ray casting.
fn inside(at: Point, outline: &[Point]) -> bool {
    let mut inside = false;
    for (index, a) in outline.iter().enumerate() {
        let b = outline[(index + 1) % outline.len()];
        if (a.y > at.y) != (b.y > at.y) && at.x < a.x + (at.y - a.y) * (b.x - a.x) / (b.y - a.y) {
            inside = !inside;
        }
    }
    inside
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_is_inside_a_region_or_not() {
        let square = [Point { x: 0., y: 0. }, Point { x: 1., y: 0. }, Point { x: 1., y: 1. }, Point { x: 0., y: 1. }];
        assert!(inside(Point { x: 0.5, y: 0.5 }, &square));
        assert!(!inside(Point { x: 1.5, y: 0.5 }, &square));
        assert!(!inside(Point { x: 0.5, y: 0.5 }, &[]));
    }
}
