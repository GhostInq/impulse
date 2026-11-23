---
paths:
  - "crates/wand/**"
---

# Wand source

- `app` registers `Wand` through the capability traits in `model`. Keep source-specific behavior in this crate.
- Never bundle or commit third-party binaries. Check hashes and publisher signatures before loading them (`PUBLISHERS`: WeMod LLC, and its earlier name Daring Development Inc. on older trainers); keep verified images locked until the run ends. The service and plugin are those of the trainer's architecture (`binaries`), as Wand picks them.
- Keep credentials inside the session code and protect them with user-scoped DPAPI. Never log tokens, auth codes, headers or response bodies.
- Treat remote ids, paths and links (`purchaseUris`, reaching `launchers::store_link` only) as untrusted. Preserve listener scope and limits. Read the user's installation and settings without modifying them.
- `launch.proxy` (`proxy.rs`) is the one write into a game's folder, for a store's start only: Wand's `GameLauncher.exe` from its install, signature-checked, stands in for the launcher, which waits beside it as `.wand-backup`, with `.wand-config` naming what to start. Its paths are plain relative files, and the folder written is the game's after links resolve. A leftover it cannot account for is refused, never overwritten; the stand-in's own `.wand` and `.wand-config` are replaced. The stand-in puts the launcher back itself once the game starts; dropping the mount does too, off the dropping thread, and a retry's mount waits for that restore. The stand-in runs on its own and is not held locked: it moves itself aside.
- The process looked for is `target.image`, else the proxy's `gameExe`: never the anti-cheat's launcher. `blueprint.notes` reach the UI as the trainer's notes, in the answer's language.
- `activate.flags` (suspended start, hidden threads, masked modules, an external host) are not done: an unmarked attach failure of such a trainer says so and logs them. A service that never dials in asks for a newer Wand.
- Wand's per-user folders come from `install::app_data`: this user's `AppData`, or the user folder `IMPULSE_WAND_HOME` names (the Linux launcher keeps Wand in a prefix of its own). Under Wine, a WebView2 runtime beside Impulse (`WebView2\`) is named to the loader, which finds none on its own.
- Never prefetch trainers. A trainer DLL the CDN cannot give (a 404 that depends on the user's network, an offline start) is taken from Wand's own download (`%APPDATA%\<brand>\App\trainers\Trainer_<id>_<hash10>.dll`, read only), only when it hashes to the one the API names, and copied into ours.
- API errors log method, path and HTTP status when available, never the query or response body. A `TypeError` triggers a credential-free `no-cors` probe of the origin; its result is not a CORS diagnosis.
