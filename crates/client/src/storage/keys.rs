//! Keys that reach a file: refs as map keys, ids as file names.

use std::fmt::Write;

/// Serde for a map keyed by `Ref`, as `source:id` strings; a key that does not parse is dropped.
pub mod by_ref {
    use std::collections::HashMap;

    use model::Ref;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer, V: Serialize>(map: &HashMap<Ref, V>, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(map.iter().map(|(key, value)| (key.to_string(), value)))
    }

    pub fn deserialize<'de, D: Deserializer<'de>, V: Deserialize<'de>>(deserializer: D) -> Result<HashMap<Ref, V>, D::Error> {
        let raw = HashMap::<String, V>::deserialize(deserializer)?;
        Ok(raw.into_iter().filter_map(|(key, value)| Some((Ref::parse(&key)?, value))).collect())
    }
}

/// A file or folder name for `text`, one-to-one and safe on NTFS, which ignores case: `a-z`, `0-9` and `-`
/// stay, every other byte is `_xx`, and so is a device name's last byte (`con`, `com1`).
pub fn name(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        escape(&mut out, byte, !(byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'));
    }
    let device = matches!(out.as_str(), "con" | "prn" | "aux" | "nul")
        || (out.len() == 4 && (out.starts_with("com") || out.starts_with("lpt")) && out.as_bytes()[3].is_ascii_digit());
    if device && let Some(last) = out.pop() {
        escape(&mut out, last as u8, true);
    }
    out
}

/// `N` random bytes in hex: a new id or token.
pub(crate) fn random_hex<const N: usize>() -> String {
    let mut bytes = [0u8; N];
    getrandom::fill(&mut bytes).expect("the system's random source");
    hex(&bytes)
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn escape(out: &mut String, byte: u8, escaped: bool) {
    match escaped {
        true => write!(out, "_{byte:02x}").unwrap_or_default(),
        false => out.push(char::from(byte)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_keep_case_apart_and_never_name_a_device() {
        assert_eq!(name("3vabaGfHIk"), "3vaba_47f_48_49k");
        assert_ne!(name("ab"), name("AB"));
        assert_eq!(name("a_b/../c"), "a_5fb_2f_2e_2e_2fc");
        assert_eq!((name("con").as_str(), name("com1").as_str(), name("console").as_str()), ("co_6e", "com_31", "console"));
    }
}
