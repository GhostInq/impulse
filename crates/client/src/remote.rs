//! The remote panel's devices: a one-time code the window shows, which a device trades for a token it keeps as a
//! cookie. Only the token's hash is kept. Pairing runs on the monotonic clock.

use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::storage::{hex, random_hex};

pub const CODE_LIFE: Duration = Duration::from_secs(5 * 60);
/// Wrong codes in a row that lock pairing for `LOCK`.
const TRIES: u32 = 5;
const LOCK: Duration = Duration::from_secs(60);

/// The code on screen and the wrong tries against it.
#[derive(Default)]
pub struct Pairing {
    code: Option<(String, Instant)>,
    wrong: u32,
    locked: Option<Instant>,
}

impl Pairing {
    /// The code to show: the current one, or a new one once it expired or was used.
    pub fn code(&mut self, now: Instant) -> &str {
        if self.code.as_ref().is_none_or(|(_, made)| now >= *made + CODE_LIFE) {
            self.code = Some((random_hex::<16>(), now));
        }
        &self.code.as_ref().expect("made above").0
    }

    /// Uses the code once; wrong tries lock pairing for a while.
    pub fn redeem(&mut self, code: &str, now: Instant) -> bool {
        if self.locked.is_some_and(|until| now < until) {
            return false;
        }
        let valid = self.code.as_ref().is_some_and(|(kept, made)| now < *made + CODE_LIFE && same(kept.as_bytes(), code.as_bytes()));
        if valid {
            (self.code, self.wrong) = (None, 0);
        } else {
            self.wrong += 1;
            if self.wrong == TRIES {
                (self.wrong, self.locked) = (0, Some(now + LOCK));
            }
        }
        valid
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Device {
    pub id: u32,
    pub name: String,
    // SHA-256 of its token, hex.
    hash: String,
}

/// The paired devices, a `Stored` file.
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Devices {
    list: Vec<Device>,
    next: u32,
}

impl Devices {
    pub fn list(&self) -> &[Device] {
        &self.list
    }

    pub fn has(&self, id: u32) -> bool {
        self.list.iter().any(|device| device.id == id)
    }

    /// A new device named from its User-Agent; its token, to give it once.
    pub fn add(&mut self, user_agent: &str) -> String {
        let token = random_hex::<32>();
        // A file without `next` still never gives a kept id twice.
        let id = self.next.max(self.list.iter().map(|device| device.id + 1).max().unwrap_or(0));
        self.next = id + 1;
        self.list.push(Device { id, name: name(user_agent), hash: hash(&token) });
        token
    }

    pub fn find(&self, token: &str) -> Option<u32> {
        let hash = hash(token);
        self.list.iter().find(|device| device.hash == hash).map(|device| device.id)
    }

    pub fn revoke(&mut self, id: u32) {
        self.list.retain(|device| device.id != id);
    }
}

/// What a person would call the device, empty when unknown; a User-Agent is remote data.
fn name(user_agent: &str) -> String {
    const KNOWN: [&str; 6] = ["iPhone", "iPad", "Android", "Windows", "Mac OS", "Linux"];
    KNOWN.into_iter().find(|known| user_agent.contains(known)).unwrap_or_default().to_owned()
}

fn hash(token: &str) -> String {
    hex(&Sha256::digest(token.as_bytes()))
}

/// Equal without a time that tells how much of `a` matched.
fn same(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0, |diff, (a, b)| diff | (a ^ b)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_code_works_once_and_not_after_five_minutes() {
        let (mut pairing, start) = (Pairing::default(), Instant::now());
        let at = |seconds| start + Duration::from_secs(seconds);
        let code = pairing.code(start).to_owned();
        assert_eq!(pairing.code(at(299)), code);
        assert!(pairing.redeem(&code, at(100)) && !pairing.redeem(&code, at(101)));
        let late = pairing.code(at(200)).to_owned();
        assert!(late != code && !pairing.redeem(&late, at(200) + CODE_LIFE));
    }

    #[test]
    fn five_wrong_codes_lock_pairing_for_a_minute_from_the_fifth() {
        let (mut pairing, start) = (Pairing::default(), Instant::now());
        let at = |seconds| start + Duration::from_secs(seconds);
        let code = pairing.code(start).to_owned();
        assert!(!pairing.redeem("wrong", start));
        for _ in 1..TRIES {
            assert!(!pairing.redeem("wrong", at(59)));
        }
        assert!(!pairing.redeem(&code, at(59) + LOCK - Duration::from_secs(1)));
        assert!(pairing.redeem(&code, at(59) + LOCK));
    }

    #[test]
    fn a_token_finds_its_device_until_revoked() {
        let mut devices = Devices::default();
        let token = devices.add("Mozilla/5.0 (Linux; Android 14)");
        let other = devices.add("Mozilla/5.0 (iPhone; CPU iPhone OS 17_0)");
        assert_eq!(devices.find(&token), Some(0));
        assert_eq!((devices.list[0].name.as_str(), devices.list[1].name.as_str()), ("Android", "iPhone"));
        assert!(!serde_json::to_string(&devices).unwrap().contains(&token));
        devices.revoke(0);
        assert_eq!((devices.find(&token), devices.find(&other)), (None, Some(1)));
        let mut kept: Devices = serde_json::from_str(r#"{ "list": [{ "id": 4, "name": "", "hash": "" }] }"#).unwrap();
        kept.add("");
        assert_eq!(kept.list[1].id, 5);
    }
}
