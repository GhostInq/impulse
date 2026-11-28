//! The open mark's pictures and clips on its card: the one shown, a strip to pick another or drop one, ADD MEDIA,
//! a paste of a picture or files, and the shown one full size.

use std::{rc::Rc, sync::Arc};

use anyhow::Result;
use blocking::unblock;
use client::map::{media, state::UserPin};
use gpui_kit::{
    AnyElement, App, ClipboardEntry, ClipboardItem, Context, Div, Entity, IntoElement, PathPromptOptions, RenderImage, Task, Window,
    component::h_flex, div, img, prelude::*, px, rgb,
};
use image::Frame;

use super::{Card, Playing, lightbox_frame};
use crate::{
    map::{
        CARD_WIDTH,
        board::{Attach, Board},
        picture::{ASPECT, fill, picture},
    },
    ui::{
        art,
        assets::Icon,
        key,
        text::t,
        theme::*,
        widgets::{button, icon, icon_button},
    },
};

const THUMB: (f32, f32) = (56., 32.);
// Inside the mark card's padding.
const WIDTH: f32 = CARD_WIDTH - 30.;
// The card's and the strip's copy of a picture: the full one is decoded only for the lightbox.
const SMALL: u32 = 640;
const LARGEST: u32 = 16384;

/// The open mark's media as the card draws them, made again when its list changes (the card redraws with each frame
/// of its clip): its pictures small, decoded off the UI thread.
#[derive(Default)]
pub(super) struct Kept {
    mark: Option<Rc<str>>,
    items: Vec<Item>,
    _loads: Vec<Task<()>>,
}

struct Item {
    name: Rc<str>,
    clip: bool,
    small: Option<Arc<RenderImage>>,
}

/// The clipboard's pictures and files go to the open mark; text still goes into the field.
pub(super) fn paste_into(board: Entity<Board>) -> impl Fn(&ClipboardItem, &mut Window, &mut App) -> bool + 'static {
    move |item, _, cx| {
        let mut text = false;
        let mut items = Vec::new();
        for entry in item.entries() {
            match entry {
                ClipboardEntry::Image(image) => items.push(Attach::Bytes(image.bytes.clone())),
                ClipboardEntry::ExternalPaths(paths) => items.extend(paths.paths().iter().cloned().map(Attach::File)),
                ClipboardEntry::String(_) => text = true,
            }
        }
        let Some(target) = board.read(cx).mark_target().filter(|_| !items.is_empty()) else { return false };
        board.update(cx, |board, cx| board.attach(target, items, cx));
        !text
    }
}

impl Card {
    /// Whether the projection is the open `mark`'s list as it is.
    pub(super) fn kept_follows(&self, mark: Option<&UserPin>) -> bool {
        match mark {
            Some(mark) => {
                self.kept.mark.as_deref() == Some(&*mark.id)
                    && self.kept.items.iter().map(|item| &*item.name).eq(mark.media.iter().map(String::as_str))
            }
            None => self.kept.mark.is_none(),
        }
    }

    /// Made again for the open mark (its id and list); a picture no longer in it leaves the atlas.
    pub(super) fn follow_kept(&mut self, mark: Option<(Rc<str>, Vec<String>)>, cx: &mut Context<Self>) {
        let old = self.forget_kept();
        self.stale.extend(old);
        let Some((id, names)) = mark else { return };
        let items = names.iter().map(|name| Item { name: name.as_str().into(), clip: media::is_clip(name), small: None });
        self.kept.items = items.collect();
        self.kept.mark = Some(id);
        let pictures: Vec<Rc<str>> = self.kept.items.iter().filter(|item| !item.clip).map(|item| item.name.clone()).collect();
        self.kept._loads = pictures.into_iter().map(|name| self.load(name, cx)).collect();
    }

    /// Drops the projection and its loads: its pictures, for the atlas to let go of.
    pub(super) fn forget_kept(&mut self) -> impl Iterator<Item = Arc<RenderImage>> + use<> {
        std::mem::take(&mut self.kept).items.into_iter().filter_map(|item| item.small)
    }

    fn load(&self, name: Rc<str>, cx: &mut Context<Self>) -> Task<()> {
        let (dir, kept) = (self.media_dir.clone(), name.to_string());
        cx.spawn(async move |card, cx| {
            let small = match unblock(move || small(&media::read(&dir, &kept)?)).await {
                Ok(small) => small,
                Err(e) => return log::warn!("pin picture not shown: {e:#}"),
            };
            card.update(cx, |card, cx| match card.kept.items.iter_mut().find(|item| item.name == name) {
                Some(item) => {
                    item.small = Some(small);
                    cx.notify();
                }
                None => card.stale.push(small),
            })
            .ok();
        })
    }

    /// The item the card shows: the one picked, else the first.
    fn shown_item(&self) -> Option<&Item> {
        let picked = self.kept.items.iter().find(|item| Some(&item.name) == self.mark_shown.as_ref());
        picked.or(self.kept.items.first())
    }

    /// The open mark's clip due on its card, by its name.
    pub(super) fn mark_clip(&self) -> Option<&str> {
        self.shown_item().filter(|item| item.clip).map(|item| &*item.name)
    }

    pub(super) fn mark_media(&self, mark: &UserPin, cx: &Context<Self>) -> impl IntoElement {
        let add = button(&t(cx, key::MAP_ADD_MEDIA)).id("mark-add-media").on_click(cx.listener(|card, _, _, cx| card.pick(cx)));
        let shown = self.shown_item().map(|item| {
            let layer = self.layer(item, !self.lightbox, || item.small.clone().map(|small| fill(img(small), "mark-small")));
            div()
                .id("mark-picture")
                .flex_none()
                .cursor_pointer()
                .child(framed(("mark-pictures", mark.created as usize), layer, (WIDTH, WIDTH / ASPECT)))
                .on_click(cx.listener(|card, _, _, cx| {
                    card.lightbox = true;
                    cx.notify();
                }))
        });
        let thumbs = self.kept.items.iter().enumerate().map(|(index, item)| self.thumb(mark, index, item, cx));
        div().flex().flex_col().gap(px(8.)).children(shown).child(h_flex().flex_wrap().gap(px(6.)).children(thumbs).child(add))
    }

    /// The open mark's shown picture full size, or its clip, over the whole map.
    pub(super) fn mark_lightbox(&self, board: &Board, cx: &Context<Self>) -> Option<AnyElement> {
        let mark = board.open_mark().filter(|_| self.lightbox)?;
        let item = self.shown_item()?;
        let full = || Some(fill(img(media::path(&self.media_dir, &item.name)), "mark-full"));
        // Its own cache: the full picture goes once the lightbox closes.
        let shown = framed(("mark-full-pictures", mark.created as usize), self.layer(item, true, full), self.lightbox_size());
        Some(lightbox_frame(shown, None::<AnyElement>, cx))
    }

    /// A clip while it plays here (`here`), else `picture`'s.
    fn layer(&self, item: &Item, here: bool, picture: impl FnOnce() -> Option<AnyElement>) -> Option<AnyElement> {
        match &self.clip {
            Some(clip) if here && matches!(&clip.playing, Playing::Kept(name) if **name == *item.name) => {
                Some(clip.view.clone().into_any_element())
            }
            _ if item.clip => None,
            _ => picture(),
        }
    }

    fn thumb(&self, mark: &UserPin, index: usize, item: &Item, cx: &Context<Self>) -> impl IntoElement {
        let (id, name, picked) = (mark.id.clone(), item.name.clone(), self.shown_item().is_some_and(|shown| shown.name == item.name));
        let content = match (&item.small, item.clip) {
            (_, true) => h_flex().size_full().justify_center().child(icon(Icon::Play, 12., rgb(MUTED))).into_any_element(),
            (Some(small), _) => fill(img(small.clone()), ("mark-thumb-media", index)),
            (None, _) => div().into_any_element(),
        };
        let remove = icon_button(("mark-media-drop", index), Icon::Close, 16., 8.).absolute().top_0().right_0().bg(rgb(PANEL));
        let dropped = name.clone();
        let remove = remove.on_click(cx.listener(move |card, _, _, cx| {
            cx.stop_propagation();
            card.board.update(cx, |board, cx| board.detach(&id, &dropped, cx));
        }));
        div()
            .id(("mark-thumb", index))
            .relative()
            .w(px(THUMB.0))
            .h(px(THUMB.1))
            .overflow_hidden()
            .cursor_pointer()
            .bg(rgb(SUNKEN))
            .border_1()
            .border_color(if picked { rgb(accent(cx)) } else { line(0.14) })
            .child(content)
            .child(remove)
            .on_click(cx.listener(move |card, _, _, cx| {
                card.mark_shown = Some(name.clone());
                cx.notify();
            }))
    }

    /// The target is taken before the picker opens: what lands goes to the mark it was asked for.
    fn pick(&mut self, cx: &mut Context<Self>) {
        let Some(target) = self.board.read(cx).mark_target() else { return };
        let prompt = Some(t(cx, key::MAP_ADD_MEDIA));
        let picked = cx.prompt_for_paths(PathPromptOptions { files: true, directories: false, multiple: true, prompt });
        let board = self.board.downgrade();
        cx.spawn(async move |_, cx| {
            let Ok(Ok(Some(files))) = picked.await else { return };
            board.update(cx, |board, cx| board.attach(target, files.into_iter().map(Attach::File).collect(), cx)).ok();
        })
        .detach();
    }
}

/// `layer` cut to `size` over the sunken stand-in; `id` keeps its pictures while drawn.
fn framed(id: (&'static str, usize), layer: Option<AnyElement>, size: (f32, f32)) -> Div {
    let under = div().absolute().size_full().bg(rgb(SUNKEN)).into_any_element();
    picture(id, under, layer, size)
}

/// A kept picture at most `SMALL` across.
fn small(bytes: &[u8]) -> Result<Arc<RenderImage>> {
    let mut small = art::picture(bytes, LARGEST)?.thumbnail(SMALL, SMALL).into_rgba8();
    for texel in small.pixels_mut() {
        texel.0.swap(0, 2);
    }
    Ok(Arc::new(RenderImage::new([Frame::new(small)])))
}
