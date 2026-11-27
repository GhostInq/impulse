//! The card of a pin of the user's own: named, noted, colored, kept with the map's state.

use client::map::state::{PALETTE, PinEdit, UserPin};
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

use super::{Card, found_button, media::paste_into};
use crate::{
    Load,
    map::board::{Board, Target, plane},
    ui::{
        assets::Icon,
        key,
        text::t,
        theme::*,
        widgets::{button, icon_button, input, mono, number_field, relabel},
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

/// The open mark's game position, X, Y and Z in the game's order: typed, it moves there on Enter or blur.
pub(super) fn axes(window: &mut Window, cx: &mut Context<Card>) -> [Entity<InputState>; 3] {
    [(); 3].map(|_| {
        let field = cx.new(|cx| InputState::new(window, cx));
        cx.subscribe_in(&field, window, |card, _, event: &InputEvent, window, cx| {
            if matches!(event, InputEvent::PressEnter { .. } | InputEvent::Blur) {
                card.place_mark(window, cx);
            }
        })
        .detach();
        field
    })
}

/// What the axes show: the mark's game position, else its plane point at height 0; none without a frame.
fn shown_position(board: &Board, target: &Target) -> Option<[f64; 3]> {
    let (Load::Ready(map), Some(mark)) = (&board.map, board.target_pin(target)) else { return None };
    let frame = map.frame.as_ref()?;
    Some(mark.position.unwrap_or_else(|| frame.to_game(plane(mark.at), 0.)))
}

/// As typed: two decimals at most, no trailing zeros.
fn axis(value: f64) -> String {
    let text = format!("{value:.2}");
    text.trim_end_matches('0').trim_end_matches('.').to_owned()
}

impl Card {
    /// What was typed for the mark shown before goes to it first: its card may close before its field blurs.
    pub(super) fn fill_mark(&mut self, board: &Entity<Board>, window: &mut Window, cx: &mut Context<Self>) {
        self.place_mark(window, cx);
        let board = board.read(cx);
        self.axes_for = board.mark_target();
        let Some(mark) = board.open_mark() else { return };
        let (name, note) = (mark.name.clone(), mark.description.clone());
        self.mark_name.update(cx, |input, cx| input.set_value(name, window, cx));
        self.mark_note.update(cx, |input, cx| input.set_value(note, window, cx));
        self.fill_axes(window, cx);
    }

    pub(super) fn fill_axes(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(position) = self.axes_for.as_ref().and_then(|target| shown_position(self.board.read(cx), target)) else { return };
        for (field, value) in self.mark_axes.iter().zip(position) {
            let text = axis(value);
            if *field.read(cx).value() != text {
                field.update(cx, |field, cx| field.set_value(text, window, cx));
            }
        }
    }

    /// A field that does not parse, or shows the value as it was, keeps it; a mark without a position gets one once
    /// typed.
    pub(super) fn place_mark(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(target) = self.axes_for.clone() else { return };
        let Some(old) = shown_position(self.board.read(cx), &target) else { return };
        let mut position = old;
        for (value, field) in position.iter_mut().zip(&self.mark_axes) {
            let text = field.read(cx).value().trim().replace(',', ".");
            if let Some(typed) = text.parse::<f64>().ok().filter(|typed| typed.is_finite())
                && axis(typed) != axis(*value)
            {
                *value = typed;
            }
        }
        if position != old {
            self.board.update(cx, |board, cx| board.place_mark(&target, position, cx));
        }
        self.fill_axes(window, cx);
    }

    /// With a frame, the typed axes; without one, a game position as it is.
    fn axes_row(&self, map: &Map, mark: &UserPin, cx: &Context<Self>) -> Option<AnyElement> {
        if map.frame.is_none() {
            return mark.position.map(|position| self.coordinates(position, cx).into_any_element());
        }
        let labels = [key::MAP_AXIS_X, key::MAP_AXIS_Y, key::MAP_AXIS_Z];
        let fields = labels.into_iter().zip(&self.mark_axes).map(|(label, field)| {
            h_flex().gap(px(4.)).child(mono(t(cx, label), TYPE_TAG).text_color(rgb(DIM))).child(number_field(field, 66., false))
        });
        Some(h_flex().gap(px(8.)).children(fields).into_any_element())
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
        // A copy of a map's pin carries its FOUND.
        let found = board.saved.found_of(mark).map(|found| {
            let id = id.clone();
            found_button("mark-found", found, cx)
                .on_click(cx.listener(move |card, _, _, cx| card.board.update(cx, |board, cx| board.toggle_mark_found(&id, cx))))
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
        // A picture or files pasted into either field go to the mark.
        let name = Input::new(&self.mark_name).appearance(false).px_0().on_paste(paste_into(self.board.clone()));
        let note = Textarea::new(&self.mark_note).appearance(false).on_paste(paste_into(self.board.clone()));
        let card = self
            .panel("mark-card")
            .p(px(14.))
            .child(head)
            .children(self.axes_row(map, mark, cx))
            .child(self.mark_media(mark, cx))
            .child(field().font_weight(FontWeight::BOLD).child(name))
            .child(field().child(note))
            .child(h_flex().gap(px(8.)).children(swatches))
            .child(h_flex().gap(px(8.)).children(found).children(teleport).child(delete))
            .child(mono(t(cx, key::MAP_PIN_DRAG), TYPE_TAG).text_color(rgb(FAINT)));
        Some(card.into_any_element())
    }
}
