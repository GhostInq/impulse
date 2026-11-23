//! Where Wand keeps its binaries. The service and plugin are read from there, never copied or shipped; a trainer DLL
//! Wand downloaded is read when the CDN cannot give it (`binary`).

use std::{
    cmp::Reverse,
    env, fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

/// The signer of the service, the plugin and every trainer DLL.
/// WeMod signs as WeMod LLC; older trainers carry its earlier name.
pub const PUBLISHERS: &[&str] = &["WeMod LLC", "Daring Development Inc."];

const TRAINERLIB: &str = r"resources\app.asar.unpacked\static\unpacked\trainerlib";
const GAME_LAUNCHER: &str = r"resources\app.asar.unpacked\static\unpacked\auxiliary\GameLauncher.exe";
// Wand was WeMod: older installs live under the old name.
const BRANDS: [&str; 2] = ["Wand", "WeMod"];
// A folder picked inside an install (down to `trainerlib`) still means that install.
const DEPTH: usize = 6;

/// An install holding the binaries we need: an `app-*` folder of Wand's Squirrel root.
#[derive(Clone, Debug, PartialEq)]
pub struct Install {
    pub root: PathBuf,
    /// Empty for a folder that is not named `app-<version>`.
    version: Vec<u32>,
}

impl Install {
    pub fn trainerlib(&self) -> PathBuf {
        self.root.join(TRAINERLIB)
    }

    /// The stand-in for an anti-cheat's launcher (`proxy`).
    pub fn game_launcher(&self) -> PathBuf {
        self.root.join(GAME_LAUNCHER)
    }

    pub fn label(&self) -> String {
        match self.version.is_empty() {
            true => self.root.file_name().unwrap_or_default().to_string_lossy().into_owned(),
            false => self.version.iter().map(u32::to_string).collect::<Vec<_>>().join("."),
        }
    }
}

/// Every installed version of every brand, newest first.
pub fn installs() -> Vec<Install> {
    let Some(local) = app_data(Scope::Local) else {
        return Vec::new();
    };
    installs_in(&local)
}

/// The folders where Wand keeps the trainer DLLs it downloaded, one per brand.
pub fn trainer_caches() -> Vec<PathBuf> {
    let Some(roaming) = app_data(Scope::Roaming) else {
        return Vec::new();
    };
    BRANDS.iter().map(|brand| roaming.join(brand).join(r"App\trainers")).collect()
}

#[derive(Clone, Copy)]
pub enum Scope {
    /// `AppData\Local`: the installs.
    Local,
    /// `AppData\Roaming`: the profiles and the trainer downloads.
    Roaming,
}

/// Wand's per-user folder: this user's, or under Wine the user folder of the prefix Wand is installed in, which the
/// Linux launcher names in `IMPULSE_WAND_HOME` (`C:\users\<name>` of that prefix, as a Unix or a Windows path).
pub fn app_data(scope: Scope) -> Option<PathBuf> {
    if let Some(home) = env::var(HOME).ok().filter(|home| !home.is_empty()) {
        let home = wine::dos_path(&home).unwrap_or_else(|| home.into());
        return Some(home.join(match scope {
            Scope::Local => r"AppData\Local",
            Scope::Roaming => r"AppData\Roaming",
        }));
    }
    env::var_os(match scope {
        Scope::Local => "LOCALAPPDATA",
        Scope::Roaming => "APPDATA",
    })
    .map(PathBuf::from)
}

const HOME: &str = "IMPULSE_WAND_HOME";

/// Wand's service and plugin in `trainerlib` for a trainer of that architecture.
pub fn binaries(trainerlib: &Path, x64: bool) -> [PathBuf; 2] {
    let arch = if x64 { "x64" } else { "x86" };
    [trainerlib.join(format!("tophat_service_{arch}.dll")), trainerlib.join(format!("TrainerLibPlugin_{arch}.dll"))]
}

fn installs_in(local: &Path) -> Vec<Install> {
    let mut all: Vec<Install> = BRANDS.iter().flat_map(|brand| versions(&local.join(brand))).collect();
    newest_first(&mut all);
    all
}

/// What a typed or picked folder means: an install, a folder inside one, or the Squirrel root
/// holding installs (its newest).
pub fn resolve(path: &Path) -> Option<Install> {
    // A relative path would be looked up from the working directory.
    if !path.is_absolute() {
        return None;
    }
    path.ancestors().take(DEPTH).find_map(|dir| complete(dir, version_of(dir))).or_else(|| versions(path).into_iter().next())
}

fn versions(squirrel_root: &Path) -> Vec<Install> {
    let Ok(entries) = fs::read_dir(squirrel_root) else {
        return Vec::new();
    };
    let mut found: Vec<Install> = entries
        .flatten()
        .filter_map(|entry| {
            let version = version_of(&entry.path());
            complete(&entry.path(), version).filter(|install| !install.version.is_empty())
        })
        .collect();
    newest_first(&mut found);
    found
}

fn complete(dir: &Path, version: Vec<u32>) -> Option<Install> {
    let install = Install { root: dir.to_owned(), version };
    binaries(&install.trainerlib(), true).iter().all(|file| file.is_file()).then_some(install)
}

/// An update can leave two folders of one version; the later-written one is the live one.
fn newest_first(installs: &mut [Install]) {
    installs.sort_by_cached_key(|install| {
        let written = fs::metadata(&install.root).and_then(|meta| meta.modified()).unwrap_or(SystemTime::UNIX_EPOCH);
        (Reverse(install.version.clone()), Reverse(written))
    });
}

/// `app-12.72.0` as numbers, so 12.100 beats 12.72 and 12.9. Anything after the number is ignored.
fn version_of(dir: &Path) -> Vec<u32> {
    let name = dir.file_name().unwrap_or_default().to_string_lossy();
    let Some(rest) = name.strip_prefix("app-") else {
        return Vec::new();
    };
    let numbers = rest.split(|c: char| !c.is_ascii_digit() && c != '.').next().unwrap_or_default();
    numbers.trim_end_matches('.').split('.').map(str::parse::<u32>).collect::<Result<_, _>>().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> PathBuf {
        let root = env::temp_dir().join(format!("impulse-wand-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        root
    }

    #[test]
    fn wands_trainer_downloads_are_under_each_brands_roaming_folder() {
        let caches = trainer_caches();
        assert_eq!(caches.len(), BRANDS.len());
        assert!(caches.iter().zip(BRANDS).all(|(dir, brand)| dir.ends_with(Path::new(brand).join(r"App\trainers"))));
    }

    fn install(root: &Path, folder: &str, complete: bool) -> PathBuf {
        let app = root.join(folder);
        let libs = app.join(TRAINERLIB);
        fs::create_dir_all(&libs).unwrap();
        let [service, plugin] = binaries(&libs, true);
        fs::write(service, b"").unwrap();
        if complete {
            fs::write(plugin, b"").unwrap();
        }
        app
    }

    #[test]
    fn versions_compare_as_numbers_across_brands() {
        let local = fixture("brands");
        install(&local.join("Wand"), "app-12.9.0", true);
        install(&local.join("Wand"), "app-12.72.0", true);
        let old = install(&local.join("WeMod"), "app-12.100.0", true);
        install(&local.join("Wand"), "app-13.0.0", false);
        install(&local.join("Wand"), "app-dev", true);
        fs::create_dir_all(local.join("Wand").join("packages")).unwrap();

        let labels: Vec<_> = installs_in(&local).iter().map(Install::label).collect();
        assert_eq!(labels, ["12.100.0", "12.72.0", "12.9.0"]);
        assert_eq!(installs_in(&local)[0].root, old);
        assert!(installs_in(&local.join("missing")).is_empty());
        fs::remove_dir_all(&local).unwrap();
    }

    #[test]
    fn a_picked_folder_resolves_from_an_install_a_folder_inside_or_the_root() {
        let local = fixture("resolve");
        let squirrel = local.join("Wand");
        install(&squirrel, "app-12.72.0", true);
        let newest = install(&squirrel, "app-12.100.0-beta", true);

        for picked in [&newest, &squirrel, &newest.join(TRAINERLIB), &newest.join("resources")] {
            let found = resolve(picked).unwrap_or_else(|| panic!("{} resolves", picked.display()));
            assert_eq!(found.root, newest, "{}", picked.display());
        }
        assert_eq!(resolve(&squirrel.join("app-12.72.0")).unwrap().label(), "12.72.0");
        assert!(resolve(&local).is_none());
        assert!(resolve(Path::new(r"relative\app-1.0")).is_none());
        assert!(resolve(Path::new("")).is_none());
        fs::remove_dir_all(&local).unwrap();
    }

    #[test]
    fn a_hand_placed_install_is_named_by_its_folder() {
        let local = fixture("portable");
        let portable = install(&local, "Wand portable", true);

        let found = resolve(&portable).unwrap();
        assert_eq!(found.label(), "Wand portable");
        assert!(resolve(&local).is_none(), "its parent is neither an install nor a Squirrel root");
        fs::remove_dir_all(&local).unwrap();
    }
}
