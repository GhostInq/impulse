use super::*;

const APP: &[(&str, &str)] = &[
    ("en", "title = SETTINGS\nplayed = { $hours ->\n    [one] { $hours } HOUR\n   *[other] { $hours } HOURS\n}\n"),
    ("xx", "title = REGLAGES\n"),
];
const XX: Language = Language { tag: "xx", name: "XX" };

#[test]
fn a_message_falls_back_to_english_then_to_its_key() {
    let catalog = Catalog::new(XX, false, [("app", APP)]);
    assert_eq!(catalog.text("app", "title", &[]), "REGLAGES");
    assert_eq!(catalog.text("app", "played", &[("hours", 3u32.into())]), "3 HOURS");
    assert_eq!(catalog.text("app", "gone", &[]), "gone");
    assert_eq!(catalog.text("other", "title", &[]), "title");
}

#[test]
fn a_message_that_cannot_format_falls_back_to_english_then_to_its_key() {
    const LEVEL: &[(&str, &str)] = &[("en", "level = Level { $name }\n"), ("xx", "level = Stufe { $name }\n")];
    let catalog = Catalog::new(XX, false, [("app", LEVEL)]);
    assert_eq!(catalog.text("app", "level", &[("name", "Ann".into())]), "Stufe Ann");
    assert_eq!(catalog.text("app", "level", &[("other", "Ann".into())]), "level");
    const PARTIAL: &[(&str, &str)] = &[("en", "level = Level\n"), ("xx", "level = Stufe { $name }\n")];
    let catalog = Catalog::new(XX, false, [("app", PARTIAL)]);
    assert_eq!(catalog.text("app", "level", &[]), "Level");
}

#[test]
fn plurals_follow_the_number() {
    let catalog = Catalog::new(ENGLISH, false, [("app", APP)]);
    assert_eq!(catalog.text("app", "played", &[("hours", 1u32.into())]), "1 HOUR");
    assert_eq!(catalog.text("app", "played", &[("hours", 1.5.into())]), "1.5 HOURS");
}

#[test]
fn french_plurals_count_zero_as_one() {
    const MODS: &[(&str, &str)] = &[("fr", "mods = { $count ->\n    [one] { $count } MOD\n   *[other] { $count } MODS\n}\n")];
    let catalog = Catalog::new(FRENCH, false, [("app", MODS)]);
    assert_eq!(catalog.text("app", "mods", &[("count", 0u32.into())]), "0 MOD");
    assert_eq!(catalog.text("app", "mods", &[("count", 2u32.into())]), "2 MODS");
}

#[test]
fn the_pseudo_locale_stretches_the_message_but_not_its_arguments() {
    const HELLO: &[(&str, &str)] = &[("en", "hello = Hi { $name }!\n")];
    let catalog = Catalog::new(ENGLISH, true, [("app", HELLO)]);
    assert_eq!(catalog.text("app", "hello", &[("name", "Ann".into())]), "[Hî ~]Ann[!]");
}

#[test]
fn a_system_tag_matches_ours_by_language() {
    assert_eq!(closest("en-GB"), Some(ENGLISH));
    assert_eq!(closest("EN_us"), Some(ENGLISH));
    assert_eq!(closest("zh"), Some(CHINESE));
    assert_eq!(closest("zh-CN"), Some(CHINESE));
    assert_eq!(closest("zh-Hans"), Some(CHINESE));
    assert_eq!(closest("zh-SG"), Some(CHINESE));
    assert_eq!(closest("zh-TW"), Some(CHINESE_TRADITIONAL));
    assert_eq!(closest("zh_TW"), Some(CHINESE_TRADITIONAL));
    assert_eq!(closest("zh-Hant"), Some(CHINESE_TRADITIONAL));
    assert_eq!(closest("zh-Hant-TW"), Some(CHINESE_TRADITIONAL));
    assert_eq!(closest("zh-HK"), Some(CHINESE_TRADITIONAL));
    assert_eq!(closest("zh-MO"), Some(CHINESE_TRADITIONAL));
    assert_eq!(closest("fr-CA"), Some(FRENCH));
    assert_eq!(closest("de-AT"), Some(GERMAN));
    assert_eq!(closest("es-MX"), Some(SPANISH));
    assert_eq!(closest("it-CH"), Some(ITALIAN));
    assert_eq!(closest("pt-PT"), Some(PORTUGUESE));
    assert_eq!(closest("pl-PL"), Some(POLISH));
    assert_eq!(closest("tr-TR"), Some(TURKISH));
    assert_eq!(closest("ru-RU"), Some(RUSSIAN));
    assert_eq!(closest("uk-UA"), Some(UKRAINIAN));
    assert_eq!(closest("th-TH"), Some(THAI));
    assert_eq!(closest("xx-YY"), None);
}

#[test]
fn only_the_cyrillic_languages_need_another_font() {
    let cyrillic: Vec<_> = LANGUAGES.iter().filter(|language| language.is_cyrillic()).map(|language| language.tag).collect();
    assert_eq!(cyrillic, ["ru", "uk"]);
    let cjk: Vec<_> = LANGUAGES.iter().filter(|language| language.is_cjk()).map(|language| language.tag).collect();
    assert_eq!(cjk, ["zh-CN", "zh-TW"]);
}

#[test]
fn the_panel_ships_every_language() {
    let folder = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../web/panel/locales");
    let tag = |entry: std::fs::DirEntry| entry.file_name().to_str()?.strip_suffix(".ftl").map(str::to_owned);
    let mut shipped: Vec<_> = std::fs::read_dir(folder).expect("web/panel/locales").flatten().filter_map(tag).collect();
    shipped.sort();
    let mut ours: Vec<_> = LANGUAGES.iter().map(|language| language.tag.to_owned()).collect();
    ours.sort();
    assert_eq!(shipped, ours);
}
