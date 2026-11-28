//! Design tokens, the single source for colors, fonts and text sizes.

use std::{borrow::Cow, sync::Once};

use gpui_kit::{
    App, Global, Rgba,
    component::{Theme, ThemeMode},
    px, rgb,
};
use i18n::{CHINESE_TRADITIONAL, Key, Language};

use super::key;

/// Latin and Thai.
const UI: &str = "Chakra Petch";
/// The UI font of Cyrillic languages: Chakra Petch has none, and Exo 2 is the closest in metrics.
const UI_CYRILLIC: &str = "Exo 2";
/// The UI font of Simplified Chinese: Chakra Petch has no han, and Windows ships this one with its own faces.
const UI_CJK: &str = "Microsoft YaHei UI";
/// The UI font of Traditional Chinese: Windows ships this one with Traditional glyph forms.
const UI_CJK_TRADITIONAL: &str = "Microsoft JhengHei UI";
pub const MONO: &str = "JetBrains Mono";

// The type scale, px: every fixed text size is one of these.
pub const TYPE_TAG: f32 = 9.5;
pub const TYPE_CAPTION: f32 = 10.5;
pub const TYPE_LABEL: f32 = 11.;
pub const TYPE_SMALL: f32 = 12.;
/// Inputs and the tracked action labels.
pub const TYPE_FIELD: f32 = 12.5;
pub const TYPE_BODY: f32 = 13.;
/// The window's default, and the names a row leads with.
pub const TYPE_LEAD: f32 = 14.;
pub const TYPE_TITLE: f32 = 16.;
pub const TYPE_BRAND: f32 = 20.;
pub const TYPE_DISPLAY: f32 = 24.;
pub const TYPE_CLOCK: f32 = 42.;
pub const TYPE_HERO: f32 = 56.;

pub const BG: u32 = 0x0A0B0A;
pub const PANEL: u32 = 0x0C0E0B;
pub const SUNKEN: u32 = 0x060706;
pub const RAISED: u32 = 0x121411;
pub const RAISED_HOVER: u32 = 0x1C201B;
pub const ROW_HOVER: u32 = 0x121511;
pub const ROW_SELECTED: u32 = 0x171B16;
pub const STATUSBAR: u32 = 0x070807;
pub const AVATAR: u32 = 0x1A1E19;

pub const TEXT_BRIGHT: u32 = 0xF2F5EE;
pub const TEXT: u32 = 0xE4E8DF;
pub const TEXT_2: u32 = 0xC9CFC3;
pub const MUTED: u32 = 0xA9B0A3;
pub const DIM: u32 = 0x8F978A;
pub const FAINT: u32 = 0x7C8476;
pub const GHOST: u32 = 0x4A5046;
pub const WARN: u32 = 0xFFB547;
pub const DANGER: u32 = 0xFF6B5A;

pub const SEGMENT_OFF: u32 = 0x262A24;
pub const SEGMENT_IDLE: u32 = 0x6E7569;
pub const TICK: u32 = 0x1E221D;
pub const TICK_IDLE: u32 = 0x5A6155;
const SCROLL_THUMB: u32 = 0x2A2F28;
/// Dark on light: some phone cameras read no inverted QR code.
pub const QR_LIGHT: u32 = 0xFFFFFF;
pub const QR_DARK: u32 = 0x000000;

/// The id kept in the prefs, the color and its name.
pub const ACCENTS: [(&str, u32, Key); 4] = [
    ("LIME", 0xC6FF3D, key::ACCENT_LIME),
    ("AMBER", 0xFFB020, key::ACCENT_AMBER),
    ("CYAN", 0x4DE2FF, key::ACCENT_CYAN),
    ("WHITE", 0xF2F2F2, key::ACCENT_WHITE),
];

macro_rules! font {
    ($file:literal) => {
        Cow::Borrowed(include_bytes!(concat!("../../assets/fonts/", $file, ".ttf")).as_slice())
    };
}

/// The look the user chose and their language's font, for every widget to read; set by `Settings`.
pub struct Look {
    pub accent: u32,
    ui: &'static str,
    /// `ui` falling back to the CJK family: DirectWrite shapes a run with the one face it is given, so han in a game's
    /// title, a pin's name or a mono field would take a family Windows picks.
    font: gpui_kit::Font,
}

impl Global for Look {}

pub fn accent(cx: &App) -> u32 {
    cx.global::<Look>().accent
}

pub fn ui_font(cx: &App) -> &'static str {
    cx.global::<Look>().ui
}

/// The UI font with its fallback, for the window's root; made once per look.
pub fn ui_font_of(cx: &App) -> gpui_kit::Font {
    cx.global::<Look>().font.clone()
}

fn ui_family(language: Language) -> &'static str {
    if language.is_cyrillic() {
        UI_CYRILLIC
    } else if language == CHINESE_TRADITIONAL {
        UI_CJK_TRADITIONAL
    } else if language.is_cjk() {
        UI_CJK
    } else {
        UI
    }
}

/// Sets the look; false when it already was. Exo 2 joins the font system when a language first needs it.
pub fn set_look(cx: &mut App, accent: u32, language: Language) -> bool {
    let ui = ui_family(language);
    if cx.try_global::<Look>().is_some_and(|look| look.accent == accent && look.ui == ui) {
        return false;
    }
    if ui == UI_CYRILLIC {
        static LOADED: Once = Once::new();
        LOADED.call_once(|| {
            let fonts = vec![font!("Exo2-Regular"), font!("Exo2-Medium"), font!("Exo2-SemiBold"), font!("Exo2-Bold")];
            cx.text_system().add_fonts(fonts).expect("bundled fonts load");
        });
    }
    Theme::global_mut(cx).font_family = ui.into();
    let mut font = gpui_kit::font(ui);
    // Thai where the family has none (a mono field) takes Chakra Petch's, han YaHei's.
    font.fallbacks = Some(gpui_kit::FontFallbacks::from_fonts(vec![UI.to_owned(), UI_CJK.to_owned()]));
    cx.set_global(Look { accent, ui, font });
    true
}

pub fn alpha(hex: u32, a: f32) -> Rgba {
    Rgba { a, ..rgb(hex) }
}

pub fn line(a: f32) -> Rgba {
    alpha(TEXT, a)
}

pub fn init(cx: &mut App) {
    cx.text_system()
        .add_fonts(vec![
            font!("ChakraPetch-Regular"),
            font!("ChakraPetch-Medium"),
            font!("ChakraPetch-SemiBold"),
            font!("ChakraPetch-Bold"),
            font!("JetBrainsMono-Regular"),
            font!("JetBrainsMono-Medium"),
            font!("JetBrainsMono-Bold"),
        ])
        .expect("bundled fonts load");
    // Stock components (inputs, scrollbars) read these.
    Theme::change(ThemeMode::Dark, None, cx);
    let theme = Theme::global_mut(cx);
    theme.font_family = UI.into();
    theme.mono_font_family = MONO.into();
    theme.radius = px(0.);
    theme.background = rgb(BG).into();
    theme.foreground = rgb(TEXT).into();
    theme.muted_foreground = rgb(DIM).into();
    theme.caret = rgb(TEXT).into();
    theme.popover = rgb(RAISED).into();
    theme.popover_foreground = rgb(TEXT).into();
    theme.border = line(0.2).into();
    theme.scrollbar = alpha(BG, 0.).into();
    theme.scrollbar_thumb = rgb(SCROLL_THUMB).into();
    theme.scrollbar_thumb_hover = rgb(GHOST).into();
}

#[cfg(test)]
mod tests {
    use i18n::{CHINESE, CHINESE_TRADITIONAL, ENGLISH, RUSSIAN, THAI, UKRAINIAN};

    use super::*;

    #[test]
    fn a_language_without_latin_letters_gets_the_font_that_has_them() {
        assert_eq!(ui_family(ENGLISH), UI);
        assert_eq!(ui_family(RUSSIAN), UI_CYRILLIC);
        assert_eq!(ui_family(UKRAINIAN), UI_CYRILLIC);
        assert_eq!(ui_family(CHINESE), UI_CJK);
        assert_eq!(ui_family(CHINESE_TRADITIONAL), UI_CJK_TRADITIONAL);
        assert_eq!(ui_family(THAI), UI);
    }
}
