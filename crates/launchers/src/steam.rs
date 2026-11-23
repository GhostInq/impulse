use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
};

use super::{
    App,
    registry::{CURRENT_USER, LOCAL_MACHINE, View, value},
    same_path,
    vdf::{self, Node},
};

const FULLY_INSTALLED: u32 = 4;

pub fn apps() -> Vec<App> {
    // Under Proton the prefix's Steam is a stand-in without records: Steam's own folder is on the Linux side.
    let roots: Vec<PathBuf> = install_dir().into_iter().chain(wine::steam_root()).collect();
    let mut seen = HashSet::new();
    roots.iter().flat_map(|root| libraries(root)).flat_map(|lib| manifests(&lib)).filter(|app| seen.insert(app.store_id.clone())).collect()
}

fn install_dir() -> Option<PathBuf> {
    let program_files = env::var_os("ProgramFiles(x86)").map(|p| Path::new(&p).join("Steam"));
    [
        value(LOCAL_MACHINE, r"SOFTWARE\Valve\Steam", "InstallPath", View::Wow32).map(PathBuf::from),
        value(CURRENT_USER, r"Software\Valve\Steam", "SteamPath", View::Wow64).map(PathBuf::from),
        program_files,
    ]
    .into_iter()
    .flatten()
    .find(|p| p.is_dir())
}

fn libraries(root: &Path) -> Vec<PathBuf> {
    let mut libs = vec![root.to_path_buf()];
    let text = fs::read_to_string(root.join(r"steamapps\libraryfolders.vdf")).unwrap_or_default();
    let Some(file) = vdf::parse(&text) else {
        return libs;
    };
    let Some(folders) = file.get("libraryfolders") else {
        return libs;
    };
    for (key, folder) in folders.entries() {
        // Numbered entries only; old files also hold stats keys. Old entries are bare paths.
        if !key.bytes().all(|b| b.is_ascii_digit()) {
            continue;
        }
        let path = match folder {
            Node::Str(path) => Some(path.as_str()),
            Node::Map(_) => folder.str("path"),
        };
        // Steam on Linux records Unix paths; Wine names them on one of its drives.
        if let Some(path) = path.map(|path| wine::dos_path(path).unwrap_or_else(|| PathBuf::from(path)))
            && path.is_dir()
            && !libs.iter().any(|lib| same_path(lib, &path))
        {
            libs.push(path);
        }
    }
    libs
}

fn manifests(lib: &Path) -> Vec<App> {
    let apps = lib.join("steamapps");
    let Ok(dir) = fs::read_dir(&apps) else {
        return Vec::new();
    };
    dir.flatten()
        .filter(|entry| {
            let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
            name.starts_with("appmanifest_") && name.ends_with(".acf")
        })
        .filter_map(|entry| manifest(&apps, &entry.path()))
        .collect()
}

fn manifest(apps: &Path, file: &Path) -> Option<App> {
    let root = vdf::parse(&fs::read_to_string(file).ok()?)?;
    let state = root.get("AppState")?;
    let flags: u32 = state.str("StateFlags")?.parse().ok()?;
    if flags & FULLY_INSTALLED == 0 {
        return None;
    }
    let id: u32 = state.str("appid")?.parse().ok()?;
    Some(App {
        store_id: format!("steam:{id}"),
        // `installdir` is usually relative to steamapps\common; join keeps an absolute one.
        location: apps.join("common").join(state.str("installdir")?),
    })
}
