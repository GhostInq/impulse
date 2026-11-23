<p align="center">
  <img src=".github/media/cover.webp" alt="Impulse trainer controls on desktop and phone, shown in the dark lime theme">
</p>

<h1 align="center">Impulse</h1>

<p align="center">
  Impulse is a lightweight alternative Windows client for game trainers.<br>
  Use hotkeys, an overlay, or your phone to control your trainer without switching out of the game.
</p>

<p align="center">
  <a href="https://github.com/tweak-for-me/impulse/releases/latest"><img src="https://img.shields.io/github/v/release/tweak-for-me/impulse?style=flat-square&labelColor=161a14&color=c6ff3d" alt="Latest release"></a>
  <a href="https://github.com/tweak-for-me/impulse/stargazers"><img src="https://img.shields.io/github/stars/tweak-for-me/impulse?style=flat-square&labelColor=161a14&color=c6ff3d&logo=github&logoColor=c6ff3d" alt="Stars"></a>
  <img src="https://img.shields.io/badge/Windows-10%20%7C%2011-c6ff3d?style=flat-square&labelColor=161a14" alt="Windows 10 and 11">
  <img src="https://img.shields.io/badge/built%20with-Rust-c6ff3d?style=flat-square&labelColor=161a14&logo=rust&logoColor=c6ff3d" alt="Built with Rust">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPLv3-c6ff3d?style=flat-square&labelColor=161a14" alt="GPLv3 license"></a>
  <a href="https://discord.gg/GgX29D7Tt"><img src="https://img.shields.io/badge/Discord-join-5865F2?style=flat-square&logo=discord&logoColor=white" alt="Join the Discord"></a>
</p>

<p align="center">
  <a href="https://github.com/tweak-for-me/impulse/releases/latest">Download</a> ·
  <a href="#usage">Usage</a> ·
  <a href="#roadmap">Roadmap</a> ·
  <a href="#build-from-source">Build</a> ·
  <a href="#support">Support</a> ·
  <a href="https://github.com/tweak-for-me/impulse/issues/new">Report a bug</a>
</p>

## Usage

1. On Windows 10 or 11 (x64), install Wand (WeMod) and sign in there. It's the trainer source in the current release.
2. Download `Impulse.exe` from the [latest release](https://github.com/tweak-for-me/impulse/releases/latest) and run it. Impulse uses your existing sign-in and trainer runtime. It ships no Wand binaries.
3. Pick a game from your library, or add its executable with **+ Add**. Click **Launch + Attach** to start the game and trainer or attach to a game that's already running.

> [!TIP]
> **Overlay:** press **Ctrl+Shift+O** while your game is in front.
>
> **Phone:** click the phone icon in the title bar, turn on **Phone access**, and scan the QR code. Use the same trusted network for your phone and PC.

<a href="https://github.com/tweak-for-me/impulse"><img src=".github/media/star.svg" alt="Star Impulse on GitHub to follow updates" width="100%"></a>

## What you can do

- **Keep your usual controls.** Import your hotkeys, set your own, or limit them to the active game. Pin favorite mods and save sets of values as collections.
- **Stay in the game.** Put your mods, map, minimap, and session in movable overlay panels, or control the trainer from your phone. On supported maps you can see your live position and heading, mark places, and teleport.
- **Keep the footprint small.** The native app is built for low memory use. Previously opened content is cached locally, trainer files download on first use, and closing the window can send it to the tray.

## Screenshots

<p align="center">
  <img src=".github/media/desktop.webp" alt="Desktop library and trainer sheet with fictional games">
</p>

<p align="center">
  <img src=".github/media/remote-panel.webp" width="300" alt="Remote panel with trainer controls on a phone">
</p>

<p align="center"><sub>The screenshots use fictional data.</sub></p>

## Roadmap

### Planned

- [ ] **Internal overlay:** draw the overlay inside the game instead of a window on top of it.
- [ ] **Overlay notifications:** short on-screen toasts, such as for a nearby map marker.
- [ ] **Search across trainers:** find a mod by name in all your trainers at once.

### Exploring

- **Plugins:** broader support for more sources of titles, maps, and trainers. The current host is alpha.
- **Cheat Engine tables:** run tables with Cheat Engine as the backend.
- **Proton and Steam Deck:** Impulse beside a game in its Proton prefix, started from Steam's launch options. Experimental: see [Impulse on Linux](linux/README.md).

## Build from source

Install current stable Rust, the Windows SDK (`rc.exe` and `fxc.exe` for release builds), Node.js (22.22.2+ on 22.x or 24.15.0+ on 24.x), and pnpm. From the repository root:

```powershell
.\scripts\panel.ps1
cargo build --release --locked
```

The executable is `target\release\Impulse.exe`. To run the checks, install the WASM target with `rustup target add wasm32-unknown-unknown`, then run `.\scripts\check.ps1`.

## License

Impulse is licensed under the [GNU General Public License v3.0](LICENSE). Code in `vendor/` keeps its own licenses.

## Support

If Impulse is useful to you, you can support its development with any option below.

[![GitHub Sponsors](https://img.shields.io/badge/GitHub%20Sponsors-sponsor-ea4aaa?style=flat-square&logo=githubsponsors&logoColor=white)](https://github.com/sponsors/k1tbyte)
[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-donate-ffdd00?style=flat-square&logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/kitbyte)
[![Patreon](https://img.shields.io/badge/Patreon-donate-f96854.svg?style=flat-square&logo=patreon)](https://www.patreon.com/kitbyte/gift)
[![USDT TRC20](https://img.shields.io/badge/USDT--TRC20-donate-26a17b.svg?style=flat-square&logo=tether)](https://tronscan.org/#/address/TQdvau8pAy5Tg1Aa588tTcPCFgbcHtuoxc)
[![BTC](https://img.shields.io/badge/BTC-donate-f7931a.svg?style=flat-square&logo=bitcoin)](https://www.blockchain.com/explorer/addresses/btc/1EZKDcyU8REm9JW5xwXJqSpn5Xaq5yAWWX)
[![ETH](https://img.shields.io/badge/ETH-donate-3c3c3d.svg?style=flat-square&logo=ethereum)](https://etherscan.io/address/0xd904d9d0557f88bbb1c4ab3582b4ca0d8a730e8d)
