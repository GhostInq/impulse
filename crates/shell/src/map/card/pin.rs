//! The selected pin's card: its picture or clip, what it is, where, and the pins near it.

use gpui_kit::{
    AnyElement, ClipboardItem, Context, Div, FontWeight, Stateful, Window,
    component::{h_flex, text::TextView, v_flex},
    div, img,
    prelude::*,
    px, rgb,
};
use i18n::Arg;
use model::{Map, PIN_LINK};

use super::{Card, PICTURE};
use crate::{
    Load,
    map::{
        CARD_WIDTH,
        board::Board,
        group, kind,
        markers::{ink, legible},
        picture::{fill, picture, stand_in},
    },
    ui::{
        assets::Icon,
        key,
        text::{t, tf},
        theme::*,
        widgets::{button, flash, icon, icon_button, list_row, mono},
    },
};

const BADGE: f32 = 40.;
const NEARBY: usize = 4;

/// The pins closest to the selected one, closest first: one pass, nothing the size of the map kept.
pub(super) fn nearest(board: &Board) -> Vec<usize> {
    let (Load::Ready(map), Some(index)) = (&board.map, board.selected) else { return Vec::new() };
    let at = map.pins[index].at;
    let mut near: Vec<(f64, usize)> = Vec::with_capacity(NEARBY + 1);
    for (other, pin) in map.pins.iter().enumerate().filter(|&(other, _)| other != index) {
        let distance = (pin.at.x - at.x).powi(2) + (pin.at.y - at.y).powi(2);
        if near.len() < NEARBY || distance < near[NEARBY - 1].0 {
            near.insert(near.partition_point(|&(closer, _)| closer <= distance), (distance, other));
            near.truncate(NEARBY);
        }
    }
    near.into_iter().map(|(_, other)| other).collect()
}

impl Card {
    /// A game position, copied on a click; the map's pins and the user's pins of the player's position show it.
    pub(super) fn coordinates(&self, [x, y, z]: [f64; 3], cx: &Context<Self>) -> Stateful<Div> {
        let text = format!("{} {x:.0}  {} {y:.0}  {} {z:.0}", t(cx, key::MAP_AXIS_X), t(cx, key::MAP_AXIS_Y), t(cx, key::MAP_AXIS_Z));
        let copy = text.clone();
        let copied = if self.copied { rgb(accent(cx)) } else { rgb(DIM) };
        h_flex()
            .id("coordinates")
            .px(px(8.))
            .py(px(4.))
            .gap(px(8.))
            .cursor_pointer()
            .border_1()
            .border_color(line(0.14))
            .bg(rgb(SUNKEN))
            .hover(|style| style.border_color(line(0.3)))
            .child(mono(text, TYPE_CAPTION).flex_1().text_color(rgb(MUTED)))
            .child(mono(t(cx, if self.copied { key::MAP_COPIED } else { key::MAP_COPY }), TYPE_TAG).text_color(copied))
            .on_click(cx.listener(move |card, _, _, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(copy.clone()));
                card.copied = true;
                card._flash = flash(cx, |card| card.copied = false);
                cx.notify();
            }))
    }

    pub(super) fn pin_card(&mut self, map: &Map, board: &Board, window: &Window, cx: &Context<Self>) -> Option<AnyElement> {
        let index = board.selected?;
        let pin = &map.pins[index];
        let category = &map.categories[pin.group.0];
        let found = board.saved.found(&pin.id);
        let teleport = pin.position.filter(|_| board.can_teleport(map));
        let accent = rgb(accent(cx));
        let badge = h_flex()
            .size(px(BADGE))
            .flex_none()
            .justify_center()
            .bg(rgb(category.color))
            .border_2()
            .border_color(rgb(PANEL))
            .child(self.images.glyph(board.icon(pin.group, cx), pin.group, ink(category.color), 20., window, cx));
        // Under the lightbox the card's clip does not play.
        let clip = self.clip.as_ref().filter(|_| !self.lightbox).map(|clip| clip.view.clone().into_any_element());
        let preview = pin.image.as_ref().map(|image| fill(img(image.preview.clone()), ("pin-media", index)));
        let shown = (pin.image.is_some() || pin.video.is_some()).then(|| {
            let under = stand_in(pin.group, map, board, &mut self.images, window, cx);
            div()
                .id("pin-picture")
                .relative()
                .h(px(PICTURE))
                .flex_none()
                .cursor_pointer()
                .bg(rgb(SUNKEN))
                .child(picture(index, under, preview.into_iter().chain(clip), (CARD_WIDTH - 2., PICTURE)))
                .on_click(cx.listener(|card, _, _, cx| {
                    (card.lightbox, card.photo) = (true, false);
                    cx.notify();
                }))
        });
        let close = icon_button("pin-close", Icon::Close, 28., 11.).bg(alpha(PANEL, 0.85)).on_click(cx.listener(|card, _, _, cx| {
            card.board.update(cx, |board, cx| board.deselect(cx));
        }));
        let title = v_flex()
            .flex_1()
            .min_w_0()
            .gap(px(3.))
            .child(div().font_weight(FontWeight::BOLD).text_size(px(TYPE_TITLE)).text_color(rgb(TEXT_BRIGHT)).child(pin.name.clone()))
            .child(mono(kind(map, pin), TYPE_CAPTION).text_color(rgb(DIM)));
        let head = h_flex().items_end().gap(px(12.)).px(px(14.)).child(badge).child(title);
        let head = match &shown {
            Some(_) => head.mt(px(-BADGE / 2.)),
            None => head.pt(px(14.)).pr(px(36.)),
        };
        let coordinates = pin.position.map(|position| self.coordinates(position, cx).mx(px(14.)));
        let mark = button(&t(cx, if found { key::MAP_FOUND } else { key::MAP_MARK_FOUND }))
            .id("pin-found")
            .flex_1()
            .when(found, |button| button.border_color(accent).text_color(accent).child(icon(Icon::Check, 11., accent)))
            .on_click(cx.listener(move |card, _, _, cx| card.board.update(cx, |board, cx| board.toggle_found(index, cx))));
        let go = teleport.map(|to| {
            button(&t(cx, if self.confirm { key::MAP_CONFIRM } else { key::MAP_TELEPORT }))
                .id("pin-teleport")
                .flex_1()
                .when(self.confirm, |button| button.border_color(accent).text_color(accent))
                .on_click(cx.listener(move |card, _, _, cx| {
                    if card.confirm {
                        card.confirm = false;
                        card.board.update(cx, |board, cx| board.teleport(to, cx));
                    } else {
                        card.confirm = true;
                        cx.notify();
                    }
                }))
        });
        let hide_id = group(map, pin).id.clone();
        let hide_text = match &self.hide {
            Some((at, text)) if *at == index => text.clone(),
            _ => {
                let name = group(map, pin).name.to_uppercase();
                let text = tf(cx, key::MAP_HIDE_GROUP, &[("group", Arg::Str(&name))]);
                self.hide = Some((index, text.clone()));
                text
            }
        };
        let hide = h_flex()
            .id("pin-hide-group")
            .px(px(14.))
            .pb(px(12.))
            .gap(px(6.))
            .cursor_pointer()
            .text_color(rgb(FAINT))
            .hover(|style| style.text_color(rgb(MUTED)))
            .child(mono(hide_text, TYPE_TAG))
            .on_click(cx.listener(move |card, _, _, cx| {
                card.board.update(cx, |board, cx| {
                    board.filter(
                        |view| {
                            view.hidden_groups.insert(hide_id.clone());
                        },
                        cx,
                    )
                })
            }));
        let card = self
            .panel("pin-card")
            .children(shown)
            .child(head)
            .children(coordinates)
            .when(!pin.description.is_empty(), |card| {
                let board = self.board.downgrade();
                let text = TextView::markdown(("pin-text", index), pin.description.clone()).on_link_click(move |url, _, _, cx| {
                    match url.strip_prefix(PIN_LINK) {
                        Some(id) => {
                            let id = id.to_owned();
                            board.update(cx, |board, cx| board.select_id(&id, cx)).ok();
                        }
                        // Links into other apps are not followed from here.
                        None if url.starts_with("https://") || url.starts_with("http://") => cx.open_url(url),
                        None => {}
                    }
                });
                card.child(div().px(px(14.)).text_size(px(TYPE_BODY)).text_color(rgb(TEXT_2)).child(text))
            })
            .child(h_flex().gap(px(8.)).px(px(14.)).child(mark).children(go))
            .children(self.nearby(map, board, window, cx))
            .child(hide)
            .child(div().absolute().top(px(6.)).right(px(6.)).child(close));
        Some(card.into_any_element())
    }

    fn nearby(&mut self, map: &Map, board: &Board, window: &Window, cx: &Context<Self>) -> Option<AnyElement> {
        if self.near.is_empty() {
            return None;
        }
        let rows: Vec<_> = self
            .near
            .iter()
            .map(|&other| {
                let pin = &map.pins[other];
                let color = legible(map.categories[pin.group.0].color);
                list_row(("pin-nearby", other), false)
                    .px(px(14.))
                    .py(px(4.))
                    .gap(px(10.))
                    .child(self.images.glyph(board.icon(pin.group, cx), pin.group, color, 14., window, cx))
                    .child(div().flex_1().min_w_0().truncate().text_size(px(TYPE_SMALL)).text_color(rgb(TEXT_2)).child(pin.name.clone()))
                    .child(mono(group(map, pin).name.to_uppercase(), TYPE_TAG).text_color(rgb(FAINT)))
                    .on_click(cx.listener(move |card, _, _, cx| card.board.update(cx, |board, cx| board.select(other, true, cx))))
            })
            .collect();
        Some(
            v_flex()
                .child(mono(t(cx, key::MAP_NEARBY), TYPE_CAPTION).text_color(rgb(DIM)).px(px(14.)).pb(px(4.)))
                .children(rows)
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, rc::Rc};

    use client::storage::Paths;
    use model::{Map, Pin, Point, Text, TileSet, Tiles};

    use super::*;

    #[test]
    fn the_nearest_pins_come_closest_first() {
        let pin = |x: f64| Pin {
            id: String::new(),
            at: Point { x, y: 0.5 },
            name: String::new(),
            description: String::new(),
            group: (0, 0),
            image: None,
            video: None,
            position: None,
            level: None,
        };
        let tiles = Tiles { template: String::new(), flipped: false, size: 256, min_zoom: 0, max_zoom: 0, bounds: None };
        let pins = [0.5, 0.9, 0.52, 0.3, 0.45, 0.6, 0.51].map(pin).into();
        let map = Map {
            tile_sets: vec![TileSet { id: String::new(), name: Text::Plain(String::new()), tiles }],
            background: 0,
            categories: Vec::new(),
            pins,
            regions: Vec::new(),
            labels: Vec::new(),
            frame: None,
            teleport: false,
            unfinished: false,
        };
        let mut board = Board::new(Paths::new(PathBuf::new()), PathBuf::new());
        board.map = Load::Ready(Rc::new(map));
        assert!(nearest(&board).is_empty());
        board.selected = Some(0);
        assert_eq!(nearest(&board), [6, 2, 4, 5]);
    }
}
