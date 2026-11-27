//! The map's model: the open game's maps (its record's, then a linked or matched title's), the one chosen,
//! loading it, and the live run both the map and the minimap follow.

use std::{path::PathBuf, sync::Arc, time::Duration};

use blocking::unblock;
use client::{
    Registry,
    library::{GameId, titles::matched},
    media::{self, Media},
    storage::Paths,
};
use futures::StreamExt;
use gpui_kit::{App, AppContext, Context, Entity, EventEmitter, Task};
use model::{Locale, MapOffer, Ref, Snapshot, SourceId};

use super::{
    board::{Board, show},
    live::{LiveMap, LiveRun},
    titles::{Titles, TitlesEvent},
};
use crate::{
    Load,
    library::{Library, LibraryEvent},
    run::{RunEvent, Runs},
    sources::{Sources, SourcesChanged},
    trainer::Trainers,
    ui::text::{Texts, content_locale},
};

// A closed map keeps its pins this long, so opening it again soon is instant; later they are read from disk again.
// While a trainer is live in its game, it keeps them until the run ends.
const KEPT_CLOSED: Duration = Duration::from_secs(3 * 60);

/// The stage's camera: center and span.
type Camera = Box<dyn Fn(&App) -> Option<(f64, f64, f64)>>;

pub enum MapsEvent {
    /// The tiles remembered absent may be there now: the stage asks for them again.
    Retile,
}

pub struct Maps {
    registry: Arc<Registry>,
    library: Entity<Library>,
    runs: Entity<Runs>,
    // The games' profiles: TELEPORT HEIGHT is the game's.
    trainers: Entity<Trainers>,
    board: Entity<Board>,
    live_map: Entity<LiveMap>,
    titles: Entity<Titles>,
    tiles: PathBuf,
    // The game whose maps are listed, and the one a trainer is live in.
    game: Option<GameId>,
    live: Option<GameId>,
    // The record's offers, then the title's; `offers` is both.
    native: Vec<MapOffer>,
    discovered: Vec<MapOffer>,
    offers: Vec<MapOffer>,
    // The title whose offers `discovered` holds, and their locale.
    asked: Option<(Option<Ref>, Locale)>,
    // The content locale last seen: a new `Texts` in the same language (a source's catalogs) reloads nothing.
    locale: Locale,
    chosen: usize,
    // The map and revision the board was last asked for.
    held: Option<(Ref, Option<String>)>,
    // The minimap's dock is shown.
    minimap: bool,
    camera: Option<Camera>,
    _load: Task<()>,
    _forget: Task<()>,
    _offers: Task<()>,
}

impl EventEmitter<MapsEvent> for Maps {}

impl Maps {
    pub fn new(
        registry: Arc<Registry>,
        paths: &Paths,
        library: Entity<Library>,
        runs: Entity<Runs>,
        trainers: Entity<Trainers>,
        sources: &Entity<Sources>,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.subscribe(&runs, |maps, _, event: &RunEvent, cx| {
            if !matches!(event, RunEvent::Pressed(_)) {
                maps.follow(cx);
            }
        })
        .detach();
        cx.observe_global::<Texts>(Self::relocalize).detach();
        cx.subscribe(&library, |maps, _, _: &LibraryEvent, cx| maps.gather(cx)).detach();
        // After `Titles`' own: a source gone has left the titles before the offers are gathered.
        let titles = cx.new(|cx| Titles::new(registry.clone(), sources, cx));
        cx.subscribe(sources, |maps, _, SourcesChanged(changed), cx| maps.sources_changed(changed, cx)).detach();
        // Whoever offers LINK A MAP reads the titles through `Maps`.
        cx.subscribe(&titles, |maps, _, _: &TitlesEvent, cx| {
            maps.gather(cx);
            cx.notify();
        })
        .detach();
        let art = Media::Pictures.dir(paths);
        let board = cx.new(|_| Board::new(paths.clone(), art.clone()));
        let live_board = cx.new(|_| Board::live(paths.clone(), art));
        let live_map = cx.new(|cx| LiveMap::new(registry.clone(), live_board, board.clone(), Media::Tiles.dir(paths), paths, cx));
        Self {
            registry,
            library,
            runs,
            trainers,
            board,
            live_map,
            titles,
            tiles: Media::Tiles.dir(paths),
            game: None,
            live: None,
            native: Vec::new(),
            discovered: Vec::new(),
            offers: Vec::new(),
            asked: None,
            locale: content_locale(cx),
            chosen: 0,
            held: None,
            minimap: false,
            camera: None,
            _load: Task::ready(()),
            _forget: Task::ready(()),
            _offers: Task::ready(()),
        }
    }

    pub fn mapped(&self) -> bool {
        !self.offers.is_empty()
    }

    pub(super) fn titles(&self) -> &Entity<Titles> {
        &self.titles
    }

    /// TELEPORT HEIGHT for the open game: kept in its profile, and given to the board at once.
    pub(super) fn set_lift(&mut self, lift: f64, cx: &mut Context<Self>) {
        let Some(game) = &self.game else { return };
        let lift = self.trainers.update(cx, |trainers, _| trainers.set_teleport_lift(game, lift));
        self.board.update(cx, |board, cx| board.set_lift(lift, cx));
    }

    pub fn is_open(&self, cx: &App) -> bool {
        self.board.read(cx).is_open()
    }

    pub fn live_map(&self) -> &Entity<LiveMap> {
        &self.live_map
    }

    pub(super) fn board(&self) -> &Entity<Board> {
        &self.board
    }

    pub(super) fn offers(&self) -> &[MapOffer] {
        &self.offers
    }

    pub(super) fn chosen(&self) -> usize {
        self.chosen
    }

    /// Where the camera is read: the board keeps it with the map's state before it lets go of the map.
    pub(super) fn set_camera(&mut self, camera: impl Fn(&App) -> Option<(f64, f64, f64)> + 'static) {
        self.camera = Some(Box::new(camera));
    }

    /// Opens on the open game's maps; the map the board still holds, in the same revision, keeps the view of it.
    pub(super) fn open(&mut self, cx: &mut Context<Self>) {
        self._forget = Task::ready(());
        let chosen = self.offers.get(self.chosen).map(|offer| (offer.id.clone(), offer.revision.clone()));
        if self.held != chosen || self.board.read(cx).unloaded() {
            self.choose(self.chosen, cx);
        }
        self.board.update(cx, |board, cx| board.set_open(true, cx));
        self.follow(cx);
        cx.notify();
    }

    pub(super) fn close(&mut self, cx: &mut Context<Self>) {
        self.keep_view(cx);
        self.board.update(cx, |board, cx| {
            board.save();
            board.set_open(false, cx);
        });
        self.forget_later(cx);
        cx.notify();
    }

    /// The board saves the map it lets go of.
    pub(super) fn choose(&mut self, index: usize, cx: &mut Context<Self>) {
        self.keep_view(cx);
        self.chosen = index;
        cx.notify();
        let Some(offer) = self.offers.get(index).cloned() else { return };
        self.held = Some((offer.id.clone(), offer.revision.clone()));
        let locale = content_locale(cx);
        let shared = self.live_map.read(cx).board.read(cx).loaded(&offer.id, &locale);
        self.board.update(cx, |board, cx| board.pending(offer.id.clone(), locale, cx));
        // The minimap's copy, when it shows this map; a load of its own unless the minimap's is done.
        if self.board.update(cx, |board, cx| board.take(shared, cx)) {
            self._load = Task::ready(());
            return;
        }
        self.load(&offer, cx);
    }

    fn load(&mut self, offer: &MapOffer, cx: &mut Context<Self>) {
        let snapshots = self.registry.map(offer, &content_locale(cx));
        let board = self.board.downgrade();
        self._load = cx.spawn(async move |_, cx| {
            let _ = show(&board, snapshots, cx).await;
        });
    }

    /// Marks the chosen map stale at its source, forgets its tiles remembered absent and loads it again.
    pub(super) fn refresh(&mut self, cx: &mut Context<Self>) {
        let Some(offer) = self.offers.get(self.chosen).cloned() else { return };
        let Load::Ready(map) = &self.board.read(cx).map else { return };
        let templates: Vec<String> = map.tile_sets.iter().map(|set| set.tiles.template.clone()).collect();
        self.registry.refresh(&offer.id);
        let (tiles, board) = (self.tiles.clone(), self.board.downgrade());
        // As `_load`: choosing another map or language drops it.
        self._load = cx.spawn(async move |maps, cx| {
            unblock(move || media::forget_missing(&tiles, &templates.iter().map(String::as_str).collect::<Vec<_>>())).await;
            let Ok(snapshots) = maps.update(cx, |maps, cx| {
                cx.emit(MapsEvent::Retile);
                maps.registry.map(&offer, &content_locale(cx))
            }) else {
                return;
            };
            let _ = show(&board, snapshots, cx).await;
        });
    }

    /// Adds linked maps after the record's. Without a usable link or maps of its own, the game uses a title match
    /// or refreshes titles once per game and locale.
    fn gather(&mut self, cx: &mut Context<Self>) {
        let game = self.library.read(cx).opened().cloned();
        let id = game.as_ref().map(|game| game.id.clone());
        let moved = self.game != id;
        if moved {
            let lift = id.as_ref().and_then(|id| self.trainers.read(cx).game_profile(id).teleport_lift).unwrap_or(0.);
            self.board.update(cx, |board, cx| board.set_lift(lift, cx));
            (self.game, self.asked, self._offers) = (id, None, Task::ready(()));
            self.discovered.clear();
        }
        self.native = game.as_ref().map_or_else(Vec::new, |game| game.source.maps.clone());
        let locale = content_locale(cx);
        let ready = self.titles.read(cx).ready(&locale);
        if !ready && self.asked.as_ref().is_some_and(|(_, asked)| *asked != locale) {
            // The old locale's answer must not land; its offers stay until the new ones come.
            self._offers = Task::ready(());
        }
        if let Some(game) = game.filter(|_| ready) {
            let titled = self.registry.titled();
            let link = self.library.read(cx).link(&game.id).filter(|title| titled.contains(title.source()));
            let unmapped = self.native.is_empty();
            let title = link.or_else(|| matched(&game, self.titles.read(cx).all()).filter(|_| unmapped).map(|title| title.id.clone()));
            let asked = (title, locale);
            if self.asked.as_ref() != Some(&asked) {
                // Another locale's offers stay until theirs replace them: the chosen map is kept by `Ref`.
                if self.asked.as_ref().is_none_or(|(title, _)| *title != asked.0) {
                    self.discovered.clear();
                }
                self._offers = match &asked.0 {
                    Some(title) => self.discover(title, &asked.1, cx),
                    // A game with maps of its own takes no match: nothing to ask.
                    None if !unmapped => Task::ready(()),
                    None => {
                        self.titles.update(cx, |titles, cx| titles.refresh(cx));
                        Task::ready(())
                    }
                };
                self.asked = Some(asked);
            }
        }
        self.list(!moved, cx);
    }

    fn discover(&self, title: &Ref, locale: &Locale, cx: &mut Context<Self>) -> Task<()> {
        let mut snapshots = self.registry.offers(title, locale);
        cx.spawn(async move |maps, cx| {
            while let Some(snapshot) = snapshots.next().await {
                let (Snapshot::Cached(Some(offers)) | Snapshot::Fresh(Ok(offers))) = snapshot else { continue };
                let listed = maps.update(cx, |maps, cx| {
                    maps.discovered = offers;
                    maps.list(true, cx);
                });
                if listed.is_err() {
                    return;
                }
            }
        })
    }

    /// The chosen map stays chosen where it moved; gone, the first is. `same`: still the shown game's list, so an
    /// open map gone or in a new revision loads again; a closed one does at its next `open`.
    fn list(&mut self, same: bool, cx: &mut Context<Self>) {
        let offers: Vec<MapOffer> = self.native.iter().chain(&self.discovered).cloned().collect();
        if offers == self.offers {
            return;
        }
        let shown = self.offers.get(self.chosen).map(|offer| (offer.id.clone(), offer.revision.clone()));
        self.offers = offers;
        let kept = shown.as_ref().and_then(|(id, revision)| {
            let at = self.offers.iter().position(|offer| offer.id == *id)?;
            Some((at, self.offers[at].revision != *revision))
        });
        let (at, stale) = kept.unwrap_or((0, shown.is_some()));
        self.chosen = at;
        // With no offer left, whoever shows the map closes it.
        if same && stale && self.is_open(cx) {
            self.choose(at, cx);
        }
        cx.notify();
    }

    /// Another content language: the open map loads again in it, a closed one lets go of its copy; the minimap's
    /// search starts over.
    fn relocalize(&mut self, cx: &mut Context<Self>) {
        let locale = content_locale(cx);
        if locale == self.locale {
            return;
        }
        self.locale = locale;
        if self.is_open(cx) {
            self.choose(self.chosen, cx);
        } else {
            self._load = Task::ready(());
            self.board.update(cx, |board, _| board.forget());
        }
        self.live_map.update(cx, |live, cx| live.relocalize(cx));
        self.gather(cx);
    }

    /// Refreshes offers and the open map on source replacement. Discards map copies that will not reload.
    fn sources_changed(&mut self, changed: &[SourceId], cx: &mut Context<Self>) {
        let of = |id: &Ref| changed.contains(id.source());
        if let Some((Some(title), locale)) = self.asked.clone().filter(|(title, _)| title.as_ref().is_some_and(of)) {
            if self.registry.has(title.source()) {
                // Replaced: its offers stay until the new ones come.
                self._offers = self.discover(&title, &locale, cx);
            } else {
                (self.asked, self._offers) = (None, Task::ready(()));
                self.discovered.clear();
            }
        }
        // A titles plugin installed now may know the open game: an unmapped one asks the sources again.
        if self.asked.as_ref().is_some_and(|(title, _)| title.is_none())
            && self.registry.titled().iter().any(|source| changed.contains(source))
        {
            self.asked = None;
        }
        let open = self.is_open(cx);
        let held = self.held.take_if(|(map, _)| of(map));
        let reload = open && held.as_ref().is_some_and(|(map, _)| self.registry.has(map.source()));
        if held.is_some() && !reload {
            if open {
                self.keep_view(cx);
            }
            self._load = Task::ready(());
            self.board.update(cx, |board, _| {
                board.save();
                board.forget();
            });
            // The stage's tiles and epoch go with the map, so no old load lands while a fallback loads.
            cx.emit(MapsEvent::Retile);
        }
        self.gather(cx);
        if reload && self.held.is_none() {
            self.choose(self.chosen, cx);
        }
    }

    fn keep_view(&mut self, cx: &mut Context<Self>) {
        let Some(camera) = &self.camera else { return };
        let view = camera(cx);
        self.board.update(cx, |board, _| board.keep_camera(view));
    }

    /// The minimap follows the live run while its dock is shown.
    pub fn set_minimap(&mut self, wanted: bool, cx: &mut Context<Self>) {
        self.minimap = wanted;
        self.follow_minimap(cx);
    }

    /// The minimap's zoom: closer for more `levels`, further for fewer. With no live map there is nothing to zoom.
    pub fn zoom_minimap(&self, levels: f64, cx: &mut Context<Self>) {
        if self.live_map.read(cx).ready(cx) {
            self.live_map.update(cx, |live, cx| live.zoom_by(levels, cx));
        }
    }

    /// The open map follows the live run of its game; the minimap the live run, wherever.
    fn follow(&mut self, cx: &mut Context<Self>) {
        let open = self.is_open(cx);
        let runs = self.runs.read(cx);
        let run = runs.live_run().filter(|(game, _)| open && self.game.as_ref() == Some(*game)).map(|(_, run)| run.clone());
        let live = runs.live().cloned();
        self.board.update(cx, |board, cx| board.set_run(run, cx));
        if self.live != live {
            self.live = live;
            self.forget_later(cx);
        }
        self.follow_minimap(cx);
    }

    /// Only a trainer that can place the player has a run to follow.
    fn follow_minimap(&mut self, cx: &mut Context<Self>) {
        let live = self.runs.read(cx).live_run().filter(|(_, run)| self.minimap && run.tracker.is_some());
        let run = live.and_then(|(game, run)| {
            let offers = self.library.read(cx).game(game)?.source.maps.clone();
            Some(LiveRun { offers, run: run.clone() })
        });
        self.live_map.update(cx, |live, cx| live.set_run(run, cx));
    }

    /// A closed map whose game has no live trainer lets go of its pins after a while.
    fn forget_later(&mut self, cx: &mut Context<Self>) {
        let kept = self.is_open(cx) || self.live.is_some() && self.live == self.game;
        if kept || self.board.read(cx).unloaded() {
            self._forget = Task::ready(());
            return;
        }
        self._forget = cx.spawn(async move |maps, cx| {
            cx.background_executor().timer(KEPT_CLOSED).await;
            maps.update(cx, |maps, cx| {
                // A load on its way would install into the map let go of.
                maps._load = Task::ready(());
                maps.board.update(cx, |board, _| board.forget());
            })
            .ok();
        });
    }
}
