//! The sidebar's list of the user's own pins, with a switch for all of them.

use client::map::state::{PALETTE, UserPin, View};
use gpui_kit::{AnyElement, App, Context, IntoElement, SharedString, div, prelude::*, px, rgb};
use i18n::Arg;

use super::{Sidebar, check, section};
use crate::{
    map::board::Board,
    ui::{
        key,
        text::{t, tf},
        theme::*,
        widgets::list_row,
    },
};

impl Sidebar {
    pub(super) fn marks_section(&self, board: &Board, cx: &Context<Self>) -> Vec<AnyElement> {
        let (count, shown) = (board.saved.pins().count(), board.saved.view().pins_shown());
        let toggle = div().id("marks-check").child(check(PALETTE[0], shown)).on_click(cx.listener(|sidebar, _, _, cx| {
            cx.stop_propagation();
            sidebar.board.update(cx, |board, cx| board.filter(View::toggle_pins, cx));
        }));
        let header =
            section("map-marks", self.marks_open, &t(cx, key::MAP_MY_PINS), count.to_string(), Some(toggle.into_any_element()), cx)
                .on_click(cx.listener(|sidebar, _, _, cx| {
                    sidebar.marks_open ^= true;
                    cx.notify();
                }));
        let mut rows = vec![header.into_any_element()];
        if !self.marks_open {
            return rows;
        }
        if count == 0 {
            let hint = div().px(px(32.)).pb(px(8.)).text_size(px(TYPE_SMALL)).text_color(rgb(FAINT)).child(t(cx, key::MAP_NO_PINS));
            rows.push(hint.into_any_element());
        }
        for (index, mark) in board.saved.pins().enumerate() {
            let id = mark.id.clone();
            let row = list_row(("map-mark", index), board.mark.as_ref() == Some(&mark.id))
                .h(px(28.))
                .pl(px(32.))
                .pr(px(14.))
                .gap(px(10.))
                .child(div().size(px(10.)).flex_none().bg(rgb(mark.color)))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .text_size(px(TYPE_BODY))
                        .text_color(rgb(if shown { TEXT_2 } else { FAINT }))
                        .child(title(index, mark, cx)),
                )
                .on_click(
                    cx.listener(move |sidebar, _, _, cx| sidebar.board.update(cx, |board, cx| board.select_mark(id.clone(), true, cx))),
                );
            rows.push(row.into_any_element());
        }
        rows
    }
}

/// An unnamed mark goes by its place in the list.
fn title(index: usize, mark: &UserPin, cx: &App) -> SharedString {
    if mark.name.trim().is_empty() {
        let number = (index + 1).to_string();
        tf(cx, key::MAP_PIN_NAME, &[("number", Arg::Str(&number)), ("count", Arg::Num((index + 1) as f64))])
    } else {
        mark.name.clone().into()
    }
}
