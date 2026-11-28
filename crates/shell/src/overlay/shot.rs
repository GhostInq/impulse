//! PIN HERE's shot of the live game as the player sees it, without the overlay drawn in it.

use std::{future::Future, time::Duration};

use anyhow::{Context as _, Result};
use blocking::unblock;
use gpui_kit::{App, Entity};

use super::in_game::InGame;

// The game presents a few frames without the layers first.
const SETTLE: Duration = Duration::from_millis(120);
const FIRST_FRAME_WITHIN: Duration = Duration::from_secs(2);

/// A JPEG of `pid`'s window; the layers in the game hide for it and show again however it ends.
pub(super) fn shot(pid: u32, in_game: &Entity<InGame>, cx: &App) -> impl Future<Output = Option<Vec<u8>>> + use<> {
    let veiled = in_game.read(cx).veil();
    let executor = cx.background_executor().clone();
    async move {
        if veiled.is_some() {
            executor.timer(SETTLE).await;
        }
        let shot = unblock(move || jpeg_of(pid)).await;
        drop(veiled);
        shot.inspect_err(|e| log::warn!("no shot of the game: {e:#}")).ok()
    }
}

/// Its window in front, else its largest.
fn jpeg_of(pid: u32) -> Result<Vec<u8>> {
    let window = topmost::foreground().filter(|&window| topmost::owner(window) == pid).or_else(|| topmost::window_of(pid));
    let shot = capture::window(window.context("the game has no window")?, FIRST_FRAME_WITHIN)?;
    client::map::media::jpeg(shot.width, shot.height, &shot.bgra)
}
