//! Painting the map: tiles, regions, labels, pins, the user's own, the player.

use std::{ops::RangeInclusive, rc::Rc, sync::Arc, time::Instant};

use anyhow::Context as _;
use blocking::unblock;
use client::{
    map::{heading::Bearing, state::Saved},
    media::{self, Fetched},
};
use gpui_kit::{
    App, BorderStyle, Bounds, ContentMask, Context, Corners, DispatchPhase, Font, FontWeight, Hitbox, MouseMoveEvent, MouseUpEvent,
    PathBuilder, Pixels, RenderImage, Rgba, SharedString, TextAlign, TextRun, TransformationMatrix, Window, font, point, px, quad, radians,
    rgb, size,
};
use image::{DynamicImage, Frame, RgbaImage};
use model::{Glyph, Map, Point, Tiles};

use super::{
    Stage, Walk,
    camera::{Camera, Pixel, extent},
    marks::mark_order,
    pins::{self, GROUP_RADIUS},
    tiles::{BACKDROP, Key, STAND_INS},
};
use crate::{
    Load,
    map::{
        board::plane,
        markers::{GroupIcon, Look, Marker, PICTURE, SIZE, TIP, ink},
    },
    ui::{
        art::{self, Router},
        assets::Icon,
        theme::*,
    },
};

const PLAYER: f32 = 34.;
const CONE: f32 = 96.;
// The way the player last went, while they stand.
const STALE: f32 = 0.4;
// Opacity of a region's fill while hovered, and of its outline at rest.
const REGION_FILL: f32 = 0.16;
const REGION_LINE: f32 = 0.7;
// A group of pins: the ring's width and opacity.
const RING: (f32, f32) = (1.5, 0.7);
// Place names come at each map's own sizes, kept readable: off the type scale.
const LABEL_SIZES: (f32, f32) = (11., 18.);
// A tile, or a map that is one picture, is held whole in memory and in the atlas: at most this many texels across.
const TILE_SIDE: u32 = 4096;

/// What a frame paints from the board, copied out of it: painting text needs the app, not a borrow of the board.
struct Shot {
    map: Rc<Map>,
    saved: Rc<Saved>,
    selected: Option<usize>,
    mark: Option<Rc<str>>,
    accent: u32,
}

impl Stage {
    /// `hitbox` is the map's own: cards, menus and the lightbox over it occlude it.
    pub(super) fn paint(&mut self, bounds: Bounds<Pixels>, hitbox: Hitbox, window: &mut Window, cx: &mut Context<Self>) {
        let (player, shot) = {
            let board = self.board.read(cx);
            let Load::Ready(map) = &board.map else { return };
            let player =
                board.player.zip(map.frame.as_ref()).map(|(position, frame)| Walk { at: frame.to_map(position), bearing: board.bearing });
            (
                player,
                Shot {
                    map: map.clone(),
                    saved: board.saved.clone(),
                    selected: board.selected,
                    mark: board.mark.clone(),
                    accent: accent(cx),
                },
            )
        };
        let (map, saved, selected, accent) = (shot.map.clone(), &shot.saved, shot.selected, shot.accent);
        let prefs = saved.view();
        let (set, pyramid) = prefs.tile_set_of(&map);
        let pyramid = &pyramid.tiles;
        // Another tile set: its own tiles and levels.
        if set != self.set {
            (self.set, self.shown.markers) = (set, None);
            self.drop_tiles();
        }
        // The overlays were laid out for the old size: one more frame places them.
        if self.bounds != bounds {
            window.request_animation_frame();
        }
        self.bounds = bounds;
        self.tile = f64::from(pyramid.size) / f64::from(window.scale_factor());
        let area = self.size();
        let spans = self.spans(pyramid);
        if self.camera.is_none() {
            self.camera = Some(home(&map, pyramid, area, &spans));
        }
        let mini = self.live.as_ref().map(|live| live.read(cx).span(pyramid.max_zoom, self.tile));
        // Within this set's levels: a view kept from another set may lie outside them.
        if let Some(camera) = &mut self.camera {
            camera.span = mini.unwrap_or(camera.span).clamp(*spans.start(), *spans.end());
        }
        self.animate(&spans, player, window);
        if let (Some(within), Some(camera)) = (pyramid.bounds, &mut self.camera) {
            camera.keep_in(within, area);
        }
        let Some(camera) = self.camera else { return };
        // Tiles load where a zoom ends and a followed player's last fix is: the views passed would only take loads.
        let mut end = self.glide.map_or(camera, |glide| glide.end(camera, area, &spans));
        if self.follow
            && let Some(player) = player
        {
            end.center = player.at;
        }
        if let Some(within) = pyramid.bounds {
            end.keep_in(within, area);
        }

        // On the window, not the element: a drag goes on outside the map. The minimap only zooms.
        let view = cx.entity();
        if mini.is_none() {
            window.on_mouse_event({
                let view = view.clone();
                move |event: &MouseMoveEvent, phase, window, cx| {
                    if phase == DispatchPhase::Bubble {
                        let over = hitbox.is_hovered(window);
                        view.update(cx, |stage, cx| stage.moved(event, over, cx));
                    }
                }
            });
            window.on_mouse_event(move |event: &MouseUpEvent, phase, _, cx| {
                if phase == DispatchPhase::Bubble {
                    view.update(cx, |stage, cx| stage.released(event, cx));
                }
            });
        }

        let place = |(x, y): Pixel| bounds.origin + point(px(x as f32), px(y as f32));
        let disc = |center: gpui_kit::Point<Pixels>, side: f32| {
            Bounds::new(center - point(px(side / 2.), px(side / 2.)), size(px(side), px(side)))
        };
        window.with_content_mask(Some(ContentMask { bounds }), |window| {
            self.paint_tiles(pyramid, &camera, &end, window, cx);

            let zoom = (camera.span / self.tile).log2() as f32;
            let shows = |(low, high): (f32, f32)| (low..=high).contains(&zoom);
            let regions = map
                .regions
                .iter()
                .zip(&self.shown.regions)
                .enumerate()
                .filter(|(_, (region, _))| prefs.regions && shows(region.zoom) && !prefs.hidden_regions.contains(&region.id));
            for (index, (region, extents)) in regions {
                // A miter (lyon's limit is 4) reaches up to twice the stroke's width past a corner.
                let (hovered, pad) = (self.shown.hovered_region == Some(index), f64::from(region.thickness) * 2.);
                // An outline is tessellated anew each frame: none for a ring off the view.
                for (ring, extent) in region.outlines.iter().zip(extents) {
                    let Some((low, high)) = *extent else { continue };
                    let ((left, top), (right, bottom)) = (camera.to_view(low, area), camera.to_view(high, area));
                    if right < -pad || bottom < -pad || left > area.0 + pad || top > area.1 + pad {
                        continue;
                    }
                    self.corners.clear();
                    self.corners.extend(ring.iter().map(|at| place(camera.to_view(*at, area))));
                    if hovered && let Some(fill) = path(&self.corners, PathBuilder::fill()) {
                        window.paint_path(fill, alpha(region.color, REGION_FILL));
                    }
                    if let Some(outline) = path(&self.corners, PathBuilder::stroke(px(region.thickness))) {
                        window.paint_path(outline, alpha(region.color, if hovered { 1. } else { REGION_LINE }));
                    }
                }
            }

            let markers = self.markers(&map, &camera, cx);
            let off = |(x, y): (f64, f64)| x < -SIZE.0 || y < -GROUP_RADIUS || x > area.0 + SIZE.0 || y > area.1 + SIZE.1;
            for marker in markers.iter() {
                let (x, y) = camera.to_view(marker.at, area);
                if off((x, y)) {
                    continue;
                }
                if marker.count > 1 {
                    let color = map.categories[map.pins[marker.pin].group.0].color;
                    let (center, side) = (place((x, y)), GROUP_RADIUS as f32 * 2.);
                    let ring = alpha(ink(color), RING.1);
                    window.paint_quad(quad(disc(center, side), px(side / 2.), rgb(color), px(RING.0), ring, BorderStyle::default()));
                    let count = self.shown.counts.entry(marker.count).or_insert_with(|| marker.count.to_string().into()).clone();
                    text(count, TYPE_CAPTION, ink(color), false, center, window, cx);
                } else if self.look(marker.pin, selected) == Look::Plain {
                    self.paint_marker(&shot, marker.pin, (x, y), window, cx);
                }
            }
            // The hovered pin over the rest, again by iterating: nothing is collected per frame.
            for marker in markers.iter().filter(|marker| marker.count == 1) {
                let at = camera.to_view(marker.at, area);
                if !off(at) && self.look(marker.pin, selected) == Look::Hovered {
                    self.paint_marker(&shot, marker.pin, at, window, cx);
                }
            }
            // The selected pin shows even when a filter or a group hides it, and over the rest.
            if let Some(pin) = selected {
                let tip = camera.to_view(map.pins[pin].at, area);
                self.paint_marker(&shot, pin, tip, window, cx);
            }
            let open = shot.mark.as_deref();
            self.paint_marks(saved, open, &camera, window, cx);
            let pins =
                markers.iter().filter(|marker| marker.count == 1).filter_map(|marker| Some((marker.at, map.pins[marker.pin].position?)));
            let marks = mark_order(saved, open).filter_map(|mark| Some((plane(mark.at), mark.position?)));
            self.paint_rises(&map, pins.chain(marks), &camera, window, cx);

            // Over the pins, as place names are on a map.
            for (label, line) in map.labels.iter().zip(&self.shown.labels).filter(|(label, _)| prefs.labels && shows(label.zoom)) {
                let at = place(camera.to_view(label.at, area));
                text(line.clone(), label.size.clamp(LABEL_SIZES.0, LABEL_SIZES.1), label.color, true, at, window, cx);
            }

            if let Some(Walk { at, bearing }) = self.player {
                let (center, scale) = (place(camera.to_view(at, area)), window.scale_factor());
                if let Some(Bearing { angle, fresh }) = bearing {
                    // About the player, in device pixels; the icon points up.
                    let turn = TransformationMatrix::unit()
                        .translate(center.scale(scale))
                        .rotate(radians(angle as f32))
                        .translate(center.scale(-scale));
                    let color = alpha(accent, if fresh { 1. } else { STALE }).into();
                    let _ = window.paint_svg(disc(center, CONE), Icon::Heading.path().into(), None, turn, color, cx);
                }
                let color = rgb(accent).into();
                let _ = window.paint_svg(disc(center, PLAYER), Icon::Player.path().into(), None, TransformationMatrix::unit(), color, cx);
            }
        });
    }

    pub(super) fn markers(&mut self, map: &Map, camera: &Camera, cx: &App) -> Rc<Vec<pins::Marker>> {
        // Half-level steps: pins do not regroup on every wheel tick.
        let step = (camera.span.log2() * 2.).round() as i32;
        if let Some((_, markers)) = self.shown.markers.as_ref().filter(|(held, _)| *held == step) {
            return markers.clone();
        }
        let board = self.board.read(cx);
        let span = 2f64.powf(step as f64 / 2.);
        let deepest = board.saved.view().tile_set_of(map).1.tiles.max_zoom;
        let close = span >= self.tile * 2f64.powi(deepest.into()) * 0.9;
        let view = board.saved.view();
        let clusters = view.clusters && (self.live.is_none() || view.mini_clusters);
        let markers = Rc::new(pins::markers(&map.pins, |index| board.shows(map, index), span, close || !clusters));
        self.shown.markers = Some((step, markers.clone()));
        markers
    }

    fn paint_marker(&mut self, shot: &Shot, index: usize, tip: Pixel, window: &mut Window, cx: &App) {
        let pin = &shot.map.pins[index];
        let category = &shot.map.categories[pin.group.0];
        let (glyph, picture) = match self.board.read(cx).icon(pin.group, cx) {
            GroupIcon::Glyph(glyph) => (Some(glyph), None),
            GroupIcon::Picture(picture) => (None, Some(picture)),
            GroupIcon::Default => (None, None),
        };
        let (look, found) = (self.look(index, shot.selected), shot.saved.found(&pin.id));
        let marker = Marker { group: Some(pin.group), fill: category.color, look, found, level: pin.level, picture: picture.is_some() };
        self.paint_look(marker, glyph, tip, window, cx);
        let Some(picture) = picture else { return };
        let (left, top, side) = PICTURE;
        let (scale, device) = (look.scale(), window.scale_factor());
        // Selected, the head turns bright: the picture takes the mark's ink, as a glyph does.
        let tint = (look == Look::Selected).then(|| marker.ink());
        let image = self.images.picture(pin.group, picture, (side * scale * f64::from(device)).round() as u32, tint);
        let corner = (tip.0 - (TIP.0 - left) * scale, tip.1 - (TIP.1 - top) * scale);
        // Gray as the faded shape under it, once found.
        self.paint_snapped(image, corner, found && look != Look::Selected, window);
    }

    /// A marker at its look's size with its tip on `tip`.
    pub(super) fn paint_look(&mut self, marker: Marker, glyph: Option<&Glyph>, tip: Pixel, window: &mut Window, cx: &App) {
        let scale = marker.look.scale();
        let Some(image) = self.images.marker(marker, glyph, window.scale_factor(), cx) else { return };
        self.paint_snapped(image, (tip.0 - TIP.0 * scale, tip.1 - TIP.1 * scale), false, window);
    }

    /// An image made at device pixels, its top left at `corner` on a whole device pixel so it stays sharp.
    fn paint_snapped(&self, image: Arc<RenderImage>, corner: Pixel, gray: bool, window: &mut Window) {
        let device = window.scale_factor();
        let snap = |at: f64| ((at * f64::from(device)).round() / f64::from(device)) as f32;
        let corner = self.bounds.origin + point(px(snap(corner.0)), px(snap(corner.1)));
        let (pixels, shrink) = (image.size(0), 1. / device);
        let rect = Bounds::new(corner, size(px(pixels.width.0 as f32 * shrink), px(pixels.height.0 as f32 * shrink)));
        let _ = window.paint_image(rect, rect, Corners::default(), image, 0, gray);
    }

    /// Draws `camera`'s tiles and loads `end`'s, the view it is on its way to.
    fn paint_tiles(&mut self, pyramid: &Tiles, camera: &Camera, end: &Camera, window: &mut Window, cx: &mut Context<Self>) {
        let (area, origin) = (self.size(), self.bounds.origin);
        let level = camera.level(self.tile, &(pyramid.min_zoom..=pyramid.max_zoom));
        let (columns, rows) = camera.tiles(level, area);
        let rect = |[left, top, right, bottom]: [f64; 4]| {
            let corner = |x: f64, y: f64| origin + point(px(x as f32), px(y as f32));
            Bounds::from_corners(corner(left, top), corner(right, bottom))
        };
        for row in rows {
            for column in columns.clone().filter(|column| pyramid.covers(level, *column, row)) {
                let place = rect(camera.tile(level, column, row, area));
                // A stand-in paints only its share of the tile: an ancestor's part scaled up, a descendant whole.
                self.tiles.draw((level, column, row), |(from, column, row), image| {
                    let whole = rect(camera.tile(from, column, row, area));
                    let part = place.intersect(&whole);
                    // Out to the padded image's edge, so `part` maps onto the real texels.
                    let texels = image.size(0).width.0.saturating_sub(2).max(1);
                    let whole = whole.dilate(whole.size.width / texels as f32);
                    let _ = window.paint_image(part, whole, Corners::default(), image, 0, false);
                });
            }
        }
        self.load_view(pyramid, end, cx);
    }

    /// Asks for what `camera` shows: its backdrop first, then its own tiles from the middle out. With every load
    /// taken it only keeps what is loaded.
    fn load_view(&mut self, pyramid: &Tiles, camera: &Camera, cx: &mut Context<Self>) {
        let (area, now, free) = (self.size(), Instant::now(), !self.tiles.busy());
        let level = camera.level(self.tile, &(pyramid.min_zoom..=pyramid.max_zoom));
        let backdrop = level.saturating_sub(BACKDROP).max(pyramid.min_zoom);
        // No higher than the view's own stand-ins reach: a backdrop's ancestor above them would never show.
        let lowest = level.saturating_sub(STAND_INS).max(pyramid.min_zoom);
        let mut wanted = std::mem::take(&mut self.wanted);
        for level in std::iter::once(backdrop).chain((level > backdrop).then_some(level)) {
            let (columns, rows) = camera.tiles(level, area);
            let middle = ((columns.start() + columns.end()) / 2, (rows.start() + rows.end()) / 2);
            for row in rows {
                for column in columns.clone().filter(|column| pyramid.covers(level, *column, row)) {
                    if let Some(load) = self.tiles.need((level, column, row), lowest, now)
                        && free
                    {
                        let distance = column.abs_diff(middle.0).pow(2) + row.abs_diff(middle.1).pow(2);
                        wanted.push((level != backdrop, distance, load));
                    }
                }
            }
        }
        wanted.sort_unstable_by_key(|(view, distance, _)| (*view, *distance));
        for (.., key) in wanted.drain(..) {
            if self.tiles.claim(key, now) {
                self.load_tile(pyramid, key, cx);
            }
        }
        self.wanted = wanted;
    }

    fn load_tile(&self, pyramid: &Tiles, key: Key, cx: &mut Context<Self>) {
        let (url, cache, epoch) = (pyramid.url(key.0, key.1, key.2), self.tile_dir.clone(), self.epoch);
        let registry = Router::registry(cx);
        cx.spawn(async move |stage, cx| {
            // Bytes that are no tile count as one the server lacks.
            let fetched = unblock(move || {
                media::tile(&cache, &url, &registry).and_then(|bytes| match decode(&bytes) {
                    Ok(image) => Fetched::Ready(image),
                    Err(e) => {
                        log::debug!("tile {url}: {e:#}");
                        Fetched::Absent
                    }
                })
            })
            .await;
            stage
                .update(cx, |stage, cx| {
                    if stage.epoch == epoch {
                        stage.tiles.loaded(key, fetched, Instant::now());
                        cx.notify();
                    }
                })
                .ok();
        })
        .detach();
    }
}

/// A tile in the atlas's BGRA with its edge texels repeated around it: the atlas has no gaps and filters across an
/// image's border, which shows as seams. One not square, as a map that is one picture may be, is made square with
/// clear texels right and below.
fn decode(bytes: &[u8]) -> anyhow::Result<Arc<RenderImage>> {
    // Most tiles are lossy RGB: read as decoded, without an RGBA copy first.
    let ((width, height), texels, depth) = match art::picture(bytes, TILE_SIDE)? {
        DynamicImage::ImageRgb8(tile) => (tile.dimensions(), tile.into_raw(), 3),
        tile => {
            let tile = tile.into_rgba8();
            (tile.dimensions(), tile.into_raw(), 4)
        }
    };
    let (columns, rows) = (width as usize, height as usize);
    let side = columns.max(rows);
    let mut padded = Vec::with_capacity((side + 2) * (side + 2) * 4);
    for y in (0..side + 2).map(|y| y.saturating_sub(1).min(side - 1)) {
        for x in (0..side + 2).map(|x| x.saturating_sub(1).min(side - 1)) {
            if x >= columns || y >= rows {
                padded.extend_from_slice(&[0; 4]);
                continue;
            }
            let texel = &texels[(y * columns + x) * depth..][..depth];
            padded.extend_from_slice(&[texel[2], texel[1], texel[0], texel.get(3).copied().unwrap_or(u8::MAX)]);
        }
    }
    let padded = RgbaImage::from_raw(side as u32 + 2, side as u32 + 2, padded).context("tile size")?;
    Ok(Arc::new(RenderImage::new([Frame::new(padded)])))
}

/// What the tiles cover, else the pins, whole with room around them; the middle of the plane for a map without either.
fn home(map: &Map, pyramid: &Tiles, area: Pixel, spans: &RangeInclusive<f64>) -> Camera {
    match pyramid.bounds.map(|[low, high]| (low, high)).or_else(|| extent(map.pins.iter().map(|pin| pin.at))) {
        Some((low, high)) => Camera::fit(low, high, (area.0 * 0.9, area.1 * 0.9), spans),
        None => Camera { center: Point { x: 0.5, y: 0.5 }, span: spans.start() * 2. },
    }
}

fn path(corners: &[gpui_kit::Point<Pixels>], mut builder: PathBuilder) -> Option<gpui_kit::Path<Pixels>> {
    let (first, rest) = corners.split_first()?;
    builder.move_to(*first);
    rest.iter().for_each(|corner| builder.line_to(*corner));
    builder.close();
    builder.build().ok()
}

/// One line centered on `at`, unless it is off the view; a shadow keeps it readable on any tile.
pub(super) fn text(
    text: SharedString,
    height: f32,
    color: u32,
    shadow: bool,
    at: gpui_kit::Point<Pixels>,
    window: &mut Window,
    cx: &mut App,
) {
    let line_height = px(height * 1.3);
    // GPUI keeps shapes between frames, and a shape does not hold its color: the shadow's costs a lookup.
    let family = ui_font(cx);
    let shape = |color: Rgba, window: &mut Window| {
        let font = Font { weight: FontWeight::BOLD, ..font(family) };
        let run = TextRun { len: text.len(), font, color: color.into(), background_color: None, underline: None, strikethrough: None };
        window.text_system().shape_line(text.clone(), px(height), &[run], None)
    };
    let line = shape(rgb(color), window);
    let origin = point(at.x - line.width / 2., at.y - line_height / 2.);
    if !window.content_mask().bounds.intersects(&Bounds::new(origin, size(line.width + px(1.), line_height + px(1.)))) {
        return;
    }
    if shadow {
        let _ = shape(alpha(BG, 0.9), window).paint(origin + point(px(1.), px(1.)), line_height, TextAlign::Left, None, window, cx);
    }
    let _ = line.paint(origin, line_height, TextAlign::Left, None, window, cx);
}

#[cfg(test)]
mod tests {
    use image::{
        ExtendedColorType, ImageEncoder,
        codecs::{jpeg::JpegEncoder, png::PngEncoder, webp::WebPEncoder},
    };

    #[test]
    fn a_tile_is_padded_with_its_edges_in_bgra_and_made_square() {
        // Red, then green: without alpha, and with the green half transparent.
        for (texels, color, alpha) in [
            (vec![255, 0, 0, 0, 255, 0], ExtendedColorType::Rgb8, 255),
            (vec![255, 0, 0, 255, 0, 255, 0, 128], ExtendedColorType::Rgba8, 128),
        ] {
            let mut webp = Vec::new();
            WebPEncoder::new_lossless(&mut webp).encode(&texels, 2, 1, color).unwrap();
            let tile = super::decode(&webp).unwrap();
            assert_eq!((tile.size(0).width.0, tile.size(0).height.0), (4, 4));
            let row = [0, 0, 255, 255, 0, 0, 255, 255, 0, 255, 0, alpha, 0, 255, 0, alpha];
            assert_eq!(tile.as_bytes(0).unwrap(), [row.repeat(2), [0; 32].to_vec()].concat());
        }
    }

    #[test]
    fn tiles_are_webp_png_or_jpeg_by_their_first_bytes() {
        let gray = [128u8; 4 * 4 * 3];
        let (mut png, mut jpeg) = (Vec::new(), Vec::new());
        PngEncoder::new(&mut png).write_image(&gray, 4, 4, ExtendedColorType::Rgb8).unwrap();
        JpegEncoder::new(&mut jpeg).write_image(&gray, 4, 4, ExtendedColorType::Rgb8).unwrap();
        for bytes in [png, jpeg] {
            assert_eq!(super::decode(&bytes).unwrap().size(0).width.0, 6);
        }
        assert!(super::decode(b"GIF89a\x01\x00\x01\x00").is_err());
        assert!(super::decode(b"<html>403</html>").is_err());
    }
}
