# Impulse

Native Rust + GPUI client for the user's own Wand (WeMod) account and install. What it does: `README.md`.

## Commands

- `scripts\check.ps1` - the gate before work is done: `cargo fmt` check, clippy with warnings denied, every test; the panel's `pnpm verify` once it is installed.
- `cargo build` - debug build (shaders compile at runtime; release needs `fxc.exe` and `rc.exe` from the Windows SDK). `cargo test -p <crate>` tests one crate; crates other than `shell` and `app` build without GPUI.
- `scripts\ui.ps1 -Steps 'click:1016,25;wait:800;shot:out.png'` - starts the debug exe, drives it with posted window messages (`wait`, `move`, `click`, `rclick`, `wheel`, `text`, `key`, `shot`, `mem`, `threads`; client pixels) and captures it with `PrintWindow`, then kills it. No real cursor, focus or screen capture; refuses to run while Impulse is open. Build first with `cargo build`.
- `scripts\panel.ps1` - builds the remote panel (`web/panel`, pnpm) into `web/panel/dist`, which the next cargo build embeds; `-Dev` runs its dev server (`/?demo`).
- `scripts\release-notes.ps1 -Tag v0.2.0` - checks a release tag against the workspace version (`v1.2.3`, a pre-release `v1.2.3-pre`) and prints its `CHANGELOG.md` section; `.github/workflows/release.yml` uses it on a tag push.
- `scripts\smoke.ps1 <label>` - screenshots of the main states into `target\smoke\<label>\`, to compare before and after a change. Set `IMPULSE_LOCALE=pseudo` first for the pseudo-locale (debug builds).

## Invariants

- Never ship or commit WeMod binaries (TrainerLib, trainer DLLs, Tophat).
- WeMod/Wand code lives in `crates/wand`; its rules are `.claude/rules/wand.md`. Read them before touching anything that reaches Wand, its API, its files or a game process.
- Never log tokens, auth codes or auth response bodies.
- Keep provider research out of Git.

## Crates

- `app` - the composition root: the elevated copy's entry (`agent::serve_elevated`), one instance, the journal, the migration, the registry, the window, the remote panel's server (feature `remote`, on by default).
- `shell` - the GPUI UI.
- `client` - the app's logic without GPUI: library, trainer sheet, runs (prepare, find or start the game, attach), hotkey choices, map state and the files that keep them.
- `wand` - everything WeMod/Wand; its one public item is the `Wand` source.
- `model` - the contract the UI, a run and the sources share: ids, data, the capability traits.
- `agent` - every operation on a game's process.
- `launchers` - installed games from the launchers' own records.
- `remote` - the remote panel's server: the embedded panel, pairing, the WebSocket; transport only.
- `plugins` - the WASM host: a plugin as a source, its limits, the import of its answers, their cache.
- `sdk` - what a plugin builds against: the v1 wire, the manifest's format, the guest's side of the ABI.
- `i18n` - the languages we ship and the catalogs: Fluent messages by domain, over English.
- `keyhook`, `topmost`, `tray`, `video`, `sound`, `capture` - Win32 and media leaves: key hooks, the overlay's window calls, the tray icon, clips on the GPU, cue sounds, a frame of a window.
- `updater` - self-update: the release feed, the download checked against its digest, the swap of the running executable, `CHANGELOG.md`'s sections.
- `journal`, `disk`, `fetch`, `pe` - the log, atomic file writes and typed JSON files, public CDN GETs, PE headers.
- `checks` - tests holding the crate graph, file sizes, the texts (no literal caption, every id in English, no other in a translation) and the rule files' globs; they cover `web/panel` too.

## Rules

- `.claude/rules/` holds one file per slice, loaded when a file matching its `paths` is read; `architecture.md` has no `paths` and is always loaded.
- A change to a boundary, a public surface or an invariant updates the matching rule file in the same change. A new crate adds a line above, a row in `GRAPH` (`crates/checks/src/graph.rs`) and a rule file or a glob in one. A rule that a lint or a test now enforces leaves the prose.
- A rule file says what its slice owns, its surface, its rules and traps, in about 50 lines: no history, no tour of the code, no line numbers.
