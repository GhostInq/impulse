//! How far above or below the player a pin near them is: an arrow and the game's units under its tip.

use gpui_kit::{App, SharedString, Window, point, px};
use model::{Frame, Map, Point};

use super::{Stage, camera::Camera, paint::text};
use crate::ui::theme::{TEXT_BRIGHT, TYPE_CAPTION};

// On the plane, where about a metre is 1e-4: pins this near get the mark, and none for a smaller difference.
const NEAR: f64 = 5e-3;
const LEVEL: f64 = 2e-4;
// Under the tip, clear of the marker.
const BELOW: f32 = 8.;

impl Stage {
    /// `places`: where pins lie on the plane and in the game. Only while the layer is on and the trainer tells the
    /// player's height.
    pub(super) fn paint_rises(
        &mut self,
        map: &Map,
        places: impl Iterator<Item = (Point, [f64; 3])>,
        camera: &Camera,
        window: &mut Window,
        cx: &mut App,
    ) {
        let board = self.board.read(cx);
        let (Some(frame), Some(player)) = (&map.frame, board.player.filter(|_| board.here && board.saved.view().rises)) else { return };
        let (ground, area, origin) = ((frame.to_map(player), frame.height(player)), self.size(), self.bounds.origin);
        for (at, position) in places {
            let Some(rise) = rise(frame, ground, at, position) else { continue };
            let label = self.shown.rises.entry(rise).or_insert_with(|| label(rise)).clone();
            let (x, y) = camera.to_view(at, area);
            text(label, TYPE_CAPTION, TEXT_BRIGHT, true, origin + point(px(x as f32), px(y as f32 + BELOW)), window, cx);
        }
    }
}

/// The rise as shown, in the game's units: whole below a thousand, by the hundred above; `None` near level or far.
fn rise(frame: &Frame, (at_player, height): (Point, f64), at: Point, position: [f64; 3]) -> Option<i64> {
    let rise = frame.height(position) - height;
    // The up axis is in the game's units, as the plane's x is.
    if (at.x - at_player.x).hypot(at.y - at_player.y) > NEAR || (rise * frame.scale.0).abs() < LEVEL {
        return None;
    }
    let shown = if rise.abs() < 1000. { rise.round() as i64 } else { (rise / 100.).round() as i64 * 100 };
    Some(shown).filter(|shown| *shown != 0)
}

fn label(rise: i64) -> SharedString {
    let (arrow, size) = (if rise > 0 { '▲' } else { '▼' }, rise.unsigned_abs());
    if size < 1000 { format!("{arrow}{size}") } else { format!("{arrow}{:.1}k", size as f64 / 1000.) }.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_near_pin_shows_its_rise_and_a_far_or_level_one_none() {
        // A game unit is 1e-4 of the plane, about a metre; z is up.
        let frame = Frame { axes: [0, 1, 2], origin: Point { x: 0.5, y: 0.5 }, scale: (1e-4, 1e-4) };
        let ground = (Point { x: 0.5, y: 0.5 }, 10.);
        let near = Point { x: 0.501, y: 0.5 };
        assert_eq!(rise(&frame, ground, near, [10., 0., 22.4]), Some(12));
        assert_eq!(rise(&frame, ground, near, [10., 0., -1490.]), Some(-1500));
        assert_eq!(rise(&frame, ground, near, [10., 0., 11.]), None);
        assert_eq!(rise(&frame, ground, Point { x: 0.6, y: 0.5 }, [1000., 0., 50.]), None);
        assert_eq!((label(12), label(-1500)), (SharedString::from("▲12"), SharedString::from("▼1.5k")));
    }
}
