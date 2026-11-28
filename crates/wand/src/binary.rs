//! The trainer DLL on disk: fetched once from the public CDN and named by its hash.

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow, ensure};
use model::{Failure, FailureKind};
use sha2::{Digest, Sha256};

use crate::texts::{TRAINER_FILE_MISSING, text};

// Not under `trainers`: that folder is the metadata's, and dropping it must not reach here.
const DIR: &str = "trainer-binaries";

pub fn size(data_dir: &Path) -> u64 {
    disk::size(&data_dir.join(DIR))
}

/// Drops every downloaded DLL; the next run fetches again. A DLL loaded in a game stays.
pub fn forget(data_dir: &Path) {
    let _ = fs::remove_dir_all(data_dir.join(DIR));
}

/// The DLL whose SHA-256 is `hash`, downloaded from `url` when it is not on disk (blocking). When the CDN cannot
/// give it, Wand's own download of it in `wand_caches` stands in. Both come from a remote response: the hash becomes
/// a file name and the URL is fetched.
pub fn ensure(data_dir: &Path, wand_caches: &[PathBuf], url: &str, hash: &str) -> Result<PathBuf> {
    ensure!(url.starts_with("https://"), "the trainer binary is not served over https");
    ensure!(hash.len() == 64 && hash.bytes().all(|b| b.is_ascii_hexdigit()), "the trainer binary hash is malformed");
    let hash = hash.to_ascii_lowercase();
    let path = data_dir.join(DIR).join(format!("{hash}.dll"));
    // Hashed again on every use: cheap next to the injection, and a damaged copy is fetched anew.
    if hash_of(&path).is_ok_and(|found| found == hash) {
        return Ok(path);
    }
    let body = match download(url, &hash) {
        Ok(body) => body,
        Err(e) => match wand_copy(wand_caches, &hash) {
            Some(body) => {
                log::debug!("trainer binary taken from Wand's own copy: {e:#}");
                body
            }
            None => return Err(e),
        },
    };
    disk::replace(&path, &body)?;
    Ok(path)
}

fn download(url: &str, hash: &str) -> Result<Vec<u8>> {
    let (mut answers, mut missing, mut offline) = (Vec::new(), true, false);
    for named in [false, true] {
        let how = if named { "named" } else { "bare" };
        let response = match if named { fetch::get(url, None) } else { fetch::get_bare(url) } {
            Ok(response) => response,
            Err(e) => {
                (missing, offline) = (false, true);
                answers.push(format!("{how}: {}", e.root_cause()));
                continue;
            }
        };
        let good = response.status == 200 && sha256(&response.body) == hash;
        let note = match (response.status, good, response.edge.as_str()) {
            (200, false, _) => " (not the trainer's hash)".to_owned(),
            (_, _, "") => String::new(),
            (_, _, edge) => format!(" ({edge})"),
        };
        answers.push(format!("status {} {how}{note}", response.status));
        if good {
            return Ok(response.body);
        }
        missing &= response.status == 404;
    }
    let why = anyhow!("GET {url}: {}", answers.join("; "));
    let failure = match (missing, offline) {
        (true, _) => Failure::new(FailureKind::NotFound).saying(text(TRAINER_FILE_MISSING)),
        (_, true) => Failure::new(FailureKind::Offline),
        _ => return Err(why),
    };
    Err(why.context(failure))
}

/// Wand names its copy `Trainer_<id>_<first ten of the hash>.dll`; only one that hashes to `hash` counts.
fn wand_copy(caches: &[PathBuf], hash: &str) -> Option<Vec<u8>> {
    let tail = format!("_{}.dll", &hash[..10]);
    caches.iter().filter_map(|dir| fs::read_dir(dir).ok()).flatten().flatten().find_map(|entry| {
        let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
        if !name.starts_with("trainer_") || !name.ends_with(&tail) {
            return None;
        }
        fs::read(entry.path()).ok().filter(|bytes| sha256(bytes) == hash)
    })
}

/// Lowercase hex SHA-256 of a file.
pub fn hash_of(path: &Path) -> io::Result<String> {
    fs::read(path).map(|bytes| sha256(&bytes))
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    fn dir(name: &str) -> PathBuf {
        let dir = env::temp_dir().join(format!("impulse-binary-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn a_hash_that_is_no_hash_never_reaches_a_path() {
        let dir = dir("hash");
        for hash in ["", "../../evil", &"g".repeat(64), &"a".repeat(63)] {
            let err = ensure(&dir, &[], "https://x.invalid/t", hash).unwrap_err();
            assert!(err.to_string().contains("malformed"), "{hash}: {err}");
        }
        assert!(ensure(&dir, &[], "http://x.invalid/t", &"a".repeat(64)).unwrap_err().to_string().contains("https"));
        assert!(!dir.exists());
    }

    #[test]
    fn a_good_copy_is_used_and_a_damaged_one_is_not() {
        let dir = dir("copy");
        let content = b"pretend dll";
        let hash = sha256(content).to_uppercase();
        let path = dir.join(DIR).join(format!("{}.dll", hash.to_lowercase()));
        disk::replace(&path, content).unwrap();
        // The URL cannot resolve: a download would fail.
        assert_eq!(ensure(&dir, &[], "https://x.invalid/t", &hash).unwrap(), path);

        assert_eq!(hash_of(&path).unwrap(), hash.to_lowercase());
        fs::write(&path, b"tampered").unwrap();
        assert!(ensure(&dir, &[], "https://x.invalid/t", &hash).is_err());
        assert_ne!(hash_of(&path).unwrap(), hash.to_lowercase());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_trainer_the_cdn_cannot_give_comes_from_wands_own_copy() {
        let (dir, wand) = (dir("fallback"), dir("wand"));
        let content = b"pretend dll";
        let hash = sha256(content);
        fs::create_dir_all(&wand).unwrap();
        fs::write(wand.join(format!("Trainer_1_{}.dll", &hash[..10])), content).unwrap();
        let caches = [wand.clone()];
        let path = ensure(&dir, &caches, "https://x.invalid/t", &hash).unwrap();
        assert_eq!(hash_of(&path).unwrap(), hash);

        let other = "b".repeat(64);
        fs::write(wand.join(format!("Trainer_2_{}.dll", &other[..10])), b"not what the hash names").unwrap();
        assert!(ensure(&dir, &caches, "https://x.invalid/t", &other).is_err());
        fs::remove_dir_all(&dir).unwrap();
        fs::remove_dir_all(&wand).unwrap();
    }
}
