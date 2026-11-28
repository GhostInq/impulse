//! Over the map: the selected pin's card or the user's own pin's, and the pin's picture or clip full size.
//! A view of its own: what moves on the map does not rebuild it.

mod clip;
mod mark;
mod media;
mod pin;

use std::{mem, path::PathBuf, rc::Rc, sync::Arc};

use gpui_kit::{
    AnyElement, App, Bounds, ClickEvent, Context, Div, Entity, IntoElement, Pixels, Render, RenderImage, SharedString, Stateful,
    Subscription, Task, Window, canvas,
    component::{
        h_flex,
        input::{InputState, TextareaState},
        v_flex,
    },
    div, img,
    prelude::*,
    px, rgb,
};
use model::Map;

use super::{
    CARD_WIDTH, CONTROLS, MARGIN,
    board::{Board, BoardEvent, Target},
    markers::Images,
    picture::{ASPECT, fill, picture, stand_in},
};
use crate::{
    Load,
    ui::{
        assets::Icon,
        key,
        text::{Texts, t},
        theme::*,
        widgets::{button, choice, icon, segmented},
    },
};
use clip::{ClipView, Source};
use video::ID3D11Device;

// The card's picture, 16:9 across it.
const PICTURE: f32 = 190.;
const LIGHTBOX_PADDING: f32 = 24.;

/// The selected pin's clip or the open mark's, on the card or in the lightbox, with what it was started for.
struct Clip {
    playing: Playing,
    // The window's GPU: after the device is lost and made again, the clip starts again on the new one.
    device: ID3D11Device,
    view: Entity<ClipView>,
}

#[derive(PartialEq)]
enum Playing {
    Pin(usize),
    /// A `map::media` name.
    Kept(String),
}

pub struct Card {
    board: Entity<Board>,
    video_dir: PathBuf,
    media_dir: PathBuf,
    // Which of the open mark's media the card shows, by name; else its first.
    mark_shown: Option<Rc<str>>,
    kept: media::Kept,
    lightbox: bool,
    // The lightbox shows the picture, not the clip.
    photo: bool,
    // Teleport was pressed once and waits for the second press.
    confirm: bool,
    copied: bool,
    mark_name: Entity<InputState>,
    mark_note: Entity<TextareaState>,
    mark_axes: [Entity<InputState>; 3],
    // The mark the axes show, on its map: typed values go to it even once its card has closed.
    axes_for: Option<Target>,
    clip: Option<Clip>,
    // The pins closest to the selected one, worked out once: the card redraws with each frame of its clip.
    near: Vec<usize>,
    // The selected pin's "hide group" line, made once per pin and language: the card redraws with each frame of its clip.
    hide: Option<(usize, SharedString)>,
    images: Images,
    // Images to drop from the atlas when the card is next drawn.
    stale: Vec<Arc<RenderImage>>,
    // As last laid out: the map's, which the card spans and the lightbox fills.
    bounds: Bounds<Pixels>,
    _flash: Task<()>,
    // The name field's placeholder follows the language.
    _relabel: Subscription,
}

impl Card {
    pub fn new(board: Entity<Board>, video_dir: PathBuf, media_dir: PathBuf, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let (mark_name, mark_note, relabel) = mark::inputs(window, cx);
        let mark_axes = mark::axes(window, cx);
        cx.observe(&board, |_, _, cx| cx.notify()).detach();
        cx.observe_global::<Texts>(|card, cx| {
            card.hide = None;
            cx.notify();
        })
        .detach();
        cx.subscribe_in(&board, window, |card, board, event: &BoardEvent, window, cx| match event {
            // Pin indexes may point elsewhere in a fresher copy.
            BoardEvent::Installed { .. } => {
                (card.lightbox, card.confirm, card.clip) = (false, false, None);
                (card.near, card.hide) = (Vec::new(), None);
                card.stale.extend(card.images.clear());
            }
            BoardEvent::Selected => {
                (card.lightbox, card.confirm, card.mark_shown) = (false, false, None);
                card.near = pin::nearest(board.read(cx));
                card.fill_mark(board, window, cx);
            }
            BoardEvent::MarkAdded => card.mark_name.update(cx, |input, cx| input.focus(window, cx)),
            BoardEvent::MarkMoved => card.fill_axes(window, cx),
            _ => {}
        })
        .detach();
        Self {
            board,
            video_dir,
            media_dir,
            mark_shown: None,
            kept: media::Kept::default(),
            lightbox: false,
            photo: false,
            confirm: false,
            copied: false,
            mark_name,
            mark_note,
            mark_axes,
            axes_for: None,
            clip: None,
            near: Vec::new(),
            hide: None,
            images: Images::default(),
            stale: Vec::new(),
            bounds: Bounds::default(),
            _flash: Task::ready(()),
            _relabel: relabel,
        }
    }

    /// Escape: the lightbox; false when it was not open.
    pub fn close_top(&mut self, cx: &mut Context<Self>) -> bool {
        let open = mem::take(&mut self.lightbox);
        cx.notify();
        open
    }

    pub fn hide(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.place_mark(window, cx);
        (self.lightbox, self.clip) = (false, None);
        self.stale.extend(self.images.clear());
        let kept = self.forget_kept();
        self.stale.extend(kept);
        for image in self.stale.drain(..) {
            let _ = window.drop_image(image);
        }
    }

    /// The clip due on screen, on the card or in the lightbox, and the window's GPU it plays on, when that is not the
    /// one playing: `Some(None)` stops it. Made only on a change: the card redraws with each frame of its clip.
    fn clip_change(&self, board: &Board, window: &Window) -> Option<Option<(Playing, ID3D11Device)>> {
        let device = window.d3d11_device();
        let wanted = match (&board.map, board.selected, board.open_mark()) {
            (Load::Ready(map), Some(index), _) => {
                let pin = &map.pins[index];
                (pin.video.is_some() && !(self.lightbox && self.photo && pin.image.is_some())).then_some(Ok(index))
            }
            (_, _, Some(_)) => self.mark_clip().map(Err),
            _ => None,
        };
        let same = match (&self.clip, wanted) {
            (None, None) => true,
            (Some(clip), Some(wanted)) => {
                let what = match (&clip.playing, wanted) {
                    (Playing::Pin(pin), Ok(index)) => *pin == index,
                    (Playing::Kept(name), Err(wanted)) => name == wanted,
                    _ => false,
                };
                what && device.as_ref() == Some(&clip.device)
            }
            _ => false,
        };
        if same {
            return None;
        }
        let playing = |wanted: Result<usize, &str>| wanted.map_or_else(|name| Playing::Kept(name.to_owned()), Playing::Pin);
        Some(wanted.zip(device).map(|(wanted, device)| (playing(wanted), device)))
    }

    /// Starts the clip wanted; the one it replaces stops.
    fn play(&mut self, wanted: Option<(Playing, ID3D11Device)>, cx: &mut Context<Self>) {
        self.clip = wanted.and_then(|(playing, device)| {
            let source = match &playing {
                Playing::Pin(pin) => {
                    let Load::Ready(map) = &self.board.read(cx).map else { return None };
                    Source::Cached { dir: self.video_dir.clone(), url: map.pins[*pin].video.clone()? }
                }
                Playing::Kept(name) => Source::File(client::map::media::path(&self.media_dir, name)),
            };
            let view = cx.new(|cx| ClipView::new(source, device.clone(), cx));
            Some(Clip { playing, device, view })
        });
    }

    fn lightbox_size(&self) -> (f32, f32) {
        let area = |side: Pixels| (f32::from(side) - LIGHTBOX_PADDING * 2.).max(1.);
        let width = area(self.bounds.size.width).min(area(self.bounds.size.height) * ASPECT);
        (width, width / ASPECT)
    }

    /// A card's frame at the map's top right.
    fn panel(&self, id: &'static str) -> Stateful<Div> {
        v_flex()
            .id(id)
            .occlude()
            .absolute()
            .top(px(MARGIN))
            .right(px(MARGIN))
            .w(px(CARD_WIDTH))
            // Above the stage's buttons, which stay at the bottom right.
            .max_h(self.bounds.size.height - px(MARGIN * 3. + CONTROLS))
            .overflow_y_scroll()
            .gap(px(10.))
            .border_1()
            .border_color(line(0.16))
            .bg(rgb(PANEL))
    }

    /// The selected pin's clip or picture over the whole map; a click anywhere but the switch closes it.
    fn lightbox(&mut self, map: &Map, board: &Board, window: &Window, cx: &Context<Self>) -> Option<AnyElement> {
        let index = board.selected.filter(|_| self.lightbox)?;
        let pin = &map.pins[index];
        let clip = self.clip.as_ref().map(|clip| clip.view.clone().into_any_element());
        // The preview is on disk already: it holds the place while the full size loads, or under the clip.
        let preview = pin.image.as_ref().map(|image| fill(img(image.preview.clone()), ("pin-media", index)));
        let top = clip.or_else(|| pin.image.as_ref().map(|image| fill(img(image.full.clone()), ("pin-full", index))));
        let under = stand_in(pin.group, map, board, &mut self.images, window, cx);
        let switch = pin.video.as_ref().zip(pin.image.as_ref()).map(|_| {
            let pick = |photo: bool| {
                cx.listener(move |card: &mut Self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>| {
                    card.photo = photo;
                    cx.stop_propagation();
                    cx.notify();
                })
            };
            let choices = segmented()
                .h(px(26.))
                .bg(alpha(PANEL, 0.9))
                .child(choice("lightbox-video", t(cx, key::MAP_VIDEO), !self.photo, cx).on_click(pick(false)))
                .child(choice("lightbox-photo", t(cx, key::MAP_PHOTO), self.photo, cx).on_click(pick(true)));
            h_flex().absolute().top(px(LIGHTBOX_PADDING / 2.)).left_0().w_full().justify_center().child(choices)
        });
        let shown = picture(("pin-pictures", index), under, preview.into_iter().chain(top), self.lightbox_size());
        Some(lightbox_frame(shown, switch, cx))
    }
}

impl Render for Card {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let board = self.board.clone();
        let open = board.read(cx).open_mark();
        if !self.kept_follows(open) {
            let mark = open.map(|mark| (mark.id.clone(), mark.media.clone()));
            self.follow_kept(mark, cx);
        }
        for image in self.stale.drain(..) {
            let _ = window.drop_image(image);
        }
        if let Some(wanted) = self.clip_change(board.read(cx), window) {
            self.play(wanted, cx);
        }
        let board = board.read(cx);
        // Its size is known once laid out: a change takes one more frame.
        let view = cx.entity();
        let measure = canvas(
            move |bounds, window, cx| {
                if view.update(cx, |card, _| mem::replace(&mut card.bounds, bounds) != bounds) {
                    window.request_animation_frame();
                }
            },
            |_, _, _, _| {},
        )
        .absolute()
        .size_full();
        let root = div().relative().size_full().child(measure);
        let Load::Ready(map) = &board.map else { return root };
        let pin = self.pin_card(map, board, window, cx);
        let lightbox = self.lightbox(map, board, window, cx).or_else(|| self.mark_lightbox(board, cx));
        root.children(pin).children(self.mark_card(map, board, cx)).children(lightbox)
    }
}

/// MARK FOUND, or FOUND in the accent with a check.
fn found_button(id: &'static str, found: bool, cx: &App) -> Stateful<Div> {
    let accent = rgb(accent(cx));
    button(&t(cx, if found { key::MAP_FOUND } else { key::MAP_MARK_FOUND }))
        .id(id)
        .flex_1()
        .when(found, |button| button.border_color(accent).text_color(accent).child(icon(Icon::Check, 11., accent)))
}

/// A picture or clip over the whole map; a click anywhere but `over` closes it.
fn lightbox_frame(shown: Div, over: Option<impl IntoElement>, cx: &Context<Card>) -> AnyElement {
    let shut = cx.listener(|card, _, _, cx| {
        card.lightbox = false;
        cx.notify();
    });
    h_flex()
        .id("pin-lightbox")
        .occlude()
        .absolute()
        .top_0()
        .left_0()
        .size_full()
        .justify_center()
        .bg(alpha(SUNKEN, 0.94))
        .cursor_pointer()
        .child(shown)
        .children(over)
        .on_click(shut)
        .into_any_element()
}
