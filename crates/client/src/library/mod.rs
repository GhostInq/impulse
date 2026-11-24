//! The game list: every installed game with what the app shows about it.

mod find;
mod id;
mod installs;
mod land;
mod links;
mod picked;
pub mod sources;
pub mod titles;

use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    mem,
    path::Path,
    sync::Arc,
    time::Instant,
};

use model::{Installation, Offer, Origin, Ref, StoreId};

pub use find::{Find, rank};
pub use id::GameId;
pub use land::Landing;
pub use links::Links;
pub use picked::Picked;

use crate::{
    media::{self, Media},
    storage::Paths,
};

#[derive(Clone, Copy, PartialEq)]
pub enum Support {
    Ready,
    /// The installed build is newer than the trainer knows.
    UpdatePending,
    /// There is a trainer, but the game version could not be read.
    Unverified,
    NoTrainer,
}

/// What a trainer copy was asked for: it is asked for again when any of it changes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Stamp {
    /// The trainer in use, its own record's or the one the user chose.
    pub trainer: Option<Ref>,
    pub revision: Option<String>,
    /// The installed build.
    pub build: Option<u32>,
}

#[derive(Clone)]
pub struct Game {
    pub id: GameId,
    /// Upper case, as the app prints it.
    pub name: Arc<str>,
    /// Lower case, for the filter.
    pub search: String,
    /// The launcher's name in upper case; `None` for a picked executable.
    pub store: Option<String>,
    pub support: Support,
    pub mods: u32,
    pub stamp: Stamp,
    pub hero: Option<String>,
    pub thumbnail: Option<String>,
    /// As the source knows it, for asking for the trainer and the maps.
    pub source: Arc<model::Game>,
    /// The release whose trainer is used instead of its own, as the user chose (`Library::relink`).
    pub trainer_from: Option<Release>,
    /// The title's other releases with a trainer, one per trainer, other than its own's.
    pub others: Vec<Release>,
    /// The install in use: a launcher's, or an executable the user picked; `None` for a catalog game (`browse`).
    pub install: Option<Installation>,
    /// The other one, when the game has both.
    pub other: Option<Installation>,
    /// The install's page in its launcher: the launcher's name and the URI.
    pub page: Option<(&'static str, String)>,
    /// Where to get a game not installed: a store as printed and its page, one per store of the title (`browse`).
    pub stores: Vec<(String, String)>,
}

impl Game {
    pub fn installed(&self) -> bool {
        self.install.is_some()
    }

    /// The trainer in use: the chosen record's, else its own.
    pub fn offer(&self) -> Option<&Offer> {
        self.trained_by()?.trainer.as_ref()
    }

    /// The record whose trainer is in use; `None` without one.
    pub fn trained_by(&self) -> Option<&model::Game> {
        Some(self.trainer_from.as_ref().map_or(&*self.source, |from| &*from.record)).filter(|record| record.trainer.is_some())
    }

    /// Uses `from`'s trainer, or with `None` its own.
    pub fn retrain(&mut self, from: Option<Release>) {
        self.trainer_from = from;
        self.judge();
    }

    pub fn switch(&mut self) {
        if let (Some(install), Some(other)) = (&mut self.install, &mut self.other) {
            mem::swap(install, other);
            self.judge();
        }
    }

    /// Uses `picked`, an executable the user picked; the launcher's install stays as the other.
    pub fn pick(&mut self, picked: Installation) {
        let previous = [self.install.replace(picked), self.other.take()];
        self.other = previous.into_iter().flatten().find(|install| sources::exe(install).is_none());
        self.judge();
    }

    /// Forgets the picked executable. `false`: it was the only install.
    pub fn unpick(&mut self) -> bool {
        if self.install.as_ref().and_then(sources::exe).is_some() {
            let Some(launcher) = self.other.take() else { return false };
            self.install = Some(launcher);
            self.judge();
        }
        self.other = None;
        true
    }

    /// What follows from the install and the trainer in use: the store, the build's support, the mods, the stamp.
    fn judge(&mut self) {
        let build = self.install.as_ref().and_then(|install| install.build);
        let (offer, borrowed) = (self.offer(), self.trainer_from.is_some());
        let support = match (offer, build) {
            (None, _) => Support::NoTrainer,
            (Some(offer), Some(build)) if offer.builds.contains(&build) => Support::Ready,
            // Another store's builds are not an update of this one.
            (Some(_), Some(_)) if !borrowed => Support::UpdatePending,
            (Some(_), _) => Support::Unverified,
        };
        let mods = offer.map_or(0, |offer| offer.cheats);
        self.stamp =
            Stamp { trainer: offer.map(|offer| offer.id.clone()), revision: offer.and_then(|offer| offer.revision.clone()), build };
        (self.support, self.mods) = (support, mods);
        let origin = self.install.as_ref().map(|install| &install.origin);
        self.store = origin.map_or_else(|| Some(store_of(&self.source)), store);
        self.page = origin.and_then(Origin::store_id).and_then(launchers::page);
    }
}

pub struct Library {
    /// Shared, so a view holds one while it draws.
    pub games: Vec<Arc<Game>>,
    /// Titles with a trainer among all the source's games.
    pub trainers: usize,
    /// The source's games the list was made of: where a game may take another record's trainer from.
    pub catalog: Arc<Vec<model::Game>>,
}

/// Another record of a game's title, whose trainer the game may use instead of its own.
#[derive(Clone)]
pub struct Release {
    pub record: Arc<model::Game>,
    /// Its store, as the app prints it.
    pub store: Arc<str>,
}

impl Release {
    fn of(record: &model::Game) -> Self {
        Self { store: store_of(record).into(), record: Arc::new(record.clone()) }
    }
}

impl Library {
    /// Gives each game the trainer its `link` chose; a link to a record gone, without a trainer or its own means its own.
    pub fn relink(&mut self, link: impl Fn(&GameId) -> Option<Ref>) {
        let catalog = &self.catalog;
        for game in &mut self.games {
            let chosen = link(&game.id).filter(|id| *id != game.source.id).and_then(|id| trainer_record(catalog, &id));
            if game.trainer_from.as_ref().map(|from| &from.record.id) != chosen.map(|record| &record.id) {
                Arc::make_mut(game).retrain(chosen.map(Release::of));
            }
        }
    }

    /// The catalog's records with a trainer, by index, in the trainer picker's order, and how many lead it as `game`'s
    /// own: its title's and the one in use, its install's other records, those named alike. The rest follow by likeness.
    pub fn trainer_records(&self, game: &Game) -> (Vec<usize>, usize) {
        let with: Vec<usize> = (0..self.catalog.len()).filter(|&at| self.catalog[at].trainer.is_some()).collect();
        let likest = titles::likeness(&game.name, with.iter().map(|&at| self.catalog[at].name.as_str())).into_iter().map(|at| with[at]);
        let (same, in_use) = (one_install(game), game.trained_by());
        let group = |record: &model::Game| {
            let title = record.title_id == game.source.title_id || in_use.is_some_and(|used| used.id == record.id);
            [title, same(record), titles::alike(&record.name, &game.source.name)].iter().position(|&is| is).unwrap_or(3)
        };
        let mut order: Vec<(usize, usize)> = likest.map(|at| (group(&self.catalog[at]), at)).collect();
        order.sort_by_key(|&(group, _)| group);
        let own = order.iter().take_while(|&&(group, _)| group < 3).count();
        (order.into_iter().map(|(_, at)| at).collect(), own)
    }
}

fn trainer_record<'a>(catalog: &'a [model::Game], id: &Ref) -> Option<&'a model::Game> {
    catalog.iter().find(|record| record.id == *id && record.trainer.is_some())
}

/// Whether a record is one install with `game`'s launcher install: the scan listed `game` for it.
fn one_install(game: &Game) -> impl Fn(&model::Game) -> bool + '_ {
    let store_id = [&game.install, &game.other].into_iter().flatten().find_map(|install| install.origin.store_id());
    move |record| store_id.is_some_and(|id| launchers::same_install(record, &game.source, id))
}

fn trainer_id(record: &model::Game) -> Option<&Ref> {
    record.trainer.as_ref().map(|offer| &offer.id)
}

/// Lower-case names in the list's order: a leading "the" is skipped, as Steam's library does.
pub fn by_name(a: &str, b: &str) -> Ordering {
    let (x, y) = (a.strip_prefix("the ").unwrap_or(a), b.strip_prefix("the ").unwrap_or(b));
    x.cmp(y).then(a.cmp(b))
}

/// A record's first store in upper case, as the app prints it.
pub fn store_of(record: &model::Game) -> String {
    record.store_ids.first().map_or(String::new(), |id| launchers::store_name(id.store()))
}

/// The list the last scan's installs make, shown while the launchers are read again; `None` before a first scan
/// (blocking). A kept picked executable counts only while it is still the picked one.
pub fn kept(all: &Arc<Vec<model::Game>>, picked: &Picked, paths: &Paths) -> Option<Library> {
    let custom = picked.custom();
    let still = |install: &Installation| match &install.origin {
        Origin::Picked(exe) => custom.iter().any(|picked| picked.game == install.game && picked.exe == *exe),
        Origin::Launcher(_) => true,
    };
    let kept: Vec<Installation> = installs::kept(paths).into_iter().filter(still).collect();
    (!kept.is_empty()).then(|| list(all, kept, picked, paths))
}

/// Reads the launchers and the picked executables now and keeps their installs for the next start (blocking;
/// minutes on a cold disk with hundreds of games).
pub fn scan(all: &Arc<Vec<model::Game>>, picked: &Picked, paths: &Paths) -> Library {
    let installed = installed(all, picked);
    installs::keep(paths, &installed);
    list(all, installed, picked, paths)
}

fn installed(all: &[model::Game], picked: &Picked) -> Vec<Installation> {
    let started = Instant::now();
    let installed = launchers::scan(all, &picked.custom());
    let mut per_store = BTreeMap::<Option<String>, usize>::new();
    for found in &installed {
        *per_store.entry(store(&found.origin)).or_default() += 1;
        let build = found.build.map_or("?".to_owned(), |v| v.to_string());
        log::debug!("{} [{:?}] v{build} @ {}", found.game, found.origin, found.location.display());
    }
    let stores: Vec<String> = per_store.iter().map(|(s, n)| format!("{} {n}", s.as_deref().unwrap_or("CUSTOM"))).collect();
    log::info!("Library: {} installed in {:?}: {}", installed.len(), started.elapsed(), stores.join(", "));
    installed
}

/// The list `installed` makes with `all`; art known to be missing is a file per game.
fn list(all: &Arc<Vec<model::Game>>, installed: Vec<Installation>, picked: &Picked, paths: &Paths) -> Library {
    let art_cache = &Media::Pictures.dir(paths);
    // Steam art also serves an Epic or GOG install when the title has a Steam release.
    let mut steam_apps = HashMap::<&str, u32>::new();
    let mut with_trainer = HashMap::<&str, Vec<usize>>::new();
    for (at, game) in all.iter().enumerate() {
        if game.trainer.is_some() {
            with_trainer.entry(&game.title_id).or_default().push(at);
        }
        if let Some(app) = game.store_ids.iter().find_map(StoreId::steam_app) {
            steam_apps.entry(&game.title_id).or_insert(app);
        }
    }
    let by_id: HashMap<&Ref, &model::Game> = all.iter().map(|game| (&game.id, game)).collect();
    let mut games = Vec::<Game>::new();
    let mut at = HashMap::<Ref, usize>::new();
    for found in installed {
        // A launcher's install comes first, a picked executable after it.
        if let Some(&index) = at.get(&found.game) {
            let game = &mut games[index];
            game.other = Some(found);
            if picked.used(&game.id) {
                game.switch();
            }
        } else if let Some(&entry) = by_id.get(&found.game) {
            at.insert(found.game.clone(), games.len());
            let title_app = steam_apps.get(entry.title_id.as_str()).copied();
            let mut listed = game(entry.clone(), Some(found), title_app, art_cache);
            for &other in with_trainer.get(entry.title_id.as_str()).map_or(&[][..], Vec::as_slice) {
                let trainer = trainer_id(&all[other]);
                if trainer != trainer_id(entry) && listed.others.iter().all(|seen| trainer_id(&seen.record) != trainer) {
                    listed.others.push(Release::of(&all[other]));
                }
            }
            games.push(listed);
        }
    }
    games.sort_by(|a, b| by_name(&a.search, &b.search));
    Library { games: games.into_iter().map(Arc::new).collect(), trainers: with_trainer.len(), catalog: all.clone() }
}

/// The catalog record `id` as a game not installed, with its title's stores (blocking: the art known to be missing is
/// a file per game, the store apps are in the registry); `None` once the record is gone.
pub fn browse(all: &[model::Game], id: &Ref, paths: &Paths) -> Option<Game> {
    let entry = all.iter().find(|record| record.id == *id)?;
    let siblings = || all.iter().filter(|record| record.title_id == entry.title_id);
    let title_app = siblings().flat_map(|record| &record.store_ids).find_map(StoreId::steam_app);
    let mut game = game(entry.clone(), None, title_app, &Media::Pictures.dir(paths));
    for record in siblings() {
        let store = store_of(record);
        if let Some(link) = launchers::store_link(&record.purchase).filter(|_| game.stores.iter().all(|(seen, _)| *seen != store)) {
            game.stores.push((store, link.to_owned()));
        }
    }
    Some(game)
}

/// `title_app`: the Steam app of the title, whose art serves any of its records.
fn game(entry: model::Game, install: Option<Installation>, title_app: Option<u32>, art_cache: &Path) -> Game {
    let app = install.as_ref().and_then(|install| install.origin.store_id()).and_then(StoreId::steam_app).or(title_app);
    let mut game = Game {
        id: GameId::of(&entry.id),
        name: entry.name.to_uppercase().into(),
        search: entry.name.to_lowercase(),
        store: None,
        support: Support::NoTrainer,
        mods: 0,
        stamp: Stamp::default(),
        // Not every Steam app has hero art.
        hero: app.map(media::steam_hero).filter(|url| !media::known_missing(art_cache, url)).or_else(|| entry.cover.clone()),
        thumbnail: entry.thumbnail.clone(),
        source: Arc::new(entry),
        trainer_from: None,
        others: Vec::new(),
        install,
        other: None,
        page: None,
        stores: Vec::new(),
    };
    game.judge();
    game
}

/// The launcher's name; `None` for a picked executable.
fn store(origin: &Origin) -> Option<String> {
    origin.store_id().map(|id| launchers::store_name(id.store()))
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(super) fn record(id: &str, title: &str, store: &str, builds: Option<&[u32]>) -> model::Game {
        let id = Ref::parse(id).unwrap();
        let trainer = builds.map(|builds| Offer { id: id.clone(), cheats: 24, builds: builds.to_vec(), revision: Some("r".into()) });
        model::Game {
            id,
            title_id: title.into(),
            name: "Chained Echoes".into(),
            terms: Vec::new(),
            store_ids: vec![StoreId::parse(store).unwrap()],
            exe: None,
            purchase: Vec::new(),
            standalone: Vec::new(),
            package_apps: Vec::new(),
            cover: None,
            thumbnail: None,
            maps: Vec::new(),
            trainer,
        }
    }

    pub(super) fn paths() -> Paths {
        Paths::new(std::env::temp_dir().join(format!("impulse-library-{}", std::process::id())))
    }

    /// The first record, installed from GOG at `build`.
    fn listed(catalog: Vec<model::Game>, build: Option<u32>) -> Library {
        let paths = paths();
        let origin = Origin::Launcher(StoreId::parse("gog:1").unwrap());
        let install = Installation {
            game: catalog[0].id.clone(),
            origin,
            location: "c:/games/echoes".into(),
            alternates: Vec::new(),
            build,
            app: None,
        };
        list(&Arc::new(catalog), vec![install], &Picked::load(&paths), &paths)
    }

    fn catalog() -> Vec<model::Game> {
        // An Xbox release with the Steam release's trainer.
        let mut shared = record("wand:5", "t", "xbox:5", Some(&[5]));
        shared.trainer.as_mut().unwrap().id = Ref::parse("wand:2").unwrap();
        vec![
            record("wand:1", "t", "gog:1", None),
            record("wand:2", "t", "steam:2", Some(&[5])),
            record("wand:3", "other", "steam:3", Some(&[5])),
            record("wand:4", "t", "epic:4", None),
            shared,
        ]
    }

    /// The listed game's link.
    fn link(to: &'static str) -> impl Fn(&GameId) -> Option<Ref> {
        move |_| Ref::parse(to)
    }

    #[test]
    fn a_game_without_a_trainer_offers_one_release_per_trainer_of_its_title() {
        let list = listed(catalog(), Some(7));
        let game = &list.games[0];
        assert!(game.support == Support::NoTrainer && game.offer().is_none());
        let others: Vec<_> = game.others.iter().map(|other| (other.record.id.to_string(), &*other.store)).collect();
        assert_eq!(others, [("wand:2".to_owned(), "STEAM")]);
    }

    #[test]
    fn a_link_lends_another_records_trainer_and_the_game_keeps_its_own_install() {
        let mut list = listed(catalog(), Some(7));
        list.relink(link("wand:2"));
        let game = &list.games[0];
        assert_eq!(game.offer().map(|offer| offer.id.to_string()).as_deref(), Some("wand:2"));
        assert_eq!((game.id.as_str(), game.store.as_deref(), game.mods), ("wand:1", Some("GOG"), 24));
        assert!(game.support == Support::Unverified, "another store's build is not an update");
        assert_eq!(game.stamp.trainer.as_ref().map(Ref::to_string).as_deref(), Some("wand:2"));

        let mut fits = listed(catalog(), Some(5));
        fits.relink(link("wand:2"));
        assert!(fits.games[0].support == Support::Ready);
        fits.relink(|_| None);
        assert!(fits.games[0].support == Support::NoTrainer && fits.games[0].trainer_from.is_none());
    }

    #[test]
    fn a_link_to_a_record_gone_without_a_trainer_or_its_own_means_its_own() {
        for to in ["wand:9", "wand:4", "wand:1"] {
            let mut list = listed(catalog(), Some(7));
            list.relink(link(to));
            assert!(list.games[0].trainer_from.is_none() && list.games[0].support == Support::NoTrainer, "{to}");
        }
    }

    #[test]
    fn a_catalog_game_has_no_install_and_its_trainer_is_unverified() {
        let mut all = catalog();
        all[0].purchase = vec!["https://www.gog.com/game/echoes".into()];
        all[1].purchase = vec!["steam://run/2".into(), "https://store.steampowered.com/app/2".into()];
        let game = browse(&all, &Ref::parse("wand:2").unwrap(), &paths()).unwrap();
        assert!(!game.installed() && game.support == Support::Unverified && game.stamp.build.is_none() && game.page.is_none());
        assert_eq!((game.id.as_str(), game.store.as_deref(), game.mods), ("wand:2", Some("STEAM"), 24));
        let stores: Vec<_> = game.stores.iter().map(|(store, link)| (store.as_str(), link.as_str())).collect();
        assert_eq!(stores, [("GOG", "https://www.gog.com/game/echoes"), ("STEAM", "https://store.steampowered.com/app/2")]);
        assert!(browse(&catalog(), &Ref::parse("wand:9").unwrap(), &paths()).is_none());
    }

    #[test]
    fn the_search_holds_the_listed_games_and_a_record_per_other_title_with_a_trainer_steams_first() {
        let mut all = catalog();
        all[0].exe = Some("echoes.exe".into());
        all.extend([record("wand:6", "x", "epic:6", Some(&[1])), record("wand:7", "x", "steam:7", Some(&[1]))]);
        // Another title's record of the listed install, with its files: the listed game stands for that title.
        let edition =
            model::Game { name: "Chained Echoes: Deluxe".into(), ..with_exe(record("wand:8", "dx", "gog:1", Some(&[1])), "Echoes.exe") };
        all.extend([edition, record("wand:9", "dx", "steam:9", Some(&[1]))]);
        // A collection's other game shares the store id, not the files.
        all.push(with_exe(record("wand:10", "pack", "gog:1", Some(&[1])), "other.exe"));
        let list = listed(all, None);
        let finds: Vec<_> = list.finds().iter().map(|find| (list.catalog[find.at].id.to_string(), find.listed)).collect();
        let row = |id: &str, listed| (id.to_owned(), listed);
        assert_eq!(finds, [row("wand:1", true), row("wand:3", false), row("wand:7", false), row("wand:10", false)]);
        assert_eq!(rank(&list.finds(), "deluxe"), [0], "the edition's name finds the install standing for it");
    }

    #[test]
    fn the_trainer_picker_opens_on_the_games_own_records_its_titles_its_installs_records_then_alike_names() {
        let mut all = catalog();
        all[0].exe = Some("echoes.exe".into());
        // A collection's other game lists the same store id with its own version file: not the game's own.
        let collected = model::Game { name: "Another Game".into(), ..with_exe(record("wand:9", "c", "gog:1", Some(&[1])), "other.exe") };
        let unrelated = model::Game { name: "007 First Light".into(), ..record("wand:10", "z", "steam:10", Some(&[1])) };
        all.extend([with_exe(record("wand:8", "dx", "gog:1", Some(&[1])), "echoes.exe"), collected, unrelated]);
        let mut list = listed(all, None);
        let (order, own) = list.trainer_records(&list.games[0]);
        assert_eq!((&order[..own], order.len()), (&[1, 4, 5, 2][..], 6));
        // The trainer in use is the game's own, whatever its name.
        list.relink(link("wand:10"));
        let (order, own) = list.trainer_records(&list.games[0]);
        assert_eq!(&order[..own], [1, 4, 7, 5, 2]);
    }

    fn with_exe(mut record: model::Game, exe: &str) -> model::Game {
        record.exe = Some(exe.into());
        record
    }
}
