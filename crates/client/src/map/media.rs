//! The user's own pictures and clips of pins, one folder for every map: a file is named by what it holds, so it is
//! kept once and travels by name.

use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};

use crate::storage;

/// A format we keep: the suffix of its name, whether it is a clip, and the bytes it holds at `at`.
struct Format {
    suffix: &'static str,
    clip: bool,
    at: usize,
    magic: &'static [u8],
}

const FORMATS: [Format; 5] = [
    Format { suffix: "png", clip: false, at: 0, magic: b"\x89PNG" },
    Format { suffix: "jpg", clip: false, at: 0, magic: b"\xFF\xD8\xFF" },
    Format { suffix: "webp", clip: false, at: 8, magic: b"WEBP" },
    Format { suffix: "mp4", clip: true, at: 4, magic: b"ftyp" },
    Format { suffix: "webm", clip: true, at: 0, magic: b"\x1A\x45\xDF\xA3" },
];
// Brands of an `ftyp` box with no clip to play: sound, or HEIF and AVIF stills and sequences.
const STILL_OR_SOUND: [&[u8]; 16] = [
    b"M4A ", b"M4B ", b"M4P ", b"F4A ", b"F4B ", b"heic", b"heix", b"heim", b"heis", b"hevc", b"hevx", b"hevm", b"hevs", b"mif1", b"msf1",
    b"avif",
];
const HEAD: usize = 12;

/// Whether the kept `name` is a clip.
pub fn is_clip(name: &str) -> bool {
    let suffix = Path::new(name).extension().and_then(|ext| ext.to_str());
    FORMATS.iter().any(|format| format.clip && Some(format.suffix) == suffix)
}

/// `bytes`, a picture or a clip, kept in `dir`: its name. Blocking.
pub fn keep(dir: &Path, bytes: &[u8]) -> Result<String> {
    let name = named(&Sha256::digest(bytes), bytes)?;
    let file = dir.join(&name);
    if !file.is_file() {
        disk::replace(&file, bytes)?;
    }
    Ok(name)
}

/// The picture or clip `file` kept in `dir`: named by the bytes copied, read once (a clip may be large). Blocking.
pub fn import(dir: &Path, file: &Path) -> Result<String> {
    let mut from = File::open(file).with_context(|| format!("open {}", file.display()))?;
    let (mut staged, mut hasher) = (disk::stage(dir)?, Sha256::new());
    let (mut head, mut buffer) = (Vec::with_capacity(HEAD), vec![0; 1 << 16]);
    loop {
        let read = from.read(&mut buffer).with_context(|| format!("read {}", file.display()))?;
        if read == 0 {
            break;
        }
        let bytes = &buffer[..read];
        head.extend_from_slice(&bytes[..(HEAD - head.len()).min(read)]);
        hasher.update(bytes);
        staged.write_all(bytes)?;
    }
    let name = named(&hasher.finalize(), &head)?;
    // Kept already, maybe open to play: left as it is.
    let kept = dir.join(&name);
    if !kept.is_file() {
        staged.publish(&kept).or_else(|e| if kept.is_file() { Ok(()) } else { Err(e) })?;
    }
    Ok(name)
}

/// The kept `name`'s bytes. Blocking.
pub fn read(dir: &Path, name: &str) -> Result<Vec<u8>> {
    let file = path(dir, name);
    std::fs::read(&file).with_context(|| format!("read {}", file.display()))
}

/// Where the kept `name` is; a name with a folder in it stays inside `dir`.
pub fn path(dir: &Path, name: &str) -> PathBuf {
    dir.join(Path::new(name).file_name().unwrap_or_default())
}

/// Told by its first bytes: a name or a URL may not say.
fn named(digest: &[u8], head: &[u8]) -> Result<String> {
    let starts = |at: usize, magic: &[u8]| head.get(at..at + magic.len()) == Some(magic);
    let format = FORMATS.iter().find(|format| starts(format.at, format.magic));
    let Some(format) = format.filter(|format| format.suffix != "mp4" || !STILL_OR_SOUND.iter().any(|brand| starts(8, brand))) else {
        bail!("not a picture or clip");
    };
    Ok(format!("{}.{}", storage::hex(digest), format.suffix))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn the_same_content_is_kept_once_under_one_name_of_its_format() {
        let dir = std::env::temp_dir().join(format!("impulse-pin-media-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let picked = dir.join("picked").join("shot.png");
        fs::create_dir_all(picked.parent().unwrap()).unwrap();
        let jpeg = b"\xFF\xD8\xFF\xE0 jpeg bytes";
        fs::write(&picked, jpeg).unwrap();
        let kept = dir.join("kept");

        let name = import(&kept, &picked).unwrap();
        assert_eq!((import(&kept, &picked).unwrap(), keep(&kept, jpeg).unwrap()), (name.clone(), name.clone()));
        assert!(name.ends_with(".jpg") && !is_clip(&name));
        let clip = keep(&kept, b"\0\0\0\x18ftypisom").unwrap();
        assert!(clip.ends_with(".mp4") && is_clip(&clip));
        assert_eq!(fs::read_dir(&kept).unwrap().count(), 2, "no staged file is left");
        for refused in [&b"MZ an exe"[..], b"\0\0\0\x18ftypM4A sound", b"\0\0\0\x18ftypheix", b"GIF89a"] {
            assert!(keep(&kept, refused).is_err());
        }
        fs::write(&picked, b"MZ an exe").unwrap();
        assert!(import(&kept, &picked).is_err());
        assert_eq!(fs::read_dir(&kept).unwrap().count(), 2, "a refused file leaves nothing");
        assert_eq!(path(&kept, "..\\..\\x.png"), kept.join("x.png"));
        fs::remove_dir_all(dir).unwrap();
    }
}
