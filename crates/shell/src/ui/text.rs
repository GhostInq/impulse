//! The app's texts in the user's language: the `Texts` global over the catalogs, set by `Settings`.

use std::{
    cell::RefCell,
    env,
    hash::{Hash, Hasher},
    sync::{Arc, LazyLock},
};

use gpui_kit::{App, Global, SharedString};
use i18n::{
    Arg, CHINESE, CHINESE_TRADITIONAL, Catalog, ENGLISH, FRENCH, GERMAN, ITALIAN, Key, Language, POLISH, PORTUGUESE, RUSSIAN, SPANISH,
    THAI, TURKISH, UKRAINIAN,
};
use model::{Failure, FailureKind, Files, Locale, SourceId, Text};
use rustc_hash::{FxHashMap, FxHasher};

use super::key;

const APP: &str = SourceId::APP;
static FILES: LazyLock<Files> = LazyLock::new(|| {
    [
        (ENGLISH, include_str!("../../locales/en.ftl")),
        (CHINESE, include_str!("../../locales/zh-CN.ftl")),
        (CHINESE_TRADITIONAL, include_str!("../../locales/zh-TW.ftl")),
        (FRENCH, include_str!("../../locales/fr.ftl")),
        (GERMAN, include_str!("../../locales/de.ftl")),
        (SPANISH, include_str!("../../locales/es.ftl")),
        (ITALIAN, include_str!("../../locales/it.ftl")),
        (PORTUGUESE, include_str!("../../locales/pt-BR.ftl")),
        (POLISH, include_str!("../../locales/pl.ftl")),
        (TURKISH, include_str!("../../locales/tr.ftl")),
        (RUSSIAN, include_str!("../../locales/ru.ftl")),
        (UKRAINIAN, include_str!("../../locales/uk.ftl")),
        (THAI, include_str!("../../locales/th.ftl")),
    ]
    .iter()
    .map(|&(language, source)| (language.tag.into(), source.into()))
    .collect()
});

pub struct Texts {
    catalog: Catalog,
    // What sources are asked in: the language's tag.
    locale: Locale,
    // Messages without arguments by domain, each made once per language.
    made: RefCell<FxHashMap<String, FxHashMap<Arc<str>, SharedString>>>,
    // Messages with arguments by a hash of key and arguments, checked against the copy kept, so a redraw formats nothing.
    formatted: RefCell<FxHashMap<u64, (Made, SharedString)>>,
}

/// A message's key and arguments, owned.
#[derive(PartialEq)]
struct Made {
    key: &'static str,
    args: Vec<(String, Owned)>,
}

#[derive(PartialEq)]
enum Owned {
    Str(String),
    Num(u64),
}

/// Bounds the formatted messages kept; past it they are made again.
const FORMATTED: usize = 1024;

impl Global for Texts {}

impl Texts {
    /// The app's catalog, then each source's under its id.
    pub fn new(language: Language, sources: &[(SourceId, Files)]) -> Self {
        // `IMPULSE_LOCALE=pseudo` in a debug build shows cuts and English left in the code.
        let pseudo = cfg!(debug_assertions) && env::var_os("IMPULSE_LOCALE").is_some_and(|value| value == "pseudo");
        let domains = [(APP, &FILES[..])].into_iter().chain(sources.iter().map(|(id, files)| (id.as_str(), &files[..])));
        let (locale, made, formatted) = (Locale::new(language.tag), RefCell::default(), RefCell::default());
        Self { catalog: Catalog::new(language, pseudo, domains), locale, made, formatted }
    }

    pub fn language(&self) -> Language {
        self.catalog.language()
    }

    fn made(&self, domain: &str, key: &str) -> SharedString {
        let mut made = self.made.borrow_mut();
        if let Some(text) = made.get(domain).and_then(|keys| keys.get(key)) {
            return text.clone();
        }
        let text: SharedString = self.catalog.text(domain, key, &[]).into();
        made.entry(domain.to_owned()).or_default().insert(key.into(), text.clone());
        text
    }

    fn formatted(&self, key: Key, args: &[(&str, Arg)]) -> SharedString {
        let mut hasher = FxHasher::default();
        key.0.hash(&mut hasher);
        for (name, arg) in args {
            name.hash(&mut hasher);
            match arg {
                Arg::Str(text) => text.hash(&mut hasher),
                Arg::Num(number) => number.to_bits().hash(&mut hasher),
            }
        }
        let hash = hasher.finish();
        let same = |made: &Made| {
            made.key == key.0
                && made.args.len() == args.len()
                && made.args.iter().zip(args).all(|((name, kept), (given, arg))| {
                    name == given
                        && match (kept, arg) {
                            (Owned::Str(kept), Arg::Str(text)) => kept == text,
                            (Owned::Num(kept), Arg::Num(number)) => *kept == number.to_bits(),
                            _ => false,
                        }
                })
        };
        let mut formatted = self.formatted.borrow_mut();
        if let Some((_, text)) = formatted.get(&hash).filter(|(made, _)| same(made)) {
            return text.clone();
        }
        if formatted.len() >= FORMATTED {
            formatted.clear();
        }
        let text: SharedString = self.catalog.text(APP, key.0, args).into();
        let owned = |arg: &Arg| match arg {
            Arg::Str(text) => Owned::Str((*text).to_owned()),
            Arg::Num(number) => Owned::Num(number.to_bits()),
        };
        let made = Made { key: key.0, args: args.iter().map(|(name, arg)| ((*name).to_owned(), owned(arg))).collect() };
        formatted.insert(hash, (made, text.clone()));
        text
    }

    fn digits(&self, printed: &str) -> String {
        let (group, decimal) = (self.made(APP, key::NUMBER_GROUP.0), self.made(APP, key::NUMBER_DECIMAL.0));
        let mut out = String::with_capacity(printed.len() + 2);
        for mark in printed.chars() {
            match mark {
                ',' => out.push_str(&group),
                '.' => out.push_str(&decimal),
                digit => out.push(digit),
            }
        }
        out
    }
}

/// The language content is asked in.
pub fn content_locale(cx: &App) -> Locale {
    cx.global::<Texts>().locale.clone()
}

pub fn t(cx: &App, key: Key) -> SharedString {
    cx.global::<Texts>().made(APP, key.0)
}

/// A message with arguments, formatted once per distinct arguments.
pub fn tf(cx: &App, key: Key, args: &[(&str, Arg)]) -> SharedString {
    cx.global::<Texts>().formatted(key, args)
}

/// A number as Rust prints it (`,` between thousands, `.` before decimals) with the language's marks.
pub fn digits(cx: &App, printed: &str) -> String {
    cx.global::<Texts>().digits(printed)
}

/// The app's own message as a `Text`, for what a source would say.
pub fn app_text(key: Key) -> Text {
    Text::key(&SourceId::app(), key.0)
}

pub fn text(cx: &App, text: &Text) -> SharedString {
    match text {
        Text::Plain(plain) => plain.clone().into(),
        Text::Key { domain, key, args } if args.is_empty() => cx.global::<Texts>().made(domain.as_str(), key),
        Text::Key { domain, key, args } => {
            let args: Vec<(&str, Arg)> = args
                .iter()
                .map(|(name, arg)| match arg {
                    model::Arg::Str(value) => (&**name, Arg::Str(value)),
                    model::Arg::Num(value) => (&**name, Arg::Num(*value)),
                })
                .collect();
            cx.global::<Texts>().catalog.text(domain.as_str(), key, &args).into()
        }
    }
}

/// A message whose `why` is the failure, worded.
pub fn failed(cx: &App, key: Key, failure: &Failure) -> SharedString {
    tf(cx, key, &[("why", Arg::Str(&why(cx, failure)))])
}

/// What went wrong: the source's words, else its kind's.
pub fn said(cx: &App, failure: &Failure) -> SharedString {
    failure.message.as_ref().map_or_else(|| t(cx, kind(failure.kind)), |message| text(cx, message))
}

/// What went wrong and, for a missing setting, where to fix it.
pub fn why(cx: &App, failure: &Failure) -> SharedString {
    match (failure.kind, &failure.message) {
        (FailureKind::SetupNeeded, Some(_)) => tf(cx, key::FAILURE_PLACE, &[("why", Arg::Str(&said(cx, failure)))]),
        _ => said(cx, failure),
    }
}

fn kind(kind: FailureKind) -> Key {
    match kind {
        FailureKind::Offline => key::FAILURE_OFFLINE,
        FailureKind::SignedOut => key::FAILURE_SIGNED_OUT,
        FailureKind::NotFound => key::FAILURE_NOT_FOUND,
        FailureKind::Unsupported => key::FAILURE_UNSUPPORTED,
        FailureKind::SetupNeeded => key::FAILURE_SETUP_NEEDED,
        FailureKind::GameNotFound => key::FAILURE_GAME_NOT_FOUND,
        FailureKind::LaunchFailed => key::FAILURE_LAUNCH_FAILED,
        FailureKind::NotStarted => key::FAILURE_NOT_STARTED,
        FailureKind::NotRunning => key::FAILURE_NOT_RUNNING,
        FailureKind::ExternalLaunch => key::FAILURE_EXTERNAL_LAUNCH,
        FailureKind::GameExited => key::FAILURE_GAME_EXITED,
        FailureKind::Refused => key::FAILURE_REFUSED,
        FailureKind::Elevation => key::FAILURE_ELEVATION,
        FailureKind::Protected => key::FAILURE_PROTECTED,
        FailureKind::Architecture { needs_x64: true } => key::FAILURE_NEEDS_64,
        FailureKind::Architecture { needs_x64: false } => key::FAILURE_NEEDS_32,
        FailureKind::LoadFailed => key::FAILURE_LOAD_FAILED,
        FailureKind::TrainerFailed => key::FAILURE_TRAINER_FAILED,
        FailureKind::Restart => key::FAILURE_RESTART,
        FailureKind::Internal => key::FAILURE_INTERNAL,
    }
}

#[cfg(test)]
mod tests {
    use i18n::{
        Arg, CHINESE, CHINESE_TRADITIONAL, ENGLISH, FRENCH, GERMAN, ITALIAN, POLISH, PORTUGUESE, RUSSIAN, SPANISH, THAI, TURKISH, UKRAINIAN,
    };

    use super::{Texts, key};

    #[test]
    fn a_number_takes_the_language_marks() {
        assert_eq!(Texts::new(ENGLISH, &[]).digits("3,645 1.5"), "3,645 1.5");
        assert_eq!(Texts::new(CHINESE, &[]).digits("3,645 1.5"), "3,645 1.5");
        assert_eq!(Texts::new(CHINESE_TRADITIONAL, &[]).digits("3,645 1.5"), "3,645 1.5");
        assert_eq!(Texts::new(FRENCH, &[]).digits("3,645 1.5"), "3\u{a0}645 1,5");
        assert_eq!(Texts::new(GERMAN, &[]).digits("3,645 1.5"), "3.645 1,5");
        assert_eq!(Texts::new(SPANISH, &[]).digits("3,645 1.5"), "3.645 1,5");
        assert_eq!(Texts::new(ITALIAN, &[]).digits("3,645 1.5"), "3.645 1,5");
        assert_eq!(Texts::new(PORTUGUESE, &[]).digits("3,645 1.5"), "3.645 1,5");
        assert_eq!(Texts::new(POLISH, &[]).digits("3,645 1.5"), "3\u{a0}645 1,5");
        assert_eq!(Texts::new(TURKISH, &[]).digits("3,645 1.5"), "3.645 1,5");
        assert_eq!(Texts::new(RUSSIAN, &[]).digits("3,645 1.5"), "3\u{a0}645 1,5");
        assert_eq!(Texts::new(UKRAINIAN, &[]).digits("3,645 1.5"), "3\u{a0}645 1,5");
        assert_eq!(Texts::new(THAI, &[]).digits("3,645 1.5"), "3,645 1.5");
    }

    #[test]
    fn a_message_is_made_again_only_for_other_arguments() {
        let texts = Texts::new(ENGLISH, &[]);
        let made = |count: f64| texts.formatted(key::TRAINER_MODS, &[("count", Arg::Num(count))]);
        assert_eq!(made(28.).as_ref(), "28 MODS");
        assert_eq!(made(28.).as_ref(), "28 MODS");
        assert_eq!(made(3.).as_ref(), "3 MODS");
        assert_eq!(texts.formatted.borrow().len(), 2);
    }
}
