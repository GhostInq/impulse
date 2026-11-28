//! Left of the map: the search, progress, quick filters, the user's pins, the hidden ones, regions, the categories and
//! their groups;
//! while something is typed, the places it finds. A view of its own: what moves on the map does not rebuild it.

mod hidden;
mod marks;
mod teleport;

use std::{collections::HashSet, rc::Rc, sync::Arc};

use client::map::state::{Tally, View};
use gpui_kit::{
    AnyElement, App, Context, Div, ElementId, Entity, Focusable, FontWeight, IntoElement, MouseButton, Render, RenderImage, SharedString,
    Stateful, Subscription, Window,
    component::{
        h_flex,
        input::{InputEvent, InputState},
        v_flex,
    },
    div,
    prelude::*,
    px, relative, rgb,
};
use i18n::Arg;
use model::Map;
use rustc_hash::FxHashSet;

use super::{
    Maps,
    board::{Board, BoardEvent},
    kind,
    markers::{Images, ink, legible},
};
use crate::{
    Load,
    ui::{
        assets::Icon,
        key,
        text::{t, tf},
        theme::*,
        widgets::{filter, icon, input, list_row, mono, square},
    },
};

pub const WIDTH: f32 = 320.;
// Search results listed at once.
const LISTED: usize = 100;
const ICON: f32 = 16.;

pub struct Sidebar {
    maps: Entity<Maps>,
    board: Entity<Board>,
    search: Entity<InputState>,
    lift: Entity<InputState>,
    // The search's placeholder follows the language.
    _relabel: Subscription,
    folded: HashSet<usize>,
    regions_open: bool,
    marks_open: bool,
    hidden_open: bool,
    // What the search finds, names first; dropped when what shows changes.
    matches: Option<Rc<[usize]>>,
    images: Images,
    // Images to drop from the atlas when the sidebar is next drawn.
    stale: Vec<Arc<RenderImage>>,
}

impl Sidebar {
    pub fn new(maps: Entity<Maps>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let board = maps.read(cx).board().clone();
        let (search, relabel) = input(key::MAP_SEARCH, window, cx);
        cx.subscribe(&search, |sidebar, search, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                let query = search.read(cx).value().trim().to_lowercase();
                sidebar.board.update(cx, |board, cx| board.set_query(query, cx));
            }
        })
        .detach();
        cx.observe_in(&board, window, |sidebar, _, window, cx| {
            sidebar.follow_lift(window, cx);
            cx.notify();
        })
        .detach();
        cx.subscribe(&board, |sidebar, _, event: &BoardEvent, _| match event {
            BoardEvent::Installed { first, .. } => {
                sidebar.stale.extend(sidebar.images.clear());
                sidebar.matches = None;
                if *first {
                    sidebar.folded.clear();
                }
            }
            BoardEvent::Filtered => sidebar.matches = None,
            _ => {}
        })
        .detach();
        let lift = teleport::lift_field(board.read(cx).lift(), window, cx);
        Self {
            maps,
            board,
            search,
            lift,
            _relabel: relabel,
            folded: HashSet::new(),
            regions_open: false,
            marks_open: true,
            hidden_open: false,
            matches: None,
            images: Images::default(),
            stale: Vec::new(),
        }
    }

    pub fn focus_search(&self, window: &mut Window, cx: &mut App) {
        self.search.update(cx, |search, cx| search.focus(window, cx));
    }

    pub fn has_focus(&self, window: &Window, cx: &App) -> bool {
        [&self.search, &self.lift].iter().any(|field| field.focus_handle(cx).is_focused(window))
    }

    pub fn hide(&mut self, window: &mut Window) {
        self.matches = None;
        self.stale.extend(self.images.clear());
        for image in self.stale.drain(..) {
            let _ = window.drop_image(image);
        }
    }

    fn filters(&mut self, map: &Map, board: &Board, window: &Window, cx: &Context<Self>) -> Vec<AnyElement> {
        let mut rows = vec![progress(board, cx).into_any_element()];
        if map.unfinished {
            let notice = h_flex().gap(px(8.)).mx(px(14.)).mb(px(8.)).child(icon(Icon::Warn, 12., rgb(WARN)));
            let text = div().text_size(px(TYPE_SMALL)).text_color(rgb(MUTED)).child(t(cx, key::MAP_UNFINISHED));
            rows.push(notice.child(text).into_any_element());
        }
        rows.push(quick_filters(map, board, cx).into_any_element());
        if teleport::lift_shown(board) {
            rows.push(self.lift_row(cx).into_any_element());
        }
        rows.extend(self.marks_section(board, cx));
        rows.extend(self.hidden_section(map, board, cx));
        rows.extend(self.regions(map, board, cx));
        for index in 0..map.categories.len() {
            rows.extend(self.category(map, board, index, window, cx));
        }
        rows
    }

    fn regions(&self, map: &Map, board: &Board, cx: &Context<Self>) -> Vec<AnyElement> {
        // Regions of one name are a region in parts: one row.
        let mut seen = HashSet::new();
        let names: Vec<(&str, u32)> = map
            .regions
            .iter()
            .filter(|region| !region.name.is_empty() && seen.insert(region.name.as_str()))
            .map(|region| (region.name.as_str(), region.color))
            .collect();
        if names.is_empty() {
            return Vec::new();
        }
        let open = self.regions_open;
        let header = section("map-regions", open, &t(cx, key::MAP_REGIONS), names.len().to_string(), None, cx).on_click(cx.listener(
            |sidebar, _, _, cx| {
                sidebar.regions_open ^= true;
                cx.notify();
            },
        ));
        let mut rows = vec![header.into_any_element()];
        if open {
            for (index, (name, color)) in names.into_iter().enumerate() {
                let parts: Vec<String> = map.regions.iter().filter(|region| region.name == name).map(|region| region.id.clone()).collect();
                let shown = parts.iter().any(|part| !board.saved.view().hidden_regions.contains(part));
                let row = item(("map-region", index), check(color, shown), name, shown).pl(px(32.));
                let toggle = cx.listener(move |sidebar, _, _, cx| flip(&sidebar.board, |view| &mut view.hidden_regions, &parts, shown, cx));
                rows.push(row.on_click(toggle).into_any_element());
            }
        }
        rows
    }

    fn category(&mut self, map: &Map, board: &Board, index: usize, window: &Window, cx: &Context<Self>) -> Vec<AnyElement> {
        let category = &map.categories[index];
        let ids: Vec<String> = category.groups.iter().map(|group| group.id.clone()).collect();
        let shown = ids.iter().any(|id| !board.saved.view().hidden_groups.contains(id));
        let counted: Tally = board.tally[index].iter().sum();
        let open = !self.folded.contains(&index);
        let toggle = cx.listener(move |sidebar, _, _, cx| flip(&sidebar.board, |view| &mut view.hidden_groups, &ids, shown, cx));
        let mark = div().id(("map-category-check", index)).child(check(category.color, shown)).on_click(move |event, window, cx| {
            cx.stop_propagation();
            toggle(event, window, cx);
        });
        let title = category.name.to_uppercase();
        let header = section(("map-category", index), open, &title, tally(counted), Some(mark.into_any_element()), cx).on_click(
            cx.listener(move |sidebar, _, _, cx| {
                if !sidebar.folded.remove(&index) {
                    sidebar.folded.insert(index);
                }
                cx.notify();
            }),
        );
        let mut rows = vec![header.into_any_element()];
        if open {
            let cells: Vec<_> = (0..category.groups.len()).map(|group| self.group_cell(map, board, (index, group), window, cx)).collect();
            rows.push(
                div().id(("map-groups", index)).grid().grid_cols(2).gap_x(px(4.)).px(px(10.)).pb(px(6.)).children(cells).into_any_element(),
            );
        }
        rows
    }

    fn group_cell(&mut self, map: &Map, board: &Board, at: (usize, usize), window: &Window, cx: &Context<Self>) -> AnyElement {
        let category = &map.categories[at.0];
        let group = &category.groups[at.1];
        let shown = !board.saved.view().hidden_groups.contains(&group.id);
        let color = if shown { legible(category.color) } else { GHOST };
        let glyph = self.images.glyph(board.icon(at, cx), at, color, ICON, window, cx);
        let (toggle_id, only_id) = (group.id.clone(), group.id.clone());
        let counted = tally(board.tally[at.0][at.1]);
        list_row(("map-group", at.1), false)
            .h(px(32.))
            .px(px(6.))
            .gap(px(9.))
            .text_color(rgb(if shown { TEXT_2 } else { GHOST }))
            .child(glyph)
            .child(div().flex_1().min_w_0().truncate().text_size(px(TYPE_BODY)).child(group.name.clone()))
            .child(mono(counted, TYPE_CAPTION).text_color(rgb(if shown { DIM } else { GHOST })))
            .on_click(cx.listener(move |sidebar, _, _, cx| {
                sidebar.board.update(cx, |board, cx| {
                    board.filter(
                        |view| {
                            if !view.hidden_groups.remove(&toggle_id) {
                                view.hidden_groups.insert(toggle_id.clone());
                            }
                        },
                        cx,
                    )
                })
            }))
            // A right click shows this group alone.
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |sidebar, _, _, cx| {
                    sidebar.board.update(cx, |board, cx| {
                        let Load::Ready(map) = &board.map else { return };
                        let others = map.categories.iter().flat_map(|category| &category.groups).filter(|group| group.id != only_id);
                        let hidden = others.map(|group| group.id.clone()).collect();
                        board.filter(|view| view.hidden_groups = hidden, cx);
                    })
                }),
            )
            .into_any_element()
    }

    fn results(&mut self, map: &Map, board: &Board, window: &Window, cx: &Context<Self>) -> Vec<AnyElement> {
        let matching = self.matches(map, board);
        let places = matching.len().to_string();
        let places = tf(cx, key::MAP_PLACES, &[("places", Arg::Str(&places)), ("count", Arg::Num(matching.len() as f64))]);
        let header = mono(places, TYPE_CAPTION).text_color(rgb(DIM)).px(px(14.)).pt(px(12.)).pb(px(6.));
        let mut rows = vec![header.into_any_element()];
        if matching.is_empty() {
            let empty = div().px(px(14.)).text_size(px(TYPE_SMALL)).text_color(rgb(FAINT)).child(t(cx, key::MAP_NO_MATCHES));
            rows.push(empty.into_any_element());
        }
        for &index in matching.iter().take(LISTED) {
            let pin = &map.pins[index];
            let category = &map.categories[pin.group.0];
            let badge = h_flex().size(px(28.)).flex_none().justify_center().bg(rgb(category.color));
            let glyph = self.images.glyph(board.icon(pin.group, cx), pin.group, ink(category.color), ICON, window, cx);
            let row = list_row(("map-found", index), board.selected == Some(index))
                .px(px(14.))
                .py(px(6.))
                .gap(px(10.))
                .child(badge.child(glyph))
                .child(
                    v_flex()
                        .flex_1()
                        .min_w_0()
                        .child(div().truncate().text_size(px(TYPE_LEAD)).text_color(rgb(TEXT_2)).child(pin.name.clone()))
                        .child(mono(kind(map, pin), TYPE_CAPTION).truncate().text_color(rgb(FAINT))),
                )
                .when(board.saved.found(&pin.id), |row| row.child(icon(Icon::Check, 12., rgb(accent(cx)))))
                .on_click(cx.listener(move |sidebar, _, _, cx| sidebar.board.update(cx, |board, cx| board.select(index, true, cx))));
            rows.push(row.into_any_element());
        }
        rows
    }

    /// The pins the search finds, names first; worked out once per query and filter.
    fn matches(&mut self, map: &Map, board: &Board) -> Rc<[usize]> {
        if let Some(matches) = &self.matches {
            return matches.clone();
        }
        let mut matching: Vec<usize> = (0..map.pins.len()).filter(|&index| board.shows(map, index)).collect();
        matching.sort_by_cached_key(|&index| !board.named(index));
        let matches: Rc<[usize]> = matching.into();
        self.matches = Some(matches.clone());
        matches
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        for image in self.stale.drain(..) {
            let _ = window.drop_image(image);
        }
        let board = self.board.clone();
        let board = board.read(cx);
        let body = match &board.map {
            Load::Ready(map) if !board.searching() => self.filters(map, board, window, cx),
            Load::Ready(map) => self.results(map, board, window, cx),
            _ => Vec::new(),
        };
        v_flex()
            .size_full()
            .border_r_1()
            .border_color(line(0.09))
            .bg(rgb(PANEL))
            .child(filter(&self.search, cx).h(px(40.)).flex_none().px(px(14.)).border_b_1().border_color(line(0.09)))
            .child(v_flex().id("map-sidebar").flex_1().min_h_0().pb(px(10.)).overflow_y_scroll().children(body))
    }
}

fn progress(board: &Board, cx: &App) -> Div {
    let Tally { found, total } = board.tally.iter().flatten().sum();
    let share = if total == 0 { 0. } else { found as f32 / total as f32 };
    v_flex()
        .gap(px(6.))
        .px(px(14.))
        .pt(px(12.))
        .pb(px(10.))
        .child(
            h_flex()
                .justify_between()
                .child(mono(t(cx, key::MAP_FOUND_COUNT), TYPE_CAPTION).text_color(rgb(DIM)))
                .child(mono(format!("{found} / {total}"), TYPE_LABEL).text_color(rgb(if found > 0 { accent(cx) } else { MUTED }))),
        )
        .child(div().h(px(3.)).bg(line(0.08)).child(div().h_full().w(relative(share)).bg(rgb(accent(cx)))))
}

fn quick_filters(map: &Map, board: &Board, cx: &Context<Sidebar>) -> impl IntoElement {
    let view = board.saved.view();
    let all_hidden = map.categories.iter().flat_map(|category| &category.groups).all(|group| view.hidden_groups.contains(&group.id));
    let cell = |id: &'static str, label: SharedString, on: bool, enabled: bool| {
        let color = match (enabled, on) {
            (false, _) => GHOST,
            (true, true) => BG,
            (true, false) => MUTED,
        };
        div()
            .id(id)
            .flex_1()
            .h(px(30.))
            .flex()
            .items_center()
            .justify_center()
            .border_1()
            .border_color(line(0.14))
            .when(on && enabled, |cell| cell.bg(rgb(accent(cx))).border_color(rgb(accent(cx))))
            .when(enabled, |cell| cell.cursor_pointer().hover(|style| style.border_color(line(0.3))))
            .child(mono(label, TYPE_CAPTION).font_weight(FontWeight::BOLD).text_color(rgb(color)))
    };
    let change = |change: fn(&mut View)| {
        cx.listener(move |sidebar: &mut Sidebar, _, _, cx| sidebar.board.update(cx, |board, cx| board.filter(change, cx)))
    };
    let toggle_all = cx.listener(move |sidebar, _, _, cx| {
        sidebar.board.update(cx, |board, cx| {
            let Load::Ready(map) = &board.map else { return };
            let ids = map.categories.iter().flat_map(|category| &category.groups).map(|group| group.id.clone());
            let hidden = if all_hidden { Default::default() } else { ids.collect() };
            board.filter(|view| view.hidden_groups = hidden, cx);
        })
    });
    h_flex()
        .gap(px(6.))
        .px(px(14.))
        .pb(px(12.))
        .child(cell("map-all", t(cx, if all_hidden { key::MAP_SHOW_ALL } else { key::MAP_HIDE_ALL }), false, true).on_click(toggle_all))
        .child(
            cell("map-teleportable", t(cx, key::MAP_FILTER_TELEPORT), view.teleport_only, map.teleport)
                .when(map.teleport, |cell| cell.on_click(change(|view| view.teleport_only ^= true))),
        )
        .child(cell("map-unfound", t(cx, key::MAP_UNFOUND), view.unfound_only, true).on_click(change(|view| view.unfound_only ^= true)))
        .child(
            cell("map-refresh", t(cx, key::MAP_REFRESH), false, true)
                .on_click(cx.listener(|sidebar, _, _, cx| sidebar.maps.update(cx, |maps, cx| maps.refresh(cx)))),
        )
}

fn tally(Tally { found, total }: Tally) -> String {
    if found > 0 { format!("{found}/{total}") } else { total.to_string() }
}

/// A box in `color`, filled while what it stands for shows.
fn check(color: u32, on: bool) -> Div {
    let color = legible(color);
    square(12., alpha(color, if on { 1. } else { 0. })).border_1().border_color(if on { alpha(color, 1.) } else { line(0.3) })
}

/// A foldable section's header: chevron, title, a note, and an optional control on the right.
fn section(id: impl Into<ElementId>, open: bool, title: &str, note: String, control: Option<AnyElement>, cx: &App) -> Stateful<Div> {
    list_row(id, false)
        .h(px(34.))
        .px(px(14.))
        .gap(px(8.))
        .child(icon(if open { Icon::ChevronDown } else { Icon::ChevronRight }, 11., rgb(if open { accent(cx) } else { DIM })))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .truncate()
                .font_weight(FontWeight::BOLD)
                .text_size(px(TYPE_FIELD))
                .text_color(rgb(TEXT))
                .child(title.to_owned()),
        )
        .child(mono(note, TYPE_CAPTION).text_color(rgb(DIM)))
        .children(control)
}

fn item(id: impl Into<ElementId>, mark: Div, name: &str, shown: bool) -> Stateful<Div> {
    list_row(id, false)
        .h(px(28.))
        .pr(px(14.))
        .gap(px(10.))
        .text_color(rgb(if shown { TEXT_2 } else { FAINT }))
        .child(mark)
        .child(div().flex_1().min_w_0().truncate().text_size(px(TYPE_BODY)).child(name.to_owned()))
}

/// A check's click: `ids` hide while any shows, else they all show.
fn flip(board: &Entity<Board>, set: fn(&mut View) -> &mut FxHashSet<String>, ids: &[String], shown: bool, cx: &mut App) {
    board.update(cx, |board, cx| {
        board.filter(
            |view| {
                let set = set(view);
                for id in ids {
                    if shown {
                        set.insert(id.clone());
                    } else {
                        set.remove(id);
                    }
                }
            },
            cx,
        )
    });
}
