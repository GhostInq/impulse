//! PIN HERE: a user's pin where the player stands in the live game, with a shot of it.

use std::{future::Future, time::Duration};

use futures::future::{self, Either};
use gpui_kit::{App, Context, Entity};
use model::{Frame, Ref};

use super::Maps;
use crate::{Load, run::Tracker};

// The trainer answers well within it; past it no pin is placed.
const FIX_WITHIN: Duration = Duration::from_secs(3);

impl Maps {
    /// A pin at the player's next fix on the live game's map, with the picture `shot` takes meanwhile: nothing
    /// without a map that places the player.
    pub fn pin_here<S>(&mut self, shot: impl FnOnce(&mut App) -> S, cx: &mut Context<Self>)
    where
        S: Future<Output = Option<Vec<u8>>> + 'static,
    {
        let Some((tracker, map, frame)) = self.pin_target(cx) else {
            return log::warn!("PIN HERE: no map of the live game places the player");
        };
        let fix = tracker.update(cx, |tracker, cx| tracker.next_fix(&frame, cx));
        let timeout = cx.background_executor().timer(FIX_WITHIN);
        // The receiver goes at the timeout: the tracker stops asking for it.
        let fix = async move {
            match future::select(fix, timeout).await {
                Either::Left((Ok(fix), _)) => Some(fix),
                _ => None,
            }
        };
        let shot = shot(cx);
        cx.spawn(async move |maps, cx| {
            // The shot is awaited whatever comes: its end shows the overlay again.
            let (fix, picture) = futures::join!(fix, shot);
            let Some(fix) = fix else { return log::warn!("PIN HERE: the trainer did not tell where the player is") };
            maps.update(cx, |maps, cx| {
                // Another run's fix must not land on this map.
                if maps.live_tracker(cx).as_ref() == Some(&tracker) {
                    maps.board.update(cx, |board, cx| board.pin_player(&map, &frame, fix.position, fix.height, picture, cx));
                }
            })
            .ok();
        })
        .detach();
    }

    /// The live run's, while its game is the one whose maps are listed.
    fn live_tracker(&self, cx: &App) -> Option<Entity<Tracker>> {
        let (game, run) = self.runs.read(cx).live_run()?;
        run.tracker.clone().filter(|_| self.game.as_ref() == Some(game))
    }

    /// The live run's tracker and the big board's map, the one board that writes the user's state: as it is when it
    /// places the player, else, closed, it takes the minimap's.
    fn pin_target(&mut self, cx: &mut Context<Self>) -> Option<(Entity<Tracker>, Ref, Frame)> {
        let tracker = self.live_tracker(cx)?;
        if self.placing(cx).is_none() && !self.is_open(cx) {
            let live = self.live_map.read(cx);
            let id = live.ready(cx).then(|| live.board.read(cx).id().cloned()).flatten()?;
            let index = self.offers.iter().position(|offer| offer.id == id)?;
            self.choose(index, cx);
        }
        let (map, frame) = self.placing(cx)?;
        Some((tracker, map, frame))
    }

    /// The big board's map and its frame, when it has one and is one of the listed game's: a closed board may still
    /// hold another game's.
    fn placing(&self, cx: &App) -> Option<(Ref, Frame)> {
        let board = self.board.read(cx);
        let Load::Ready(map) = &board.map else { return None };
        let id = board.id().filter(|id| self.offers.iter().any(|offer| offer.id == **id))?;
        Some((id.clone(), map.frame.clone()?))
    }
}
