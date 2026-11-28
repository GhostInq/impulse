//! Wand's messages, one const per message of `locales/en.ftl`; `checks` holds the two together.

use model::Text;

const FILES: [(&str, &str); 13] = [
    ("en", include_str!("../locales/en.ftl")),
    ("zh-CN", include_str!("../locales/zh-CN.ftl")),
    ("zh-TW", include_str!("../locales/zh-TW.ftl")),
    ("fr", include_str!("../locales/fr.ftl")),
    ("de", include_str!("../locales/de.ftl")),
    ("es", include_str!("../locales/es.ftl")),
    ("it", include_str!("../locales/it.ftl")),
    ("pt-BR", include_str!("../locales/pt-BR.ftl")),
    ("pl", include_str!("../locales/pl.ftl")),
    ("tr", include_str!("../locales/tr.ftl")),
    ("ru", include_str!("../locales/ru.ftl")),
    ("uk", include_str!("../locales/uk.ftl")),
    ("th", include_str!("../locales/th.ftl")),
];

pub fn files() -> model::Files {
    FILES.iter().map(|&(tag, source)| (tag.into(), source.into())).collect()
}

pub struct Key(&'static str);

pub const STORE_MAPS: Key = Key("store-maps");
pub const STORE_MAPS_HINT: Key = Key("store-maps-hint");
pub const STORE_CATALOG: Key = Key("store-catalog");
pub const STORE_CATALOG_HINT: Key = Key("store-catalog-hint");
pub const STORE_TRAINERS: Key = Key("store-trainers");
pub const STORE_TRAINERS_HINT: Key = Key("store-trainers-hint");
pub const STORE_TRAINER_FILES: Key = Key("store-trainer-files");
pub const STORE_TRAINER_FILES_HINT: Key = Key("store-trainer-files-hint");
pub const STORE_BROWSER: Key = Key("store-browser");
pub const STORE_BROWSER_HINT: Key = Key("store-browser-hint");
pub const FOLDER: Key = Key("folder");
pub const FOLDER_HINT: Key = Key("folder-hint");
pub const FOLDER_EMPTY: Key = Key("folder-empty");
pub const FOLDER_UNSET: Key = Key("folder-unset");
pub const CATEGORY_CHALLENGE: Key = Key("category-challenge");
pub const CATEGORY_ENEMIES: Key = Key("category-enemies");
pub const CATEGORY_GAME: Key = Key("category-game");
pub const CATEGORY_INVENTORY: Key = Key("category-inventory");
pub const CATEGORY_PHYSICS: Key = Key("category-physics");
pub const CATEGORY_PLAYER: Key = Key("category-player");
pub const CATEGORY_STATS: Key = Key("category-stats");
pub const CATEGORY_TELEPORT: Key = Key("category-teleport");
pub const CATEGORY_VEHICLES: Key = Key("category-vehicles");
pub const CATEGORY_WEAPONS: Key = Key("category-weapons");
pub const SIGNED_OUT: Key = Key("signed-out");
pub const NOT_INSTALLED: Key = Key("not-installed");
pub const PROBE_FAILED: Key = Key("probe-failed");
pub const PROBE_OK: Key = Key("probe-ok");
pub const PROXY_MISSING: Key = Key("proxy-missing");
pub const PROXY_FAILED: Key = Key("proxy-failed");
pub const PROXY_RESTART: Key = Key("proxy-restart");
pub const SERVICE_SILENT: Key = Key("service-silent");
pub const ACTIVATE_UNSUPPORTED: Key = Key("activate-unsupported");
pub const TRAINER_FILE_MISSING: Key = Key("trainer-file-missing");

pub fn text(key: Key) -> Text {
    Text::key(&crate::source(), key.0)
}

/// A trainer category's name; `None` for an id Wand's catalog does not have.
pub fn category(id: &str) -> Option<Text> {
    let key = match id {
        "challenge" => CATEGORY_CHALLENGE,
        "enemies" => CATEGORY_ENEMIES,
        "game" => CATEGORY_GAME,
        "inventory" => CATEGORY_INVENTORY,
        "physics" => CATEGORY_PHYSICS,
        "player" => CATEGORY_PLAYER,
        "stats" => CATEGORY_STATS,
        "teleport" => CATEGORY_TELEPORT,
        "vehicles" => CATEGORY_VEHICLES,
        "weapons" => CATEGORY_WEAPONS,
        _ => return None,
    };
    Some(text(key))
}
