# Impulse on Linux (Proton)

Impulse runs on Linux through Steam's Proton, next to the game it trains. This is experimental: it is tested with GE-Proton 10 (Wine 10) and Wine 11, not yet with every game.

## How it works

`impulse-proton` goes in a game's launch options in Steam. When Steam starts the game, it asks Proton to start Impulse in the game's place, with the game's command after `--game`. Impulse starts the game exactly as Proton would have and runs beside it **in the game's own prefix**, so:

- it sees the game's process and loads the trainer into it, as on Windows;
- its hotkeys work while the game's window is in front (one Wine session);
- it closes when the game exits, and Steam gets the game's exit code. A launcher that starts the game and exits does not end it early.

Nothing of Wand's app runs during play: Impulse needs Wand's files (its trainer engine and the sign-in Wand keeps) from a prefix of Wand's own, and a WebView2 runtime for Wand's API. No Electron and no .NET in the game's prefix.

## Requirements

- Steam with Proton 9 or newer, or GE-Proton 9 or newer.
- `python3`, `curl` or `wget`, `sha256sum`.
- About 800 MB of disk, most of it the WebView2 runtime.

## Set up

1. Get the script and make it executable:

   ```sh
   curl -fLO https://raw.githubusercontent.com/tweak-for-me/impulse/master/linux/impulse-proton
   chmod +x impulse-proton
   ```

2. Get Impulse and the WebView2 runtime. The script copies itself to `~/.local/share/impulse-proton/`, where Steam will run it from:

   ```sh
   ./impulse-proton setup
   ```

   It downloads the latest `Impulse.exe`, checked against the digest of its release, and the WebView2 Fixed Version runtime 135.0.3179.98, checked against a pinned SHA-256. Newer runtimes are reported never to finish loading a page under Wine 10. Use `--exe path/to/Impulse.exe` for a build of your own, or `--tag vX.Y.Z` for another release. Run `setup` again to update.

3. Install Wand in its own prefix and sign in. Download the Wand installer, then:

   ```sh
   ~/.local/share/impulse-proton/impulse-proton wand ~/Downloads/Wand-Setup.exe
   ```

   Wand opens when it is installed: sign in, then close it. Run `impulse-proton wand` again (without a file) to sign in again or to let Wand update. A package (`Wand-<version>-full.nupkg`) can be given instead of the installer.

4. In Steam, open the game's **Properties → General → Launch options** and enter:

   ```
   ~/.local/share/impulse-proton/impulse-proton %command%
   ```

   Other wrappers can stay: `gamemoderun ~/.local/share/impulse-proton/impulse-proton %command%` works.

5. Start the game from Steam. Impulse opens with it: pick the game and click **Launch + Attach**, which attaches to the game that is already running.

`impulse-proton doctor` shows what is set up and where.

## Where things are

Everything is in `~/.local/share/impulse-proton/` (`IMPULSE_HOME` moves it):

| Path | What |
| --- | --- |
| `impulse-proton` | The script Steam runs |
| `bin/Impulse.exe`, `bin/WebView2/` | Impulse, and the WebView2 runtime it finds beside itself |
| `data/` | Impulse's settings, library, sign-in and logs, shared by every game (`IMPULSE_DATA_DIR`) |
| `wand/pfx/` | Wand's prefix: its install and its sign-in, read by Impulse (`IMPULSE_WAND_HOME`) |
| `impulse-proton.log` | What the script did |

Impulse in a game's prefix reads Wand from Wand's prefix and keeps its own files in `data/`, so nothing is copied into each game's prefix. The sign-in Impulse keeps is readable from every Proton prefix.

## Steam Deck

- Desktop mode works as any Linux desktop.
- In Game mode one window shows at a time: switch between the game and Impulse with the Steam button, or control the trainer from your phone (Impulse's phone icon), which suits Game mode best. Impulse's overlay is a window over the game, which Game mode does not show.

## Limits

- Proton games only: Impulse cannot reach a native Linux game.
- A trainer that must start the game itself is refused while the game runs, and under Proton the game always starts first.
- Games with an anti-cheat launcher are untested.
- The window's close button hides Impulse in the notification area by default. On a desktop without one, Wine shows a small window at the top left with Impulse's icon; Settings can make the close button quit instead.
- A Steam install in Flatpak is untested: keep `IMPULSE_HOME` inside the Flatpak's home (`~/.var/app/com.valvesoftware.Steam/`).

## Troubleshooting

- `impulse-proton.log` says how each start went. Impulse's own log is `data/logs/impulse.log`; it names the Wine version.
- Proton's log: add `PROTON_LOG=1` before the command in the launch options (`PROTON_LOG=1 ~/.local/share/impulse-proton/impulse-proton %command%`); it is written to your home folder as `steam-<appid>.log`.
- To start a game without Impulse, remove the launch options.

## Building Impulse for Linux testing

From Linux, with `mingw-w64` and the Rust target `x86_64-pc-windows-gnu`:

```sh
cargo build --target x86_64-pc-windows-gnu -p impulse
```

A GNU build loads `WebView2Loader.dll` from beside the executable (the build copies it into `target/x86_64-pc-windows-gnu/debug/build/webview2-com-sys-*/out/x64/`). A debug build compiles its shaders when it starts, and Wine's own shader compiler cannot compile them: put Microsoft's `d3dcompiler_47.dll` beside the executable (the WebView2 runtime has one) and set `WINEDLLOVERRIDES=d3dcompiler_47=n`. Release builds carry compiled shaders and need neither.

Under Wine, Impulse draws without DirectComposition and makes its whole window the client area, so a window manager adds no title bar over its own; `crates/wine` holds what else differs.
