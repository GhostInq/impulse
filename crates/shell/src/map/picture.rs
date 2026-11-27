//! A pin's picture or clip in a box: cut to fill it, its edges trimmed.

use gpui_kit::{AnyElement, App, Div, ElementId, Img, ObjectFit, Window, component::h_flex, div, image_cache, prelude::*, px, retain_all};
use model::Map;

use super::{
    board::Board,
    markers::{Images, legible},
};
use crate::ui::theme::alpha;

// Pin pictures and clips are 16:9 screen captures, some with the capture's border along their edges.
pub const ASPECT: f32 = 16. / 9.;
// Of the width, cut from each edge.
const TRIM: f32 = 0.006;
const MARK: f32 = 40.;

/// `layers` (pictures, a clip over them) covering a box of `size`, each over the one before once it shows, all over
/// `stand_in`. The pictures it decodes (a full size one is about 8 MB) are freed once no frame draws `pin`'s (an id
/// per pin).
pub fn picture(
    pin: impl Into<ElementId>,
    stand_in: AnyElement,
    layers: impl IntoIterator<Item = AnyElement>,
    (width, height): (f32, f32),
) -> Div {
    let trim = width * TRIM;
    let layer = |content| div().absolute().top(px(-trim)).left(px(-trim)).w(px(width + trim * 2.)).h(px(height + trim * 2.)).child(content);
    let cached = image_cache(retain_all(pin)).size_full().children(layers.into_iter().map(layer));
    div().relative().flex_none().w(px(width)).h(px(height)).overflow_hidden().child(stand_in).child(cached)
}

/// A picture filling its layer; the id keeps its state (an animated one's frame).
pub fn fill(image: Img, id: impl Into<ElementId>) -> AnyElement {
    image.id(id).size_full().object_fit(ObjectFit::Cover).into_any_element()
}

/// What `picture` shows until a layer covers it (a cold CDN takes up to a second) or when none loads:
/// the pin's group mark, faint, on its category's color.
pub fn stand_in(group: (usize, usize), map: &Map, board: &Board, images: &mut Images, window: &Window, cx: &App) -> AnyElement {
    let color = map.categories[group.0].color;
    let mark = images.glyph(board.icon(group, cx), group, legible(color), MARK, window, cx);
    let tint = h_flex().absolute().size_full().justify_center().bg(alpha(color, 0.08));
    tint.child(div().opacity(0.4).child(mark)).into_any_element()
}
