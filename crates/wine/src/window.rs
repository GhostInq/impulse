//! The windows of a program that draws its own title bar.

use anyhow::{Context, Result};

/// Asks Wine not to have the window manager decorate the windows of `exe` (a file name such as `Impulse.exe`), unless
/// the user chose otherwise. A window that draws its own title bar keeps the caption style, so Wine leaves room at its
/// top for a window manager's title bar and the program's own title bar is cut off. Wine reads the setting when the
/// program makes its first window: call this before that.
pub fn own_title_bar(exe: &str) -> Result<()> {
    let key = windows_registry::CURRENT_USER
        .create(format!(r"Software\Wine\AppDefaults\{exe}\X11 Driver"))
        .context("cannot open Wine's settings for this program")?;
    if key.get_string(DECORATED).is_err() {
        key.set_string(DECORATED, "N").context("cannot change Wine's window decoration setting")?;
    }
    Ok(())
}

const DECORATED: &str = "Decorated";
