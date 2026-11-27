//! The card of a pin of the user's own: named, noted, colored, kept with the map's state.

use client::map::state::{PALETTE, PinEdit};
use gpui_kit::{
    AnyElement, Context, Entity, EventEmitter, FontWeight, Subscription, Window,
    component::{
        h_flex,
        input::{Input, InputEvent, InputState, Textarea, TextareaState},
    },
    div,
    prelude::*,
    px, rgb,
};
use model::Map;

use super::Card;
use crate::{
    map::board::Board,
    ui::{
        assets::Icon,
        key,
        text::t,
        theme::*,
        widgets::{button, icon_button, input, mono, relabel},
    },
};

const SWATCH: f32 = 18.;

/// The open mark's name and note fields; edits go straight into the mark.
pub(super) fn inputs(window: &mut Window, cx: &mut Context<Card>) -> (Entity<InputState>, Entity<TextareaState>, Subscription) {
    let (name, named) = input(key::MAP_NAME_FIELD, window, cx);
    let note = cx.new(|cx| {
        let mut note = TextareaState::new(window, cx).placeholder(t(cx, key::MAP_NOTE_FIELD));
        note.set_auto_grow(2, 6, cx);
        note
    });
    let relabels = Subscription::join(named, relabel(&note, key::MAP_NOTE_FIELD, window, cx));
    edits(&name, |input| input.value().to_string(), PinEdit::Name, cx);
    edits(&note, |input| input.value().to_string(), PinEdit::Description, cx);
    (name, note, relabels)
}

fn edits<T: EventEmitter<InputEvent>>(input: &Entity<T>, text: fn(&T) -> String, edit: fn(String) -> PinEdit, cx: &mut Context<Card>) {
    cx.subscribe(input, move |card, input, event: &InputEvent, cx| match event {
        InputEvent::Change => {
            let text = text(input.read(cx));
            card.board.update(cx, |board, cx| board.edit_mark(edit(text), cx));
        }
        InputEvent::Blur => card.board.update(cx, |board, _| board.save()),
        _ => {}
    })
    .detach();
}

impl Card {
    pub(super) fn fill_mark(&mut self, board: &Entity<Board>, window: &mut Window, cx: &mut Context<Self>) {
        let board = board.read(cx);
        let Some(mark) = board.open_mark() else { return };
        let (name, note) = (mark.name.clone(), mark.description.clone());
        self.mark_name.update(cx, |input, cx| input.set_value(name, window, cx));
        self.mark_note.update(cx, |input, cx| input.set_value(note, window, cx));
    }

    pub(super) fn mark_card(&self, map: &Map, board: &Board, cx: &Context<Self>) -> Option<AnyElement> {
        let mark = board.open_mark()?;
        let (id, color) = (mark.id.clone(), mark.color);
        let field = || div().px(px(8.)).py(px(4.)).border_1().border_color(line(0.14)).bg(rgb(SUNKEN)).text_size(px(TYPE_BODY));
        let swatches = PALETTE.iter().enumerate().map(|(slot, &swatch)| {
            div()
                .id(("mark-color", slot))
                .size(px(SWATCH))
                .cursor_pointer()
                .bg(rgb(swatch))
                .border_2()
                .border_color(if swatch == color { rgb(TEXT_BRIGHT) } else { rgb(PANEL) })
                .on_click(cx.listener(move |card, _, _, cx| {
                    card.board.update(cx, |board, cx| {
                        board.edit_mark(PinEdit::Color(swatch), cx);
                        board.save();
                    })
                }))
        });
        let teleport = board.can_teleport_to_mark(map, mark).then(|| {
            let id = id.clone();
            button(&t(cx, key::MAP_TELEPORT))
                .id("mark-teleport")
                .flex_1()
                .on_click(cx.listener(move |card, _, _, cx| card.board.update(cx, |board, cx| board.teleport_to_mark(&id, cx))))
        });
        let delete = button(&t(cx, key::MAP_DELETE))
            .id("mark-delete")
            .flex_1()
            .on_click(cx.listener(move |card, _, _, cx| card.board.update(cx, |board, cx| board.delete_mark(&id, cx))));
        let close = icon_button("mark-close", Icon::Close, 28., 11.).on_click(cx.listener(|card, _, _, cx| {
            card.board.update(cx, |board, cx| board.deselect(cx));
        }));
        let head = h_flex()
            .gap(px(10.))
            .child(div().size(px(14.)).flex_none().bg(rgb(color)))
            .child(mono(t(cx, key::MAP_MY_PIN), TYPE_CAPTION).flex_1().text_color(rgb(DIM)))
            .child(close);
        let card = self
            .panel("mark-card")
            .p(px(14.))
            .child(head)
            .children(mark.position.map(|position| self.coordinates(position, cx)))
            .child(field().font_weight(FontWeight::BOLD).child(Input::new(&self.mark_name).appearance(false).px_0()))
            .child(field().child(Textarea::new(&self.mark_note).appearance(false)))
            .child(h_flex().gap(px(8.)).children(swatches))
            .child(h_flex().gap(px(8.)).children(teleport).child(delete));
        Some(card.into_any_element())
    }
}
