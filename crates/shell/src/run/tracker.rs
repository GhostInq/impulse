//! Where the player is and which way they head, asked of the live trainer one question at a time, only while
//! a lease is held or a fix is waited for. The heading lasts the run, so a map opened later shows the way the player
//! last went.

use std::{
    rc::{Rc, Weak},
    sync::Arc,
    time::{Duration, Instant},
};

use futures::channel::oneshot;
use gpui_kit::{Context, EventEmitter, Task};
use model::{Failure, Frame, Session};

use client::map::heading::{Bearing, Heading};

// Failed answers in a row before the player counts as lost (a menu, a loading screen); asking goes on,
// slower each time, but soon enough for the minimap to come back with the player.
const GIVE_UP: u32 = 3;
const SLOWEST: Duration = Duration::from_secs(3);
// A fix asked for once comes as soon as the trainer answers; after a failed answer it is asked again this soon.
const AT_ONCE: Duration = Duration::from_millis(50);
const RETRY: Duration = Duration::from_millis(250);

/// The player's position in the game's own coordinates, and which way they head.
#[derive(Clone, Copy)]
pub struct Fix {
    pub position: [f64; 3],
    pub bearing: Option<Bearing>,
    /// The trainer told the height; else it is 0.
    pub height: bool,
}

pub enum Tracked {
    Fix(Fix),
    Lost,
}

/// Wants a fix this often while held.
pub struct Lease {
    _every: Rc<Duration>,
}

/// One per live run whose trainer can tell where the player is.
pub struct Tracker {
    session: Arc<dyn Session>,
    // The plane the heading is worked out on.
    frame: Option<Frame>,
    heading: Heading,
    // The last position and whether its height was told.
    last: Option<([f64; 3], bool)>,
    leases: Vec<Weak<Duration>>,
    // Each waits for the next answer, placed on its own map: the heading's stays as it is.
    waiting: Vec<(oneshot::Sender<Fix>, Frame)>,
    asking: bool,
    // Between two questions: a fix waited for cuts the pause short.
    sleeping: bool,
    failures: u32,
    _asks: Task<()>,
}

impl EventEmitter<Tracked> for Tracker {}

impl Tracker {
    pub fn new(session: Arc<dyn Session>) -> Self {
        Self {
            session,
            frame: None,
            heading: Heading::default(),
            last: None,
            leases: Vec::new(),
            waiting: Vec::new(),
            asking: false,
            sleeping: false,
            failures: 0,
            _asks: Task::ready(()),
        }
    }

    /// `frame`: the leaseholder's map; another one starts the heading over. A rate change applies after the
    /// wait under way.
    pub fn lease(&mut self, every: Duration, frame: &Frame, cx: &mut Context<Self>) -> Lease {
        if self.frame.as_ref() != Some(frame) {
            (self.frame, self.heading, self.last) = (Some(frame.clone()), Heading::default(), None);
        }
        let every = Rc::new(every);
        self.leases.push(Rc::downgrade(&every));
        if !self.asking {
            self.ask(cx);
        }
        Lease { _every: every }
    }

    /// The next fix on `frame`'s map, asked for now; none when it is off that map or the player counts as lost.
    pub fn next_fix(&mut self, frame: &Frame, cx: &mut Context<Self>) -> oneshot::Receiver<Fix> {
        let (sender, fix) = oneshot::channel();
        self.waiting.push((sender, frame.clone()));
        // A pause (up to `SLOWEST` while the player is lost) ends; a question under way is answered first.
        if !self.asking || self.sleeping {
            self.ask(cx);
        }
        fix
    }

    /// The bearing as of now: it goes stale while no map asks.
    pub fn last(&self) -> Option<Fix> {
        let (position, height) = self.last.filter(|_| self.failures < GIVE_UP)?;
        Some(Fix { position, bearing: self.heading.bearing(Instant::now()), height })
    }

    /// The shortest wait a held lease wants; at once while a fix is waited for.
    fn every(&mut self) -> Option<Duration> {
        self.sleeping = false;
        self.waiting.retain(|(sender, _)| !sender.is_canceled());
        self.leases.retain(|lease| lease.strong_count() > 0);
        let waited = (!self.waiting.is_empty()).then_some(AT_ONCE);
        self.leases.iter().filter_map(Weak::upgrade).map(|every| *every).chain(waited).min()
    }

    fn ask(&mut self, cx: &mut Context<Self>) {
        (self.asking, self.sleeping) = (true, false);
        let session = self.session.clone();
        self._asks = cx.spawn(async move |tracker, cx| {
            while let Ok(Some(every)) = tracker.update(cx, |tracker, _| tracker.every()) {
                let answer = session.position().await;
                let Ok(wait) = tracker.update(cx, |tracker, cx| tracker.answered(answer, every, cx)) else { return };
                cx.background_executor().timer(wait).await;
            }
            tracker.update(cx, |tracker, _| tracker.asking = false).ok();
        });
    }

    /// How long to wait before asking again.
    fn answered(&mut self, answer: Result<[f64; 3], Failure>, every: Duration, cx: &mut Context<Self>) -> Duration {
        self.sleeping = true;
        let position = answer.inspect_err(|e| log::debug!("player position: {e}")).ok();
        if let Some(position) = position {
            for (sender, frame) in self.waiting.drain(..) {
                if let Some((position, height)) = placed(Some(&frame), position) {
                    let _ = sender.send(Fix { position, bearing: None, height });
                }
            }
        }
        // Asked only for fixes waited for: what the maps follow stays as it was.
        if !self.leases.iter().any(|lease| lease.strong_count() > 0) {
            return if position.is_some() { every } else { RETRY };
        }
        match position.and_then(|position| placed(self.frame.as_ref(), position)) {
            Some((position, height)) => {
                self.failures = 0;
                let at = Instant::now();
                if let Some(frame) = &self.frame {
                    self.heading.track(frame.to_map(position), at);
                }
                self.last = Some((position, height));
                cx.emit(Tracked::Fix(Fix { position, bearing: self.heading.bearing(at), height }));
                every
            }
            None => {
                self.failures += 1;
                if self.failures == GIVE_UP {
                    log::warn!("the trainer does not tell where the player is");
                    self.heading = Heading::default();
                    self.waiting.clear();
                    cx.emit(Tracked::Lost);
                }
                (every * 2u32.saturating_pow(self.failures)).min(SLOWEST)
            }
        }
    }
}

/// The plane's two axes must be numbers; a height the trainer does not tell is 0 (many trainers send none). With
/// whether it was told.
fn placed(frame: Option<&Frame>, mut position: [f64; 3]) -> Option<([f64; 3], bool)> {
    let Some(frame) = frame else { return Some((position, position.iter().all(|axis| axis.is_finite()))) };
    let up = frame.axes[2];
    let told = position[up].is_finite();
    if !told {
        position[up] = 0.;
    }
    let at = frame.to_map(position);
    let on = at.x.is_finite() && at.y.is_finite();
    if !on {
        log::debug!("player position off the map's axes");
    }
    on.then_some((position, told))
}

#[cfg(test)]
mod tests {
    use model::{Frame, Point};

    use super::placed;

    #[test]
    fn a_missing_height_is_zero_and_a_missing_plane_axis_no_fix() {
        let frame = Frame { axes: [0, 2, 1], origin: Point { x: 0.5, y: 0.5 }, scale: (1e-4, -1e-4) };
        assert_eq!(placed(Some(&frame), [1., f64::NAN, 2.]), Some(([1., 0., 2.], false)));
        assert_eq!(placed(Some(&frame), [1., 5., 2.]), Some(([1., 5., 2.], true)));
        assert_eq!(placed(Some(&frame), [1., 3., f64::NAN]), None);
    }
}
