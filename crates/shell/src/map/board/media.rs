//! The user's pins' pictures and clips: kept off the UI thread, then added to the pin they were asked for while its
//! map still shows.

use std::{path::PathBuf, rc::Rc};

use anyhow::Result;
use blocking::unblock;
use client::{
    map::{
        media,
        state::{PinEdit, UserPin},
    },
    media::Media,
};
use gpui_kit::Context;
use image::ImageFormat;
use model::{Pin, Ref};

use super::Board;
use crate::{Load, ui::art::Router};

/// What to keep as a pin's picture or clip.
pub enum Attach {
    File(PathBuf),
    Bytes(Vec<u8>),
    /// A map pin's media, through its cache folder.
    Cached(PathBuf, String),
}

impl Attach {
    /// The map `pin`'s full picture and its clip.
    pub fn of(pin: &Pin, board: &Board) -> Vec<Self> {
        let picture = pin.image.as_ref().map(|image| Self::Cached(Media::Pictures.dir(&board.paths), image.full.clone()));
        let clip = pin.video.clone().map(|url| Self::Cached(Media::Videos.dir(&board.paths), url));
        picture.into_iter().chain(clip).collect()
    }
}

/// A user's pin on the map shown when it was asked for: an attachment or a typed place lands there or nowhere.
#[derive(Clone)]
pub struct Target {
    map: Ref,
    id: Rc<str>,
}

impl Board {
    /// The open mark, taken before a picker opens.
    pub fn mark_target(&self) -> Option<Target> {
        self.target(self.mark.clone()?)
    }

    pub(super) fn target(&self, id: Rc<str>) -> Option<Target> {
        let target = Target { map: self.id.clone()?, id };
        self.holds(&target).then_some(target)
    }

    /// `target`'s pin while its map shows.
    pub fn target_pin(&self, target: &Target) -> Option<&UserPin> {
        let shows = self.is_open() && matches!(self.map, Load::Ready(_)) && self.id.as_ref() == Some(&target.map);
        self.saved.pin(&target.id).filter(|_| shows)
    }

    pub(super) fn holds(&self, target: &Target) -> bool {
        self.target_pin(target).is_some()
    }

    /// Keeps `items` and adds them to `target`.
    pub fn attach(&mut self, target: Target, items: Vec<Attach>, cx: &mut Context<Self>) {
        if items.is_empty() || !self.holds(&target) {
            return;
        }
        let (dir, registry) = (self.paths.map_media(), Router::registry(cx));
        let kept = unblock(move || {
            let keep = |item| match item {
                Attach::File(file) => media::import(&dir, &file),
                Attach::Bytes(bytes) => media::keep(&dir, &png_if_bitmap(bytes)?),
                Attach::Cached(cache, url) => client::media::path(&cache, &url, &registry).and_then(|file| media::import(&dir, &file)),
            };
            items.into_iter().filter_map(|item| keep(item).inspect_err(|e| log::warn!("pin media not kept: {e:#}")).ok()).collect()
        });
        cx.spawn(async move |board, cx| {
            let names: Vec<String> = kept.await;
            board.update(cx, |board, cx| board.attached(&target, names, cx)).ok();
        })
        .detach();
    }

    fn attached(&mut self, target: &Target, names: Vec<String>, cx: &mut Context<Self>) {
        if names.is_empty() || !self.holds(target) {
            return;
        }
        for name in names {
            self.saved_mut().edit_pin(&target.id, PinEdit::AddMedia(name));
        }
        self.save();
        cx.notify();
    }

    /// The file stays: another pin or a later sync may hold it.
    pub fn detach(&mut self, id: &str, name: &str, cx: &mut Context<Self>) {
        self.saved_mut().edit_pin(id, PinEdit::RemoveMedia(name.to_owned()));
        self.save();
        cx.notify();
    }
}

/// A bitmap (Windows hands a copied picture over as one) as a PNG: bitmaps are not kept.
fn png_if_bitmap(bytes: Vec<u8>) -> Result<Vec<u8>> {
    if !bytes.starts_with(b"BM") {
        return Ok(bytes);
    }
    let mut png = std::io::Cursor::new(Vec::new());
    image::load_from_memory_with_format(&bytes, ImageFormat::Bmp)?.write_to(&mut png, ImageFormat::Png)?;
    Ok(png.into_inner())
}
