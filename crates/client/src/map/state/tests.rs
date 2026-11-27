use std::{fs, path::PathBuf};

use model::{Category, Group, Text, Tiles};
use serde_json::json;

use super::*;

fn data(name: &str) -> (PathBuf, Paths) {
    let dir = std::env::temp_dir().join(format!("impulse-map-state-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    (dir.clone(), Paths::new(dir))
}

fn map() -> Map {
    let tiles = Tiles { template: String::new(), flipped: false, size: 256, min_zoom: 0, max_zoom: 0, bounds: None };
    let group = Group { id: "g".to_owned(), name: String::new(), icon: None };
    Map {
        tile_sets: vec![TileSet { id: String::new(), name: Text::Plain(String::new()), tiles }],
        background: 0,
        categories: vec![Category { name: String::new(), color: 0, shown: false, groups: vec![group] }],
        pins: Vec::new(),
        regions: Vec::new(),
        labels: Vec::new(),
        frame: None,
        teleport: false,
        unfinished: false,
    }
}

fn id(text: &str) -> Ref {
    Ref::parse(&format!("s:{text}")).unwrap()
}

fn older(paths: &Paths, file: &str, value: serde_json::Value) {
    let file = paths.map_state().join("s").join(file);
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(file, value.to_string()).unwrap();
}

#[test]
fn a_saved_region_title_hides_every_region_of_that_name() {
    let region = |id: &str, name: &str| Region {
        id: id.to_owned(),
        name: name.to_owned(),
        outlines: Vec::new(),
        color: 0,
        thickness: 1.,
        zoom: (0., 20.),
    };
    let regions = [region("a", "Camp"), region("b", "Camp"), region("c", "Keep")];
    let hidden = |ids: &[&str]| ids.iter().map(|id| (*id).to_owned()).collect::<HashSet<String>>();
    // An older build's file: titles under `hidden_regions`.
    let mut view: View = serde_json::from_str(r#"{"hidden_regions": ["Camp", "Gone", "Keep"], "hidden_region_ids": ["Camp"]}"#).unwrap();
    view.resolve(&regions);
    assert_eq!((view.hidden_regions, view.hidden_labels), (hidden(&["a", "b", "c", "Camp"]), hidden(&["Gone"])));
}

#[test]
fn state_survives_a_round_trip_and_a_delete_leaves_a_tombstone() {
    let (dir, paths) = data("trip");
    let mut saved = Saved::load(&paths, &id("../aB"), &map());
    assert!(saved.view().hidden_groups.contains("g"), "a new map hides what its source hides");
    saved.toggle_found("p1");
    saved.toggle_found("p2");
    saved.toggle_found("p2");
    saved.view_mut().camera = Some((0.5, 0.25, 4096.));
    let kept = saved.add_pin(Point { x: 0.5, y: 0.5 }, Some([1., 2., 3.]));
    let gone = saved.add_pin(Point { x: 0.1, y: 0.1 }, None);
    saved.edit_pin(&kept, PinEdit::Name("Camp".to_owned()));
    saved.delete_pin(&gone);
    saved.flush();

    assert!(Saved::load(&paths, &id("../ab"), &map()).pins().next().is_none());
    let back = Saved::load(&paths, &id("../aB"), &map());
    assert_eq!((back.found("p1"), back.found("p2"), back.view().camera), (true, false, Some((0.5, 0.25, 4096.))));
    let pins: Vec<_> = back.pins().map(|pin| (pin.name.as_str(), pin.position)).collect();
    assert_eq!(pins, [("Camp", Some([1., 2., 3.]))]);
    assert!(back.mine.pins.iter().any(|pin| pin.id == gone && pin.deleted && pin.updated > pin.created));
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn an_older_file_is_taken_over_once_with_the_same_ids_anywhere() {
    let mark = json!({"at": [0.5, 0.5], "position": [1., 2., 3.], "name": "Camp", "note": "fire", "color": 7});
    let old = json!({"found": ["x"], "hidden": [], "view": [0.5, 0.25, 64.], "marks": [mark, mark], "marks_shown": false});
    let (dir, paths) = data("older");
    older(&paths, "old-map.json", old.clone());
    let saved = Saved::load(&paths, &id("old-map"), &map());
    assert!(saved.found("x") && !saved.view().pins_shown() && saved.view().hidden_groups.is_empty());
    assert_eq!(saved.view().camera, Some((0.5, 0.25, 64.)));
    let pins: Vec<_> = saved.pins().map(|pin| (pin.id.clone(), pin.description.as_str(), pin.created)).collect();
    assert_eq!((pins[0].1, pins[0].2, pins[1].2), ("fire", 0, 1));
    assert_ne!(pins[0].0, pins[1].0);
    // The older file stays as it was, and the folder wins from now on.
    let file = paths.map_state().join("s").join("old-map.json");
    assert_eq!(fs::read_to_string(&file).unwrap(), old.to_string());
    older(&paths, "old-map.json", json!({"found": ["y"]}));
    assert!(Saved::load(&paths, &id("old-map"), &map()).found("x"));

    let (elsewhere, other) = data("elsewhere");
    older(&other, "old-map.json", old);
    let again: Vec<_> = Saved::load(&other, &id("old-map"), &map()).pins().map(|pin| pin.id.clone()).collect();
    assert_eq!(again, [pins[0].0.clone(), pins[1].0.clone()]);
    fs::remove_dir_all(dir).unwrap();
    fs::remove_dir_all(elsewhere).unwrap();
}

#[test]
fn the_older_file_wins_over_the_v1_one_unless_it_is_no_map_state() {
    let (dir, paths) = data("v1");
    for (name, v2) in [("oldmap", json!({"found": ["v2"]})), ("badmap", json!({"found": 7}))] {
        older(&paths, &format!("{name}.json"), v2);
        older(&paths, &format!("v1/{name}.json"), json!({"found": ["v1"]}));
    }
    assert!(Saved::load(&paths, &id("oldmap"), &map()).found("v2"));
    assert!(Saved::load(&paths, &id("badmap"), &map()).found("v1"));
    // v1 kept letters and digits only: the file may be another map's.
    older(&paths, "v1/OldMap.json", json!({"found": ["v"]}));
    assert!(Saved::load(&paths, &id("OldMap"), &map()).found("v"));
    assert!(!Saved::load(&paths, &id("Old-Map"), &map()).found("v"));
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn an_older_file_not_taken_over_shows_from_memory_and_nothing_is_written() {
    let (dir, paths) = data("blocked");
    older(&paths, "m.json", json!({"found": ["x"]}));
    // A file where the staged folder goes.
    let staged = paths.map_files(&id("m")).staged();
    fs::create_dir_all(staged.dir().parent().unwrap()).unwrap();
    fs::write(staged.dir(), "").unwrap();
    let mut saved = Saved::load(&paths, &id("m"), &map());
    assert!(saved.found("x"));
    fs::remove_file(staged.dir()).unwrap();
    saved.toggle_found("y");
    saved.flush();
    assert!(!paths.map_files(&id("m")).dir().exists());
    let again = Saved::load(&paths, &id("m"), &map());
    assert!(again.found("x") && !again.found("y"));

    // One that is there but cannot be read: not taken for missing.
    fs::create_dir_all(paths.map_state().join("s").join("locked.json")).unwrap();
    let mut locked = Saved::load(&paths, &id("locked"), &map());
    assert!(locked.view().hidden_groups.contains("g"));
    locked.toggle_found("y");
    locked.flush();
    assert!(!paths.map_files(&id("locked")).dir().exists());
    fs::remove_dir_all(dir).unwrap();
}
