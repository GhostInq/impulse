//! The map itself: tiles, regions, labels, pins and the player, moved and zoomed, with what floats over it.
//! A view of its own, so what moves each frame (a glide, the player, a drag) repaints the map alone.

mod camera;
mod input;
mod marks;
mod motion;
mod overlays;
mod paint;
mod pins;
mod rise;
mod tiles;

use crate::ui::{
    key,
    text::{failed, t},
};
use std::{ops::RangeInclusive, path::PathBuf, rc::Rc, sync::Arc, time::Instant};

use gpui_kit::{
    AnyElement, App, Bounds, Context, Entity, FocusHandle, HitboxBehavior, IntoElement, Pixels, Render, RenderImage, SharedString, Task,
    Window, canvas, div, prelude::*, px, rgb,
};
use model::{Map, Point};
use rustc_hash::FxHashMap as HashMap;

use super::{
    board::{Board, BoardEvent},
    keys::STAGE,
    live::LiveMap,
    markers::Images,
};
use crate::{
    Load,
    ui::{theme::Look, widgets::notice},
};
use camera::{Camera, Glide, Pixel, extent};
use motion::Walk;
use pins::Marker;
use tiles::Tiles;

struct Drag {
    last: gpui_kit::Point<Pixels>,
    moved: bool,
    // Alt on one of the user's pins: it moves, not the map.
    moving: Option<Moving>,
}

/// A pin of the user's on its way: its tip keeps its offset from the cursor on the view, under a moving camera too.
struct Moving {
    id: Rc<str>,
    // On the window: the stage may move under it.
    cursor: gpui_kit::Point<Pixels>,
    grab: Pixel,
}

/// What was worked out from the shown map's pins or points at them: forgotten with the map.
#[derive(Default)]
struct Shown {
    // By the zoom step they were grouped for.
    markers: Option<(i32, Rc<Vec<Marker>>)>,
    // Made once, not per frame: each label's line, each region ring's extent (a frame skips those off the view), group
    // counts, rises.
    labels: Vec<SharedString>,
    regions: Vec<Vec<Option<(Point, Point)>>>,
    counts: HashMap<usize, SharedString>,
    rises: HashMap<i64, SharedString>,
    hovered: Option<usize>,
    // The hovered pin once the cursor has rested on it.
    chip: Option<usize>,
    hovered_region: Option<usize>,
    hovered_mark: Option<Rc<str>>,
    // Where a right click opened its menu.
    target: Option<Point>,
}

impl Shown {
    fn of(map: &Map) -> Self {
        Self {
            labels: map.labels.iter().map(|label| label.text.replace('\n', " ").into()).collect(),
            regions: map.regions.iter().map(|region| region.outlines.iter().map(|ring| extent(ring.iter().copied())).collect()).collect(),
            ..Self::default()
        }
    }
}

pub struct Stage {
    board: Entity<Board>,
    tile_dir: PathBuf,
    // Counts the tile sets shown: a tile loaded for an earlier one is dropped.
    epoch: usize,
    // The index of the map's tile set drawn last.
    set: usize,
    tiles: Tiles,
    images: Images,
    // Images to drop from the atlas when the map is next drawn.
    stale: Vec<Arc<RenderImage>>,
    // `None` until the map is first painted: the view's size decides it.
    camera: Option<Camera>,
    glide: Option<Glide>,
    // When the last frame that moved something was painted.
    clock: Option<Instant>,
    // As last painted: the map's place in the window and a tile's side in its pixels.
    bounds: Bounds<Pixels>,
    tile: f64,
    shown: Shown,
    // A region ring's corners on the view and the tiles to load (backdrop or not, distance, tile), kept so a frame
    // allocates none.
    corners: Vec<gpui_kit::Point<Pixels>>,
    wanted: Vec<(bool, u32, tiles::Key)>,
    drag: Option<Drag>,
    layers_open: bool,
    follow: bool,
    // The player as drawn, easing toward the board's fixes.
    player: Option<Walk>,
    // A minimap: it only follows the player at the live map's zoom, which the wheel changes.
    live: Option<Entity<LiveMap>>,
    focus: FocusHandle,
    _chip: Task<()>,
}

impl Stage {
    pub fn new(board: Entity<Board>, tile_dir: PathBuf, cx: &mut Context<Self>) -> Self {
        Self::with(board, tile_dir, None, cx)
    }

    pub fn mini(live: Entity<LiveMap>, cx: &mut Context<Self>) -> Self {
        cx.observe(&live, |_, _, cx| cx.notify()).detach();
        let (board, tile_dir) = (live.read(cx).board.clone(), live.read(cx).tile_dir.clone());
        Self { tiles: Tiles::small(), follow: true, ..Self::with(board, tile_dir, Some(live), cx) }
    }

    fn with(board: Entity<Board>, tile_dir: PathBuf, live: Option<Entity<LiveMap>>, cx: &mut Context<Self>) -> Self {
        cx.observe(&board, |stage, board, cx| {
            stage.followed(board.read(cx));
            cx.notify();
        })
        .detach();
        cx.subscribe(&board, |stage, board, event: &BoardEvent, cx| match event {
            BoardEvent::Installed { first, retiled } => stage.installed(*first, *retiled, board.read(cx)),
            BoardEvent::Moved => {
                stage.followed(board.read(cx));
                cx.notify();
            }
            BoardEvent::Filtered => {
                stage.shown.markers = None;
                stage.hover(None, cx);
            }
            BoardEvent::Selected => (stage.shown.target, stage.shown.hovered_mark) = (None, None),
            BoardEvent::MarkAdded | BoardEvent::MarkMoved => {}
            BoardEvent::Bring { to, close } => {
                let board = board.read(cx);
                if let Load::Ready(map) = &board.map {
                    stage.bring(*to, *close, &board.saved.view().tile_set_of(map).1.tiles);
                }
            }
        })
        .detach();
        // Markers are drawn in the accent.
        cx.observe_global::<Look>(|stage, _| stage.stale.extend(stage.images.clear())).detach();
        // A minimap made for a map already up missed its `Installed`.
        let shown = match &board.read(cx).map {
            Load::Ready(map) => Shown::of(map),
            _ => Shown::default(),
        };
        Self {
            board,
            tile_dir,
            epoch: 0,
            set: 0,
            tiles: Tiles::default(),
            images: Images::default(),
            stale: Vec::new(),
            camera: None,
            glide: None,
            clock: None,
            bounds: Bounds::default(),
            tile: 256.,
            shown,
            corners: Vec::new(),
            wanted: Vec::new(),
            drag: None,
            layers_open: false,
            follow: false,
            player: None,
            live,
            focus: cx.focus_handle(),
            _chip: Task::ready(()),
        }
    }

    pub fn take_focus(&self, window: &mut Window, cx: &mut App) {
        self.focus.focus(window, cx);
    }

    /// The camera's center and span, to keep with the map's state.
    pub fn view(&self) -> Option<(f64, f64, f64)> {
        self.camera.map(|camera| (camera.center.x, camera.center.y, camera.span))
    }

    /// Lets go of what only an open map needs.
    pub fn hide(&mut self, window: &mut Window) {
        // Markers are grouped again on the next frame.
        (self.drag, self.layers_open, self.shown.markers, self.player) = (None, false, None, None);
        self.drop_tiles();
        self.stale.extend(self.images.clear());
        for image in self.stale.drain(..) {
            let _ = window.drop_image(image);
        }
    }

    /// Escape: the layers or the right click menu; false when neither was open.
    pub fn close_top(&mut self, cx: &mut Context<Self>) -> bool {
        let open = self.layers_open || self.shown.target.is_some();
        if self.layers_open {
            self.layers_open = false;
        } else {
            self.shown.target = None;
        }
        cx.notify();
        open
    }

    /// A fresher copy of the same map keeps its tiles; its icons may have changed.
    fn installed(&mut self, first: bool, retiled: bool, board: &Board) {
        let Load::Ready(map) = &board.map else { return };
        if retiled {
            self.drop_tiles();
        }
        self.stale.extend(self.images.clear());
        // A gesture begun on another map ends with it.
        (self.shown, self.glide, self.clock, self.drag) = (Shown::of(map), None, None, None);
        if first {
            self.camera = board.saved.view().camera.map(|(x, y, span)| Camera { center: Point { x, y }, span });
        }
    }

    /// Asks for every tile again, those it gave up on included.
    pub fn retile(&mut self, cx: &mut Context<Self>) {
        self.drop_tiles();
        cx.notify();
    }

    // Loads on their way report to an epoch that is gone.
    fn drop_tiles(&mut self) {
        self.epoch += 1;
        self.stale.extend(self.tiles.clear());
    }

    /// Centers on `to`; `close` also zooms in near enough to tell pins apart.
    fn bring(&mut self, to: Point, close: bool, tiles: &model::Tiles) {
        let Some(camera) = &mut self.camera else { return };
        camera.center = to;
        if close {
            // Not deeper: many maps declare levels their CDN does not have, which shows as blur.
            let level = self.tile * 2f64.powi(i32::from(tiles.max_zoom.saturating_sub(3).max(tiles.min_zoom)));
            camera.span = camera.span.max(level);
        }
        (self.glide, self.follow) = (None, false);
    }

    // A run that ended leaves nothing to follow; the next frame moves the view with the player.
    fn followed(&mut self, board: &Board) {
        self.follow = self.live.is_some() || self.follow && board.player.is_some();
    }

    fn center_on_player(&mut self) {
        if let (Some(camera), Some(player)) = (&mut self.camera, self.player) {
            camera.center = player.at;
        }
    }

    fn map(&self, cx: &App) -> Option<Rc<Map>> {
        match &self.board.read(cx).map {
            Load::Ready(map) => Some(map.clone()),
            _ => None,
        }
    }

    fn size(&self) -> Pixel {
        (self.bounds.size.width.into(), self.bounds.size.height.into())
    }

    fn pixel(&self, position: gpui_kit::Point<Pixels>) -> Pixel {
        ((position.x - self.bounds.origin.x).into(), (position.y - self.bounds.origin.y).into())
    }

    /// From half the smallest level, or as far out as the bounds need to fit the view (smallest level's tiles drawn
    /// at an eighth at least, so a deep set still draws few), to twice the largest.
    fn spans(&self, tiles: &model::Tiles) -> RangeInclusive<f64> {
        let level = |zoom: u8| self.tile * 2f64.powi(zoom.into());
        let (width, height) = self.size();
        let whole = tiles.bounds.map_or(f64::MAX, |[low, high]| (width / (high.x - low.x)).min(height / (high.y - low.y)) * 0.9);
        let lowest = level(tiles.min_zoom);
        (lowest / 2.).min(whole.max(lowest / 8.))..=level(tiles.max_zoom) * 2.
    }
}

impl Render for Stage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        for image in self.tiles.begin_frame().into_iter().chain(self.stale.drain(..)) {
            let _ = window.drop_image(image);
        }
        let board = self.board.clone();
        let board = board.read(cx);
        // Keeps the focus in the map, so Escape still closes it.
        let waiting = |tag: SharedString, title: SharedString, text: SharedString| -> AnyElement {
            div().size_full().p(px(24.)).track_focus(&self.focus).child(notice(tag, title, text)).into_any_element()
        };
        let map = match &board.map {
            Load::Ready(map) => map.clone(),
            Load::Pending => {
                return waiting(t(cx, key::MAP_LOADING_TAG), t(cx, key::MAP_LOADING), t(cx, key::MAP_LOADING_BODY));
            }
            Load::Failed(failure) => {
                let body = failed(cx, key::MAP_FAILED_BODY, failure);
                return waiting(t(cx, key::MAP_FAILED_TAG), t(cx, key::MAP_FAILED), body);
            }
        };
        let view = cx.entity();
        let stage = div().id("map-stage").relative().size_full().overflow_hidden().bg(rgb(map.background));
        let canvas = canvas(
            |bounds, window, _| window.insert_hitbox(bounds, HitboxBehavior::Normal),
            move |bounds, hitbox, window, cx| view.update(cx, |stage, cx| stage.paint(bounds, hitbox, window, cx)),
        )
        .size_full();
        if self.live.is_some() {
            return stage.on_scroll(cx.listener(Self::mini_wheel)).child(canvas).into_any_element();
        }
        self.bind_keys(stage.track_focus(&self.focus).key_context(STAGE), cx)
            .child(canvas)
            .children(self.chip(&map, board, window, cx))
            .children(self.target_menu(&map, board, cx))
            .child(self.readout(board, cx))
            .child(self.buttons(&map, board, cx))
            .into_any_element()
    }
}
