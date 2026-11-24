//! Which discovered title is a library game's: by store id, never by name; names only order the picker and say which
//! records it opens on.

use std::{cmp::Reverse, collections::HashSet};

use model::{StoreId, Title};

use super::Game;

/// The first title sharing a store id with the game's record or its active install. Another release of the title
/// (a sibling record) does not count.
pub fn matched<'a>(game: &Game, titles: &'a [Title]) -> Option<&'a Title> {
    let keys: Vec<&StoreId> =
        game.source.store_ids.iter().chain(game.install.as_ref().and_then(|install| install.origin.store_id())).collect();
    titles.iter().find(|title| title.stores.iter().any(|store| keys.contains(&store)))
}

/// A picker's order of `names` for a game named `name`, by index: the same name first, then a name holding the other,
/// then by the words they share; ties by name.
pub fn likeness<'a>(name: &str, names: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    let wanted = plain(name);
    let words: HashSet<&str> = wanted.split_whitespace().collect();
    let mut order: Vec<(u8, Reverse<usize>, String, usize)> = names
        .into_iter()
        .enumerate()
        .map(|(at, name)| {
            let have = plain(name);
            let rank = if have == wanted {
                0
            } else if !have.is_empty() && (have.contains(&wanted) || wanted.contains(&have)) {
                1
            } else {
                2
            };
            let shared = have.split_whitespace().filter(|word| words.contains(word)).count();
            (rank, Reverse(shared), have, at)
        })
        .collect();
    order.sort();
    order.into_iter().map(|(.., at)| at).collect()
}

/// Words that name an edition rather than a game, as they end a name ("Definitive Edition", "Game of the Year").
const EDITION_WORDS: &[&str] = &[
    "anniversary",
    "collector",
    "collectors",
    "complete",
    "cut",
    "definitive",
    "deluxe",
    "digital",
    "director",
    "directors",
    "edition",
    "enhanced",
    "extended",
    "game",
    "gold",
    "goty",
    "hd",
    "legendary",
    "of",
    "premium",
    "remaster",
    "remastered",
    "special",
    "standard",
    "the",
    "ultimate",
    "year",
];

/// Whether two names are one game's: the same once a trailing edition is dropped, as "Mafia III" and "Mafia III:
/// Definitive Edition" are; never "Mafia" or "Mafia II" for those.
pub fn alike(a: &str, b: &str) -> bool {
    let (a, b) = (game_words(a), game_words(b));
    !a.is_empty() && a == b
}

/// A name's words without its trailing edition words; all of them when nothing else is left.
fn game_words(name: &str) -> Vec<String> {
    let mut words: Vec<String> = plain(name).split_whitespace().map(str::to_owned).collect();
    let edition = words.iter().rev().take_while(|word| EDITION_WORDS.contains(&word.as_str())).count();
    if edition < words.len() {
        words.truncate(words.len() - edition);
    }
    words
}

/// Letters, digits and spaces, in lower case.
fn plain(text: &str) -> String {
    text.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect::<String>().to_lowercase()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use model::{Installation, Origin, Ref};

    use super::*;

    fn title(id: &str, name: &str, stores: &[&str]) -> Title {
        let stores = stores.iter().map(|store| StoreId::parse(store).unwrap()).collect();
        Title { id: Ref::parse(&format!("plugin-maps:{id}")).unwrap(), name: name.into(), stores }
    }

    fn game(stores: &[&str], installed: &str) -> Game {
        let id = Ref::parse("wand:1").unwrap();
        let record = model::Game {
            id: id.clone(),
            title_id: "t".into(),
            name: "Night City".into(),
            terms: Vec::new(),
            store_ids: stores.iter().map(|store| StoreId::parse(store).unwrap()).collect(),
            exe: None,
            purchase: Vec::new(),
            standalone: Vec::new(),
            package_apps: Vec::new(),
            cover: None,
            thumbnail: None,
            maps: Vec::new(),
            trainer: None,
        };
        let origin = Origin::Launcher(StoreId::parse(installed).unwrap());
        let install = Installation { game: id, origin, location: "c:/games".into(), alternates: Vec::new(), build: None, app: None };
        super::super::game(record, Some(install), None, Path::new("."))
    }

    #[test]
    fn a_title_matches_by_the_records_or_the_active_installs_store_id_only() {
        let titles = [title("a", "Night City", &["steam:1"]), title("b", "Other", &["epic:2"]), title("c", "Gog", &["gog:3"])];
        let found = |game: &Game| matched(game, &titles).map(|title| title.id.id().to_owned());
        assert_eq!(found(&game(&["steam:1"], "steam:9")).as_deref(), Some("a"), "the record's store id");
        assert_eq!(found(&game(&["steam:9"], "epic:2")).as_deref(), Some("b"), "the active install's");
        assert_eq!(found(&game(&["steam:9"], "steam:9")), None, "a name alone never matches");
    }

    #[test]
    fn the_picker_puts_the_same_name_first_then_a_name_holding_it_then_shared_words() {
        let titles =
            [title("a", "Far Cry", &[]), title("b", "Night City Wire", &[]), title("c", "Night City", &[]), title("d", "City Lights", &[])];
        let order: Vec<_> = likeness("NIGHT CITY", titles.iter().map(|title| title.name.as_str()))
            .into_iter()
            .map(|at| titles[at].id.id().to_owned())
            .collect();
        assert_eq!(order, ["c", "b", "d", "a"]);
    }

    #[test]
    fn names_are_alike_when_they_are_the_same_but_for_a_trailing_edition() {
        assert!(alike("MAFIA III", "Mafia III: Definitive Edition") && alike("Mafia III: Definitive Edition", "mafia iii"));
        assert!(alike("The Witcher 3: Wild Hunt", "The Witcher 3: Wild Hunt - Game of the Year Edition"));
        assert!(!alike("Mafia", "Mafia III: Definitive Edition") && !alike("Mafia III", "Mafia II") && !alike("Doom", "Doom Eternal"));
        assert!(!alike("Dreadmoor", "007 First Light") && !alike("Rage", "Courage"));
        assert!(!alike("", "Dreadmoor") && !alike("Edition", "Mafia III: Definitive Edition"));
    }
}
