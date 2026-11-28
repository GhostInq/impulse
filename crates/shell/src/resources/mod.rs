//! What Impulse keeps on disk, by kind: sizes, clearing, and the limit on media.
//! The Settings RESOURCES tab shows it (`ResourcesView`).

mod view;

use crate::{
    sources::{Sources, SourcesChanged},
    ui::{key, text::app_text},
};
use std::{sync::Arc, time::Duration};

use blocking::unblock;
use gpui_kit::{Context, Entity, EventEmitter, Task};
use i18n::Key;
use model::{Clear, Ref, SourceId, Store};

use client::{
    Registry,
    media::{Media, limit},
    storage::{Paths, Stored},
};

pub use view::{ResourcesView, sized};

const SWEEP_EVERY: Duration = Duration::from_secs(600);

pub enum ResourcesEvent {
    /// These sources' stores were emptied: what came from them is asked for again.
    Cleared(Vec<SourceId>),
}

/// Sizes as last read.
struct Usage {
    caches: Vec<Store>,
    progress: u64,
    hotkeys: u64,
    logs: u64,
}

pub struct Resources {
    registry: Arc<Registry>,
    paths: Paths,
    usage: Option<Usage>,
    // A measurement was asked for, so a source change while it is in flight is not lost.
    measuring: bool,
    kept: Stored<limit::Kept>,
    _measure: Task<()>,
    _sweep: Task<()>,
}

impl EventEmitter<ResourcesEvent> for Resources {}

impl Resources {
    pub fn new(registry: Arc<Registry>, paths: &Paths, sources: &Entity<Sources>, cx: &mut Context<Self>) -> Self {
        // A source's stores come and go with it: read again while they show.
        cx.subscribe(sources, |resources, _, _: &SourcesChanged, cx| {
            if resources.measuring {
                resources.measure(cx);
            }
        })
        .detach();
        let kept = Stored::load(paths.resources());
        let mut resources = Self {
            registry,
            paths: paths.clone(),
            usage: None,
            measuring: false,
            kept,
            _measure: Task::ready(()),
            _sweep: Task::ready(()),
        };
        resources.keep_limit(cx);
        resources
    }

    pub fn measure(&mut self, cx: &mut Context<Self>) {
        self.measuring = true;
        let (stores, paths) = (self.registry.stores(), self.paths.clone());
        self._measure = cx.spawn(async move |view, cx| {
            let own = unblock(move || {
                let media = Media::ALL.map(|media| {
                    let (_, name, hint) = describe(media);
                    let (name, hint) = (app_text(name), app_text(hint));
                    Store { id: store_ref(media), name, hint, bytes: disk::size(&media.dir(&paths)), clear: Clear::Now }
                });
                let logs = journal::dir().map_or(0, disk::size);
                let progress = [paths.map_state(), paths.map_data(), paths.map_media()].iter().map(|dir| disk::size(dir)).sum();
                (media, progress, disk::size(&paths.hotkeys()), logs)
            });
            let ((media, progress, hotkeys, logs), provided) = (own.await, stores.await);
            let usage = Usage { caches: media.into_iter().chain(provided).collect(), progress, hotkeys, logs };
            view.update(cx, |view, cx| {
                view.usage = Some(usage);
                cx.notify();
            })
            .ok();
        });
    }

    pub fn limit(&self) -> Option<u64> {
        self.kept.limit
    }

    /// Empties the stores `ids` name, then reads the sizes again: the app's media here, the rest through their source.
    pub fn clear(&mut self, ids: Vec<Ref>, cx: &mut Context<Self>) {
        let (registry, paths) = (self.registry.clone(), self.paths.clone());
        cx.spawn(async move |view, cx| {
            let mut provided = Vec::new();
            for id in ids {
                match media_of(&id) {
                    Some(media) => {
                        let dir = media.dir(&paths);
                        unblock(move || std::fs::remove_dir_all(dir)).await.ok();
                    }
                    None => {
                        registry.clear(&id).await;
                        if !provided.contains(id.source()) {
                            provided.push(id.source().clone());
                        }
                    }
                }
            }
            view.update(cx, |view, cx| {
                if !provided.is_empty() {
                    cx.emit(ResourcesEvent::Cleared(provided));
                }
                view.measure(cx);
            })
            .ok();
        })
        .detach();
    }

    pub fn set_limit(&mut self, limit: Option<u64>, cx: &mut Context<Self>) {
        self.kept.edit(|kept| kept.limit = limit);
        self.keep_limit(cx);
        cx.notify();
    }

    /// Holds the media under the limit: now, then every few minutes, and measures again after.
    fn keep_limit(&mut self, cx: &mut Context<Self>) {
        let Some(limit) = self.kept.limit else {
            self._sweep = Task::ready(());
            return;
        };
        let dirs = Media::ALL.map(|media| media.dir(&self.paths));
        self._sweep = cx.spawn(async move |view, cx| {
            loop {
                let dirs = dirs.clone();
                unblock(move || limit::sweep(&dirs, limit)).await;
                if view.update(cx, |view, cx| view.usage.is_some().then(|| view.measure(cx))).is_err() {
                    return;
                }
                cx.background_executor().timer(SWEEP_EVERY).await;
            }
        });
    }
}

fn store_ref(media: Media) -> Ref {
    Ref::new(&SourceId::app(), describe(media).0)
}

fn media_of(id: &Ref) -> Option<Media> {
    let app = *id.source() == SourceId::app();
    Media::ALL.into_iter().find(|media| app && describe(*media).0 == id.id())
}

fn describe(media: Media) -> (&'static str, Key, Key) {
    match media {
        Media::Pictures => ("pictures", key::MEDIA_PICTURES, key::MEDIA_PICTURES_HINT),
        Media::Tiles => ("tiles", key::MEDIA_TILES, key::MEDIA_TILES_HINT),
        Media::Videos => ("videos", key::MEDIA_VIDEOS, key::MEDIA_VIDEOS_HINT),
    }
}
