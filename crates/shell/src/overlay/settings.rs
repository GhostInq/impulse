//! The Settings `OVERLAY` tab and the overlay's and the extra hotkeys' rebinding.

use gpui_kit::{App, Context, Entity, IntoElement, Render, SharedString, Window, component::v_flex, prelude::*, px, rgb};
use i18n::Key as Message;
use keyhook::{Key, Outcome};
use model::Chord;

use super::{Layer, Slot, keys};
use crate::{
    hotkeys::{Hotkeys, Target},
    settings::{Settings, row},
    trainer::Trainers,
    ui::{
        key,
        text::t,
        theme::*,
        widgets::{choice, keycap, segmented, switch},
    },
};

const OVERLAY: Target = Target::Key { name: "overlay", required: true };

/// Each extra hotkey's capture target, cap id, title and hint.
const EXTRA_ROWS: [(Slot, Target, &str, Message, Message); Slot::ALL.len()] = [
    (
        Slot::Minimap,
        Target::Key { name: "minimap", required: false },
        "minimap-hotkey",
        key::OVERLAY_MINIMAP_HOTKEY,
        key::OVERLAY_MINIMAP_HOTKEY_HINT,
    ),
    (
        Slot::ZoomIn,
        Target::Key { name: "zoom-in", required: false },
        "zoom-in-hotkey",
        key::OVERLAY_ZOOM_IN_HOTKEY,
        key::OVERLAY_ZOOM_IN_HOTKEY_HINT,
    ),
    (
        Slot::ZoomOut,
        Target::Key { name: "zoom-out", required: false },
        "zoom-out-hotkey",
        key::OVERLAY_ZOOM_OUT_HOTKEY,
        key::OVERLAY_ZOOM_OUT_HOTKEY_HINT,
    ),
    (
        Slot::PinHere,
        Target::Key { name: "pin-here", required: false },
        "pin-here-hotkey",
        key::OVERLAY_PIN_HERE_HOTKEY,
        key::OVERLAY_PIN_HERE_HOTKEY_HINT,
    ),
];

pub struct OverlaySettings {
    layer: Entity<Layer>,
    hotkeys: Entity<Hotkeys>,
    trainers: Entity<Trainers>,
    settings: Entity<Settings>,
}

impl OverlaySettings {
    pub fn new(
        layer: Entity<Layer>,
        hotkeys: Entity<Hotkeys>,
        trainers: Entity<Trainers>,
        settings: Entity<Settings>,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe(&layer, |_, _, cx| cx.notify()).detach();
        cx.observe(&hotkeys, |_, _, cx| cx.notify()).detach();
        cx.observe(&settings, |_, _, cx| cx.notify()).detach();
        Self { layer, hotkeys, trainers, settings }
    }

    fn hotkey_cap(
        &self,
        id: &'static str,
        target: Target,
        slot: Option<Slot>,
        key: Option<Key>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let held = self.hotkeys.read(cx).held(&target);
        let text: SharedString = match held {
            Some(held) => held.map_or_else(|| t(cx, key::HOTKEYS_PRESS_KEYS), |chord| chord.to_string().into()),
            None => key_text(key),
        };
        let accent = rgb(accent(cx));
        keycap(id, text)
            // A long preview while binding truncates instead of covering the label.
            .max_w(px(220.))
            .text_size(px(TYPE_SMALL))
            .px(px(10.))
            .py(px(5.))
            .cursor_pointer()
            .hover(|style| style.border_color(line(0.3)))
            .when(held.is_some(), |cap| {
                cap.border_color(accent)
                    .text_color(accent)
                    .on_mouse_down_out(cx.listener(|view, _, _, cx| view.hotkeys.update(cx, |hotkeys, cx| hotkeys.cancel(cx))))
            })
            .on_click(cx.listener(move |view, _, window, cx| view.bind(target.clone(), slot, window, cx)))
    }

    /// Binds the overlay's hotkey (`slot` none) or an extra one by what the user presses.
    fn bind(&mut self, target: Target, slot: Option<Slot>, window: &mut Window, cx: &mut Context<Self>) {
        let Some(outcome) = self.hotkeys.update(cx, |hotkeys, cx| hotkeys.capture(target, window, cx)) else { return };
        cx.spawn(async move |view, cx| {
            let Ok(outcome) = outcome.await else { return };
            view.update(cx, |view, cx| match slot {
                None => view.rebind_overlay(outcome, cx),
                Some(slot) => view.rebind_extra(slot, outcome, cx),
            })
            .ok();
        })
        .detach();
    }

    /// A refused chord keeps the old hotkey; the tab's hint says what is allowed.
    fn rebind_overlay(&mut self, outcome: Outcome, cx: &mut Context<Self>) {
        let key = match outcome {
            Outcome::Bound(chord) => Key::from_vks(chord.vks()),
            Outcome::Reset => Some(keys::DEFAULT),
            _ => None,
        };
        if let Some(key) = key.filter(|&key| !self.extras_hold(key, None, cx) && !self.cheats_hold(key, cx)) {
            self.layer.update(cx, |layer, cx| layer.set_hotkey(key, cx));
        }
    }

    fn rebind_extra(&mut self, slot: Slot, outcome: Outcome, cx: &mut Context<Self>) {
        let key = match outcome {
            Outcome::Bound(chord) => match Key::from_vks(chord.vks()) {
                Some(key) => Some(key),
                None => return,
            },
            Outcome::Reset => Some(slot.default_key()),
            Outcome::Unbound => None,
            Outcome::Cancelled => return,
        };
        let taken = key
            .is_some_and(|key| key == self.layer.read(cx).hotkey() || self.extras_hold(key, Some(slot), cx) || self.cheats_hold(key, cx));
        if !taken {
            self.layer.update(cx, |layer, cx| layer.set_key(slot, key, cx));
        }
    }

    /// Whether an extra hotkey other than `except` is `key`.
    fn extras_hold(&self, key: Key, except: Option<Slot>, cx: &App) -> bool {
        let layer = self.layer.read(cx);
        Slot::ALL.into_iter().any(|slot| Some(slot) != except && layer.key(slot) == Some(key))
    }

    fn cheats_hold(&self, key: Key, cx: &App) -> bool {
        self.trainers.read(cx).sheets().any(|sheet| sheet.hotkeys.iter().flatten().flatten().any(|&bound| key.clashes(bound)))
    }
}

impl Render for OverlaySettings {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let layer = self.layer.read(cx);
        let overlay = Some(layer.hotkey());
        let extras = EXTRA_ROWS.map(|(slot, ..)| layer.key(slot));
        let extra_rows: Vec<_> = EXTRA_ROWS
            .into_iter()
            .zip(extras)
            .map(|((slot, target, id, title, hint), key)| {
                row(t(cx, title), t(cx, hint), 64.).child(self.hotkey_cap(id, target, Some(slot), key, cx))
            })
            .collect();
        let prefs = self.settings.read(cx).prefs();
        // On until DON'T SHOW AGAIN was pressed in the question itself.
        let (asked, in_game) = (!prefs.skip_exclusive_warning, prefs.overlay_in_game());
        let mode = |id: &'static str, label: Message, choose: bool| {
            choice(id, t(cx, label), in_game == choose, cx).on_click(cx.listener(move |view, _, _, cx| {
                view.settings.update(cx, |settings, cx| settings.edit(|prefs| prefs.set_overlay_in_game(choose), cx));
            }))
        };
        v_flex()
            .flex_1()
            .px(px(18.))
            .py(px(6.))
            .child(row(t(cx, key::OVERLAY_MODE), t(cx, key::OVERLAY_MODE_HINT), 64.).child(
                segmented().h(px(26.)).child(mode("overlay-over", key::OVERLAY_MODE_OVER, false)).child(mode(
                    "overlay-in-game",
                    key::OVERLAY_MODE_IN_GAME,
                    true,
                )),
            ))
            .child(row(t(cx, key::OVERLAY_HOTKEY), t(cx, key::OVERLAY_HOTKEY_HINT), 64.).child(self.hotkey_cap(
                "overlay-hotkey",
                OVERLAY,
                None,
                overlay,
                cx,
            )))
            .children(extra_rows)
            .child(row(t(cx, key::OVERLAY_EXCLUSIVE_TITLE), t(cx, key::OVERLAY_EXCLUSIVE_ASK_HINT), 64.).border_b_0().child(
                switch("exclusive-ask", asked, cx).on_click(cx.listener(move |view, _, _, cx| {
                    view.settings.update(cx, |settings, cx| settings.edit(|prefs| prefs.skip_exclusive_warning = asked, cx));
                })),
            ))
    }
}

/// The overlay's hotkey as the hero shows it.
pub fn overlay_hotkey(layer: &Layer) -> SharedString {
    key_text(Some(layer.hotkey()))
}

fn key_text(key: Option<Key>) -> SharedString {
    let vks: Vec<u16> = key.map(|key| key.vks().map(u16::from).collect()).unwrap_or_default();
    Chord::from_vks(&vks).map_or_else(|| "-".into(), |chord| chord.to_string().into())
}
