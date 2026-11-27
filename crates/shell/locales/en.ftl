## Shared

off = OFF
on = ON
# A number's marks: between thousands, before decimals.
number-group = ,
number-decimal = .

## Tray: Windows menu words

tray-open = Open Impulse
tray-stop = Stop trainer
tray-overlay = Overlay
tray-quit = Quit

## Failures: a kind's words fit after "It could not be loaded:"

failure-offline = no connection
failure-signed-out = signed out
failure-not-found = not found
failure-unsupported = not supported yet
failure-setup-needed = a setting is missing: see Settings, APP
failure-game-not-found = the game was not found
failure-launch-failed = the launcher could not start the game: the log has the details
failure-not-started = the game did not start in time: check its launcher, then try again
failure-not-running = the game was not started in time: start it yourself, then try again
failure-external-launch = this trainer works only with a game you start yourself: start it, then press LAUNCH + ATTACH
failure-game-exited = the game closed before the trainer got in
failure-refused = refused
failure-elevation = the game runs as administrator: accept the prompt to attach
failure-protected = the game's protection keeps trainers out
failure-needs-64 = the trainer needs the game's 64-bit build: start that one
failure-needs-32 = the trainer needs the game's 32-bit build: start that one
failure-load-failed = the trainer could not get into the game: the log has the details
failure-trainer-failed = the trainer did not start in the game: the log has the details
failure-restart = the game is already running: close it, then start it from here
failure-internal = something went wrong: the log has the details
failure-place = { $why }: see Settings, APP

## Page

catalog-failed-tag = [ NO CATALOG ]
catalog-failed = THE GAME CATALOG IS UNAVAILABLE
catalog-failed-body = It could not be downloaded: { $why }. Impulse tries again on the next start.
library-filter = filter
picker-filter = game name
page-search = search { $trainers } trainers
page-search-none = search trainers
page-search-key = CTRL K
page-add-game = ADD GAME
page-get-on = GET ON { $store }
page-add-manually = ADD GAME MANUALLY...
page-signed-out = NOT SIGNED IN
page-elevated = Running as administrator: games run as administrator attach without a prompt
page-overlay = OVERLAY
page-star = Star
page-folder = Open the game's folder
page-launcher = Open in { $launcher }
page-source = Game options
page-overlay-default = OVERLAY: AS IN SETTINGS
page-overlay-in-game = OVERLAY: IN-GAME
page-overlay-over = OVERLAY: OVER THE GAME
page-custom = CUSTOM
page-live = LIVE
page-active = { $active }/{ $total } ACTIVE
page-idle = IDLE
page-log = LOG
page-binding = PRESS KEYS · ESC CANCEL · BKSP DEFAULT
page-binding-unbind = PRESS KEYS · ESC CANCEL · BKSP DEFAULT · DEL UNBIND
page-hotkeys-on = HOTKEYS ON
page-hotkeys-off = HOTKEYS OFF
disclaimer-title = BEFORE YOU START
disclaimer-body = Impulse is an unofficial client for your own { $sources } account and install. It signs in through the session that app already has on this PC, uses the files in its folder and asks the service for your games and trainers on your behalf. It never asks for your password, and your account data stays on this PC apart from those requests. Impulse is not affiliated with or endorsed by the service. Trainers can get you banned in online games: use them at your own risk.
disclaimer-accept = I AGREE
disclaimer-exit = EXIT
signed-out-title = SIGN-IN FAILED
signed-out-body = Impulse could not sign in to { $sources }. Make sure it is installed and signed in on this PC and that you are online, then try again. Trainers cannot load until it works.
signed-out-retry = TRY AGAIN
signed-out-why = Reason: { $why }
signed-out-trying = Signing in…
signed-out-close = CLOSE

## Library

library-favorites = FAVORITES
library-games = GAMES
library-no-match = NO GAMES MATCH
library-add = + ADD
library-live = LIVE
library-scanning-tag = [ SCANNING ]
library-scanning = LOOKING FOR INSTALLED GAMES
library-scanning-body = Reading the game catalog and your launchers.
library-empty-tag = [ NO GAMES ]
library-empty = NO INSTALLED GAMES FOUND
library-empty-body = Impulse looks in Steam, Epic, GOG, Ubisoft, Xbox, Rockstar and EA for games it has trainers for.
library-source-exe = EXE  { $file }
library-source-add = + ADD .EXE
library-source-change = CHANGE .EXE
library-source-remove = REMOVE .EXE
library-source-trainer = CHANGE TRAINER...
library-source-own-trainer = ITS OWN TRAINER

## Picker

picker-title = ADD A GAME
picker-owned = MORE THAN ONE GAME MATCHES: WHICH ONE IS IT?
picker-unowned = NO CATALOG GAME MATCHES: FIND IT BY NAME
picker-in-library = { $store }  IN LIBRARY
picker-prompt = Select the game's .exe
picker-trainer-title = CHOOSE A TRAINER
picker-trainer = ITS OTHER STORES FIRST, THEN BY NAME: A TRAINER FOR ANOTHER BUILD MAY NOT FIT
picker-in-use = IN USE
picker-find-title = FIND A GAME
picker-find = EVERY TRAINER BY NAME · ↑ ↓ TO CHOOSE, ENTER TO OPEN

## Cheats

cheats-failed-tag = [ NO DATA ]
cheats-failed = THE CHEAT LIST IS NOT DOWNLOADED
cheats-failed-body = It could not be downloaded: { $why }. Impulse tries again when you open the game.
cheats-filter = filter mods
cheats-pinned = PINNED
cheats-kept = It is kept on disk after the first download.
cheats-notes = READ BEFORE PLAYING
cheats-no-trainer-tag = [ NO TRAINER ]
cheats-no-trainer = NO TRAINER FOR { $game } YET
cheats-no-trainer-body = There is no trainer for this game yet. Impulse will flag it in your library as soon as one ships.
cheats-other-store = There is none for this store yet, but its { $store } release has one. Builds can differ between stores, so it may not fit yours.
cheats-use-trainer = USE THE { $store } TRAINER
cheats-other-trainer = OTHER TRAINER...
cheats-not-installed = NOT INSTALLED · ADD THE GAME TO USE ITS MODS
cheats-loading-tag = [ LOADING ]
cheats-loading = LOADING THE CHEAT LIST
cheats-shown = { $shown } MODS
cheats-no-match = NO MODS MATCH THE FILTER
cheats-updated = GAME UPDATED
cheats-updated-body = This build is newer than the trainer supports. Mods may not work until the trainer is updated.
cheats-set = SET
cheats-set-done = OK
cheats-execute = EXECUTE
cheats-execute-done = APPLIED
cheats-save = SAVE
cheats-saved = SAVED { $saved }
cheats-save-tip = The mods a run leaves on come back on at the next one.
cheats-reset = RESET
cheats-no-collections = NO COLLECTIONS
cheats-collect-tip = Keeps the mods that are on now as a collection.
cheats-edit = EDIT
cheats-delete = DELETE
cheats-name-field = name
cheats-cancel = CANCEL
cheats-name = NAME
cheats-icon = ICON
cheats-collection-mods = { $mods } MODS
cheats-edit-collection = EDIT COLLECTION
cheats-new-collection = NEW COLLECTION

## Hotkeys

hotkeys-press-keys = PRESS KEYS
hotkeys-off = OFF
hotkeys-in-game = IN GAME
hotkeys-global = GLOBAL
hotkeys-enabled = HOTKEYS
hotkeys-enabled-hint = trainer hotkeys during a run
hotkeys-works-in = WORKS IN
hotkeys-works-in-hint = in game: only while the game is in front
hotkeys-bind-hint = ESC CANCEL · BKSP DEFAULT · DEL UNBIND
hotkeys-clash = { $keys } is used by { $name }
hotkeys-clash-overlay = { $keys } is reserved for the overlay
hotkeys-keep = KEEP BOTH
hotkeys-rebind = REBIND
hotkeys-shared-tip = another cheat has this hotkey too: both fire

## Trainer

trainer-none = NO TRAINER
trainer-mods = { $count } MODS
trainer-from = { $store } TRAINER
trainer-version = V{ $version }
trainer-version-unknown = VERSION UNKNOWN
trainer-not-installed = NOT INSTALLED
trainer-update-pending = UPDATE PENDING
trainer-sessions = { $count ->
    [one] { $count } SESSION
   *[other] { $count } SESSIONS
}
trainer-played-min = { $count } MIN PLAYED
trainer-played-hours = { $hours } H PLAYED

## Run

run-no-cheats = the cheat list is not loaded
run-preparing = PREPARING THE TRAINER
run-launching = STARTING THE GAME
run-waiting = WAITING FOR THE GAME
run-attaching = LOADING THE TRAINER
run-stopping = STOPPING THE TRAINER
run-stop = STOP
run-launch = LAUNCH + ATTACH
run-enter = ENTER
run-session = SESSION
run-last-session = LAST SESSION
run-active = ACTIVE
run-hotkeys = HOTKEYS
run-idle = TRAINER IDLE · CONTROLS UNLOCK AFTER LAUNCH
run-unconfirmed-title = GAME UPDATED
run-unconfirmed-body = This build is newer than the trainer supports, so some mods may not work until the trainer is updated. Play anyway?
run-unconfirmed-skip = ALWAYS PLAY
run-ask-cancel = CANCEL
run-unconfirmed-play = PLAY ANYWAY
run-unconfirmed-ask = ASK ON A GAME UPDATE
run-unconfirmed-ask-hint = ask before a launch when the game's build is newer than the trainer supports
run-outside-title = START THE GAME YOURSELF
run-outside-body = This trainer attaches only to a game started outside Impulse, and the game is not running. Start it, wait until it loads, then press LAUNCH + ATTACH.
run-outside-start = START GAME

## Map

teleport-failed = TELEPORT FAILED: { $why }
map-failed-tag = [ NO MAP ]
map-failed = THE MAP IS UNAVAILABLE
map-failed-body = It could not be loaded: { $why }.
map-title = MAP
map-none = NO INTERACTIVE MAP FOR THIS GAME
map-link = LINK A MAP
map-unlink = UNLINK MAP
map-link-title = LINK A MAP
map-link-caption = WHICH OF THESE TITLES IS THIS GAME?
map-link-empty = No map source has a title by that name.
map-refresh = REFRESH
map-loading-tag = [ LOADING ]
map-loading = LOADING THE MAP
map-loading-body = Pins and tiles are kept on disk after the first time.
map-search = find a place, a group, a word
map-unfinished = Still being mapped: pins may be missing or move.
map-found = FOUND
map-found-count = FOUND
map-mark-found = MARK FOUND
map-unfound = UNFOUND
map-show-all = SHOW ALL
map-hide-all = HIDE ALL
map-teleport = TELEPORT
map-filter-teleport = TELEPORT
map-teleport-here = TELEPORT HERE
map-teleport-height = TELEPORT HEIGHT
map-teleport-height-hint = Added to the height of a teleport to the map's pins and points, in the game's units
map-pin-position = A pin where the player stands
map-confirm = CONFIRM
map-delete = DELETE
map-add-pin = ADD MY PIN
map-my-pin = MY PIN
map-my-pins = MY PINS
map-pin-name = My pin { $number }
map-name-field = name this pin
map-note-field = a note
map-no-pins = Right click the map to add one.
map-regions = REGIONS
map-layer-regions = REGIONS
map-places = { $places } PLACES
map-no-matches = Nothing matches. Try a group or a shorter word.
map-nearby = NEARBY
map-copy = COPY
map-copied = COPIED
map-hide-group = HIDE { $group } ON THE MAP
map-pin-hide = HIDE THIS PIN
map-pin-unhide = SHOW THIS PIN
map-pin-move = MOVE
map-hidden-pins = HIDDEN PINS
map-pin-drag = ALT + DRAG MOVES IT
map-video = VIDEO
map-photo = PHOTO
map-layer-labels = PLACE NAMES
map-layer-clusters = GROUP CLOSE PINS
map-layer-mini-clusters = ON THE MINIMAP TOO
map-layer-rises = HEIGHTS NEARBY
map-axis-x = X
map-axis-y = Y
map-axis-z = Z

## Overlay

overlay-dock-mods = MODS
overlay-dock-map = MAP
overlay-dock-session = SESSION
overlay-dock-minimap = MINIMAP
overlay-minimap-tag = [ MINIMAP ]
overlay-no-live-map = NO LIVE MAP
overlay-no-live-map-hint = It follows the player while a trainer that can place them runs in a game with a map.
overlay-mode = MODE
overlay-mode-hint = in-game, the overlay is drawn inside the game's picture where the game allows it, and the game keeps the focus
overlay-mode-over = OVER THE GAME
overlay-mode-in-game = IN-GAME
overlay-hotkey = HOTKEY
overlay-hotkey-hint = modifiers and one key, free in other apps and mods; held only while the game is in front
overlay-minimap-hotkey = MINIMAP HOTKEY
overlay-minimap-hotkey-hint = shows or hides the minimap; held only while the game is in front
overlay-zoom-in-hotkey = MINIMAP ZOOM IN
overlay-zoom-in-hotkey-hint = brings the minimap closer; held only while the game is in front
overlay-zoom-out-hotkey = MINIMAP ZOOM OUT
overlay-zoom-out-hotkey-hint = moves the minimap further; held only while the game is in front
overlay-exclusive-title = EXCLUSIVE FULLSCREEN
overlay-exclusive-body = This game runs in exclusive fullscreen, where the overlay may not show and the game may minimize. For the best result, switch the game to borderless windowed in its video settings.
overlay-exclusive-chip = EXCLUSIVE FULLSCREEN: SWITCH THE GAME TO BORDERLESS
overlay-exclusive-skip = DON'T SHOW AGAIN
overlay-exclusive-ok = OK
overlay-exclusive-ask-hint = ask when the overlay opens over a game in exclusive fullscreen

## Settings

settings = SETTINGS
settings-appearance = APPEARANCE
settings-overlay = OVERLAY
settings-hotkeys = HOTKEYS
settings-app = APP
settings-resources = RESOURCES
settings-plugins = PLUGINS
settings-about = ABOUT
accent = ACCENT
accent-hint = marks live state only
accent-lime = LIME
accent-amber = AMBER
accent-cyan = CYAN
accent-white = WHITE
sidebar = SIDEBAR
sidebar-hint = leading item of a game row
sidebar-number = NUMBER
sidebar-thumbnail = THUMBNAIL
panels = PANELS
panels-hint = map and session over the mods, or beside them
panels-top = TOP
panels-side = SIDE
reduce-motion = REDUCE MOTION
reduce-motion-hint = no blinking LIVE, no transitions
language = LANGUAGE
language-hint = the language of Impulse's own texts
language-system = SYSTEM
close-to-tray = CLOSE TO TRAY
close-to-tray-hint = the close button hides Impulse; quit from the tray icon
hide-from-capture = HIDE FROM CAPTURE
hide-from-capture-hint = screenshots, recordings and streams show no Impulse window
hide-from-capture-inside = the in-game overlay is part of the game's picture: only Impulse's window is hidden, not what it draws inside the game
folder-path = path to the folder
folder-select = Select the folder
settings-sounds = SOUNDS
sounds-toggle-on = SWITCHED ON
sounds-toggle-on-hint = a rising tone when a mod turns on
sounds-toggle-off = SWITCHED OFF
sounds-toggle-off-hint = a falling tone when a mod turns off
sounds-slider = SLIDERS
sounds-slider-hint = a tick for each step; plays often while you drag
sounds-number = NUMBERS
sounds-number-hint = a tick when a value changes
sounds-button = BUTTONS
sounds-button-hint = a click when a button mod is applied
sounds-selection = CHOICES
sounds-selection-hint = a tone when another option is picked
sounds-listen = Listen
sounds-choose = Pick a WAV file to play instead
sounds-reset = Back to the built-in sound
sounds-refused = not a WAV file, or over 1 MB
sounds-prompt = Pick a WAV file

## Remote

remote = REMOTE PANEL
remote-open = Remote panel: control Impulse from a phone
remote-switch = PHONE ACCESS
remote-hint = control Impulse from a phone on this network
remote-address = ADDRESS
remote-address-hint = scan it with the phone's camera: the code pairs once and renews every five minutes
remote-failed = no network address to listen on
remote-devices = DEVICES
remote-devices-hint = paired phones and browsers; a revoked one is signed out at once
remote-no-devices = no device yet
remote-device = device { $number }
remote-unknown-device = BROWSER
remote-revoke = REVOKE
remote-trust = plain HTTP: anyone on this network can watch it. Trusted networks only.
remote-tips = IF THE PHONE DOES NOT CONNECT
remote-tip-network = The phone must be on this PC's network: not on mobile data, a guest Wi-Fi or a VPN.
remote-tip-firewall = Windows Firewall can block Impulse without asking, mostly on a Public network: make this network Private, or allow Impulse in Windows Security > Firewall & network protection > Allow an app through firewall.
remote-tip-isolation = Some routers keep Wi-Fi devices apart (AP or client isolation): turn it off, or try another network.
remote-note-changed = The list changed: try again.
remote-note-not-live = Start the game first.
remote-note-busy = Another run is on, or the trainer is not there yet.
remote-note-nothing-on = Turn some mods on first.
remote-note-name = Name the collection first.

## Plugins

plugins-install = INSTALL
plugins-install-hint = a folder or a .zip holding a manifest.json; you approve what it may reach first
plugins-folder = FOLDER
plugins-zip = ZIP
plugins-pick-folder = Select the plugin's folder
plugins-pick-zip = Select the plugin's .zip
plugins-none = no plugin installed
plugins-approve-title = INSTALL { $name } { $version }?
plugins-replaces = replaces version { $version }
plugins-hosts = FETCHES FROM
plugins-links = OPENS LINKS TO
plugins-no-hosts = nothing
plugins-new = NEW
plugins-sandbox = It runs in a sandbox and reaches only these hosts. Install only plugins you trust.
plugins-approve = APPROVE
plugins-cancel = CANCEL
plugins-version = version { $version }, keeps { $size }
plugins-remove = REMOVE
plugins-value = value

## Resources

media-pictures = PICTURES
media-pictures-hint = game art and pin pictures
media-tiles = MAP TILES
media-tiles-hint = the ground of the maps you opened
media-videos = VIDEOS
media-videos-hint = pin clips
resources-reading = READING SIZES...
resources-in-use = IN USE
resources-in-use-hint = everything Impulse keeps on this PC
resources-clear = CLEAR
resources-on-restart = ON RESTART
resources-clears-on-restart = CLEARS ON RESTART
resources-clear-caches = CLEAR ALL CACHES
resources-clear-caches-hint = your data below stays
resources-clear-all = CLEAR ALL
resources-limit = MEDIA LIMIT
resources-limit-hint = past it, the media unused longest goes
resources-limit-500m = 500 MB
resources-limit-1g = 1 GB
resources-limit-2g = 2 GB
resources-limit-5g = 5 GB
resources-limit-none = NONE
resources-kept = KEPT
resources-progress = MAP PROGRESS
resources-progress-hint = found pins and your own pins
resources-hotkeys = HOTKEYS
resources-hotkeys-hint = the keys you bound
resources-logs = LOGS
resources-logs-hint = two files at most, read by LOG
resources-bytes = { $size } B
resources-kilobytes = { $size } KB
resources-megabytes = { $size } MB
resources-gigabytes = { $size } GB

## Log

log-title = LOG
log-error = ERROR
log-warn = WARN
log-info = INFO
log-debug = DEBUG
log-filter = filter log
log-follow = FOLLOW
log-copy-all = COPY ALL
log-copy = COPY { $count }
log-copied = COPIED
log-clear = CLEAR
log-folder = FOLDER
log-detail-empty = CLICK A ROW TO READ AND SELECT THE FULL RECORD

## About

about-version = VERSION { $version }
about-tagline = FREE AND OPEN SOURCE
about-intro = Impulse is built by one person in their spare time. It is free and open source. A star, a bug report or a coffee is what keeps it going.
about-github = GITHUB
about-github-hint = read the code, follow the releases, leave a star so others find it
about-star = STAR
about-discord = DISCORD
about-discord-hint = ask a question, ask for a game or a feature, chat with other users
about-join = JOIN
about-bug = FOUND A BUG?
about-bug-hint = open an issue with what happened and what you expected; the LOG in the status bar helps
about-report = REPORT
about-support = SUPPORT IMPULSE
about-support-hint = free and open source; a coffee pays for the time behind each update
about-coffee = BUY A COFFEE
about-contact = CONTACT
about-contact-hint = anything else, or something private: { $address }
about-email = EMAIL
alpha = ALPHA
plugins-alpha-hint = Plugins are experimental. The format may change, and a plugin may stop working after an update.
remote-copy = copy the address

## Updates

update-title = UPDATES
update-state-idle = version { $version }
update-state-current = you are on the latest version, { $version }
update-state-skipped = version { $version } is skipped: CHECK NOW offers it again
update-state-checking = checking for updates...
update-state-available = version { $version } is available
update-state-downloading = downloading { $version }...
update-state-ready = { $version } is ready: restart Impulse to install it
update-check-failed = could not reach the release feed
update-install-failed = the update could not be installed
update-check = CHECK NOW
update-changelog = CHANGELOG
update-channel = UPDATE CHANNEL
update-channel-hint = pre-releases come earlier and may be unstable; going back to stable never downgrades
update-channel-stable = STABLE
update-channel-pre = PRE-RELEASE
update-auto = CHECK FOR UPDATES
update-auto-hint = at start and every few hours; only the release list is requested, nothing about you is sent
update-title-available = UPDATE AVAILABLE
update-title-new = WHAT'S NEW IN { $version }
update-from-to = from { $current } to { $latest }
update-later = LATER
update-skip = SKIP THIS VERSION
update-restart = RESTART NOW
update-close = CLOSE
update-busy = a trainer is running: the update installs when you quit Impulse
update-no-notes = no release notes
update-chip-available = UPDATE { $version }
update-chip-downloading = DOWNLOADING...
update-chip-ready = RESTART TO UPDATE
update-changelog-hint = what changed in every release
update-open = OPEN
