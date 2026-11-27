---
paths:
  - "crates/{keyhook,topmost,tray,video,sound,journal,disk,fetch,pe,updater}/**"
---

# Platform leaves

Small Windows and I/O crates with no dependency on our other crates but `model` (`keyhook`). Each is data in, data out.

## keyhook
- `Matcher` over `Trigger`s: a `model::Chord` (matches as Wand does: longest active chord wins, a chord bound twice fires both) or a `Key` (modifiers of either side, exactly, and one key, as `RegisterHotKey` matched; fires on its own key's down). `active` is asked only of the bindings that match; `replace` keeps the keys held; `modifiers` (`Modifiers`) reads them; `Key::clashes` tells a chord that fires a key too. Also `Capture`, `Hook`, `Stroke`, `foreground_pid`, `caps_lock`. A feature's default keys are its own (`Key::new`).
- `Hook` runs on its own thread, never the tao/WebView2 thread: Windows removes a hook that stalls. Its handler returns at once (forward to a channel); true on a down hides it from every app. A repeat reaches it only for a key it showed (`repeat`, its return ignored). The keys down as it starts come first as `held` (read after the hook is set, so their release comes), then only at their release. A key it showed that the system has up at the next press (let go on the lock screen or a UAC prompt) is released then.
- The mouse hook runs on every move: on only while asked (`set_mouse`, posted to the same thread without waiting, so the held keys stay). A later hook sees keys first.

## topmost
- `Watch` (one thread: the foreground hook) and raw-HWND calls (`foreground`, `raise`, `lower`, `cloak`, `place`, `show`, `pass_through`, `exclusive`, ...). `window_of(pid)`: a process's largest visible unowned top-level window; `first_window_of(pid)`: its first visible one, front to back. `raise` says whether the window took the foreground: a hooked press gives no right to it (a registered hotkey's did), so `take_input` sends an empty input first.
- `client_area(window)`: a window's client area as an `Area` (screen physical pixels, as `place`, `show` and `resize` take), for placing a layer inside a game's frame; data only, `None` for a gone window. `resize` keeps the z-order; `restore` says how the window was (`Away`) for `put_away`.
- `post_input(window, message, wparam, lparam)`: posts only mouse messages; a wheel's client point becomes the screen one Win32 wants.
- `layout(game)` (the `HKL` of `game`'s thread) and `key_state(Held)` (the held modifiers and Caps Lock as `GetKeyboardState` gives them): what the vendored GPUI's `keystroke_in` takes to make a keystroke of a key typed into the game.
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
- `replace(path, data)`: how every app file is written (temp file, rename); `stage(dir)` for one written as a stream or named only once written (`Staged`: `publish`, removed if dropped or not published). `size(path)`: a file's or folder's bytes, blocking.
- `Stored<T>`: a typed JSON file (`client`'s and `plugins`' alike; `client::storage` re-exports it): missing means the defaults, not JSON moves aside, an object carries `VERSION` and one of another version is never written. `edit` logs a failed save and keeps the change; `get_mut` changes it now and `flush` saves it if changed; `try_edit` (`T: Clone`) saves a copy first, publishes it only when saved, and errors on a store that does not save. The default has no file and never saves. `read` for a file kept by hand.

## fetch
- `get(url, etag)`: blocking GET for public CDNs only (they send no CORS headers). The WeMod API goes through `wand`'s bridge. Body capped at 64 MiB and left unread on a 304 (it sends none, and GitHub's names an encoding the decoder would fail on) or a 4xx or 5xx (their `edge`: `server`, `cf-ray` and `cf-cache-status`, names who answered), 120 s timeout. Run it off the UI thread.
- `range(url, from, len)`: the stored bytes (no encoding) as a 206 of exactly that range (shorter only at the file's end), with its ETag; a 200 (the range ignored) or another range is an error, a redirect is not followed and comes back with any other status, without a body.
- `guarded(url, allowed, limit, timeout)` for plugins: https only, every hop (5 redirects at most, followed by hand) must pass `allowed(host)`, public addresses only (the resolver drops the rest; IPv6 is global unicast, mapped and NAT64 follow their IPv4), no proxy, no headers but the user agent. An out-of-scope URL, hop or host, or a body past `limit`, is an error carrying `Refused` (`downcast_ref`).

## updater
- No GPUI; every call blocks, run it off the UI thread. `check(feed, etag)` reads a GitHub-shaped release list (Gitea, Forgejo and a static copy answer alike) into `Release`s: a draft, a tag that is not semver and a release without `Impulse.exe`, its SHA-256 `digest` and an https asset address are dropped, so nothing unverified is offered; a feed is read to its first 30 releases and 16 KiB of notes each. Build metadata does not make a version newer. The tag is `v1.30.1`, a pre-release `v1.30.1-pre`; `pre` is the tag's suffix or the host's flag. `newer(releases, current, channel, skipped)`: Stable skips pre-releases, Pre takes both, newest by semver, never a downgrade nor the skipped version or an older one.
- The feed is the publisher: https only (plain http to this machine for a feed under test), and the digest vouches for the transport, not for whoever runs the feed. A feed other than the project's own is the user's choice (`update_feed` in `settings.json`). The executable stays under `fetch`'s 64 MiB cap.
- `download` writes `<exe>.new` after the digest matches. `swap(exe)` renames the running executable to `<exe>.old`, then `<exe>.new` into its place, and puts the old one back if that fails; Windows renames a running executable but never overwrites it. A crash between the two renames leaves no executable at the path. `cleanup` (at start) removes `.old` and a stale `.new`. `restart` starts the new executable with `--restarted <pid>`; `restarted()` reads it for `tray::claim`.
- `sections(CHANGELOG.md)`: `## [x.y.z] - YYYY-MM-DD` then markdown; `base` drops a `-pre`, so one section covers a version's pre-releases and its release.

## pe
- `header` (machine, timestamp) and `export` (RVA, forwarders skipped). `header` reads headers only: executables can be gigabytes.
