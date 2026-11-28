use serde_json::{Value, json};

use super::*;

fn parse(raw: &Value) -> Result<(model::Map, Vec<Vec<String>>)> {
    map(&raw.to_string())
}

fn xyz() -> Value {
    json!({"type": "xyz", "urlTemplate": "https://cdn/{z}/{y}/{x}.webp", "scheme": "xyz", "minZoom": 1, "maxZoom": 8})
}

fn deltas(names: Value) -> Value {
    json!({"deltas": {"scaleX": 0.00042038190573136084, "scaleY": 0.0004196856967604749,
        "offsetX": 5118.295039094836, "offsetY": 5113.537427568494, "gameCoordsMap": names},
        "referencePoints": []})
}

fn straight() -> Value {
    deltas(json!({"x": "x", "y": "y", "z": "z"}))
}

fn response(source: Value, located: Value) -> Value {
    json!({"metadata": {
        "backgroundColor": "#604c44", "supportsTeleport": true, "tileSource": source,
        "categories": [
            {"externalId": "late", "title": "Late", "displayOrder": null, "color": "#5750b4"},
            {"externalId": "second", "title": "Second", "displayOrder": 2, "visibleByDefault": false},
            {"externalId": "first", "title": "First", "displayOrder": 1, "color": "nope"}],
        "subcategories": [
            {"externalId": "a", "categoryExternalId": "first", "title": "A", "icon": "i-a"},
            {"externalId": "b", "categoryExternalId": "late", "title": "B", "icon": "i-b"},
            {"externalId": "c", "categoryExternalId": "first", "title": "C", "icon": "i-c"},
            {"externalId": "lost", "categoryExternalId": "unknown", "title": "L", "icon": "x"}],
        "isInActiveDevelopment": true, "playerLocationDeltas": located},
      "geojson": {"features": [
        {"geometry": {"type": "Point", "coordinates": [0, 0]},
         "properties": {"externalId": "p1", "verticalLevel": "below", "title": "Vault", "description": null, "subcategoryExternalId": "c",
                        "gameCoords": {"x": 1, "y": 2, "z": 3}, "imageId": 7, "videoPath": "https://v/x"}},
        {"geometry": {"type": "Point", "coordinates": [10, 10]},
         "properties": {"verticalLevel": "sideways", "subcategoryExternalId": "b", "imagePath": "/game_map_pin/9/{size}.webp", "description": "text"}},
        {"geometry": {"type": "Point", "coordinates": [20, 20]},
         "properties": {"subcategoryExternalId": "a", "imagePath": "https://o/{size}.webp"}},
        {"geometry": {"type": "Point", "coordinates": [30, 30]}, "properties": {"subcategoryExternalId": "lost"}},
        {"geometry": {"type": "Point", "coordinates": [30, 30]}, "properties": {"subcategoryExternalId": "zzz"}},
        {"geometry": {"type": "Polygon", "coordinates": []}, "properties": {"subcategoryExternalId": "a"}},
        {"geometry": {"type": "Point", "coordinates": ["bad"]}, "properties": {"subcategoryExternalId": "a"}}]},
      "regions": {"features": [
        {"geometry": {"type": "Polygon", "coordinates": [[[0, 0], [180, 0], [180, 45]]]},
         "properties": {"title": "Tower", "color": "#01579B", "lineThickness": 2, "minZoom": 3}},
        {"geometry": {"type": "Point", "coordinates": [0, 0]}, "properties": {"title": "Not a region"}}]},
      "labels": {"features": [
        {"geometry": {"type": "Point", "coordinates": [0, 0]}, "properties": {"title": "Passage", "maxZoom": 15}}]}})
}

#[test]
fn lng_lat_land_on_the_unit_square() {
    let at = point(&[0., 0.]).unwrap();
    assert!((at.x - 0.5).abs() < 1e-12 && (at.y - 0.5).abs() < 1e-12);
    assert!((point(&[180., 0.]).unwrap().x - 1.).abs() < 1e-12);
    assert!(point(&[0., 85.0511287798]).unwrap().y.abs() < 1e-6);
    assert!(point(&[0., 100.]).is_none() && point(&[1.]).is_none());
}

#[test]
fn the_frame_lands_on_wands_reference_point() {
    let map = parse(&response(xyz(), straight())).unwrap().0;
    let frame = map.frame.unwrap();
    assert_eq!(frame.axes, [0, 1, 2]);
    let at = frame.to_map([3066., 1376., 1002.]);
    let want = point(&[-43.904679178966944, -62.23691636132844]).unwrap();
    assert!((at.x - want.x).abs() < 1e-6 && (at.y - want.y).abs() < 1e-6, "{at:?} {want:?}");
    assert!(map.teleport);
    assert_eq!(map.background, 0x604c44);
}

#[test]
fn no_deltas_no_frame_and_no_teleport() {
    let map = parse(&response(xyz(), Value::Null)).unwrap().0;
    assert!(map.frame.is_none() && !map.teleport);
    assert_eq!(map.pins[0].position, Some([1., 2., 3.]));
    let broken = json!({"deltas": {"scaleX": 0, "scaleY": 1, "offsetX": 0, "offsetY": 0, "gameCoordsMap": {"x": "x", "y": "y", "z": "z"}}});
    assert!(parse(&response(xyz(), broken)).unwrap().0.frame.is_none());
    let twice = deltas(json!({"x": "x", "y": "x", "z": "z"}));
    assert!(parse(&response(xyz(), twice)).unwrap().0.frame.is_none());
}

#[test]
fn swapped_axes_reach_the_frame_and_the_pin_keeps_the_games_order() {
    let map = parse(&response(xyz(), deltas(json!({"x": "y", "y": "x", "z": "z"})))).unwrap().0;
    assert_eq!(map.frame.unwrap().axes, [1, 0, 2]);
    assert_eq!(map.pins[0].position, Some([1., 2., 3.]));
}

#[test]
fn up_is_the_axis_the_plane_leaves_whatever_z_names() {
    // Castor Woods: `z` names the plane's y axis again; its reference point lands with y up.
    let located = json!({"deltas": {"scaleX": 0.00011179562954028567, "scaleY": 0.00011208634994290028,
        "offsetX": 236.06631381320608, "offsetY": 241.17866687476067, "gameCoordsMap": {"x": "z", "y": "x", "z": "x"}}});
    let frame = parse(&response(xyz(), located)).unwrap().0.frame.unwrap();
    assert_eq!(frame.axes, [2, 0, 1]);
    let at = frame.to_map([865.3792724609375, 16.39686393737793, -620.7015380859375]);
    let want = point(&[-68.9212846367867, 44.70478244273227]).unwrap();
    assert!((at.x - want.x).abs() < 1e-6 && (at.y - want.y).abs() < 1e-6, "{at:?} {want:?}");
}

#[test]
fn reads_tms_the_pmtiles_fallback_and_an_archive_of_the_maps_cdn() {
    let tiles = |source| parse(&response(source, Value::Null)).unwrap().0.tile_sets.remove(0).tiles;
    let tms = json!({"type": "xyz", "urlTemplate": "u/{z}/{x}/{y}.webp", "scheme": "tms", "tileSize": 512, "minZoom": 1, "maxZoom": 6});
    let want = model::Tiles { template: "u/{z}/{x}/{y}.webp".into(), flipped: true, size: 512, min_zoom: 1, max_zoom: 6, bounds: None };
    assert_eq!(tiles(tms), want);
    let pmtiles = json!({"type": "pmtiles", "url": "https://cdn/map.pmtiles", "minZoom": 0, "maxZoom": 9, "fallback": xyz()});
    let want =
        model::Tiles { template: "https://cdn/{z}/{y}/{x}.webp".into(), flipped: false, size: 256, min_zoom: 1, max_zoom: 8, bounds: None };
    assert_eq!(tiles(pmtiles), want);
    let uuid = "00000000-0000-4000-8000-000000000001";
    let url = format!("https://maps-cdn.wemod.com/maps/{uuid}/map.pmtiles");
    let archive = json!({"type": "pmtiles", "url": url, "minZoom": 0, "maxZoom": 7, "fallback": {"type": "xyz", "urlTemplate": 123}});
    let template = format!("wand:pmtiles/{uuid}/{{z}}/{{x}}/{{y}}");
    assert_eq!(tiles(archive), model::Tiles { template, flipped: false, size: 256, min_zoom: 0, max_zoom: 7, bounds: None });
    let deep = json!({"type": "xyz", "urlTemplate": "u", "minZoom": 40, "maxZoom": 60});
    let deep = tiles(deep);
    assert_eq!((deep.min_zoom, deep.max_zoom), (model::Tiles::MAX_ZOOM, model::Tiles::MAX_ZOOM));
}

#[test]
fn an_unsupported_tile_source_is_an_error() {
    for source in [json!({"type": "pmtiles", "url": "u"}), json!({"type": "raster"}), Value::Null] {
        let Err(e) = parse(&response(source, Value::Null)) else { panic!("accepted") };
        assert!(e.to_string().contains("unsupported tile source"));
    }
}

#[test]
fn categories_follow_their_order_and_pins_their_group() {
    let (map, icons) = parse(&response(xyz(), Value::Null)).unwrap();
    let names: Vec<_> = map.categories.iter().map(|c| (c.name.as_str(), c.color, c.shown, c.groups.len())).collect();
    assert_eq!(names, [("First", GRAY, true, 2), ("Second", GRAY, false, 0), ("Late", 0x5750b4, true, 1)]);
    assert_eq!(map.categories[0].groups[1].name, "C");
    assert_eq!(icons[0][1], "i-c");
    assert!(map.categories[0].groups[1].icon.is_none());
    let pins: Vec<_> =
        map.pins.iter().map(|pin| (pin.name.as_str(), pin.group, pin.image.as_ref().map(|image| image.preview.as_str()))).collect();
    assert_eq!(
        pins,
        [
            ("Vault", (0, 1), Some("https://api-cdn.wemod.com/game_map_pin/7/750.webp")),
            ("B", (2, 0), Some("https://storage-cdn.wemod.com/game_map_pin/9/750.webp")),
            ("A", (0, 0), Some("https://o/750.webp")),
        ]
    );
    assert_eq!((map.pins[0].video.as_deref(), map.pins[0].description.as_str()), (Some("https://v/x"), ""));
    assert!((map.pins[0].at.x - 0.5).abs() < 1e-12);
    assert_eq!(map.pins[0].image.as_ref().unwrap().full, "https://api-cdn.wemod.com/game_map_pin/7/1920.webp");
}

#[test]
fn regions_and_labels_take_their_defaults() {
    let map = parse(&response(xyz(), Value::Null)).unwrap().0;
    let [region] = &map.regions[..] else { panic!("one region") };
    assert_eq!((region.name.as_str(), region.color, region.thickness, region.zoom), ("Tower", 0x01579B, 2., (3., 24.)));
    assert_eq!(region.outlines.iter().map(Vec::len).collect::<Vec<_>>(), [3]);
    let [label] = &map.labels[..] else { panic!("one label") };
    assert_eq!((label.text.as_str(), label.color, label.size, label.zoom), ("Passage", GRAY, 14., (0., 15.)));
}

#[test]
fn ids_level_unfinished_and_icon_names() {
    let (map, icons) = parse(&response(xyz(), Value::Null)).unwrap();
    assert_eq!(icons, [vec!["i-a", "i-c"], vec![], vec!["i-b"]]);
    let ids: Vec<_> = map.categories.iter().flat_map(|c| c.groups.iter().map(|g| g.id.as_str())).collect();
    assert_eq!(ids, ["a", "c", "b"]);
    let pins: Vec<_> = map.pins.iter().map(|pin| (pin.id.as_str(), pin.level)).collect();
    assert_eq!(pins, [("p1", Some(Level::Below)), ("@0.5277778,0.4720801", None), ("@0.5555556,0.4432806", None)]);
    assert!(map.unfinished);
    let mut raw = response(xyz(), Value::Null);
    raw["metadata"]["isInActiveDevelopment"] = Value::Null;
    raw["metadata"]["subcategories"][0]["icon"] = Value::Null;
    let (map, icons) = parse(&raw).unwrap();
    assert!(!map.unfinished);
    assert_eq!(icons[0], ["", "i-c"]);
}

#[test]
fn links_to_pins_stay_inside_and_other_wand_links_lose_their_target() {
    let pins = HashSet::from(["COEX".to_owned()]);
    let text = "Go to [Glenwood's End](wemod://maps/75523/590?location=COEX), then [elsewhere](wemod://maps/1/2?location=gone) \
                or [the wiki](https://w.x/a) and [an app](wemod://titles/9).";
    assert_eq!(relink(text, &pins), "Go to [Glenwood's End](pin:COEX), then elsewhere or [the wiki](https://w.x/a) and an app.");
    assert_eq!(relink("[broken](wemod://x", &pins), "[broken](wemod://x");
    assert_eq!(relink("See [Gate [North]](wemod://maps/1/2?location=gone).", &pins), "See Gate [North].");
}

#[test]
fn a_region_goes_by_its_external_id_else_its_title() {
    let outline = json!({"type": "Polygon", "coordinates": [[[0, 0], [1, 0], [1, 1]]]});
    let raw = json!({
        "metadata": {"tileSource": xyz()},
        "regions": {"features": [
            {"geometry": outline, "properties": {"externalId": "r1", "title": "Tour"}},
            {"geometry": outline, "properties": {"title": "Tower"}}]}
    });
    let (map, _) = parse(&raw).unwrap();
    let ids: Vec<(&str, &str)> = map.regions.iter().map(|region| (region.id.as_str(), region.name.as_str())).collect();
    assert_eq!(ids, [("r1", "Tour"), ("Tower", "Tower")]);
}
