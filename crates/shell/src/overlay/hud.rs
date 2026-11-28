//! The overlay's layout over the game: the game's title at the top left, its run's button at the top right,
//! the docks' toggles between them, and the docks with the slices' views in them.

use client::{library::Game, storage::Paths};
use gpui_kit::{
    App, Bounds, Context, Div, Entity, EventEmitter, FocusHandle, IntoElement, KeyBinding, Pixels, Render, Size, Window, actions,
    component::{h_flex, scroll::ScrollableElement, v_flex},
    div, point,
    prelude::*,
    px, size,
};

use super::{
    Layer, Pressed, Slot,
    docks::{self, Docks, Spec},
    glance::Glance,
    in_game::InGame,
    shot,
    warning::{Closed, Warning, chip},
};
use crate::{
    cheats::{CheatList, EditorClosed},
    hotkeys::{Hotkeys, hotkeys_scope},
    library::{GamePicker, Library, LibraryEvent, vacant},
    map::{MapView, Maps, Minimap},
    run::{Ask, AskClosed, Runs, run_button, session_panel},
    settings::Settings,
    trainer::{Trainers, title},
    ui::{
        assets::Icon,
        key,
        text::t,
        theme::*,
        widgets::{icon_button, notice},
    },
};

actions!(hud, [LeaveOverlay]);

const CONTEXT: &str = "hud";
const MODS: &str = "mods";
const MAP: &str = "map";
const SESSION: &str = "session";
const MINIMAP: &str = "minimap";
// The top bar's height, where the docks start; the screen edge's margin; the first columns' widths.
const TOP: f32 = 88.;
const EDGE: f32 = 24.;
const GAP: f32 = 16.;
const LIST: f32 = 560.;
const SIDE: f32 = 320.;
// A zoom hotkey's step, in map levels: the span is seven, so a dozen presses cross it.
const ZOOM_STEP: f64 = 0.5;

static MODS_DOCK: Spec = Spec {
    id: MODS,
    title: key::OVERLAY_DOCK_MODS,
    icon: Icon::Mods,
    first: list_first,
    smallest: Some(size(px(420.), px(240.))),
    shown: true,
    body_moves: false,
};

static MAP_DOCK: Spec = Spec {
    id: MAP,
    title: key::OVERLAY_DOCK_MAP,
    icon: Icon::Map,
    first: map_first,
    smallest: Some(size(px(360.), px(280.))),
    shown: true,
    body_moves: false,
};

static SESSION_DOCK: Spec = Spec {
    id: SESSION,
    title: key::OVERLAY_DOCK_SESSION,
    icon: Icon::Clock,
    first: side_first,
    smallest: None,
    shown: true,
    body_moves: false,
};

// Also over the game while Impulse is behind it.
pub(super) static MINIMAP_DOCK: Spec = Spec {
    id: MINIMAP,
    title: key::OVERLAY_DOCK_MINIMAP,
    icon: Icon::Player,
    first: minimap_first,
    smallest: Some(size(px(200.), px(200.))),
    shown: false,
    body_moves: true,
};

/// The toolbar's order.
pub(super) static DOCKS: [&Spec; 4] = [&MODS_DOCK, &MAP_DOCK, &SESSION_DOCK, &MINIMAP_DOCK];

fn list_first(screen: Size<Pixels>) -> Bounds<Pixels> {
    Bounds::new(point(px(EDGE), px(TOP)), size(px(LIST), screen.height - px(TOP + EDGE)))
}

fn map_first(screen: Size<Pixels>) -> Bounds<Pixels> {
    let left = px(EDGE + LIST + GAP);
    Bounds::new(point(left, px(TOP)), size(screen.width - left - px(SIDE + GAP + EDGE), screen.height - px(TOP + EDGE)))
}

fn side_first(screen: Size<Pixels>) -> Bounds<Pixels> {
    Bounds::new(point(screen.width - px(SIDE + EDGE), px(TOP)), size(px(SIDE), px(250.)))
}

fn minimap_first(screen: Size<Pixels>) -> Bounds<Pixels> {
    Bounds::new(point(screen.width - px(SIDE + EDGE), screen.height - px(SIDE + EDGE)), size(px(SIDE), px(SIDE)))
}

pub fn bind_keys(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("escape", LeaveOverlay, Some(CONTEXT))]);
}

/// The window went over a game: what belongs to the window closes.
pub struct Entered;

/// The overlay's view, drawn in place of the page while over a game.
pub struct Hud {
    layer: Entity<Layer>,
    library: Entity<Library>,
    trainers: Entity<Trainers>,
    runs: Entity<Runs>,
    ask: Entity<Ask>,
    hotkeys: Entity<Hotkeys>,
    maps: Entity<Maps>,
    map: Entity<MapView>,
    cheat_list: Entity<CheatList>,
    /// The NO TRAINER card's OTHER TRAINER opens it here too.
    picker: Entity<GamePicker>,
    docks: Entity<Docks>,
    minimap: Entity<Minimap>,
    // The HUD window over the game while Impulse is behind it.
    _glance: Entity<Glance>,
    // PIN HERE's shots hide its layers.
    in_game: Entity<InGame>,
    warning: Entity<Warning>,
    over: bool,
    focus: FocusHandle,
}

impl EventEmitter<Entered> for Hud {}

impl Hud {
    #[expect(clippy::too_many_arguments, reason = "the slices the overlay draws")]
    pub fn new(
        layer: Entity<Layer>,
        library: Entity<Library>,
        trainers: Entity<Trainers>,
        runs: Entity<Runs>,
        ask: Entity<Ask>,
        hotkeys: Entity<Hotkeys>,
        maps: Entity<Maps>,
        map: Entity<MapView>,
        cheat_list: Entity<CheatList>,
        picker: Entity<GamePicker>,
        in_game: Entity<InGame>,
        settings: &Entity<Settings>,
        paths: &Paths,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let docks = cx.new(|_| Docks::load(paths, &DOCKS));
        let live_map = maps.read(cx).live_map().clone();
        let glance = cx.new(|cx| Glance::new(&layer, in_game.clone(), docks.clone(), live_map.clone(), settings, cx));
        let minimap = cx.new(|cx| Minimap::new(live_map.clone(), cx));
        let warning = cx.new(|cx| Warning::new(settings.clone(), cx));
        cx.observe(&warning, |_, _, cx| cx.notify()).detach();
        cx.subscribe_in(&warning, window, |hud, _, _: &Closed, window, cx| {
            // The focus must not stay in the gone dialog, or Escape would reach nothing.
            hud.focus.focus(window, cx);
        })
        .detach();
        cx.observe(&ask, |_, _, cx| cx.notify()).detach();
        cx.subscribe_in(&ask, window, |hud, _, _: &AskClosed, window, cx| {
            if hud.over {
                hud.focus.focus(window, cx);
            }
        })
        .detach();
        cx.observe(&library, |_, _, cx| cx.notify()).detach();
        cx.observe(&trainers, |_, _, cx| cx.notify()).detach();
        cx.observe(&runs, |_, _, cx| cx.notify()).detach();
        cx.observe(&hotkeys, |_, _, cx| cx.notify()).detach();
        cx.observe(&cheat_list, |_, _, cx| cx.notify()).detach();
        cx.observe_in(&layer, window, |hud, layer, window, cx| {
            let over = layer.read(cx).is_over();
            if over != hud.over {
                hud.switched(over, window, cx);
            }
            cx.notify();
        })
        .detach();
        cx.subscribe(&layer, |hud, _, &Pressed(slot), cx| match slot {
            Slot::Minimap => hud.minimap_key(cx),
            Slot::ZoomIn => hud.maps.update(cx, |maps, cx| maps.zoom_minimap(ZOOM_STEP, cx)),
            Slot::ZoomOut => hud.maps.update(cx, |maps, cx| maps.zoom_minimap(-ZOOM_STEP, cx)),
            Slot::PinHere => hud.pin_here(cx),
        })
        .detach();
        cx.observe_in(&docks, window, |hud, _, window, cx| {
            hud.place_map(window, cx);
            hud.dock_minimap(cx);
            hud.let_go_minimap(window, cx);
            cx.notify();
        })
        .detach();
        // Another game's map closes by itself (`MapView`); over a game the dock opens the new game's, whichever
        // of the two hears `Opened` first.
        cx.subscribe_in(&library, window, |hud, _, event: &LibraryEvent, window, cx| {
            if matches!(event, LibraryEvent::Opened) && hud.over {
                // The old game's map may have gone with the focus. `place_map` ends with the warning's refocus, so
                // the question keeps the focus (the map's Escape would leave the overlay with it still up).
                if !hud.docked(cx) {
                    hud.focus.focus(window, cx);
                }
                hud.place_map(window, cx);
            }
        })
        .detach();
        cx.observe_in(&maps, window, |hud, _, window, cx| hud.place_map(window, cx)).detach();
        cx.observe_in(&live_map, window, |hud, _, window, cx| hud.let_go_minimap(window, cx)).detach();
        // The focus must not stay in the gone editor, or Escape would reach nothing.
        cx.subscribe_in(&cheat_list, window, |hud, _, _: &EditorClosed, window, cx| {
            if hud.over {
                hud.focus.focus(window, cx);
            }
        })
        .detach();
        // Nor in the gone picker.
        cx.observe_in(&picker, window, |hud, picker, window, cx| {
            if hud.over && !picker.read(cx).is_open() {
                hud.focus.focus(window, cx);
            }
            cx.notify();
        })
        .detach();
        let hud = Self {
            layer,
            library,
            trainers,
            runs,
            ask,
            hotkeys,
            maps,
            map,
            cheat_list,
            picker,
            docks,
            minimap,
            _glance: glance,
            in_game,
            warning,
            over: false,
            focus: cx.focus_handle(),
        };
        hud.dock_minimap(cx);
        hud
    }

    fn switched(&mut self, over: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.over = over;
        self.map.update(cx, |map, cx| map.set_docked(over, cx));
        self.let_go_minimap(window, cx);
        if !over {
            self.warning.update(cx, |warning, cx| warning.hide(cx));
            return self.map.update(cx, |map, cx| map.hide(window, cx));
        }
        // What belongs to the window closes, and the docks show the running game.
        self.cheat_list.update(cx, |list, cx| list.close(cx));
        self.hotkeys.update(cx, |hotkeys, cx| hotkeys.cancel(cx));
        cx.emit(Entered);
        let live = self.runs.read(cx).live().filter(|&live| self.library.read(cx).open_id() != Some(live)).cloned();
        match live {
            // Its `Opened` places the map.
            Some(live) => self.library.update(cx, |library, cx| library.open(live, cx)),
            None => self.place_map(window, cx),
        }
        // An open map keeps the focus: its Escape closes its own things first, then leaves.
        if !self.maps.read(cx).is_open(cx) {
            self.focus.focus(window, cx);
        }
        // Last: the question takes the focus from the above.
        if self.layer.read(cx).exclusive() {
            self.warning.update(cx, |warning, cx| warning.entered(window, cx));
        }
    }

    /// The minimap follows the live run while its dock is shown.
    fn dock_minimap(&self, cx: &mut Context<Self>) {
        let wanted = self.docks.read(cx).is_shown(&MINIMAP_DOCK);
        self.maps.update(cx, |maps, cx| maps.set_minimap(wanted, cx));
    }

    /// The map places the pin; the shot is the overlay's, given to it, so the map never names the overlay.
    fn pin_here(&mut self, cx: &mut Context<Self>) {
        let Some(pid) = self.runs.read(cx).pid() else { return };
        let in_game = self.in_game.clone();
        self.maps.update(cx, |maps, cx| maps.pin_here(|cx| shot::shot(pid, &in_game, cx), cx));
    }

    /// Only a live game with a map has a minimap to toggle.
    fn minimap_key(&mut self, cx: &mut Context<Self>) {
        let live = self.runs.read(cx).live().and_then(|live| self.library.read(cx).game(live));
        if live.is_some_and(|game| !game.source.maps.is_empty()) {
            self.docks.update(cx, |docks, cx| docks.toggle(&MINIMAP_DOCK, cx));
        }
    }

    /// The dock's minimap lets go of its tiles once it is not drawn: out of the overlay, its dock closed, or no
    /// live map.
    fn let_go_minimap(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let drawn = self.over && self.docks.read(cx).is_shown(&MINIMAP_DOCK) && self.maps.read(cx).live_map().read(cx).ready(cx);
        if !drawn {
            self.minimap.update(cx, |minimap, cx| minimap.hide(window, cx));
        }
    }

    /// Over a game, the map is open while its dock shows and the game has one. Out of the overlay the open map
    /// is the page's modal.
    fn place_map(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.over {
            return;
        }
        match (self.docked(cx), self.maps.read(cx).is_open(cx)) {
            (true, false) => self.map.update(cx, |map, cx| map.show(window, cx)),
            (false, true) => {
                self.map.update(cx, |map, cx| map.hide(window, cx));
                // The focus must not stay in the gone map, or Escape would reach nothing.
                self.focus.focus(window, cx);
            }
            _ => {}
        }
        self.warning.update(cx, |warning, cx| warning.refocus(window, cx));
    }

    fn docked(&self, cx: &App) -> bool {
        self.docks.read(cx).is_shown(&MAP_DOCK) && self.maps.read(cx).mapped()
    }

    fn leave_overlay(&mut self, _: &LeaveOverlay, window: &mut Window, cx: &mut Context<Self>) {
        // An open dialog or menu closes first; the warning even when a late `Opened` took the focus from it.
        if self.warning.update(cx, |warning, cx| warning.hide(cx)) {
            self.focus.focus(window, cx);
        } else if !self.cheat_list.update(cx, |list, cx| list.close_top(window, cx)) {
            self.layer.read(cx).leave();
        }
    }

    fn session(&self, game: &Game, cx: &App) -> Div {
        session_panel(self.runs.read(cx), self.trainers.read(cx), game, false, hotkeys_scope(&self.hotkeys, cx), cx)
    }
}

impl Render for Hud {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let game = self.library.read(cx).opened().cloned();
        let game = game.as_deref();
        // The minimap follows a record's maps alone: a discovered map has no `Frame`.
        let (mapped, minimapped) = (self.maps.read(cx).mapped(), game.is_some_and(|game| !game.source.maps.is_empty()));
        let shown = self.docks.read(cx).shown();
        let docked = shown
            .into_iter()
            .filter_map(|spec| {
                let content = match (spec.id, game) {
                    (MODS, Some(_)) => {
                        div().id(MODS).size_full().p(px(16.)).overflow_y_scrollbar().child(self.cheat_list.clone()).into_any_element()
                    }
                    (MODS, None) => div().p(px(16.)).child(vacant(self.library.read(cx), cx)).into_any_element(),
                    (MAP, Some(_)) if mapped => self.map.clone().into_any_element(),
                    (MINIMAP, Some(_)) if minimapped && self.maps.read(cx).live_map().read(cx).ready(cx) => {
                        self.minimap.clone().into_any_element()
                    }
                    (MINIMAP, Some(_)) if minimapped => div()
                        .p(px(16.))
                        .child(notice(
                            t(cx, key::OVERLAY_MINIMAP_TAG),
                            t(cx, key::OVERLAY_NO_LIVE_MAP),
                            t(cx, key::OVERLAY_NO_LIVE_MAP_HINT),
                        ))
                        .into_any_element(),
                    (SESSION, Some(game)) if game.installed() => div().p(px(16.)).child(self.session(game, cx)).into_any_element(),
                    _ => return None,
                };
                Some((spec, content))
            })
            .collect();
        let frames = docks::frame(&self.docks, docked, window, cx);
        let layer = self.layer.clone();
        let bar = h_flex()
            .absolute()
            .top_0()
            .left_0()
            .w_full()
            .h(px(TOP - GAP))
            .px(px(EDGE))
            .gap(px(GAP))
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .items_start()
                    .gap(px(4.))
                    // With the warning the group is taller than the bar: from the top it still ends above the docks.
                    .when(self.layer.read(cx).exclusive(), |left| left.self_start())
                    .when_some(game, |left, game| {
                        left.child(
                            title(self.trainers.read(cx), game, TYPE_DISPLAY, cx)
                                .max_w_full()
                                .px(px(14.))
                                .py(px(8.))
                                .bg(alpha(BG, 0.85))
                                .border_1()
                                .border_color(line(0.1)),
                        )
                    })
                    .when(self.layer.read(cx).exclusive(), |left| left.child(chip(cx))),
            )
            .child(
                h_flex()
                    .gap(px(4.))
                    .child(docks::toolbar(&self.docks, cx, |spec| match spec.id {
                        MAP => mapped,
                        MINIMAP => minimapped,
                        _ => true,
                    }))
                    .child(div().w(px(1.)).h(px(24.)).mx(px(6.)).bg(line(0.16)))
                    .child(
                        icon_button("hud-close", Icon::Close, 40., 14.)
                            .border_1()
                            .border_color(line(0.12))
                            .bg(alpha(BG, 0.85))
                            .on_click(move |_, _, cx| layer.read(cx).leave()),
                    ),
            )
            .child(h_flex().flex_1().justify_end().when_some(game, |right, game| right.child(run_button(&self.runs, &self.ask, game, cx))));
        div()
            .id("hud")
            .key_context(CONTEXT)
            .track_focus(&self.focus)
            .on_action(cx.listener(Self::leave_overlay))
            .relative()
            .size_full()
            .bg(alpha(0x000000, 0.4))
            .child(bar)
            .children(frames)
            .child(docks::input(&self.docks))
            .children(self.cheat_list.read(cx).editor().cloned())
            .children(Some(self.picker.clone()).filter(|picker| picker.read(cx).is_open()))
            .children(Some(self.warning.clone()).filter(|warning| warning.read(cx).is_open()))
            .children(Some(self.ask.clone()).filter(|ask| ask.read(cx).is_open()))
    }
}
