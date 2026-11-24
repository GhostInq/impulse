//! `+ ADD`: a game by its executable. The catalog games that keep their `exe` beside it are offered;
//! one is added at once, several or none open the picker, where the user finds it by name. The same picker chooses
//! the trainer of a listed game: it opens on the game's own records, and any other trainer is found by name. And it
//! is the search (Ctrl+K): a listed game or a catalog one to open, by name.

use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use futures::channel::oneshot;
use gpui_kit::{
    App, Context, Entity, FocusHandle, IntoElement, PathPromptOptions, Render, ScrollHandle, SharedString, Subscription, Window,
    component::{
        h_flex,
        input::{Input, InputEvent, InputState, MoveDown, MoveUp},
        scroll::ScrollableElement,
        v_flex,
    },
    div,
    prelude::*,
    px, rgb,
};
use i18n::Arg;
use model::Ref;

use super::{Found, Library};
use crate::ui::{
    dialog::dialog,
    key,
    text::{t, tf},
    theme::*,
    widgets::{badge, input, list_row, mono},
};
use client::{
    Load,
    library::{Find, GameId, by_name, rank, store_of},
};

/// Rows shown at once: the filter narrows the rest.
const SHOWN: usize = 50;

enum Purpose {
    /// `owned`: the choices are the games the executable belongs to.
    Add { exe: PathBuf, owned: bool },
    /// `current`: the record whose trainer the game uses now; `own`: the leading choices that are the game's own.
    Trainer { game: GameId, current: Option<Ref>, own: usize },
    /// A row per choice.
    Find { finds: Vec<Find> },
}

/// A row of `games[at]`, its texts made once.
struct Choice {
    at: usize,
    name: SharedString,
    /// Lower case, for the filter.
    search: String,
    store: SharedString,
}

impl Choice {
    fn of(games: &[model::Game], at: usize) -> Self {
        let record = &games[at];
        Self { at, name: record.name.to_uppercase().into(), search: record.name.to_lowercase(), store: store_of(record).into() }
    }
}

struct Picker {
    purpose: Purpose,
    games: Arc<Vec<model::Game>>,
    /// In the order shown.
    choices: Vec<Choice>,
    /// The choices the filter shows, by index.
    shown: Vec<usize>,
    /// Into `shown`: what Enter picks.
    selected: usize,
    scroll: ScrollHandle,
    /// The line over the caption: the executable, the game, or none.
    subject: SharedString,
    filter: Entity<InputState>,
    _typed: Subscription,
}

pub struct GamePicker {
    library: Entity<Library>,
    picker: Option<Picker>,
    focus: FocusHandle,
}

impl GamePicker {
    pub fn new(library: Entity<Library>, cx: &mut Context<Self>) -> Self {
        Self { library, picker: None, focus: cx.focus_handle() }
    }

    pub fn is_open(&self) -> bool {
        self.picker.is_some()
    }

    /// Asks for the game's executable, then adds it or opens the picker.
    pub fn add(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let (paths, library) = (prompt(cx), self.library.clone());
        cx.spawn_in(window, async move |view, cx| {
            let Some(exe) = picked_file(paths).await else { return };
            let Some(found) = library.update(cx, |library, cx| library.find(exe, cx)).await else { return };
            view.update_in(cx, |view, window, cx| view.offer(found, window, cx)).ok();
        })
        .detach();
    }

    /// ADD GAME MANUALLY: asks for the executable of the catalog record `record` and adds it as that game.
    pub fn add_to(&mut self, record: Ref, window: &mut Window, cx: &mut Context<Self>) {
        let (paths, library) = (prompt(cx), self.library.clone());
        cx.spawn_in(window, async move |_, cx| {
            let Some(exe) = picked_file(paths).await else { return };
            library.update(cx, |library, cx| library.add_as(exe, &record, cx));
        })
        .detach();
    }

    /// Opens the trainer's choice for the listed game `id`.
    pub fn trainer(&mut self, id: &GameId, window: &mut Window, cx: &mut Context<Self>) {
        let library = self.library.read(cx);
        let (Load::Ready(list), Some(game)) = (library.list(), library.game(id)) else { return };
        let (records, own) = list.trainer_records(game);
        let choices = records.into_iter().map(|at| Choice::of(&list.catalog, at)).collect();
        let subject = match &game.store {
            Some(store) => format!("{}  {store}", game.name),
            None => game.name.to_string(),
        };
        let purpose = Purpose::Trainer { game: id.clone(), current: game.trained_by().map(|record| record.id.clone()), own };
        let games = list.catalog.clone();
        self.open(purpose, games, choices, subject.into(), window, cx);
    }

    /// The search: every listed game and every other title with a trainer.
    pub fn find(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Load::Ready(list) = self.library.read(cx).list() else { return };
        let finds = list.finds();
        let choices = finds.iter().map(|find| Choice::of(&list.catalog, find.at)).collect();
        let games = list.catalog.clone();
        self.open(Purpose::Find { finds }, games, choices, SharedString::default(), window, cx);
    }

    fn offer(&mut self, found: Found, window: &mut Window, cx: &mut Context<Self>) {
        let Found { exe, games, owners } = found;
        if let [at] = owners[..] {
            return self.library.update(cx, |library, cx| library.add(exe, games, at, cx));
        }
        let owned = !owners.is_empty();
        let shown = match owned {
            true => owners,
            false => (0..games.len()).filter(|&at| games[at].trainer.is_some()).collect(),
        };
        let mut choices: Vec<Choice> = shown.into_iter().map(|at| Choice::of(&games, at)).collect();
        choices.sort_by(|a, b| by_name(&a.search, &b.search));
        let subject = exe.display().to_string().into();
        self.open(Purpose::Add { exe, owned }, games, choices, subject, window, cx);
    }

    fn open(
        &mut self,
        purpose: Purpose,
        games: Arc<Vec<model::Game>>,
        choices: Vec<Choice>,
        subject: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let (filter, relabel) = input(key::PICKER_FILTER, window, cx);
        let typed = cx.subscribe(&filter, |view, _, event: &InputEvent, cx| match event {
            InputEvent::Change => view.refilter(cx),
            InputEvent::PressEnter { .. } => view.choose_selected(cx),
            _ => {}
        });
        filter.update(cx, |filter, cx| filter.focus(window, cx));
        let (shown, scroll, _typed) = (Vec::new(), ScrollHandle::new(), Subscription::join(typed, relabel));
        self.picker = Some(Picker { purpose, games, choices, shown, selected: 0, scroll, subject, filter, _typed });
        self.refilter(cx);
    }

    /// The rows for the filter's text, the first selected.
    fn refilter(&mut self, cx: &mut Context<Self>) {
        let Some(picker) = self.picker.as_mut() else { return };
        let query = picker.filter.read(cx).value().trim().to_lowercase();
        let choices = &picker.choices;
        picker.shown = match &picker.purpose {
            Purpose::Find { finds } => rank(finds, &query),
            // The whole catalog waits for a name.
            Purpose::Add { owned: false, .. } if query.is_empty() => Vec::new(),
            Purpose::Trainer { own, .. } if query.is_empty() => (0..*own).collect(),
            _ => (0..choices.len()).filter(|&at| choices[at].search.contains(&query)).collect(),
        };
        picker.shown.truncate(SHOWN);
        picker.selected = 0;
        picker.scroll.scroll_to_item(0);
        cx.notify();
    }

    /// Moves the selection `by` rows, round the ends.
    fn step(&mut self, by: isize, cx: &mut Context<Self>) {
        let Some(picker) = self.picker.as_mut().filter(|picker| !picker.shown.is_empty()) else { return };
        picker.selected = (picker.selected as isize + by).rem_euclid(picker.shown.len() as isize) as usize;
        picker.scroll.scroll_to_item(picker.selected);
        cx.notify();
    }

    fn choose_selected(&mut self, cx: &mut Context<Self>) {
        let at = self.picker.as_ref().and_then(|picker| Some(picker.choices[*picker.shown.get(picker.selected)?].at));
        if let Some(at) = at {
            self.choose(at, cx);
        }
    }

    fn choose(&mut self, at: usize, cx: &mut Context<Self>) {
        let Some(Picker { purpose, games, .. }) = self.picker.take() else { return };
        match purpose {
            Purpose::Add { exe, .. } => self.library.update(cx, |library, cx| library.add(exe, games, at, cx)),
            Purpose::Trainer { game, .. } => {
                let record = games[at].id.clone();
                self.library.update(cx, |library, cx| library.set_trainer(&game, Some(record), cx));
            }
            Purpose::Find { .. } => {
                let record = games[at].id.clone();
                self.library.update(cx, |library, cx| library.browse(record, cx));
            }
        }
        cx.notify();
    }

    /// Closes the dialog, as entering the overlay does.
    pub fn dismiss(&mut self, cx: &mut Context<Self>) {
        if self.picker.take().is_some() {
            cx.notify();
        }
    }

    fn close(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.dismiss(cx);
    }
}

impl Render for GamePicker {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(picker) = self.picker.as_ref() else { return div().into_any_element() };
        let library = self.library.read(cx);
        let rows = picker.shown.iter().enumerate().map(|(row, &index)| {
            let choice = &picker.choices[index];
            let (at, record, store) = (choice.at, &picker.games[choice.at], &choice.store);
            let mods = record.trainer.as_ref().map(|offer| tf(cx, key::TRAINER_MODS, &[("count", Arg::Num(offer.cheats.into()))]));
            let in_use = matches!(&picker.purpose, Purpose::Trainer { current, .. } if current.as_ref() == Some(&record.id));
            let (tag, mods) = match &picker.purpose {
                // Picking it adds a source to the listed game.
                Purpose::Add { .. } if library.game(&GameId::of(&record.id)).is_some() => {
                    (tf(cx, key::PICKER_IN_LIBRARY, &[("store", Arg::Str(store))]), None)
                }
                Purpose::Add { .. } => (store.clone(), None),
                Purpose::Trainer { .. } => (store.clone(), mods),
                Purpose::Find { finds } if finds[index].listed => (tf(cx, key::PICKER_IN_LIBRARY, &[("store", Arg::Str(store))]), mods),
                Purpose::Find { .. } => (SharedString::default(), mods),
            };
            list_row(("choice", at), row == picker.selected)
                .h(px(32.))
                .flex_none()
                .px(px(10.))
                .gap(px(12.))
                .justify_between()
                .on_click(cx.listener(move |view, _, _, cx| view.choose(at, cx)))
                .child(mono(choice.name.clone(), TYPE_SMALL).text_color(rgb(TEXT_2)).min_w_0().truncate())
                .child(
                    h_flex()
                        .flex_none()
                        .gap(px(12.))
                        .when_some(mods, |tags, mods| tags.child(mono(mods, TYPE_CAPTION).text_color(rgb(DIM))))
                        .child(mono(tag, TYPE_CAPTION).text_color(rgb(FAINT)))
                        .when(in_use, |tags| tags.child(badge(t(cx, key::PICKER_IN_USE), accent(cx)))),
                )
        });
        let (title, caption) = match picker.purpose {
            Purpose::Add { owned: true, .. } => (key::PICKER_TITLE, key::PICKER_OWNED),
            Purpose::Add { owned: false, .. } => (key::PICKER_TITLE, key::PICKER_UNOWNED),
            Purpose::Trainer { .. } => (key::PICKER_TRAINER_TITLE, key::PICKER_TRAINER),
            Purpose::Find { .. } => (key::PICKER_FIND_TITLE, key::PICKER_FIND),
        };
        let field = h_flex()
            .h(px(36.))
            .px(px(10.))
            .border_1()
            .border_color(line(0.14))
            .bg(rgb(SUNKEN))
            .child(Input::new(&picker.filter).appearance(false).px_0().font_family(MONO).text_size(px(TYPE_FIELD)));
        let body = v_flex()
            .p(px(16.))
            .gap(px(12.))
            // Ahead of the field's own: the arrows move the selection.
            .capture_action(cx.listener(|view, _: &MoveUp, _, cx| {
                view.step(-1, cx);
                cx.stop_propagation();
            }))
            .capture_action(cx.listener(|view, _: &MoveDown, _, cx| {
                view.step(1, cx);
                cx.stop_propagation();
            }))
            .when(!picker.subject.is_empty(), |body| {
                body.child(mono(picker.subject.clone(), TYPE_LABEL).text_color(rgb(MUTED)).min_w_0().truncate())
            })
            .child(mono(t(cx, caption), TYPE_CAPTION).text_color(rgb(DIM)))
            .child(field)
            .child(
                div()
                    .relative()
                    .child(
                        div()
                            .id("choices")
                            .max_h(px(320.))
                            .overflow_y_scroll()
                            .track_scroll(&picker.scroll)
                            .flex()
                            .flex_col()
                            .children(rows),
                    )
                    .vertical_scrollbar(&picker.scroll),
            );
        dialog("picker", t(cx, title), 520., &self.focus, Self::close, body, cx).into_any_element()
    }
}

pub(super) fn prompt(cx: &mut App) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
    cx.prompt_for_paths(PathPromptOptions { files: true, directories: false, multiple: false, prompt: Some(t(cx, key::PICKER_PROMPT)) })
}

/// The file picked, if it is an executable; a shortcut comes as its target.
pub(super) async fn picked_file(paths: oneshot::Receiver<Result<Option<Vec<PathBuf>>>>) -> Option<PathBuf> {
    let exe = match paths.await {
        Ok(Ok(Some(paths))) => paths.into_iter().next()?,
        Ok(Err(e)) => {
            log::warn!("file picker: {e:#}");
            return None;
        }
        _ => return None,
    };
    let is_exe = exe.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("exe"));
    if !is_exe {
        log::warn!("{} is not an .exe", exe.display());
    }
    is_exe.then_some(exe)
}
