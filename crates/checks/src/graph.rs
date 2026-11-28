//! Which crates of ours each crate uses. An edge added or removed edits `GRAPH` in the same change.

use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    process::Command,
    sync::OnceLock,
};

use serde_json::Value;

use crate::{root, verdict};

/// Each package and the packages of ours it names, in any dependency section.
const GRAPH: &[(&str, &[&str])] = &[
    ("agent", &["model", "pe"]),
    ("capture", &[]),
    ("checks", &[]),
    ("client", &["agent", "disk", "fetch", "launchers", "model"]),
    ("disk", &[]),
    ("fetch", &[]),
    ("i18n", &[]),
    ("impulse", &["agent", "client", "journal", "model", "plugins", "remote", "shell", "tray", "updater", "wand"]),
    ("journal", &[]),
    ("keyhook", &["model"]),
    ("launchers", &["model", "pe"]),
    ("model", &[]),
    ("pe", &[]),
    ("plugins", &["disk", "fetch", "model", "sdk"]),
    ("remote", &["client"]),
    ("sdk", &[]),
    ("shell", &["capture", "client", "disk", "i18n", "journal", "keyhook", "model", "sound", "topmost", "tray", "updater", "video"]),
    ("sound", &[]),
    ("topmost", &[]),
    ("tray", &[]),
    ("updater", &["disk", "fetch"]),
    ("video", &[]),
    ("wand", &["agent", "disk", "fetch", "model"]),
];

/// The packages that may use GPUI: the UI and the app that opens its window.
const GPUI: &[&str] = &["impulse", "shell"];

#[derive(Default)]
struct Dependencies {
    /// By path: the workspace's own crates.
    local: BTreeSet<String>,
    all: Vec<String>,
}

/// Each workspace package's dependencies, from one `cargo metadata` for every test.
fn packages() -> &'static BTreeMap<String, Dependencies> {
    static PACKAGES: OnceLock<BTreeMap<String, Dependencies>> = OnceLock::new();
    PACKAGES.get_or_init(|| {
        let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".into());
        let output =
            Command::new(cargo).args(["metadata", "--no-deps", "--offline", "--format-version", "1"]).current_dir(root()).output().unwrap();
        assert!(output.status.success(), "cargo metadata: {}", String::from_utf8_lossy(&output.stderr));
        let metadata: Value = serde_json::from_slice(&output.stdout).unwrap();
        let name = |value: &Value| value["name"].as_str().unwrap().to_owned();
        let mut packages = BTreeMap::new();
        for package in metadata["packages"].as_array().unwrap() {
            let entry: &mut Dependencies = packages.entry(name(package)).or_default();
            for dependency in package["dependencies"].as_array().unwrap() {
                if dependency["path"].is_string() {
                    entry.local.insert(name(dependency));
                }
                entry.all.push(name(dependency));
            }
        }
        packages
    })
}

#[test]
fn the_crate_graph_is_the_table() {
    let packages = packages();
    let table: BTreeMap<&str, BTreeSet<&str>> = GRAPH.iter().map(|(name, uses)| (*name, uses.iter().copied().collect())).collect();
    let mut complaints = Vec::new();
    for (package, dependencies) in packages {
        let uses: BTreeSet<&str> =
            dependencies.local.iter().map(String::as_str).filter(|dependency| packages.contains_key(*dependency)).collect();
        let Some(allowed) = table.get(package.as_str()) else {
            complaints.push(format!("{package} is not in GRAPH: add its row"));
            continue;
        };
        for added in uses.difference(allowed) {
            complaints.push(format!("{package} -> {added} is not in GRAPH: review the dependency before adding the edge"));
        }
        for gone in allowed.difference(&uses) {
            complaints.push(format!("{package} no longer uses {gone}: remove the edge from GRAPH"));
        }
    }
    complaints.extend(
        table.keys().filter(|name| !packages.contains_key(**name)).map(|name| format!("{name} is in GRAPH but not in the workspace")),
    );
    verdict(complaints);
}

#[test]
fn gpui_stays_in_the_ui() {
    let complaints = packages()
        .iter()
        .filter(|(package, dependencies)| !GPUI.contains(&package.as_str()) && dependencies.all.iter().any(|name| name.starts_with("gpui")))
        .map(|(package, _)| format!("{package} uses GPUI: only {GPUI:?} may"))
        .collect();
    verdict(complaints);
}
