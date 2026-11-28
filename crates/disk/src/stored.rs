//! A typed JSON file the app keeps: `client`'s and `plugins`' files alike.

use std::{
    fs, io,
    ops::Deref,
    path::{Path, PathBuf},
};

use anyhow::{Result, ensure};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

/// What `Stored` writes into an object; an object without it is not this build's.
pub const VERSION: u64 = 2;

/// A typed JSON file, read once and written whole on every edit. Missing means the defaults; not JSON is
/// moved to `<name>.damaged.json`, then the defaults. A file this build cannot read gives the defaults and is
/// never written: an older one waits for `migrate`, a newer one for its build. The default has no file: memory only.
#[derive(Clone, Default)]
pub struct Stored<T> {
    file: PathBuf,
    value: T,
    saves: bool,
    // Changed through `get_mut` since the last save.
    dirty: bool,
}

impl<T: Serialize + DeserializeOwned + Default> Stored<T> {
    pub fn load(file: PathBuf) -> Self {
        let (value, saves) = match fs::read(&file) {
            Ok(bytes) => match serde_json::from_slice::<Value>(&bytes) {
                Ok(json) => match current(json).and_then(|json| serde_json::from_value(json).ok()) {
                    Some(value) => (value, true),
                    None => {
                        log::warn!("{} is not this version's: kept as it is, changes are not saved", file.display());
                        (T::default(), false)
                    }
                },
                Err(_) => {
                    let aside = file.with_extension("damaged.json");
                    if let Err(e) = fs::rename(&file, &aside) {
                        log::warn!("{} not moved aside: {e}", file.display());
                    }
                    (T::default(), true)
                }
            },
            Err(e) if e.kind() == io::ErrorKind::NotFound => (T::default(), true),
            Err(e) => {
                log::warn!("{} not read: {e}", file.display());
                (T::default(), false)
            }
        };
        Self { file, value, saves, dirty: false }
    }

    /// Changes the value and saves it. A failed save is logged and the change holds until the app closes.
    pub fn edit<R>(&mut self, change: impl FnOnce(&mut T) -> R) -> R {
        let changed = change(self.get_mut());
        self.flush();
        changed
    }

    /// The value to change now and save at the next `flush`.
    pub fn get_mut(&mut self) -> &mut T {
        self.dirty = true;
        &mut self.value
    }

    /// Saves what changed since the last save, if anything; a failure is logged and the next flush tries again.
    pub fn flush(&mut self) {
        if !self.dirty || !self.saves {
            return;
        }
        match self.save(&self.value) {
            Ok(()) => self.dirty = false,
            Err(e) => log::warn!("{} not saved: {e:#}", self.file.display()),
        }
    }

    /// Changes a copy and saves it; the value changes only when the save worked. A file this build does not save is an error.
    pub fn try_edit<R>(&mut self, change: impl FnOnce(&mut T) -> R) -> Result<R>
    where
        T: Clone,
    {
        ensure!(self.saves, "{} is not this version's: changes are not saved", self.file.display());
        let mut copy = self.value.clone();
        let changed = change(&mut copy);
        self.save(&copy)?;
        self.value = copy;
        Ok(changed)
    }

    fn save(&self, value: &T) -> Result<()> {
        let mut json = serde_json::to_value(value)?;
        if let Value::Object(object) = &mut json {
            object.insert("version".into(), VERSION.into());
        }
        keep(&self.file, &json)
    }
}

/// A list has no version; an object must carry this one.
fn current(json: Value) -> Option<Value> {
    match &json {
        Value::Object(object) if object.get("version").and_then(Value::as_u64) != Some(VERSION) => None,
        _ => Some(json),
    }
}

impl<T> Deref for Stored<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

/// `None` for a missing or damaged file.
pub fn read<T: DeserializeOwned>(file: &Path) -> Option<T> {
    serde_json::from_slice(&fs::read(file).ok()?).ok()
}

fn keep<T: Serialize>(file: &Path, value: &T) -> Result<()> {
    crate::replace(file, &serde_json::to_vec_pretty(value)?)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Clone, Default, Serialize, Deserialize)]
    #[serde(default)]
    struct Note {
        text: String,
    }

    fn dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("impulse-stored-{name}-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn edited(file: &Path) -> String {
        Stored::<Note>::load(file.to_owned()).edit(|note| note.text = "new".into());
        fs::read_to_string(file).unwrap()
    }

    #[test]
    fn an_object_carries_the_version_and_a_list_none() {
        let dir = dir("version");
        let file = dir.join("note.json");
        assert!(edited(&file).contains("\"version\": 2"));
        assert_eq!(Stored::<Note>::load(file).text, "new");
        let list = dir.join("list.json");
        Stored::<Vec<u32>>::load(list.clone()).edit(|list| list.push(1));
        assert_eq!(Stored::<Vec<u32>>::load(list).as_slice(), [1]);
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn try_edit_publishes_what_it_saved_and_a_refused_store_keeps_its_value() {
        let dir = dir("try");
        let file = dir.join("note.json");
        let mut note = Stored::<Note>::load(file.clone());
        assert_eq!(note.try_edit(|note| std::mem::replace(&mut note.text, "new".into())).unwrap(), "");
        assert_eq!(note.text, "new");
        assert_eq!(Stored::<Note>::load(file).text, "new");
        let later = dir.join("later.json");
        fs::write(&later, r#"{"version":3,"text":"later"}"#).unwrap();
        let mut refused = Stored::<Note>::load(later.clone());
        assert!(refused.try_edit(|note| note.text = "new".into()).is_err());
        assert_eq!(refused.text, "");
        assert_eq!(fs::read_to_string(later).unwrap(), r#"{"version":3,"text":"later"}"#);
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_change_waits_for_flush_and_a_store_without_a_file_writes_nothing() {
        let dir = dir("flush");
        let file = dir.join("note.json");
        let mut note = Stored::<Note>::load(file.clone());
        note.get_mut().text = "new".into();
        assert!(!file.exists());
        note.flush();
        assert_eq!(Stored::<Note>::load(file.clone()).text, "new");
        fs::remove_file(&file).unwrap();
        note.flush();
        assert!(!file.exists());
        // A failed save is tried again.
        let blocked = dir.join("blocked");
        let mut note = Stored::<Note>::load(blocked.join("note.json"));
        // A file where its folder goes.
        fs::write(&blocked, "").unwrap();
        note.get_mut().text = "new".into();
        note.flush();
        fs::remove_file(&blocked).unwrap();
        note.flush();
        assert_eq!(Stored::<Note>::load(blocked.join("note.json")).text, "new");
        let mut memory = Stored::<Note>::default();
        memory.edit(|note| note.text = "new".into());
        assert_eq!(memory.text, "new");
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn an_older_or_newer_file_is_never_written_and_a_damaged_one_moves_aside() {
        let dir = dir("policy");
        for (name, text) in [("v1", r#"{"text":"old"}"#), ("v3", r#"{"version":3,"text":"later"}"#), ("odd", r#"{"version":2,"text":7}"#)] {
            let file = dir.join(format!("{name}.json"));
            fs::write(&file, text).unwrap();
            assert_eq!(edited(&file), text, "{name}");
        }
        let damaged = dir.join("damaged.json");
        fs::write(&damaged, "{").unwrap();
        assert!(edited(&damaged).contains("new"));
        assert_eq!(fs::read_to_string(dir.join("damaged.damaged.json")).unwrap(), "{");
        fs::remove_dir_all(dir).unwrap();
    }
}
