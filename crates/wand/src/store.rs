use std::{
    collections::{BTreeMap, HashMap},
    env, fmt, fs,
    panic::{self, AssertUnwindSafe},
    path::{Path, PathBuf},
    process,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, anyhow, bail};
use model::{Chord, Failure, FailureKind};
use rusty_leveldb::{DB, Options};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    install::{self, Scope},
    texts::{NOT_INSTALLED, SIGNED_OUT, text},
};

// Chromium layout: `_<origin>\0` + script key; both keys and values carry an encoding byte.
const KEY: &[u8] = b"_file://\x00\x01infinity:globalStore";
// Wand 12.x keeps its Electron profile in `wemod`; `Wand` is a stale profile from an older build.
const PROFILES: [&str; 2] = ["wemod", "Wand"];

pub struct WandToken {
    access_token: String,
}

impl WandToken {
    pub(crate) fn expose(&self) -> &str {
        &self.access_token
    }
}

impl fmt::Debug for WandToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WandToken(<redacted>)")
    }
}

/// A cheat's hotkey slots the user changed in Wand: VKs, or `None` for unbound.
pub type Slots = BTreeMap<usize, Option<Vec<u16>>>;
/// By game id, then cheat id.
pub type Custom = HashMap<String, HashMap<String, Slots>>;

/// Reads Wand's access token from a copy of its localStorage leveldb (blocking).
pub fn read() -> Result<WandToken> {
    let found: Vec<_> = stores()?.iter().filter_map(token).collect();
    if found.is_empty() && !profiles().iter().any(|dir| dir.is_dir()) {
        let missing = Failure::new(FailureKind::SignedOut).saying(text(NOT_INSTALLED));
        return Err(anyhow!("Wand has no profile on this PC").context(missing));
    }
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs_f64();
    newest(found, now)
}

/// The hotkeys the user changed in Wand, from the profile with the newest session (blocking).
pub fn hotkeys() -> Result<Custom> {
    let stores = stores()?;
    let expiry = |store: &Value| store["token"]["expiresAt"].as_f64().unwrap_or(f64::MIN);
    stores.iter().max_by(|a, b| expiry(a).total_cmp(&expiry(b))).map_or(Ok(Custom::new()), custom)
}

/// Lays the user's Wand hotkeys over the trainer's.
pub fn customize(cheats: &mut [model::Cheat], custom: &HashMap<String, Slots>) {
    for cheat in cheats {
        let slots = cheat.kind.slots();
        for (&slot, vks) in custom.get(&cheat.id).into_iter().flatten().filter(|(slot, _)| **slot < slots) {
            cheat.hotkeys.resize(cheat.hotkeys.len().max(slots), None);
            cheat.hotkeys[slot] = vks.as_deref().and_then(Chord::from_vks);
        }
    }
}

fn profiles() -> Vec<PathBuf> {
    let root = install::app_data(Scope::Roaming).unwrap_or_default();
    PROFILES.iter().map(|profile| root.join(profile).join(r"Local Storage\leveldb")).collect()
}

/// Each profile's store; a profile's error only when none could be read.
fn stores() -> Result<Vec<Value>> {
    install::app_data(Scope::Roaming).context("APPDATA is not set")?;
    let mut found = Vec::new();
    let mut failed = None;
    for dir in profiles() {
        match read_profile(&dir) {
            Ok(Some(store)) => found.push(store),
            Ok(None) => {}
            Err(e) => failed = Some(e),
        }
    }
    match failed {
        Some(e) if found.is_empty() => Err(e),
        _ => Ok(found),
    }
}

fn signed_out() -> Failure {
    Failure::new(FailureKind::SignedOut).saying(text(SIGNED_OUT))
}

fn newest(found: Vec<(f64, String)>, now: f64) -> Result<WandToken> {
    let (expires_at, access_token) =
        found.into_iter().max_by(|a, b| a.0.total_cmp(&b.0)).context("Wand session not found").context(signed_out())?;
    if expires_at <= now {
        return Err(anyhow!("Wand session expired").context(signed_out()));
    }
    Ok(WandToken { access_token })
}

fn read_profile(src: &Path) -> Result<Option<Value>> {
    if !src.is_dir() {
        return Ok(None);
    }
    // One folder per read: the sign-in and the hotkeys may read at once.
    static READS: AtomicU64 = AtomicU64::new(0);
    let read = READS.fetch_add(1, Ordering::Relaxed);
    let dst = env::temp_dir().join(format!("impulse-wand-{}-{read}", process::id()));
    let _ = fs::remove_dir_all(&dst);
    let result = copy_dir(src, &dst).and_then(|()| read_db(&dst));
    let _ = fs::remove_dir_all(&dst);
    result
}

// The live dir is never opened: Wand holds LOCK, and opening would write to it.
fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst).context("create temp dir")?;
    for entry in fs::read_dir(src).context("list Wand store")? {
        let name = entry.context("list Wand store")?.file_name();
        // Files can vanish mid-copy (leveldb compaction) or be locked; the probe skipped them too.
        if name != "LOCK" {
            let _ = fs::copy(src.join(&name), dst.join(&name));
        }
    }
    Ok(())
}

fn read_db(dir: &Path) -> Result<Option<Value>> {
    let opt = Options { create_if_missing: false, ..Options::default() };
    // The decoder can panic on a damaged store; that must not end the app.
    panic::catch_unwind(AssertUnwindSafe(|| {
        let mut db = DB::open(dir, opt).context("open Wand store copy")?;
        db.get(KEY).map(|raw| parse(&raw)).transpose()
    }))
    .unwrap_or_else(|_| Err(anyhow!("Wand store is damaged")))
}

fn parse(raw: &[u8]) -> Result<Value> {
    serde_json::from_str(&decode(raw)?).map_err(|_| anyhow!("Wand store is not valid JSON"))
}

fn token(store: &Value) -> Option<(f64, String)> {
    let token = &store["token"];
    token["expiresAt"].as_f64().zip(token["accessToken"].as_str()).map(|(at, t)| (at, t.to_owned()))
}

#[derive(Deserialize)]
struct Preferences {
    #[serde(default, rename = "customHotkeys")]
    custom_hotkeys: HashMap<String, Slots>,
}

fn custom(store: &Value) -> Result<Custom> {
    let Some(games) = store.get("gamePreferences").filter(|games| !games.is_null()) else {
        return Ok(Custom::new());
    };
    let games: HashMap<String, Preferences> =
        serde_json::from_value(games.clone()).context("Wand's game preferences have an unknown shape")?;
    Ok(games
        .into_iter()
        .filter(|(_, preferences)| !preferences.custom_hotkeys.is_empty())
        .map(|(game, preferences)| (game, preferences.custom_hotkeys))
        .collect())
}

// First byte: 0 = UTF-16LE, 1 = Latin-1.
fn decode(raw: &[u8]) -> Result<String> {
    match raw.split_first() {
        Some((0, rest)) => {
            let units: Vec<u16> = rest.as_chunks::<2>().0.iter().map(|&c| u16::from_le_bytes(c)).collect();
            Ok(String::from_utf16_lossy(&units))
        }
        Some((1, rest)) => Ok(rest.iter().map(|&b| b as char).collect()),
        _ => bail!("unknown Wand store encoding"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const JSON: &str = r#"{"token":{"accessToken":"tok-1","expiresAt":2000.5}}"#;

    fn utf16(s: &str) -> Vec<u8> {
        std::iter::once(0).chain(s.encode_utf16().flat_map(u16::to_le_bytes)).collect()
    }

    fn latin1(s: &str) -> Vec<u8> {
        std::iter::once(1).chain(s.bytes()).collect()
    }

    #[test]
    fn parses_both_encodings() {
        for raw in [utf16(JSON), latin1(JSON)] {
            assert_eq!(token(&parse(&raw).unwrap()), Some((2000.5, "tok-1".to_owned())));
        }
        assert!(decode(&[7, 1]).is_err());
        assert_eq!(token(&parse(&latin1(r#"{"token":{}}"#)).unwrap()), None);
    }

    #[test]
    fn custom_hotkeys_lay_over_the_trainer_and_null_unbinds() {
        let store: Value =
            serde_json::from_str(r#"{"gamePreferences":{"g":{"customHotkeys":{"a":{"1":[17,113],"0":null,"7":[65]}}},"h":{}}}"#).unwrap();
        let custom = custom(&store).unwrap();
        assert_eq!(custom.keys().collect::<Vec<_>>(), ["g"]);
        let mut cheats = vec![model::Cheat {
            id: "a".into(),
            key: None,
            name: String::new(),
            category: String::new(),
            category_name: None,
            kind: model::Kind::Number,
            args: model::Args::default(),
            hotkeys: vec![Chord::from_vks(&[112])],
            instructions: None,
            description: None,
        }];
        customize(&mut cheats, &custom["g"]);
        assert_eq!(cheats[0].hotkeys, [None, Chord::from_vks(&[17, 113])]);
    }

    #[test]
    fn picks_newest_expiry() {
        let found = vec![(100.0, "old".to_owned()), (300.0, "new".to_owned()), (200.0, "mid".to_owned())];
        assert_eq!(newest(found, 50.0).unwrap().expose(), "new");
    }

    #[test]
    fn rejects_expired_and_missing() {
        let expired = Failure::from(newest(vec![(100.0, "old".to_owned())], 100.0).unwrap_err());
        assert_eq!((expired.kind, expired.detail.as_str()), (FailureKind::SignedOut, "SignedOut: Wand session expired"));
        let missing = Failure::from(newest(vec![], 0.0).unwrap_err());
        assert_eq!((missing.kind, missing.detail.as_str()), (FailureKind::SignedOut, "SignedOut: Wand session not found"));
    }

    #[test]
    fn debug_is_redacted() {
        let token = WandToken { access_token: "secret-value".to_owned() };
        assert!(!format!("{token:?}").contains("secret-value"));
    }
}
