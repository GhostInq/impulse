//! What floats over the map and moves with it: the hovered pin's chip, the right click menu,
//! the buttons and layers, the player's readout.

use client::map::state::Saved;
use gpui_kit::{
    AnyElement, App, Context, Div, FontWeight, IntoElement, SharedString, Stateful, Window,
    component::{h_flex, v_flex},
    div, img,
    prelude::*,
    px, rgb,
};
use model::Map;

use super::{Stage, input::KEY_ZOOM};
use crate::{
    map::{
        BUTTON, MARGIN,
        board::Board,
        kind,
        markers::HEAD_ABOVE,
        picture::{fill, picture, stand_in},
    },
    ui::{
        assets::Icon,
        key,
        text::{failed, t, text},
        theme::*,
        widgets::{button, icon, list_row, mono, square, tip},
    },
};

// The right click menu's size, about.
const MENU: (f64, f64) = (150., 70.);
const CHIP_WIDTH: f32 = 260.;
// 16:9 across the chip.
const CHIP_PICTURE: f32 = 146.;
// The chip's height below its picture, about.
const CHIP_TEXT: f32 = 76.;

impl Stage {
    /// The pin the cursor rests on, beside it: its picture across the top, then what it is.
    pub(super) fn chip(&mut self, map: &Map, board: &Board, window: &Window, cx: &App) -> Option<AnyElement> {
        let (index, camera) = (self.shown.chip.filter(|pin| board.selected != Some(*pin))?, self.camera?);
        let pin = &map.pins[index];
        let ((x, y), (width, height)) = (camera.to_view(pin.at, self.size()), self.size());
        let (wide, tall) = (f64::from(CHIP_WIDTH), f64::from(CHIP_TEXT + if pin.image.is_some() { CHIP_PICTURE } else { 0. }));
        let left = if x + 24. + wide > width { x - 24. - wide } else { x + 24. };
        let top = (y - HEAD_ABOVE - 40.).min(height - tall - f64::from(MARGIN)).max(f64::from(MARGIN));
        let about = plain(&pin.description);
        let text = v_flex()
            .p(px(10.))
            .gap(px(3.))
            .child(
                div()
                    .truncate()
                    .text_size(px(TYPE_LEAD))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(TEXT_BRIGHT))
                    .child(pin.name.clone()),
            )
            .child(mono(kind(map, pin), TYPE_TAG).truncate().text_color(rgb(DIM)))
            .when(!about.is_empty(), |text| {
                text.child(div().pt(px(3.)).line_clamp(2).text_size(px(TYPE_SMALL)).text_color(rgb(TEXT_2)).child(about))
            })
            .when(board.saved.found.contains(&pin.id), |text| {
                let accent = rgb(accent(cx));
                text.child(
                    h_flex()
                        .pt(px(3.))
                        .gap(px(5.))
                        .child(icon(Icon::Check, 10., accent))
                        .child(mono(t(cx, key::MAP_FOUND), TYPE_TAG).text_color(accent)),
                )
            });
        let preview = pin.image.as_ref().map(|image| {
            let under = stand_in(pin.group, map, board, &mut self.images, window, cx);
            picture(index, under, [fill(img(image.preview.clone()), ("pin-media", index))], (CHIP_WIDTH - 2., CHIP_PICTURE))
        });
        let chip = v_flex()
            .absolute()
            .left(px(left as f32))
            .top(px(top as f32))
            .w(px(CHIP_WIDTH))
            .border_1()
            .border_color(line(0.16))
            .bg(alpha(PANEL, 0.97))
            .children(preview)
            .child(text);
        Some(chip.into_any_element())
    }

    /// What a right click beside the pins offers: a pin of the user's own, the player moved there.
    pub(super) fn target_menu(&self, map: &Map, board: &Board, cx: &Context<Self>) -> Option<AnyElement> {
        let (target, camera) = (self.shown.target?, self.camera?);
        let ((x, y), (width, height)) = (camera.to_view(target, self.size()), self.size());
        let (x, y) = (x.min(width - MENU.0).max(0.), y.min(height - MENU.1).max(0.));
        let teleport = board.can_teleport_to_plane(map).then(|| {
            button(&t(cx, key::MAP_TELEPORT_HERE)).id("map-teleport").border_color(rgb(accent(cx))).on_click(cx.listener(
                move |stage, _, _, cx| {
                    stage.shown.target = None;
                    stage.board.update(cx, |board, cx| board.teleport_to_plane(target, cx));
                },
            ))
        });
        let add = button(&t(cx, key::MAP_ADD_PIN))
            .id("map-add-mark")
            .on_click(cx.listener(move |stage, _, _, cx| stage.board.update(cx, |board, cx| board.add_mark(target, None, cx))));
        let menu = v_flex().occlude().absolute().left(px(x as f32)).top(px(y as f32)).gap(px(4.)).child(add).children(teleport);
        Some(menu.into_any_element())
    }

    /// Bottom right: follow the player, pin the player's position, zoom, layers.
    pub(super) fn buttons(&self, map: &Map, board: &Board, cx: &Context<Self>) -> impl IntoElement {
        let locate = board.player.filter(|_| map.frame.is_some()).map(|_| {
            let follow = control("map-follow", Icon::Player, self.follow, cx).on_click(cx.listener(|stage, _, _, cx| {
                stage.follow ^= true;
                stage.center_on_player();
                cx.notify();
            }));
            let hint = t(cx, key::MAP_PIN_POSITION);
            let pin = board.here.then(|| {
                control("map-pin-position", Icon::Pin, false, cx)
                    .border_t_0()
                    .tooltip(move |window, cx| tip(hint.clone(), window, cx))
                    .on_click(cx.listener(|stage, _, _, cx| stage.board.update(cx, |board, cx| board.mark_player(cx))))
            });
            v_flex().child(follow).children(pin)
        });
        let zoom_in = control("map-zoom-in", Icon::Plus, false, cx).on_click(cx.listener(|stage, _, _, cx| stage.key_zoom(KEY_ZOOM, cx)));
        let zoom_out =
            control("map-zoom-out", Icon::Minus, false, cx).on_click(cx.listener(|stage, _, _, cx| stage.key_zoom(1. / KEY_ZOOM, cx)));
        let layers = control("map-layers", Icon::Layers, self.layers_open, cx).on_click(cx.listener(|stage, _, _, cx| {
            stage.layers_open ^= true;
            cx.notify();
        }));
        h_flex()
            .occlude()
            .absolute()
            .right(px(MARGIN))
            .bottom(px(MARGIN))
            .items_end()
            .gap(px(8.))
            .when(self.layers_open, |row| row.child(layers_menu(map, board, cx)))
            .child(v_flex().gap(px(8.)).children(locate).child(v_flex().child(zoom_in).child(zoom_out.border_t_0())).child(layers))
    }

    /// Where the player is, and what went wrong last.
    pub(super) fn readout(&self, board: &Board, cx: &App) -> impl IntoElement {
        let chip = |text: SharedString, color: u32| {
            mono(text, TYPE_CAPTION)
                .px(px(7.))
                .py(px(3.))
                .border_1()
                .border_color(alpha(color, 0.4))
                .bg(alpha(BG, 0.85))
                .text_color(rgb(color))
        };
        h_flex()
            .absolute()
            .left(px(MARGIN))
            .bottom(px(MARGIN))
            .gap(px(8.))
            .when_some(board.player, |strip, [x, y, z]| {
                strip.child({
                    let (ax, ay, az) = (t(cx, key::MAP_AXIS_X), t(cx, key::MAP_AXIS_Y), t(cx, key::MAP_AXIS_Z));
                    chip(format!("{ax} {x:.0} · {ay} {y:.0} · {az} {z:.0}").into(), accent(cx))
                })
            })
            .when_some(board.note.as_ref(), |strip, failure| strip.child(chip(failed(cx, key::TELEPORT_FAILED, failure), WARN)))
    }
}

/// The map's tile sets, when it has more than one, then its layers: grouping on the minimap for a map it shows, the
/// rises while the player's height is told.
fn layers_menu(map: &Map, board: &Board, cx: &Context<Stage>) -> impl IntoElement {
    let (accent, saved) = (accent(cx), &board.saved);
    let option = |id: (&'static str, usize), label: SharedString, on: bool| {
        list_row(id, false)
            .h(px(28.))
            .px(px(10.))
            .gap(px(10.))
            .child(square(11., alpha(accent, if on { 1. } else { 0. })).border_1().border_color(if on { rgb(accent) } else { line(0.3) }))
            .child(mono(label, TYPE_CAPTION).font_weight(FontWeight::BOLD).text_color(rgb(if on { TEXT } else { DIM })))
    };
    let layer = |slot: usize, label: SharedString, on: bool, flip: fn(&mut Saved)| {
        option(("map-layer", slot), label, on)
            .on_click(cx.listener(move |stage, _, _, cx| stage.board.update(cx, |board, cx| board.filter(flip, cx))))
    };
    let chosen = saved.tile_set_of(map).0;
    let sets = map.tile_sets.iter().enumerate().map(|(index, set)| {
        let id = set.id.clone();
        option(("map-tiles", index), text(cx, &set.name).to_uppercase().into(), index == chosen).on_click(cx.listener(
            move |stage, _, _, cx| {
                let id = id.clone();
                stage.board.update(cx, |board, cx| board.filter(|saved| saved.tile_set = Some(id), cx));
            },
        ))
    });
    v_flex()
        .w(px(170.))
        .py(px(4.))
        .border_1()
        .border_color(line(0.16))
        .bg(rgb(PANEL))
        .when(map.tile_sets.len() > 1, |menu| {
            menu.child(v_flex().pb(px(4.)).mb(px(4.)).border_b_1().border_color(line(0.16)).children(sets))
        })
        .when(!map.labels.is_empty(), |menu| menu.child(layer(0, t(cx, key::MAP_LAYER_LABELS), saved.labels, |saved| saved.labels ^= true)))
        .when(!map.regions.is_empty(), |menu| {
            menu.child(layer(1, t(cx, key::MAP_LAYER_REGIONS), saved.regions, |saved| saved.regions ^= true))
        })
        .child(layer(2, t(cx, key::MAP_LAYER_CLUSTERS), saved.clusters, |saved| saved.clusters ^= true))
        .when(saved.clusters && map.frame.is_some(), |menu| {
            // Under its square: a choice within the one above.
            let mini = layer(3, t(cx, key::MAP_LAYER_MINI_CLUSTERS), saved.mini_clusters, |saved| saved.mini_clusters ^= true);
            menu.child(mini.pl(px(31.)))
        })
        .when(board.here, |menu| menu.child(layer(4, t(cx, key::MAP_LAYER_RISES), saved.rises, |saved| saved.rises ^= true)))
}

/// A square map button; `on` lights it in the accent.
fn control(id: &'static str, glyph: Icon, on: bool, cx: &App) -> Stateful<Div> {
    let accent = accent(cx);
    div()
        .id(id)
        .size(px(BUTTON))
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .border_1()
        .border_color(if on { rgb(accent) } else { line(0.2) })
        .bg(rgb(if on { RAISED_HOVER } else { PANEL }))
        .hover(|style| style.bg(rgb(RAISED_HOVER)))
        .child(icon(glyph, 15., rgb(if on { accent } else { TEXT_2 })))
}

/// The first paragraph of markdown as plain text.
fn plain(markdown: &str) -> String {
    let mut rest = markdown.lines().map(str::trim).find(|line| !line.is_empty()).unwrap_or_default();
    // Links keep their text.
    let mut text = String::new();
    while let Some(at) = rest.find("](") {
        text.push_str(&rest[..at]);
        rest = closing(&rest[at + 1..]).map_or("", |end| &rest[at + end + 2..]);
    }
    text.push_str(rest);
    text.chars().filter(|c| !"*_#>`[".contains(*c)).collect::<String>().trim().to_owned()
}

/// The `)` closing the `(` that `text` starts with, past pairs inside it.
fn closing(text: &str) -> Option<usize> {
    let mut depth = 0;
    text.char_indices().find_map(|(at, c)| {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        (depth == 0).then_some(at)
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn plain_text_keeps_what_links_say() {
        assert_eq!(
            super::plain(
                "
**Read** [Fallout](https://w/Fallout_(series)) or [Gate](pin:a).
More"
            ),
            "Read Fallout or Gate."
        );
    }
}
