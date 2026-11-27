//! Wand as a source: the user's WeMod account handed off from Wand, its catalog, trainers, maps, folder and caches.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, PoisonError},
};

use blocking::unblock;
use futures::{
    FutureExt, StreamExt,
    channel::mpsc::{self, UnboundedSender},
    future::{self, BoxFuture, Shared},
    stream::{self, BoxStream},
};
use model::{Account, Clear, Failure, FailureKind, Folder, Game, Locale, MapOffer, Offer, Ref, SignIn, Snapshot, SourceId, Store};

use crate::{
    binary,
    catalog::{self, Mark},
    folder::Setting,
    locale, map, overlay,
    run::Host,
    session::{self, Session, Source, account},
    store,
    texts::{self, *},
    trainer::{self, Stamp, Trainer},
    transport::{self, Transport},
};

type Link = Result<(Arc<Session>, Transport, Source), Failure>;

const MAPS: &str = "maps";
const CATALOG: &str = "catalog";
const TRAINERS: &str = "trainers";
const TRAINER_FILES: &str = "trainer-files";
const BRIDGE: &str = "bridge";
const WEMOD: &str = "wemod.com";

pub struct Wand {
    data_dir: PathBuf,
    transport: Result<Transport, Failure>,
    // Shared by every caller; a failed or rejected connection is tried again by the next. The flag: it refreshes,
    // so a refresh that failed is tried again as one.
    session: Mutex<(Shared<BoxFuture<'static, Link>>, bool)>,
    folder: Arc<Setting>,
    host: Host,
    marks: catalog::Marks,
    archives: map::Archives,
    // The hotkeys the user changed in Wand, read when a trainer first needs them.
    hotkeys: Shared<BoxFuture<'static, Arc<store::Custom>>>,
}

impl Wand {
    /// The source id of everything it mints.
    pub const SOURCE: &str = crate::SOURCE;
    /// The signers of the DLLs it puts into a game.
    pub const PUBLISHERS: &[&str] = crate::install::PUBLISHERS;

    /// Starts the browser bridge at once: it takes a moment to come up.
    pub fn new(data_dir: PathBuf) -> Self {
        // On some networks WeMod's edge turns away a client that names itself otherwise.
        fetch::name_as(WEMOD, crate::install::user_agent());
        let transport = Transport::spawn(data_dir.clone()).map_err(|e| {
            log::error!("Transport: {e:#}");
            Failure::from(e)
        });
        let folder = Arc::new(Setting::load(&data_dir));
        Self {
            session: Mutex::new((connect(&transport, &data_dir, false), false)),
            host: Host::new(data_dir.clone(), folder.clone()),
            marks: catalog::Marks::default(),
            archives: map::Archives::default(),
            transport,
            folder,
            data_dir,
            hotkeys: async {
                unblock(store::hotkeys).await.map(Arc::new).unwrap_or_else(|e| {
                    log::warn!("Wand's own hotkeys: {e:#}");
                    Arc::default()
                })
            }
            .boxed()
            .shared(),
        }
    }

    fn session(&self) -> Shared<BoxFuture<'static, Link>> {
        let mut guard = self.session.lock().unwrap_or_else(PoisonError::into_inner);
        let (link, refreshes) = &mut *guard;
        let refresh = match link.peek() {
            Some(Err(_)) => Some(*refreshes),
            Some(Ok((session, ..))) if session.is_rejected() => Some(true),
            _ => None,
        };
        if let Some(refresh) = refresh {
            (*link, *refreshes) = (connect(&self.transport, &self.data_dir, refresh), refresh);
        }
        link.clone()
    }
}

impl model::Source for Wand {
    fn id(&self) -> SourceId {
        crate::source()
    }

    fn texts(&self) -> model::Files {
        texts::files()
    }

    fn name(&self) -> model::Text {
        model::Text::Plain("Wand".to_owned())
    }
}

impl model::Accounts for Wand {
    fn sign_in(&self) -> BoxStream<'static, SignIn> {
        let (session, dir) = (self.session(), self.data_dir.clone());
        channel(|tx| async move {
            let cached = {
                let dir = dir.clone();
                unblock(move || account::cached(&dir)).await
            };
            let known = cached.as_ref().is_some_and(|(_, fresh)| *fresh);
            if let Some((account, _)) = cached {
                send(&tx, SignIn::Account(Account { name: account.username }));
            }
            let (session, transport, source) = match session.await {
                Ok(link) => link,
                Err(failure) => return send(&tx, SignIn::Failed(failure)),
            };
            // A new session from Wand may be another account than the one kept.
            if known && !matches!(source, Source::Wand) {
                return;
            }
            match account::fetch(&session, &transport, &dir).await {
                Ok(account) => send(&tx, SignIn::Account(Account { name: account.username })),
                Err(e) => {
                    log::warn!("account: {e:#}");
                    let failure = Failure::from(e);
                    if failure.kind == FailureKind::SignedOut {
                        send(&tx, SignIn::Failed(failure));
                    }
                }
            }
        })
    }
}

impl model::Games for Wand {
    fn games(&self) -> BoxStream<'static, Snapshot<Vec<Game>>> {
        let (dir, marks) = (self.data_dir.clone(), self.marks.clone());
        channel(|tx| async move {
            let cached = {
                let (dir, marks) = (dir.clone(), marks.clone());
                unblock(move || catalog::cached(&dir, &marks)).await
            };
            let have_copy = cached.is_some();
            send(&tx, Snapshot::Cached(cached));
            match unblock(move || catalog::refresh(&dir, have_copy, &marks)).await {
                Ok(Some(games)) => {
                    log::info!("catalog updated");
                    send(&tx, Snapshot::Fresh(Ok(games)));
                }
                Ok(None) => log::debug!("catalog is up to date"),
                Err(e) => {
                    log::warn!("catalog not refreshed: {e:#}");
                    send(&tx, Snapshot::Fresh(Err(e.into())));
                }
            }
        })
    }
}

impl model::Overlays for Wand {
    fn enter(&self, game: &Ref, process: &model::Process) -> Result<model::Canvas, Failure> {
        overlay::enter(&self.marks, &self.folder, game, process).map_err(Failure::from)
    }
}

impl model::Trainers for Wand {
    /// The API is asked only when the disk copy is missing or was fetched for another trainer, build or locale.
    fn trainer(&self, offer: &Offer, build: Option<u32>, locale: &Locale) -> BoxStream<'static, Snapshot<model::Trainer>> {
        let stamp = Stamp { updated_at: offer.revision.clone(), version: build, locale: locale::served(locale).to_owned() };
        // A trainer's id is its game's.
        let (dir, id, session, host) = (self.data_dir.clone(), offer.id.id().to_owned(), self.session(), self.host.clone());
        let (hotkeys, mark) = (self.hotkeys.clone(), self.marks.of(offer.id.id()));
        channel(|tx| async move {
            let copy = {
                let (dir, id) = (dir.clone(), id.clone());
                unblock(move || trainer::cached(&dir, &id)).await
            };
            let custom = hotkeys.await;
            let custom = custom.get(&*id);
            let snapshot = |trainer| snapshot(trainer, &host, custom, mark);
            let fresh = copy.as_ref().is_some_and(|(_, found)| *found == stamp);
            send(&tx, Snapshot::Cached(copy.map(|(trainer, _)| snapshot(trainer))));
            if fresh {
                return;
            }
            let fetched = async {
                let (session, transport, _) = session.await?;
                trainer::fetch(&session, &transport, &dir, &id, stamp).await
            }
            .await;
            if let Err(e) = &fetched {
                log::warn!("trainer {id}: {e:#}");
            }
            send(&tx, Snapshot::Fresh(fetched.map(snapshot).map_err(Failure::from)));
        })
    }
}

impl model::Maps for Wand {
    /// The API is asked only when the disk copy is missing or was fetched for another revision or locale.
    fn map(&self, offer: &MapOffer, locale: &Locale) -> BoxStream<'static, Snapshot<model::Map>> {
        let (dir, offer, session) = (self.data_dir.clone(), offer.clone(), self.session());
        let served = locale::served(locale);
        channel(|tx| async move {
            let mut copy = {
                let (dir, id) = (dir.clone(), offer.id.id().to_owned());
                unblock(move || map::cached(&dir, &id)).await
            };
            let fresh = copy.as_ref().is_some_and(|copy| copy.stamp == offer.revision && copy.locale == served);
            let mut icons = copy.as_mut().and_then(|copy| copy.icons.take());
            send(&tx, Snapshot::Cached(copy.map(|copy| copy.map)));
            if !fresh {
                let mut fetched = async {
                    let (session, transport, _) = session.await?;
                    map::fetch(&session, &transport, &dir, offer.id.id(), offer.revision.clone(), served).await
                }
                .await;
                if let Err(e) = &fetched {
                    log::warn!("map {}: {e:#}", offer.id);
                }
                icons = fetched.as_mut().ok().and_then(|copy| copy.icons.take());
                send(&tx, Snapshot::Fresh(fetched.map(|copy| copy.map).map_err(Failure::from)));
            }
            if let Some(icons) = icons
                && let Some(map) = unblock(move || icons.download(&dir)).await
            {
                send(&tx, Snapshot::Fresh(Ok(map)));
            }
        })
    }
}

/// The tiles of the maps kept in PMTiles archives.
impl model::Downloads for Wand {
    fn download(&self, url: &str) -> Result<Vec<u8>, Failure> {
        self.archives.tile(url)
    }
}

impl model::Folders for Wand {
    fn folders(&self) -> BoxFuture<'static, Vec<Folder>> {
        let folder = self.folder.clone();
        unblock(move || vec![folder.folder()]).boxed()
    }

    // Wand has one folder.
    fn set_folder(&self, _id: &Ref, path: PathBuf) -> BoxFuture<'static, Folder> {
        let (folder, call) = (self.folder.clone(), self.folder.call());
        unblock(move || folder.set(call, path)).boxed()
    }
}

impl model::Caches for Wand {
    fn stores(&self) -> BoxFuture<'static, Vec<Store>> {
        let dir = self.data_dir.clone();
        unblock(move || {
            let id = |id| Ref::new(&crate::source(), id);
            let store = |store, name, hint, bytes, clear| Store { id: id(store), name: text(name), hint: text(hint), bytes, clear };
            let (profile, asked) = transport::profile(&dir);
            vec![
                store(MAPS, STORE_MAPS, STORE_MAPS_HINT, map::size(&dir), Clear::Now),
                store(CATALOG, STORE_CATALOG, STORE_CATALOG_HINT, catalog::size(&dir) + account::size(&dir), Clear::Now),
                store(TRAINERS, STORE_TRAINERS, STORE_TRAINERS_HINT, trainer::size(&dir), Clear::Now),
                store(TRAINER_FILES, STORE_TRAINER_FILES, STORE_TRAINER_FILES_HINT, binary::size(&dir), Clear::Now),
                store(BRIDGE, STORE_BROWSER, STORE_BROWSER_HINT, profile, Clear::NextStart { asked }),
            ]
        })
        .boxed()
    }

    fn clear(&self, id: &Ref) -> BoxFuture<'static, ()> {
        let (dir, id) = (self.data_dir.clone(), id.id().to_owned());
        unblock(move || match id.as_str() {
            MAPS => map::forget(&dir),
            CATALOG => {
                catalog::forget(&dir);
                account::forget(&dir);
            }
            TRAINERS => trainer::forget(&dir),
            TRAINER_FILES => binary::forget(&dir),
            BRIDGE => transport::clear_profile_on_start(&dir),
            _ => {}
        })
        .boxed()
    }
}

/// The cheat list with a runner for exactly its plan.
/// `custom` holds the hotkeys the user changed in Wand for this game.
fn snapshot(trainer: Trainer, host: &Host, custom: Option<&HashMap<String, store::Slots>>, mark: Mark) -> model::Trainer {
    let runner = host.runner(trainer.load());
    let mut trainer = trainer.into_model(runner);
    // A title's map flags say it too: many blueprints leave their own bits out.
    trainer.location |= mark.live;
    trainer.teleport |= mark.teleport;
    if let Some(custom) = custom {
        store::customize(&mut trainer.cheats, custom);
    }
    trainer
}

fn connect(transport: &Result<Transport, Failure>, data_dir: &Path, refresh: bool) -> Shared<BoxFuture<'static, Link>> {
    let (transport, vault, dir) = (transport.clone(), data_dir.join("session.bin"), data_dir.to_owned());
    async move {
        let transport = transport?;
        let (session, source) = session::connect(&transport, vault, refresh).await.map_err(|e| {
            let failure = Failure::from(e);
            // No Wand, no sign-in or no network is the user's setup, not a fault.
            match failure.kind {
                FailureKind::Internal => log::error!("Session: {failure}"),
                _ => log::warn!("Session: {failure}"),
            }
            failure
        })?;
        // The kept name may be another account's.
        if matches!(source, Source::Wand) {
            unblock(move || account::forget(&dir)).await;
        }
        let how = match source {
            Source::Restored => "restored",
            Source::Refreshed => "refreshed",
            Source::Wand => "new from Wand",
        };
        let days = (session.expires_in().as_secs() + 43_200) / 86_400;
        log::info!("Session: {how}, expires in {days} d");
        Ok((Arc::new(session), transport, source))
    }
    .boxed()
    .shared()
}

/// A stream of what `body` sends; polling the stream runs the body.
fn channel<T, F>(body: impl FnOnce(UnboundedSender<T>) -> F) -> BoxStream<'static, T>
where
    T: Send + 'static,
    F: Future<Output = ()> + Send + 'static,
{
    let (tx, rx) = mpsc::unbounded();
    stream::select(rx.map(Some), stream::once(body(tx)).map(|()| None)).filter_map(future::ready).boxed()
}

// The receiver is gone only when nobody listens any more.
fn send<T>(tx: &UnboundedSender<T>, item: T) {
    let _ = tx.unbounded_send(item);
}
