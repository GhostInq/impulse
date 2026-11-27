//! The sidebar's list of the map's pins the user hid: a row opens its pin, whose card shows it again.

use gpui_kit::{AnyElement, Context, IntoElement, div, prelude::*, px, rgb};
use model::Map;

use super::{Sidebar, section};
use crate::{
    map::{board::Board, group},
    ui::{
        key,
        text::t,
        theme::*,
        widgets::{list_row, mono},
    },
};

impl Sidebar {
    pub(super) fn hidden_section(&self, map: &Map, board: &Board, cx: &Context<Self>) -> Vec<AnyElement> {
        let hidden: Vec<usize> = (0..map.pins.len()).filter(|&index| board.saved.hidden(&map.pins[index].id)).collect();
        if hidden.is_empty() {
            return Vec::new();
        }
        let header = section("map-hidden", self.hidden_open, &t(cx, key::MAP_HIDDEN_PINS), hidden.len().to_string(), None, cx).on_click(
            cx.listener(|sidebar, _, _, cx| {
                sidebar.hidden_open ^= true;
                cx.notify();
            }),
        );
        let mut rows = vec![header.into_any_element()];
        if !self.hidden_open {
            return rows;
        }
        for index in hidden {
            let pin = &map.pins[index];
            let row = list_row(("map-hidden-pin", index), board.selected == Some(index))
                .h(px(28.))
                .pl(px(32.))
                .pr(px(14.))
                .gap(px(10.))
                .child(div().size(px(10.)).flex_none().bg(rgb(map.categories[pin.group.0].color)))
                .child(div().flex_1().min_w_0().truncate().text_size(px(TYPE_BODY)).text_color(rgb(FAINT)).child(pin.name.clone()))
                .child(mono(group(map, pin).name.to_uppercase(), TYPE_TAG).text_color(rgb(FAINT)))
                .on_click(cx.listener(move |sidebar, _, _, cx| sidebar.board.update(cx, |board, cx| board.select(index, true, cx))));
            rows.push(row.into_any_element());
        }
        rows
    }
}
