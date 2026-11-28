# Changelog

This file is the source of truth for release notes: the release workflow publishes the newest section and the app shows it.
The newest entry must match the version in `Cargo.toml`. A `-pre` build reads the section of its version.

## [0.7.0] - 2026-11-28

### Features

- PIN HERE (Settings, OVERLAY; Ctrl+Shift+P) puts one of your pins where you stand, with a screenshot of the game on it. The in-game overlay hides for the shot. It works while the game's map is open or the HUD's minimap shows it.
- Your pins take pictures and videos: ADD MEDIA on a pin's card, or paste a picture or files into its fields. They are copies kept with your map data, so clearing the caches leaves them.
- A pin of yours shows its X Y Z on its card: type a position, height included. Hold Alt and drag it to move it on the map; its height stays.
- HIDE THIS PIN takes a map's pin off the map and out of its group's count; HIDDEN PINS in the sidebar brings it back. MOVE copies a map's pin into your pins, with its picture and clip, and hides the original, for a pin a game update moved: put the copy where the place really is. Its FOUND counts for the original.
- HEIGHTS NEARBY (map layers) shows how far above or below you the pins near you are, when the trainer tells your height. GROUP CLOSE PINS can group them ON THE MINIMAP TOO.
- Your pins and found marks are kept per map in a new format. A map's first open in 0.7.0 takes over the older file and leaves it as it was.
- Thai (Settings, APP, LANGUAGE).
- The 0.6.1 and 0.7.0 map texts in every language.

### Fixes

- On a map whose game's up axis is not z, TELEPORT to a pin lands on the pin, and the Castor Woods map shows where you are.
- Downloads on a slow connection (an update, a trainer) no longer fail after two minutes with "timeout: global": only one that stops for 30 s is given up. (#40)

## [0.6.1] - 2026-11-26

### Features

- A new install draws the overlay IN-GAME (Settings, OVERLAY, MODE), inside the game's picture where the game allows it, else over the game. An install that already has settings keeps its mode.
- A pin's picture shows its mark on its category's color until it loads, or when it cannot.
- TELEPORT HEIGHT in the map's sidebar raises the game's teleports to the map's pins and points by the value you set, in the game's units: for a game that drops you through the ground after a teleport (Gothic Remake). 0 is the height the map gives.
- The pin button under the map's follow button puts one of your pins where you stand, when the trainer tells your height. Its card shows the coordinates to copy, as a map pin's does, and TELEPORT on it brings you back to that exact spot, without TELEPORT HEIGHT.

### Fixes

- The HUD's minimap no longer blinks when the overlay closes: it comes up as the overlay goes, with its map, instead of after it, empty.
- A map pin's picture or clip shows the pin's icon in its category's color while it loads, or when it cannot load, instead of a black box. The lightbox shows the small picture at once while the full size loads.

## [0.6.0] - 2026-11-25

### Features

- Each game has its own overlay mode in its ⋯ menu: AS IN SETTINGS, IN-GAME or OVER THE GAME. It changes at once, even while the game runs. For a game where the in-game overlay shows nothing (007 First Light), choose OVER THE GAME.
- A slider mod's value is a field beside its slider: type a value, then press Enter or click away. It takes any value as precise as the slider's step (any whole number when the step is whole) and stays within its limits; the phone panel's field does the same. (#34)
- The run's button and the game's actions stay at the top of the page while it scrolls, so STOP is always in reach. (#33; thanks @GhostInq)
- Traditional Chinese (Settings, APP, LANGUAGE). (#31; thanks @toby0622)

### Fixes

- A number mod's field shows every decimal of its value (0.25, not 0.3), so clicking in and out of it no longer changes the value.
- The map follows the player and offers teleport wherever Wand's does: the game's maps decide it, not only the trainer's own flags.
- A trainer that reports the player's position without a height is placed on the map with height 0, as in Wand, instead of not at all.
- Maps stored in a single archive file (Phantom Blade Zero) show instead of "unsupported tile source".
- Map tiles load while you zoom, twice as many at once, and a blurry version of the view comes first, so zooming out or panning far no longer shows empty patches.
- Games whose own store's release has no trainer (most often GOG) are found again, as in 0.4.0, and can use another store's trainer. Only a game whose version file is gone stays out.
- The search finds a game under the name of another edition on the same install (World War Z: Aftermath under World War Z).
- A trainer that works only with a game you start yourself (007 First Light) says so at once when the game is not running, with START GAME to start it through its store, instead of waiting for it. Press LAUNCH + ATTACH once the game runs.
- The in-game overlay waits for the game's own window before it goes in. In an Unreal game started from Impulse (Deep Rock Galactic), it no longer took a hidden window of the game's loading screen, which left its cursor stuck as a pointer.

## [0.5.0] - 2026-11-25

### Features

- IN-GAME overlay mode (Settings, OVERLAY, MODE): the overlay is drawn inside the game's picture where the game allows it, with mouse and keyboard, and the game keeps the focus. Its fields take typing, and hotkeys can be bound in it.
- The in-game overlay also works in a game run as administrator, after the UAC prompt, as the trainer does.
- The overlay's hotkeys are held only while the game is in front: other apps get those keys the rest of the time.
- The window opens where you left it: on its monitor, at its size and place, maximized or not. (#26; thanks @GhostInq)
- Games start through their store's launcher: Epic, GOG Galaxy, Ubisoft Connect, Rockstar and Xbox, besides Steam. Before, Impulse waited for you to start them. (#21)
- A Steam game with a launch option that turns its anti-cheat off starts through that option. (#21)
- A trainer that asks for a special way of loading gets it: the game is first given time to take input, held still while the trainer sets up, or the trainer runs outside the game in Wand's trainer host. (#21)
- A trainer's own launch arguments and its rules for finding the game's process (its command line, the oldest or newest copy, how long to wait) are followed.
- LAUNCH asks first when the game's build is newer than the trainer supports: PLAY ANYWAY, CANCEL or ALWAYS PLAY. ASK ON A GAME UPDATE in Settings, APP turns the question back on.
- A number mod has a field between its - and + buttons: type a value, then press Enter or click away. The value stays within the mod's limits. (#22)
- Simplified Chinese (Settings, APP, LANGUAGE). (#23; thanks @yzxxy010)
- A failed launch says which step failed: the launcher did not start the game, the game did not start in time, the game you start yourself did not appear in time, the game closed before the trainer got in, the trainer could not get in or did not start, or the trainer needs the game's other (64-bit or 32-bit) build.
- A translation can ship before all of it is done: a text not translated yet shows in English.
- The game list sorts as Steam's library does: a leading "The" does not count, so THE WITCHER 3 is under W. (#27)
- In the release notes, an issue number or a contributor's name opens its GitHub page.

### Fixes

- Xbox and Game Pass games are found in the folders their package uses, attach without reading the game's protected executable, and start the app the catalog names (Halo: The Master Chief Collection without its anti-cheat). (#21)
- A game's build is read from its version file whatever its kind (an Xbox package's signature, a Java archive), so the version mark and the trainer's game version match Wand's. (#21)
- A game you have in several stores uses the store its catalog entry lists first.
- A game listed under several catalog entries on one install shows once: the edition its launcher names (Mafia III: Definitive Edition), one entry per title otherwise. The games of a collection (Halo: The Master Chief Collection) are still listed apart. (#24; thanks @GhostInq)
- A game whose version file is gone (moved, uninstalled, or rebuilt on a new engine, like RimWorld 1.7) no longer counts as installed, as in Wand, so a trainer that does not fit it is not started. Add it by its executable to play anyway.
- LAUNCH right after starting the game yourself waits for it instead of starting a second copy, and a small helper process in the game's folder is no longer taken for the game.
- Slow-loading games get more time: 45 s for the trainer to connect, 90 s for it to get ready.
- Starting a trainer again right after stopping it in the same game waits for the last one to unload.
- When the game closes before the trainer connects, the launch fails at once instead of waiting out the timeout.
- Steam libraries written with forward slashes in Steam's settings are matched.
- Scrollbars in the trainer picker and the LINK A MAP lists. (#17)
- The caret and selection show in the in-game overlay's fields.
- No error in the log when a window closes.

## [0.4.0] - 2026-11-23

### Features

- Search every trainer by name with Ctrl+K or the search box in the title bar. A game you have opens as usual; any other opens with its mods, marked NOT INSTALLED.
- A game you do not have shows ADD GAME: GET ON a store opens its page (in the store's app when it is installed), and ADD GAME MANUALLY adds it from its executable.
- A game you do not have can be starred: it shows under FAVORITES, marked NOT INSTALLED.
- A game with no trainer for its store can use the trainer of its release on another store: USE THE STEAM TRAINER on the NO TRAINER card, or any trainer through OTHER TRAINER... and CHANGE TRAINER... in the game's menu. Builds can differ between stores, so it may not fit. (#7)
- The game list shows thumbnails by default.

### Fixes

- A game's releases on different stores list in the same order at every start.

## [0.3.1] - 2026-11-22

### Fixes

- A trainer file the download server does not give is taken from Wand's own copy of it, when Wand has downloaded it. (#15)
- A missing trainer file says "not found (404)" instead of a generic error, and the log names the server that answered. (#15)
- The window no longer stays fullscreen with dead close and minimize buttons after the overlay was used while it was hidden in the tray.
- Opening the window from the tray icon no longer drops its first frame.

## [0.3.0] - 2026-11-22

### Features

- Games with an anti-cheat launcher start the way Wand starts them: through Wand's own launcher, so the trainer can attach. The game's launcher is put back right after. When the trainer must start the game itself and the game already runs, Impulse asks to close it first. (#9)
- A trainer's notes show above its cheats as READ BEFORE PLAYING, folded like a section. (#9)

### Fixes

- Wand sign-in no longer fails with "Failed to fetch". (#6, #11)
- Long dropdown lists scroll and stay inside the window, and a long option ends in "..." instead of running past the list. (#12)
- For games with an anti-cheat launcher, Impulse attaches to the game itself, never to the launcher. (#9)
- An attach the game's protection blocks says so, instead of a bare "Access is denied". (#9)
- A trainer that needs Wand's own way of loading says so when its attach fails, and the log names how. (#8)
- When Wand's service does not answer, Impulse asks to update Wand. (#5)
- Wand sign-in shows why it failed and gives feedback when you click Try Again. (#6)
- Sign-in errors in the log name the failed API request and distinguish a fresh sign-in from a session refresh. The log includes the HTTP status when WebView2 can read it. The connectivity check reports only whether the probe passed, not why the original request failed. (#6)

## [0.2.0] - 2026-11-21

### Features

- 32-bit games: their trainers load like the 64-bit ones. A trainer made for the game's other build says so.
- Games run as administrator: Impulse asks for administrator rights (UAC) to attach, as Wand does. Run as administrator itself, it needs no prompt and shows a shield in the title bar. A game whose protection keeps trainers out says so.
- From the second start, the game list shows at once from the last scan and refreshes in the background, instead of waiting for every launcher after a reboot.
- The log shows how long each launcher's scan takes.

### Fixes

- Older trainers signed with WeMod's earlier company name (Daring Development, such as Mad Max) load instead of being refused.
- A UAC prompt or the lock screen no longer leaves an error in the log.

## [0.1.1] - 2026-11-20

### Features

- Binding a hotkey shows its keys (cancel, default, unbind) above the cap. A chord another cheat already holds asks to KEEP BOTH or REBIND; one the overlay or the same cheat holds cannot be kept. Esc or a click aside drops the new chord.
- Hotkeys that two cheats of a game share are outlined in orange.

### Fixes

- The update check no longer fails with a gzip error from the second check on.
- A trainer whose cheats send an empty `args` array (Spyro Reignited Trilogy) loads instead of failing with "cannot download the cheat list".

## [0.1.0] - 2026-11-20

### Features

- Your installed games from Steam, Epic, GOG, Ubisoft, Xbox, Rockstar and EA, or any executable you add, matched to Wand's catalog.
- Trainers from your Wand account: launch, attach, every kind of control, pins, saved mods and named collections.
- Imported and custom hotkeys, firing anywhere or only while the game is in front.
- Interactive maps with pins, regions, search and found pins, and your position and heading while a trainer runs.
- An overlay over the game with movable docks (mods, map, minimap, session) and a minimap that screen capture can skip.
- A remote panel to control a trainer from your phone on the local network.
- Plugins (alpha) that add map and title sources.
- Ten languages, cue sounds, a tray icon and a log.
- Updates: Impulse checks for a new version on the stable or the pre-release channel and shows what changed. A version can be skipped, and the checks can be turned off.
