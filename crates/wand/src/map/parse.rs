//! The typed part of `GET /v3/game_maps/:id` and its conversion into `model::Map`.

use std::{
    collections::{HashMap, HashSet},
    f64::consts::PI,
};

use anyhow::{Context, Result};
use model::{Frame, Group, Level, Point};
use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;

use super::archives;
use crate::catalog::ART_CDN;

const STORAGE_CDN: &str = "https://storage-cdn.wemod.com";
const GRAY: u32 = 0x80_80_80;
/// The sphere Wand's coordinates are projected on.
const CIRCUMFERENCE: f64 = 2. * PI * 6_371_000.;

fn or_default<'de, D: Deserializer<'de>, T: Deserialize<'de> + Default>(d: D) -> Result<T, D::Error> {
    Ok(Option::deserialize(d)?.unwrap_or_default())
}

#[derive(Deserialize)]
struct Response<'a> {
    #[serde(default, deserialize_with = "or_default")]
    metadata: Metadata,
    #[serde(borrow, default, deserialize_with = "or_default")]
    geojson: Collection<'a>,
    #[serde(borrow, default, deserialize_with = "or_default")]
    regions: Collection<'a>,
    #[serde(borrow, default, deserialize_with = "or_default")]
    labels: Collection<'a>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    background_color: Option<String>,
    supports_teleport: Option<bool>,
    tile_source: Option<TileSource>,
    #[serde(default, deserialize_with = "or_default")]
    categories: Vec<Category>,
    #[serde(default, deserialize_with = "or_default")]
    subcategories: Vec<Subcategory>,
    player_location_deltas: Option<Located>,
    is_in_active_development: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TileSource {
    #[serde(rename = "type")]
    kind: Option<String>,
    url_template: Option<String>,
    /// A PMTiles archive's.
    url: Option<String>,
    scheme: Option<String>,
    tile_size: Option<u32>,
    min_zoom: Option<u8>,
    max_zoom: Option<u8>,
    /// Read only when used: a broken one must not lose a map its archive can show.
    fallback: Option<Box<RawValue>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Category {
    external_id: String,
    title: Option<String>,
    display_order: Option<f64>,
    color: Option<String>,
    visible_by_default: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Subcategory {
    external_id: String,
    category_external_id: String,
    title: Option<String>,
    icon: Option<String>,
}

#[derive(Deserialize)]
struct Located {
    deltas: Option<Deltas>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Deltas {
    scale_x: Option<f64>,
    scale_y: Option<f64>,
    offset_x: Option<f64>,
    offset_y: Option<f64>,
    game_coords_map: Option<AxisNames>,
}

/// Which of the game's coordinates (`x`, `y`, `z`) runs along each of the plane's axes.
#[derive(Deserialize)]
struct AxisNames {
    x: Option<String>,
    y: Option<String>,
}

#[derive(Default, Deserialize)]
struct Collection<'a> {
    // Each feature is read on its own from the source text: a malformed one is skipped, and no JSON tree is built.
    #[serde(borrow, default, deserialize_with = "or_default")]
    features: Vec<&'a RawValue>,
}

#[derive(Deserialize)]
struct Feature<P> {
    geometry: Geometry,
    properties: P,
}

#[derive(Deserialize)]
#[serde(tag = "type", content = "coordinates")]
enum Geometry {
    Point(Vec<f64>),
    Polygon(Vec<Vec<Vec<f64>>>),
    #[serde(other)]
    Other,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PinProperties {
    external_id: Option<String>,
    title: Option<String>,
    description: Option<String>,
    subcategory_external_id: Option<String>,
    game_coords: Option<Coords>,
    image_id: Option<u64>,
    image_path: Option<String>,
    video_path: Option<String>,
    vertical_level: Option<String>,
}

#[derive(Deserialize)]
struct Coords {
    x: f64,
    y: f64,
    z: f64,
}

/// What regions and labels share.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Style {
    external_id: Option<String>,
    title: Option<String>,
    color: Option<String>,
    line_thickness: Option<f32>,
    font_size: Option<f32>,
    min_zoom: Option<f32>,
    max_zoom: Option<f32>,
}

impl Style {
    fn zoom(&self) -> (f32, f32) {
        (self.min_zoom.unwrap_or(0.), self.max_zoom.unwrap_or(24.))
    }
}

/// The map and, per group in category order, the raw name of its icon.
pub fn map(raw: &str) -> Result<(model::Map, Vec<Vec<String>>)> {
    let Response { metadata, geojson, regions, labels } = serde_json::from_str(raw).context("unexpected map format")?;
    let tiles = tiles(metadata.tile_source)?;
    let frame = metadata.player_location_deltas.and_then(|located| located.deltas).and_then(frame);

    let mut ordered = metadata.categories;
    // Without an order a category goes last, in the given order.
    ordered.sort_by(|a, b| {
        a.display_order
            .is_none()
            .cmp(&b.display_order.is_none())
            .then_with(|| a.display_order.unwrap_or_default().total_cmp(&b.display_order.unwrap_or_default()))
    });
    let position: HashMap<&str, usize> =
        ordered.iter().enumerate().map(|(index, category)| (category.external_id.as_str(), index)).collect();
    let mut categories: Vec<model::Category> = ordered
        .iter()
        .map(|category| model::Category {
            name: category.title.clone().unwrap_or_default(),
            color: color(category.color.as_deref(), GRAY),
            shown: category.visible_by_default.unwrap_or(true),
            groups: Vec::new(),
        })
        .collect();
    let mut icons = vec![Vec::new(); categories.len()];
    let mut groups = HashMap::new();
    for sub in metadata.subcategories {
        let Some(&category) = position.get(sub.category_external_id.as_str()) else { continue };
        groups.insert(sub.external_id.clone(), (category, categories[category].groups.len()));
        icons[category].push(sub.icon.unwrap_or_default());
        categories[category].groups.push(Group { id: sub.external_id, name: sub.title.unwrap_or_default(), icon: None });
    }

    let pins = features::<PinProperties>(geojson).filter_map(|Feature { geometry, properties: p }| {
        let Geometry::Point(at) = geometry else { return None };
        let at = point(&at)?;
        let group = *groups.get(p.subcategory_external_id.as_deref()?)?;
        // The CDN has 250, 750 and 1920 px; 750 stays sharp on the card.
        let path = match (p.image_id, p.image_path.filter(|path| !path.is_empty())) {
            (Some(id), _) => Some(format!("{ART_CDN}/game_map_pin/{id}/{{size}}.webp")),
            (None, Some(path)) if path.starts_with("http") => Some(path),
            (None, Some(path)) => Some(format!("{STORAGE_CDN}{path}")),
            (None, None) => None,
        };
        let image = path.map(|path| model::Picture { preview: path.replace("{size}", "750"), full: path.replace("{size}", "1920") });
        Some(model::Pin {
            // Where it lies outlives its place in the list, so found marks stay on it.
            id: p.external_id.unwrap_or_else(|| format!("@{:.7},{:.7}", at.x, at.y)),
            at,
            // Untitled pins go by their group, as on Wand's map.
            name: p.title.filter(|title| !title.is_empty()).unwrap_or_else(|| categories[group.0].groups[group.1].name.clone()),
            description: p.description.unwrap_or_default(),
            group,
            image,
            video: p.video_path.filter(|video| !video.is_empty()),
            // In the game's own order, as the trainer's position is.
            position: p.game_coords.map(|game| [game.x, game.y, game.z]),
            level: match p.vertical_level.as_deref() {
                Some("above") => Some(Level::Above),
                Some("below") => Some(Level::Below),
                _ => None,
            },
        })
    });
    let mut pins: Vec<model::Pin> = pins.collect();
    let ids: HashSet<String> = pins.iter().map(|pin| pin.id.clone()).collect();
    for pin in &mut pins {
        pin.description = relink(&pin.description, &ids);
    }

    let regions = features::<Style>(regions)
        .filter_map(|Feature { geometry, properties: style }| {
            let Geometry::Polygon(rings) = geometry else { return None };
            Some(model::Region {
                outlines: vec![rings.first()?.iter().filter_map(|at| point(at)).collect()],
                color: color(style.color.as_deref(), GRAY),
                thickness: style.line_thickness.unwrap_or(1.),
                zoom: style.zoom(),
                id: style.external_id.or_else(|| style.title.clone()).unwrap_or_default(),
                name: style.title.unwrap_or_default(),
            })
        })
        .collect();
    let labels = features::<Style>(labels)
        .filter_map(|Feature { geometry, properties: style }| {
            let Geometry::Point(at) = geometry else { return None };
            Some(model::Label {
                at: point(&at)?,
                color: color(style.color.as_deref(), GRAY),
                size: style.font_size.unwrap_or(14.),
                zoom: style.zoom(),
                text: style.title.unwrap_or_default(),
            })
        })
        .collect();

    let map = model::Map {
        // Wand has one; a lone set's name never shows.
        tile_sets: vec![model::TileSet { id: "base".into(), name: model::Text::Plain(String::new()), tiles }],
        background: color(metadata.background_color.as_deref(), 0),
        categories,
        pins,
        regions,
        labels,
        teleport: metadata.supports_teleport == Some(true) && frame.is_some(),
        frame,
        unfinished: metadata.is_in_active_development == Some(true),
    };
    Ok((map, icons))
}

fn features<'a, P: Deserialize<'a>>(collection: Collection<'a>) -> impl Iterator<Item = Feature<P>> {
    collection.features.into_iter().filter_map(|feature| serde_json::from_str(feature.get()).ok())
}

/// An archive's xyz fallback first: its tiles cost one request each, no directory.
fn tiles(source: Option<TileSource>) -> Result<model::Tiles> {
    let tiles = source.and_then(|source| {
        if source.kind.as_deref() != Some("pmtiles") {
            return xyz(source);
        }
        let fallback = source.fallback.and_then(|fallback| serde_json::from_str(fallback.get()).ok()).and_then(xyz);
        fallback.or_else(|| {
            let template = archives::template(source.url.as_deref()?)?;
            Some(levels(template, false, source.tile_size, source.min_zoom, source.max_zoom))
        })
    });
    tiles.context("unsupported tile source")
}

fn xyz(source: TileSource) -> Option<model::Tiles> {
    let template = source.url_template.filter(|_| source.kind.as_deref() == Some("xyz"))?;
    Some(levels(template, source.scheme.as_deref() == Some("tms"), source.tile_size, source.min_zoom, source.max_zoom))
}

fn levels(template: String, flipped: bool, size: Option<u32>, min_zoom: Option<u8>, max_zoom: Option<u8>) -> model::Tiles {
    let max_zoom = max_zoom.unwrap_or_default().min(model::Tiles::MAX_ZOOM);
    model::Tiles {
        template,
        flipped,
        size: size.unwrap_or(256),
        min_zoom: min_zoom.unwrap_or_default().min(max_zoom),
        max_zoom,
        bounds: None,
    }
}

/// `#rrggbb`; anything else is `fallback`.
fn color(text: Option<&str>, fallback: u32) -> u32 {
    text.and_then(|text| text.strip_prefix('#'))
        .filter(|hex| hex.len() == 6 && hex.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .and_then(|hex| u32::from_str_radix(hex, 16).ok())
        .unwrap_or(fallback)
}

/// Wand's links to pins (`wemod://maps/<title>/<map>?location=<pin id>`) point at the pin with `model::PIN_LINK`;
/// other links into Wand keep their text only, so a click never starts it.
fn relink(markdown: &str, pins: &HashSet<String>) -> String {
    let (mut out, mut rest) = (String::with_capacity(markdown.len()), markdown);
    while let Some(start) = rest.find("](wemod:") {
        let Some(end) = rest[start..].find(')').map(|end| start + end) else { break };
        let pin =
            rest[start + 2..end].split_once("location=").map(|(_, id)| id.split('&').next().unwrap_or(id)).filter(|id| pins.contains(*id));
        match (pin, label_start(&rest[..start])) {
            (Some(id), _) => out.push_str(&format!("{}]({}{id})", &rest[..start], model::PIN_LINK)),
            (None, Some(open)) => out.push_str(&format!("{}{}", &rest[..open], &rest[open + 1..start])),
            (None, None) => out.push_str(&rest[..start]),
        }
        rest = &rest[end + 1..];
    }
    out.push_str(rest);
    out
}

/// Where the link text that `before` ends with opens, past brackets nested in it.
fn label_start(before: &str) -> Option<usize> {
    let mut depth = 0;
    before.char_indices().rev().find_map(|(at, c)| match c {
        ']' => {
            depth += 1;
            None
        }
        '[' if depth == 0 => Some(at),
        '[' => {
            depth -= 1;
            None
        }
        _ => None,
    })
}

/// Projects `[lng, lat]` in degrees to normalized Web Mercator (y down), without clamping.
fn point(coordinates: &[f64]) -> Option<Point> {
    let &[lng, lat, ..] = coordinates else { return None };
    let at = Point { x: (lng + 180.) / 360., y: 0.5 - (PI / 4. + lat * PI / 360.).tan().ln() / (2. * PI) };
    (at.x.is_finite() && at.y.is_finite()).then_some(at)
}

fn frame(deltas: Deltas) -> Option<Frame> {
    let names = deltas.game_coords_map?;
    let axis = |name: Option<String>| ["x", "y", "z"].iter().position(|known| Some(*known) == name.as_deref());
    let (along_x, along_y) = (axis(names.x)?, axis(names.y)?);
    // Up is the axis the plane leaves, whatever `z` names: a map may name one of the plane's there (Castor Woods).
    let axes = [along_x, along_y, 3usize.checked_sub(along_x + along_y)?];
    let (x, y) = (deltas.scale_x? * CIRCUMFERENCE, deltas.scale_y? * CIRCUMFERENCE);
    let scale = (1. / x, -1. / y);
    let origin = Point { x: 0.5 - deltas.offset_x? / x, y: 0.5 + deltas.offset_y? / y };
    let sound = along_x != along_y && scale.0.is_normal() && scale.1.is_normal() && origin.x.is_finite() && origin.y.is_finite();
    sound.then_some(Frame { axes, origin, scale })
}

#[cfg(test)]
mod tests;
