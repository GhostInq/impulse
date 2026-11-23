---
paths:
  - "crates/{hotkeys,overlay,tray,video,sound,journal,disk,fetch,pe,updater,wine}/**"
---

# Platform leaves

Small Windows and I/O crates with no dependency on our other crates but `model` (`hotkeys`). Each is data in, data out.

## hotkeys
- `Matcher` over `model::Chord` (matches as Wand does: longest chord wins, a chord bound twice fires both), `Capture`, `Hook`, `foreground_pid`.
- `Hook` runs on its own thread, never the tao/WebView2 thread: Windows removes a hook that stalls. Its handler returns at once (forward to a channel); true on a down hides it from every app.
- The mouse hook runs on every move: start it only when a chord `uses_mouse()`. A later hook sees keys first.

## overlay
- `Key` (modifiers and one key), `Watch` (one thread: `RegisterHotKey`, a foreground hook, `EXTRAS` extra keys by number: `set_extra`, `Event::Extra`), and raw-HWND calls (`foreground`, `raise`, `lower`, `cloak`, `place`, `pass_through`, `exclusive`, ...).
- `exclusive(game)`: `SHQueryUserNotificationState` says D3D fullscreen and `game` is the window in front covering its monitor. The state is the session's and lags a mode change; borderless gives `QUNS_BUSY`.
- `Watch`'s sink runs on the watch thread: forward to a channel. The calls take `isize` handles and are made outside GPUI updates.

## tray
- `Tray` lives on GPUI's main thread (its hidden window needs that loop); dropping it removes the icon. `set_menu` feeds the next right click.
- `claim(after)` is the single-instance mutex; a second start wakes the first and gets `false`. `after` is the pid of an Impulse that is ending (an update's restart): it is waited for (15 s at most), not woken, then the claim is made as any other, so a start that wins the race wakes the child's Impulse instead. `hide` and `show` take the app window's HWND.
- The menu's words come from `shell` in `Menu`.

## video
- `Playback::start(file, device)`: MP4 (H.264) decoded on its own thread to NV12 textures on the window's D3D11 device, which must be multithread protected. `frame(elapsed)` never blocks.
- A `Frame`'s texture returns to the pool when dropped: hold none longer than needed. Dropping the `Playback` ends the thread.

## sound
- `Cue` (toggle on and off, slider, number, button, selection; serde, so `shell` saves it by name), `play(cue, file)` and `is_clip(file)`: async `PlaySoundW` of WAVs generated in memory once (no files ship: `synth` makes struck notes, a pair a fourth apart with a soft click and an exponential decay, the off sound a fourth below the on), or of the user's `file` when winmm takes it.
- One sound at a time: a new cue replaces the one playing. `play` never waits for the sound and may be called from any thread; a `file` is opened inside the call (winmm answers `FALSE` for a missing one: the built-in tone plays), so only local files are expected. `is_clip` (RIFF/WAVE with a format and a nonempty data chunk, 1 MiB, a path of 255 units) reads the file: call it off the UI thread.

## journal
- `init` first in `main`, before anything logs. `since`, `subscribe`, `closing` (call when the last window closes), `dir`.
- `OURS` lists the workspace crates (Debug up; others Warn up). A test fails when a crate is missing: add a new crate there.
- Messages are flattened to one line and the profile path is scrubbed.

## disk
- `replace(path, data)`: how every app file is written (temp file, rename). `size(path)`: a file's or folder's bytes, blocking.
- `Stored<T>`: a typed JSON file (`client`'s and `plugins`' alike; `client::storage` re-exports it): missing means the defaults, not JSON moves aside, an object carries `VERSION` and one of another version is never written. `edit` logs a failed save and keeps the change; `try_edit` (`T: Clone`) saves a copy first, publishes it only when saved, and errors on a store that does not save. `read`, `write` for files kept by hand.

## fetch
- `get(url, etag)`: blocking GET for public CDNs only (they send no CORS headers). The WeMod API goes through `wand`'s bridge. Body capped at 64 MiB and left unread on a 304 (it sends none, and GitHub's names an encoding the decoder would fail on) or a 4xx or 5xx (their `edge`: `server`, `cf-ray` and `cf-cache-status`, names who answered), 120 s timeout. Run it off the UI thread.
- `guarded(url, allowed, limit, timeout)` for plugins: https only, every hop (5 redirects at most, followed by hand) must pass `allowed(host)`, public addresses only (the resolver drops the rest; IPv6 is global unicast, mapped and NAT64 follow their IPv4), no proxy, no headers but the user agent. An out-of-scope URL, hop or host, or a body past `limit`, is an error carrying `Refused` (`downcast_ref`).

## updater
- No GPUI; every call blocks, run it off the UI thread. `check(feed, etag)` reads a GitHub-shaped release list (Gitea, Forgejo and a static copy answer alike) into `Release`s: a draft, a tag that is not semver and a release without `Impulse.exe`, its SHA-256 `digest` and an https asset address are dropped, so nothing unverified is offered; a feed is read to its first 30 releases and 16 KiB of notes each. Build metadata does not make a version newer. The tag is `v1.30.1`, a pre-release `v1.30.1-pre`; `pre` is the tag's suffix or the host's flag. `newer(releases, current, channel)`: Stable skips pre-releases, Pre takes both, newest by semver, never a downgrade.
- The feed is the publisher: https only (plain http to this machine for a feed under test), and the digest vouches for the transport, not for whoever runs the feed. A feed other than the project's own is the user's choice (`update_feed` in `settings.json`). The executable stays under `fetch`'s 64 MiB cap.
- `download` writes `<exe>.new` after the digest matches. `swap(exe)` renames the running executable to `<exe>.old`, then `<exe>.new` into its place, and puts the old one back if that fails; Windows renames a running executable but never overwrites it. A crash between the two renames leaves no executable at the path. `cleanup` (at start) removes `.old` and a stale `.new`. `restart` starts the new executable with `--restarted <pid>`; `restarted()` reads it for `tray::claim`.
- `sections(CHANGELOG.md)`: `## [x.y.z] - YYYY-MM-DD` then markdown; `base` drops a `-pre`, so one section covers a version's pre-releases and its release.

## wine
- `version` (Wine's own, `None` on Windows), `dos_path` (a Unix path as Wine names it: `Z:\...`, or a drive Proton made), `steam_root` (Steam's folder on the Linux side, from `STEAM_COMPAT_CLIENT_INSTALL_PATH`), `own_title_bar(exe)`, `command`, `Command::start`, `Game`.
- Every answer is `None` on Windows: callers ask unconditionally and Windows behaves as before.
- `--game <exe> <args>` first on the command line is the Linux launcher's: Proton turned only Impulse's path into a Windows one and waits on Impulse, so `start` does what Proton would have (the exe's Unix path through Wine, the rest of the line as written, the inherited folder and environment, a console program gets a console) inside a job, and `Game::wait` returns when the job's last process ends: a launcher that starts the game and exits does not end it. The first process's exit code goes back to Proton.
- `own_title_bar` writes Wine's per-program `X11 Driver\Decorated=N` unless the user set it: a caption-styled window that draws its own title bar is otherwise cut off under a window manager's. Wine reads it when the first window is made.

## pe
- `header` (machine, timestamp) and `export` (RVA, forwarders skipped). `header` reads headers only: executables can be gigabytes.
