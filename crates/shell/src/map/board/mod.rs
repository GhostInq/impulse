//! The shown map and what the user did with it, shared by the stage, the sidebar and the card.
//! They observe it; it notifies on what a person does (clicks, typing, the player's position), never per frame.

mod marks;
mod media;
mod teleport;

use std::{mem, path::PathBuf, rc::Rc, time::Duration};

use anyhow::Result;
use client::{
    map::{
        heading::Bearing,
        state::{Saved, Tally, View},
    },
    storage::Paths,
};
use futures::{StreamExt, stream::BoxStream};
use gpui_kit::{App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, Subscription, WeakEntity};
use model::{Failure, Icon, Locale, Map, Point, Ref, Snapshot};

pub use media::{Attach, Target};

use super::{group, icons::Icons, markers::GroupIcon};
use crate::{
    Load,
    run::{Fix, Lease, Run, Tracked},
};

// How often the open map wants the player's position, and the minimap, which follows the player.
const EVERY: Duration = Duration::from_secs(1);
const LIVE_EVERY: Duration = Duration::from_millis(250);

pub enum BoardEvent {
    /// A map went up; `first`: another map than the one shown, not a fresher copy of it; `retiled`: its tiles differ.
    Installed { first: bool, retiled: bool },
    /// The player moved; only the stage draws where. A notify tells whether there is a player at all.
    Moved,
    /// What shows changed: filters, layers, the search, found pins.
    Filtered,
    /// A pin or a mark of the user's was opened or closed.
    Selected,
    /// The new mark is already open.
    MarkAdded,
    /// The open mark's place changed: dragged or typed.
    MarkMoved,
    /// The view should show `to`; `close`: near enough to tell pins apart.
    Bring { to: Point, close: bool },
}

impl EventEmitter<BoardEvent> for Board {}

pub struct Board {
    paths: Paths,
    // The shown map's, which its saved state is kept under, and the locale it was asked in: boards share a copy only
    // when they asked for the same.
    id: Option<Ref>,
    locale: Option<Locale>,
    pub map: Load<Rc<Map>>,
    // The copy's picture icons, shared with a board that takes the copy; it notifies this board when they are in.
    icons: Option<(Entity<Icons>, Subscription)>,
    art: PathBuf,
    // The copy is its load's last word: a board taking it needs no load of its own.
    settled: bool,
    // Shared so a frame can hold it while it paints.
    pub saved: Rc<Saved>,
    // Pins per category and group: all of them and the found ones.
    /// By category and group (`Saved::tally`).
    pub tally: Vec<Vec<Tally>>,
    // Each pin's name, group, category and description, lowercased; made on the map's first search.
    haystacks: Vec<String>,
    query: String,
    pub selected: Option<usize>,
    // The user's own pin whose card is open.
    pub mark: Option<Rc<str>>,
    run: Option<Run>,
    pub player: Option<[f64; 3]>,
    pub bearing: Option<Bearing>,
    // `player` is a current fix with its height: a pin can keep it.
    pub here: bool,
    // What went wrong last, shown on the map.
    pub note: Option<Failure>,
    // The open game's TELEPORT HEIGHT, which `Maps` keeps here.
    lift: f64,
    // The player's position is asked for only while the map is on screen, this often.
    open: bool,
    every: Duration,
    // Off for the minimap's board: the user's state of a map is written by the board they change it on.
    saves: bool,
    _fixes: Option<(Lease, Subscription)>,
}

/// A board's copy of a map, as another board takes it.
pub struct Shared {
    pub map: Rc<Map>,
    icons: Entity<Icons>,
    settled: bool,
}

impl Board {
    /// `art`: the pictures' cache, which icon sheets come from.
    pub fn new(paths: Paths, art: PathBuf) -> Self {
        Self::with(paths, art, EVERY, true)
    }

    /// The minimap's: it follows the player closely and never writes the user's state.
    pub fn live(paths: Paths, art: PathBuf) -> Self {
        Self::with(paths, art, LIVE_EVERY, false)
    }

    fn with(paths: Paths, art: PathBuf, every: Duration, saves: bool) -> Self {
        Self {
            paths,
            id: None,
            locale: None,
            map: Load::Pending,
            icons: None,
            art,
            settled: false,
            saved: Rc::default(),
            tally: Vec::new(),
            haystacks: Vec::new(),
            query: String::new(),
            selected: None,
            mark: None,
            run: None,
            player: None,
            bearing: None,
            here: false,
            note: None,
            lift: 0.,
            open: false,
            every,
            saves,
            _fixes: None,
        }
    }

    pub fn id(&self) -> Option<&Ref> {
        self.id.as_ref()
    }

    pub fn asked(&self) -> Option<(&Ref, &Locale)> {
        self.id.as_ref().zip(self.locale.as_ref())
    }

    pub fn loaded(&self, id: &Ref, locale: &Locale) -> Option<Shared> {
        let (Load::Ready(map), Some((icons, _))) = (&self.map, &self.icons) else { return None };
        (self.asked() == Some((id, locale))).then(|| Shared { map: map.clone(), icons: icons.clone(), settled: self.settled })
    }

    /// Shows another board's copy, if any; true when it is settled and needs no load of its own.
    pub fn take(&mut self, shared: Option<Shared>, cx: &mut Context<Self>) -> bool {
        let Some(shared) = shared else { return false };
        let settled = shared.settled;
        self.share(shared, cx);
        self.settled = settled;
        settled
    }

    /// Shows another board's copy while this board's own load may still be on its way.
    pub fn share(&mut self, shared: Shared, cx: &mut Context<Self>) {
        self.install(shared.map, Some(shared.icons), cx);
    }

    pub fn icon<'a>(&'a self, at: (usize, usize), cx: &'a App) -> GroupIcon<'a> {
        let Load::Ready(map) = &self.map else { return GroupIcon::Default };
        let picture = || self.icons.as_ref().and_then(|(icons, _)| icons.read(cx).get(at));
        match &map.categories[at.0].groups[at.1].icon {
            Some(Icon::Glyph(glyph)) => GroupIcon::Glyph(glyph),
            Some(Icon::Image { .. }) => picture().map_or(GroupIcon::Default, GroupIcon::Picture),
            None => GroupIcon::Default,
        }
    }

    /// Waits for the map `id` in `locale`; the one shown keeps its state on disk first.
    pub fn pending(&mut self, id: Ref, locale: Locale, cx: &mut Context<Self>) {
        self.save();
        (self.id, self.locale, self.map, self.icons) = (Some(id), Some(locale), Load::Pending, None);
        (self.selected, self.mark, self.settled) = (None, None, false);
        self.watch(cx);
        cx.notify();
    }

    /// A fresh copy of the map on screen keeps the user's state, not what refers to its pins.
    /// `icons`: those of another board's copy; `None` cuts this copy's own.
    fn install(&mut self, map: Rc<Map>, icons: Option<Entity<Icons>>, cx: &mut Context<Self>) {
        let icons = icons.unwrap_or_else(|| cx.new(|cx| Icons::new(&map, self.art.clone(), cx)));
        self.icons = Some((icons.clone(), cx.observe(&icons, |_, _, cx| cx.notify())));
        // A copy shared after this board's own load failed is a first one too.
        let first = !matches!(self.map, Load::Ready(_));
        let retiled = !matches!(&self.map, Load::Ready(shown) if shown.tile_sets == map.tile_sets);
        if first {
            self.saved = Rc::new(self.id.as_ref().map_or_else(Saved::default, |id| Saved::load(&self.paths, id, &map)));
        }
        // Titles an older build saved turn into ids once a copy names them.
        if !self.saved.view().hidden_labels.is_empty() {
            self.saved_mut().view_mut().resolve(&map.regions);
        }
        (self.map, self.selected, self.mark, self.haystacks) = (Load::Ready(map), None, None, Vec::new());
        self.index();
        self.recount();
        self.watch(cx);
        cx.emit(BoardEvent::Installed { first, retiled });
        cx.notify();
    }

    // Most maps are never searched: the lowercased copy of every description waits for the first query.
    fn index(&mut self) {
        let Load::Ready(map) = &self.map else { return };
        if self.query.is_empty() || !self.haystacks.is_empty() {
            return;
        }
        self.haystacks = map
            .pins
            .iter()
            .map(|pin| {
                format!("{}\n{}\n{}\n{}", pin.name, group(map, pin).name, map.categories[pin.group.0].name, pin.description).to_lowercase()
            })
            .collect();
    }

    /// The map could not be loaded; a copy on screen stays.
    pub fn fail(&mut self, why: Failure, cx: &mut Context<Self>) {
        if matches!(self.map, Load::Pending) {
            self.map = Load::Failed(why);
            cx.notify();
        }
    }

    /// Keeps what changed of the user's state of the map on screen; blocking, so saves land in order (small, on a click).
    pub fn save(&mut self) {
        if self.saves && matches!(self.map, Load::Ready(_)) {
            self.saved_mut().flush();
        }
    }

    /// The user's state of this map as another board changed it.
    pub fn adopt(&mut self, saved: Rc<Saved>, cx: &mut Context<Self>) {
        self.saved = saved;
        self.recount();
        cx.emit(BoardEvent::Filtered);
        cx.notify();
    }

    fn saved_mut(&mut self) -> &mut Saved {
        Rc::make_mut(&mut self.saved)
    }

    /// The stage's camera, kept with the map at the next save.
    pub fn keep_camera(&mut self, camera: Option<(f64, f64, f64)>) {
        if self.saved.view().camera != camera {
            self.saved_mut().view_mut().camera = camera;
        }
    }

    /// Changes what the user set (filters, layers) and keeps it.
    pub fn filter(&mut self, change: impl FnOnce(&mut View), cx: &mut Context<Self>) {
        change(self.saved_mut().view_mut());
        self.filtered(cx);
    }

    fn filtered(&mut self, cx: &mut Context<Self>) {
        self.save();
        cx.emit(BoardEvent::Filtered);
        cx.notify();
    }

    pub fn set_query(&mut self, query: String, cx: &mut Context<Self>) {
        self.query = query;
        self.index();
        cx.emit(BoardEvent::Filtered);
        cx.notify();
    }

    pub fn searching(&self) -> bool {
        !self.query.is_empty()
    }

    pub fn toggle_found(&mut self, pin: usize, cx: &mut Context<Self>) {
        let Load::Ready(map) = &self.map else { return };
        let id = map.pins[pin].id.clone();
        self.toggle_found_id(&id, cx);
    }

    fn toggle_found_id(&mut self, id: &str, cx: &mut Context<Self>) {
        self.saved_mut().toggle_found(id);
        self.recount();
        self.filtered(cx);
    }

    fn recount(&mut self) {
        if let Load::Ready(map) = &self.map {
            self.tally = self.saved.tally(map);
        }
    }

    /// Whether pin `index` shows; a search reaches hidden groups too, never a hidden pin.
    pub fn shows(&self, map: &Map, index: usize) -> bool {
        let (pin, view) = (&map.pins[index], self.saved.view());
        if self.saved.hidden(&pin.id) || view.unfound_only && self.saved.found(&pin.id) || view.teleport_only && pin.position.is_none() {
            return false;
        }
        if self.query.is_empty() { !view.hidden_groups.contains(&group(map, pin).id) } else { self.haystacks[index].contains(&self.query) }
    }

    /// Whether pin `index` is found by its name (its haystack's first line), not its group or description.
    pub fn named(&self, index: usize) -> bool {
        self.haystacks[index].lines().next().is_some_and(|name| name.contains(&self.query))
    }

    /// Opens a pin's card; `bring` also shows the pin up close.
    pub fn select(&mut self, pin: usize, bring: bool, cx: &mut Context<Self>) {
        let Load::Ready(map) = &self.map else { return };
        if bring {
            cx.emit(BoardEvent::Bring { to: map.pins[pin].at, close: true });
        }
        (self.selected, self.mark) = (Some(pin), None);
        cx.emit(BoardEvent::Selected);
        cx.notify();
    }

    /// A pin a description links to.
    pub fn select_id(&mut self, id: &str, cx: &mut Context<Self>) {
        let Load::Ready(map) = &self.map else { return };
        if let Some(index) = map.pins.iter().position(|pin| pin.id == id) {
            self.select(index, true, cx);
        }
    }

    /// Closes the open card; false when none was open.
    pub fn deselect(&mut self, cx: &mut Context<Self>) -> bool {
        let open = self.selected.is_some() || self.mark.is_some();
        if open {
            (self.selected, self.mark) = (None, None);
            cx.emit(BoardEvent::Selected);
            cx.notify();
        }
        open
    }

    pub fn set_run(&mut self, run: Option<Run>, cx: &mut Context<Self>) {
        self.run = run;
        if self.run.is_none() {
            (self.player, self.bearing, self.here) = (None, None, false);
        }
        self.watch(cx);
        cx.notify();
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Whether showing the map has to load it: it failed, or it was let go of.
    pub fn unloaded(&self) -> bool {
        matches!(self.map, Load::Failed(_)) || self.id.is_none()
    }

    /// Lets go of the map, closed for a while; the user's state of it was saved when it closed.
    pub fn forget(&mut self) {
        (self.id, self.locale, self.map, self.icons, self.selected, self.mark) = (None, None, Load::Pending, None, None, None);
        (self.settled, self.saved, self.tally, self.haystacks) = (false, Rc::default(), Vec::new(), Vec::new());
    }

    pub fn set_open(&mut self, open: bool, cx: &mut Context<Self>) {
        self.open = open;
        self.watch(cx);
    }

    /// Asks the run's tracker for fixes while the map is open and both the trainer and the map can place the player;
    /// closed, it shows where the player last was.
    fn watch(&mut self, cx: &mut Context<Self>) {
        let frame = match &self.map {
            Load::Ready(map) => map.frame.clone(),
            _ => None,
        };
        let tracker = self.run.as_ref().and_then(|run| run.tracker.clone());
        let (Some(tracker), Some(frame)) = (tracker, frame) else {
            self._fixes = None;
            return;
        };
        let lease = self.open.then(|| tracker.update(cx, |tracker, cx| tracker.lease(self.every, &frame, cx)));
        // Where the player was and the way they last went, before the next fix.
        match tracker.read(cx).last() {
            Some(fix) => self.fixed(fix, cx),
            None => self.lost(cx),
        }
        let Some(lease) = lease else {
            self._fixes = None;
            return;
        };
        let fixes = cx.subscribe(&tracker, |board, _, tracked: &Tracked, cx| match tracked {
            Tracked::Fix(fix) => board.fixed(*fix, cx),
            Tracked::Lost => board.lost(cx),
        });
        self._fixes = Some((lease, fixes));
    }

    /// The last position stays, to teleport from at its height; the way is no longer known.
    fn lost(&mut self, cx: &mut Context<Self>) {
        let here = mem::take(&mut self.here);
        if self.bearing.take().is_some() || here {
            cx.emit(BoardEvent::Moved);
        }
    }

    /// Tells the stage only when the player, the bearing or `here` changed.
    fn fixed(&mut self, fix: Fix, cx: &mut Context<Self>) {
        if self.player == Some(fix.position) && self.bearing == fix.bearing && self.here == fix.height {
            return;
        }
        if self.player.replace(fix.position).is_none() {
            cx.notify();
        }
        (self.bearing, self.here) = (fix.bearing, fix.height);
        cx.emit(BoardEvent::Moved);
    }
}

pub fn plane((x, y): (f64, f64)) -> Point {
    Point { x, y }
}

/// Shows each copy on `board`, then marks it settled; a cached one only when the board holds none, as the other board
/// may have shared its own.
pub async fn show(board: &WeakEntity<Board>, mut snapshots: BoxStream<'static, Snapshot<Map>>, cx: &mut AsyncApp) -> Result<()> {
    while let Some(snapshot) = snapshots.next().await {
        board.update(cx, |board, cx| match snapshot {
            Snapshot::Cached(Some(_)) if matches!(board.map, Load::Ready(_)) => {}
            Snapshot::Cached(Some(map)) | Snapshot::Fresh(Ok(map)) => board.install(Rc::new(map), None, cx),
            // The network is next.
            Snapshot::Cached(None) => {}
            Snapshot::Fresh(Err(e)) => board.fail(e, cx),
        })?;
    }
    board.update(cx, |board, _| board.settled = true)
}
