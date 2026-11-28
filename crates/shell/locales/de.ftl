## Shared

off = AUS
on = AN
# A number's marks: between thousands, before decimals.
number-group = .
number-decimal = ,

## Tray: Windows menu words

tray-open = Impulse öffnen
tray-stop = Trainer stoppen
tray-overlay = Overlay
tray-quit = Beenden

## Failures: a kind's words fit after "It could not be loaded:"

failure-offline = keine Verbindung
failure-signed-out = nicht angemeldet
failure-not-found = nicht gefunden
failure-unsupported = noch nicht unterstützt
failure-setup-needed = eine Einstellung fehlt (Einstellungen > APP)
failure-game-not-found = Spiel nicht gefunden
failure-launch-failed = der Launcher konnte das Spiel nicht starten (Details im Log)
failure-not-started = das Spiel ist nicht rechtzeitig gestartet: prüfe seinen Launcher und versuche es erneut
failure-not-running = das Spiel wurde nicht rechtzeitig gestartet: starte es selbst und versuche es erneut
failure-external-launch = dieser Trainer funktioniert nur mit einem Spiel, das du selbst startest: starte es und drücke dann START MIT TRAINER
failure-game-exited = das Spiel wurde geschlossen, bevor der Trainer hineinkam
failure-refused = abgelehnt
failure-elevation = das Spiel läuft als Administrator: bestätige die Abfrage, um dich zu verbinden
failure-protected = der Schutz des Spiels lässt keine Trainer zu
failure-needs-64 = der Trainer braucht die 64-Bit-Version des Spiels: starte diese
failure-needs-32 = der Trainer braucht die 32-Bit-Version des Spiels: starte diese
failure-load-failed = der Trainer konnte nicht ins Spiel geladen werden (Details im Log)
failure-trainer-failed = der Trainer ist im Spiel nicht gestartet (Details im Log)
failure-restart = das Spiel läuft bereits: schließe es und starte es dann von hier
failure-internal = etwas ist schiefgelaufen (Details im Log)
failure-place = { $why } (Einstellungen > APP)

## Page

catalog-failed-tag = [ KEIN KATALOG ]
catalog-failed = SPIELEKATALOG NICHT VERFÜGBAR
catalog-failed-body = Der Download ist fehlgeschlagen: { $why }. Impulse versucht es beim nächsten Start erneut.
library-filter = filtern
picker-filter = Spielname
page-search = { $trainers } Trainer durchsuchen
page-search-none = Trainer durchsuchen
page-search-key = CTRL+K
page-add-game = SPIEL HINZUFÜGEN
page-get-on = BEI { $store } HOLEN
page-add-manually = SPIEL MANUELL HINZUFÜGEN...
page-signed-out = NICHT ANGEMELDET
page-elevated = Läuft als Administrator: Spiele mit Administratorrechten werden ohne Nachfrage verbunden
page-overlay = OVERLAY
page-star = Favorit
page-folder = Spielordner öffnen
page-launcher = In { $launcher } öffnen
page-source = Spieloptionen
page-overlay-default = OVERLAY: WIE IN DEN EINSTELLUNGEN
page-overlay-in-game = OVERLAY: IM SPIEL
page-overlay-over = OVERLAY: ÜBER DEM SPIEL
page-custom = MANUELL
page-live = LIVE
page-active = { $active }/{ $total } AKTIV
page-idle = INAKTIV
page-log = LOG
page-binding = TASTEN DRÜCKEN · ESC ABBRUCH · RÜCKTASTE STANDARD
page-binding-unbind = TASTEN DRÜCKEN · ESC ABBRUCH · RÜCKTASTE STANDARD · ENTF LÖSCHEN
page-hotkeys-on = HOTKEYS AN
page-hotkeys-off = HOTKEYS AUS
disclaimer-title = BEVOR ES LOSGEHT
disclaimer-body = Impulse ist ein inoffizieller Client für { $sources }. Er nutzt die vorhandene Anmeldung und die Dateien der Original-App auf diesem PC, um Spiele und Trainer für das angemeldete Konto abzurufen. Impulse fragt nie nach dem Passwort. Abgesehen von diesen Anfragen bleiben die Kontodaten auf diesem PC. Impulse ist nicht mit dem Anbieter verbunden und wird von ihm nicht unterstützt. Wer in Online-Spielen Trainer nutzt, riskiert einen Bann. Nutzung auf eigenes Risiko.
disclaimer-accept = ICH STIMME ZU
disclaimer-exit = BEENDEN
signed-out-title = ANMELDUNG FEHLGESCHLAGEN
signed-out-body = Impulse konnte sich nicht bei { $sources } anmelden. Die App muss auf diesem PC installiert sein. Dort anmelden, die Internetverbindung prüfen und es erneut versuchen. Erst dann kann Impulse Trainer laden.
signed-out-retry = NOCH MAL
signed-out-why = Grund: { $why }
signed-out-trying = Anmeldung…
signed-out-close = SCHLIESSEN

## Library

library-favorites = FAVORITEN
library-games = SPIELE
library-no-match = KEINE TREFFER
library-add = + SPIEL
library-live = LIVE
library-scanning-tag = [ SUCHE ]
library-scanning = SUCHE NACH INSTALLIERTEN SPIELEN
library-scanning-body = Impulse gleicht den Spielekatalog mit den installierten Launchern ab.
library-empty-tag = [ KEINE SPIELE ]
library-empty = KEINE INSTALLIERTEN SPIELE GEFUNDEN
library-empty-body = Impulse sucht in Steam, Epic, GOG, Ubisoft, Xbox, Rockstar und EA nach installierten Spielen mit Trainer.
library-source-exe = EXE  { $file }
library-source-add = + .EXE
library-source-change = .EXE ÄNDERN
library-source-remove = .EXE ENTFERNEN
library-source-trainer = TRAINER WECHSELN...
library-source-own-trainer = EIGENER TRAINER

## Picker

picker-title = SPIEL HINZUFÜGEN
picker-owned = MEHRERE TREFFER. WELCHES SPIEL IST ES?
picker-unowned = KEIN TREFFER IM KATALOG. NACH SPIELNAME SUCHEN
picker-in-library = { $store } · IN BIBLIOTHEK
picker-prompt = .exe des Spiels wählen
picker-trainer-title = TRAINER WÄHLEN
picker-trainer = ZUERST ANDERE STORES, DANN NACH NAME: EIN TRAINER FÜR EINE ANDERE VERSION PASST MÖGLICHERWEISE NICHT
picker-in-use = IN VERWENDUNG
picker-find-title = SPIEL FINDEN
picker-find = ALLE TRAINER NACH NAME · ↑ ↓ ZUM WÄHLEN, ENTER ZUM ÖFFNEN

## Cheats

cheats-failed-tag = [ KEINE DATEN ]
cheats-failed = CHEATLISTE NICHT GELADEN
cheats-failed-body = Der Download ist fehlgeschlagen: { $why }. Impulse versucht es beim nächsten Öffnen des Spiels erneut.
cheats-filter = Mods filtern
cheats-pinned = ANGEHEFTET
cheats-kept = Nach dem ersten Download ist die Liste lokal gespeichert.
cheats-notes = VOR DEM SPIELEN LESEN
cheats-no-trainer-tag = [ KEIN TRAINER ]
cheats-no-trainer = NOCH KEIN TRAINER FÜR { $game }
cheats-no-trainer-body = Für dieses Spiel gibt es noch keinen Trainer. Sobald einer verfügbar ist, markiert Impulse das Spiel in der Bibliothek.
cheats-other-store = Für diesen Store gibt es noch keinen, aber die { $store }-Version hat einen. Versionen können sich zwischen Stores unterscheiden, daher passt er eventuell nicht zu deiner.
cheats-use-trainer = DEN { $store }-TRAINER VERWENDEN
cheats-other-trainer = ANDERER TRAINER...
cheats-not-installed = NICHT INSTALLIERT · SPIEL HINZUFÜGEN, UM SEINE MODS ZU NUTZEN
cheats-loading-tag = [ LADEN ]
cheats-loading = CHEATLISTE WIRD GELADEN
cheats-shown = { $count ->
    [one] { $shown } MOD
   *[other] { $shown } MODS
}
cheats-no-match = KEINE PASSENDEN MODS
cheats-updated = SPIEL-UPDATE
cheats-updated-body = Der Trainer unterstützt die aktuelle Spielversion noch nicht. Manche Mods funktionieren eventuell erst nach einem Trainer-Update.
cheats-set = OK
cheats-set-done = OK
cheats-execute = AUSFÜHREN
cheats-execute-done = OK
cheats-save = SPEICHERN
cheats-saved = GESPEICHERT { $saved }
cheats-save-tip = Mods, die am Ende einer Sitzung aktiv sind, werden beim nächsten Start wieder aktiviert.
cheats-reset = RESET
cheats-no-collections = KEINE SAMMLUNGEN
cheats-collect-tip = Speichert die aktuell aktiven Mods als Sammlung.
cheats-edit = BEARBEITEN
cheats-delete = LÖSCHEN
cheats-name-field = Name
cheats-cancel = ABBRECHEN
cheats-name = NAME
cheats-icon = SYMBOL
cheats-collection-mods = { $count ->
    [one] { $mods } MOD
   *[other] { $mods } MODS
}
cheats-edit-collection = SAMMLUNG BEARBEITEN
cheats-new-collection = NEUE SAMMLUNG

## Hotkeys

hotkeys-press-keys = TASTEN DRÜCKEN
hotkeys-off = AUS
hotkeys-in-game = IM SPIEL
hotkeys-global = GLOBAL
hotkeys-enabled = HOTKEYS
hotkeys-enabled-hint = Trainer-Hotkeys während einer Sitzung
hotkeys-works-in = AKTIV IN
hotkeys-works-in-hint = „Im Spiel“: Hotkeys nur bei aktivem Spielfenster
hotkeys-bind-hint = ESC ABBRUCH · RÜCKTASTE STANDARD · ENTF LÖSCHEN
hotkeys-clash = { $keys } ist schon belegt durch { $name }
hotkeys-clash-overlay = { $keys } ist für das Overlay reserviert
hotkeys-keep = BEIDE BEHALTEN
hotkeys-rebind = NEU BELEGEN
hotkeys-shared-tip = ein anderer Mod hat diesen Hotkey auch: beide werden ausgelöst

## Trainer

trainer-none = KEIN TRAINER
trainer-mods = { $count ->
    [one] { $count } MOD
   *[other] { $count } MODS
}
trainer-from = { $store }-TRAINER
trainer-version = V{ $version }
trainer-version-unknown = VERSION UNBEKANNT
trainer-not-installed = NICHT INSTALLIERT
trainer-update-pending = UPDATE AUSSTEHEND
trainer-sessions = { $count ->
    [one] { $count } SITZUNG
   *[other] { $count } SITZUNGEN
}
trainer-played-min = { $count } MIN GESPIELT
trainer-played-hours = { $hours } H GESPIELT

## Run

run-no-cheats = Cheatliste nicht geladen
run-preparing = TRAINER WIRD VORBEREITET
run-launching = SPIEL STARTET
run-waiting = WARTE AUF DAS SPIEL
run-attaching = TRAINER WIRD GELADEN
run-stopping = TRAINER STOPPT
run-stop = STOPP
run-launch = START MIT TRAINER
run-enter = ENTER
run-session = SITZUNG
run-last-session = LETZTE SITZUNG
run-active = AKTIV
run-hotkeys = HOTKEYS
run-idle = TRAINER INAKTIV · MODS ERST NACH DEM START
run-unconfirmed-title = SPIEL-UPDATE
run-unconfirmed-body = Der Trainer unterstützt diese Spielversion noch nicht. Manche Mods funktionieren eventuell erst nach einem Trainer-Update. Trotzdem spielen?
run-unconfirmed-skip = IMMER SPIELEN
run-ask-cancel = ABBRECHEN
run-unconfirmed-play = TROTZDEM SPIELEN
run-unconfirmed-ask = BEI SPIEL-UPDATE FRAGEN
run-unconfirmed-ask-hint = vor dem Start fragen, wenn die Spielversion neuer ist als die vom Trainer unterstützte
run-outside-title = STARTE DAS SPIEL SELBST
run-outside-body = Dieser Trainer verbindet sich nur mit einem außerhalb von Impulse gestarteten Spiel, und das Spiel läuft nicht. Starte es, warte, bis es geladen ist, und drücke dann START MIT TRAINER.
run-outside-start = SPIEL STARTEN

## Map

teleport-failed = TELEPORT FEHLGESCHLAGEN: { $why }
map-failed-tag = [ KEINE KARTE ]
map-failed = KARTE NICHT VERFÜGBAR
map-failed-body = Die Karte konnte nicht geladen werden: { $why }.
map-title = KARTE
map-none = KEINE INTERAKTIVE KARTE FÜR DIESES SPIEL
map-link = KARTE VERKNÜPFEN
map-unlink = VERKNÜPFUNG LÖSEN
map-link-title = KARTE VERKNÜPFEN
map-link-caption = WELCHER EINTRAG PASST ZUM SPIEL?
map-link-empty = Unter diesem Namen gibt es bei keinem Kartenanbieter einen Treffer.
map-refresh = NEU LADEN
map-loading-tag = [ LADEN ]
map-loading = KARTE WIRD GELADEN
map-loading-body = Marker und Kartenkacheln werden beim ersten Laden lokal gespeichert.
map-search = Ort, Gruppe oder Stichwort
map-unfinished = Die Karte ist noch in Arbeit: Manche Marker fehlen oder können sich noch ändern.
map-found = GEFUNDEN
map-found-count = GEFUNDEN
map-mark-found = ABHAKEN
map-unfound = OFFEN
map-show-all = ALLE ZEIGEN
map-hide-all = ALLE AUSBLENDEN
map-teleport = TELEPORT
map-filter-teleport = TELEPORT
map-teleport-here = TELEPORT HIERHER
map-teleport-height = TELEPORT-HÖHE
map-teleport-height-hint = Wird zur Höhe von Teleports zu Markern und Punkten der Karte addiert, in Spieleinheiten
map-pin-position = Ein Marker dort, wo der Spieler steht
map-confirm = BESTÄTIGEN
map-delete = LÖSCHEN
map-add-pin = NEUER MARKER
map-my-pin = MEIN MARKER
map-my-pins = MEINE MARKER
map-pin-name = Mein Marker { $number }
map-name-field = Name des Markers
map-note-field = Notiz
map-no-pins = Per Rechtsklick auf die Karte einen Marker setzen.
map-regions = REGIONEN
map-layer-regions = REGIONEN
map-places = { $count ->
    [one] { $places } ORT
   *[other] { $places } ORTE
}
map-no-matches = Keine Treffer. Nach einer Gruppe oder einem kürzeren Begriff suchen.
map-nearby = UMGEBUNG
map-copy = KOPIEREN
map-copied = KOPIERT
map-hide-group = { $group } AUF DER KARTE AUSBLENDEN
map-pin-hide = DIESEN MARKER AUSBLENDEN
map-pin-unhide = DIESEN MARKER ZEIGEN
map-pin-move = VERSCHIEBEN
map-add-media = MEDIEN HINZUFÜGEN
map-hidden-pins = AUSGEBLENDETE MARKER
map-pin-drag = ALT + ZIEHEN VERSCHIEBT IHN
map-video = VIDEO
map-photo = FOTO
map-layer-labels = ORTSNAMEN
map-layer-clusters = NAHE MARKER BÜNDELN
map-layer-mini-clusters = AUCH AUF DER MINIMAP
map-layer-rises = HÖHEN IN DER NÄHE
map-axis-x = X
map-axis-y = Y
map-axis-z = Z

## Overlay

overlay-dock-mods = MODS
overlay-dock-map = KARTE
overlay-dock-session = SITZUNG
overlay-dock-minimap = MINIMAP
overlay-minimap-tag = [ MINIMAP ]
overlay-no-live-map = KEINE LIVE-KARTE
overlay-no-live-map-hint = Die Minimap folgt der Spielfigur, wenn das Spiel eine Karte hat und der aktive Trainer die Position erkennen kann.
overlay-mode = MODUS
overlay-mode-hint = im Spiel wird das Overlay direkt ins Bild des Spiels gezeichnet, wo das Spiel es zulässt, und das Spiel behält den Fokus
overlay-mode-over = ÜBER DEM SPIEL
overlay-mode-in-game = IM SPIEL
overlay-hotkey = HOTKEY
overlay-hotkey-hint = Mindestens eine Taste aus Ctrl, Shift, Alt und Win plus eine weitere Taste. Die Kombination darf nicht schon in anderen Apps oder Mods belegt sein. Sie wirkt nur bei aktivem Spielfenster.
overlay-minimap-hotkey = MINIMAP-HOTKEY
overlay-minimap-hotkey-hint = blendet die Minimap ein oder aus, nur bei aktivem Spielfenster
overlay-zoom-in-hotkey = MINIMAP-ZOOM +
overlay-zoom-in-hotkey-hint = zoomt auf der Minimap rein, nur bei aktivem Spielfenster
overlay-zoom-out-hotkey = MINIMAP-ZOOM −
overlay-zoom-out-hotkey-hint = zoomt auf der Minimap raus, nur bei aktivem Spielfenster
overlay-pin-here-hotkey = MARKER HIER SETZEN
overlay-pin-here-hotkey-hint = setzt einen Marker an der Spielerposition, mit Spielbild; nur bei aktivem Spielfenster
overlay-exclusive-title = EXKLUSIVES VOLLBILD
overlay-exclusive-body = Im exklusiven Vollbild kann es sein, dass das Overlay nicht erscheint oder sich das Spiel minimiert. Am besten in den Videoeinstellungen des Spiels auf „Randloses Fenster“ umstellen.
overlay-exclusive-chip = EXKLUSIVES VOLLBILD · AUF RANDLOS WECHSELN
overlay-exclusive-skip = NICHT MEHR ZEIGEN
overlay-exclusive-ok = OK
overlay-exclusive-ask-hint = nachfragen, wenn das Overlay über einem Spiel im exklusiven Vollbild geöffnet wird.

## Settings

settings = EINSTELLUNGEN
settings-appearance = DESIGN
settings-overlay = OVERLAY
settings-hotkeys = HOTKEYS
settings-app = APP
settings-resources = SPEICHER
settings-plugins = PLUGINS
settings-about = INFO
accent = AKZENT
accent-hint = Farbe nur für Live-Anzeigen
accent-lime = LIMETTE
accent-amber = BERNSTEIN
accent-cyan = CYAN
accent-white = WEISS
sidebar = SEITENLEISTE
sidebar-hint = was in der Spieleliste vor dem Titel steht
sidebar-number = NUMMER
sidebar-thumbnail = VORSCHAU
panels = LAYOUT
panels-hint = Karte und Sitzung über den Mods oder daneben
panels-top = OBEN
panels-side = DANEBEN
reduce-motion = WENIGER ANIMATION
reduce-motion-hint = keine animierten Übergänge und kein blinkendes LIVE
language = SPRACHE
language-hint = Anzeigesprache von Impulse
language-system = SYSTEM
close-to-tray = IN DEN TRAY
close-to-tray-hint = Beim Schließen läuft Impulse im Tray weiter. Beenden über das Tray-Symbol.
hide-from-capture = IN AUFNAHMEN AUSBLENDEN
hide-from-capture-hint = Das Impulse-Fenster bleibt in Screenshots, Aufnahmen und Streams ausgeblendet.
hide-from-capture-inside = Das Overlay im Spiel gehört zum Bild des Spiels: Nur das Impulse-Fenster wird ausgeblendet, nicht das, was Impulse im Spiel zeichnet.
folder-path = Pfad zum Ordner
folder-select = Ordner wählen
settings-sounds = TÖNE
sounds-toggle-on = EINSCHALTEN
sounds-toggle-on-hint = ein ansteigender Ton beim Einschalten eines Mods
sounds-toggle-off = AUSSCHALTEN
sounds-toggle-off-hint = ein abfallender Ton beim Ausschalten eines Mods
sounds-slider = REGLER
sounds-slider-hint = ein kurzes Ticken bei jeder Reglerstufe, auch beim Ziehen
sounds-number = ZAHLEN
sounds-number-hint = ein kurzes Ticken, wenn sich ein Wert ändert
sounds-button = BUTTONS
sounds-button-hint = ein Klick, wenn ein Mod per Button ausgeführt wird
sounds-selection = AUSWAHL
sounds-selection-hint = ein Ton beim Wechsel zu einer anderen Option
sounds-listen = Anhören
sounds-choose = Stattdessen eine WAV-Datei wählen
sounds-reset = Standardton wiederherstellen
sounds-refused = keine WAV-Datei oder größer als 1 MB
sounds-prompt = WAV-Datei wählen

## Remote

remote = REMOTE-PANEL
remote-open = Remote-Panel: Impulse vom Handy aus steuern
remote-switch = HANDYZUGRIFF
remote-hint = Impulse über ein Handy im selben Netzwerk steuern
remote-address = ADRESSE
remote-address-hint = QR-Code mit der Handykamera scannen. Er ist nur einmal gültig und wird alle fünf Minuten erneuert.
remote-failed = keine nutzbare Netzwerkadresse gefunden
remote-devices = GERÄTE
remote-devices-hint = gekoppelte Handys und Browser; beim Entkoppeln endet der Zugriff sofort
remote-no-devices = noch kein Gerät
remote-device = Gerät { $number }
remote-unknown-device = BROWSER
remote-revoke = ENTKOPPELN
remote-trust = Unverschlüsseltes HTTP: Andere im selben Netzwerk können mitlesen. Nur in vertrauenswürdigen Netzwerken nutzen.
remote-tips = WENN SICH DAS HANDY NICHT VERBINDET
remote-tip-network = Das Handy muss im selben Netzwerk wie der PC sein. Mobilfunk, Gast-WLAN oder VPN funktionieren dafür nicht.
remote-tip-firewall = Die Windows-Firewall kann Impulse ohne Nachfrage blockieren, besonders in öffentlichen Netzwerken. Das Netzwerkprofil auf „Privat“ umstellen oder eine Ausnahme für Impulse hinzufügen: Windows-Sicherheit > Firewall- & Netzwerkschutz > App durch die Firewall zulassen.
remote-tip-isolation = Manche Router trennen Geräte im WLAN voneinander (AP- oder Client-Isolation). Die Einstellung im Router abschalten oder ein anderes Netzwerk nutzen.
remote-note-changed = Die Liste hat sich geändert. Bitte noch mal versuchen.
remote-note-not-live = Zuerst das Spiel starten.
remote-note-busy = Es läuft schon eine andere Sitzung oder der Trainer ist noch nicht verfügbar.
remote-note-nothing-on = Zuerst Mods aktivieren.
remote-note-name = Zuerst die Sammlung benennen.

## Plugins

plugins-install = INSTALLIEREN
plugins-install-hint = einen Ordner oder eine .zip mit manifest.json wählen und vor der Installation die Zugriffe bestätigen
plugins-folder = ORDNER
plugins-zip = ZIP
plugins-pick-folder = Plugin-Ordner wählen
plugins-pick-zip = ZIP-Datei des Plugins wählen
plugins-none = kein Plugin installiert
plugins-approve-title = { $name } { $version } INSTALLIEREN?
plugins-replaces = ersetzt Version { $version }
plugins-hosts = LÄDT VON
plugins-links = LINKS ZU
plugins-no-hosts = keine
plugins-new = NEU
plugins-sandbox = Das Plugin läuft in einer Sandbox und darf nur auf die aufgeführten Domains zugreifen. Nur vertrauenswürdige Plugins installieren.
plugins-approve = ZULASSEN
plugins-cancel = ABBRECHEN
plugins-version = Version { $version }, belegt { $size }
plugins-remove = ENTFERNEN
plugins-value = Wert

## Resources

media-pictures = BILDER
media-pictures-hint = Bilder zu Spielen und Markern
media-tiles = KARTENKACHELN
media-tiles-hint = Kartenhintergründe aus bisher geöffneten Karten
media-videos = VIDEOS
media-videos-hint = Videos zu Markern
resources-reading = SPEICHERBELEGUNG …
resources-in-use = BELEGT
resources-in-use-hint = alles, was Impulse auf diesem PC speichert
resources-clear = LEEREN
resources-on-restart = BEIM NEUSTART
resources-clears-on-restart = LEERUNG BEIM NEUSTART
resources-clear-caches = ALLE CACHES LEEREN
resources-clear-caches-hint = eigene Daten bleiben erhalten
resources-clear-all = ALLES LEEREN
resources-limit = MEDIENLIMIT
resources-limit-hint = Wird das Limit überschritten, löscht Impulse zuerst die am längsten nicht genutzten Mediendateien.
resources-limit-500m = 500 MB
resources-limit-1g = 1 GB
resources-limit-2g = 2 GB
resources-limit-5g = 5 GB
resources-limit-none = OHNE
resources-kept = EIGENE DATEN
resources-progress = KARTENFORTSCHRITT
resources-progress-hint = gefundene und eigene Marker
resources-hotkeys = HOTKEYS
resources-hotkeys-hint = gespeicherte Tastenbelegungen
resources-logs = LOGS
resources-logs-hint = bis zu zwei Dateien, einsehbar unter LOG
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
log-filter = Log filtern
log-follow = MITSCROLLEN
log-copy-all = ALLE KOPIEREN
log-copy = KOPIEREN ({ $count })
log-copied = KOPIERT
log-clear = LEEREN
log-folder = ORDNER
log-detail-empty = ZEILE ANKLICKEN: EINTRAG LESEN UND TEXT MARKIEREN

## About

about-version = VERSION { $version }
about-tagline = KOSTENLOS UND OPEN SOURCE
about-intro = Impulse wird von einer einzelnen Person in der Freizeit entwickelt. Es ist kostenlos und quelloffen. Ein Stern, ein Fehlerbericht oder ein Kaffee halten das Projekt am Laufen.
about-github = GITHUB
about-github-hint = Code lesen, Releases verfolgen und mit einem Stern helfen, dass andere es finden.
about-star = STERN
about-discord = DISCORD
about-discord-hint = Fragen stellen, ein Spiel oder eine Funktion vorschlagen, mit anderen Nutzern plaudern.
about-join = BEITRETEN
about-bug = FEHLER GEFUNDEN?
about-bug-hint = ein Issue eröffnen: was passiert ist und was erwartet wurde. Das LOG in der Statusleiste hilft dabei.
about-report = MELDEN
about-support = IMPULSE UNTERSTÜTZEN
about-support-hint = kostenlos und quelloffen; ein Kaffee bezahlt die Zeit, die in jedes Update fließt.
about-coffee = KAFFEE SPENDEN
about-contact = KONTAKT
about-contact-hint = für alles andere oder etwas Privates: { $address }
about-email = E-MAIL
alpha = ALPHA
plugins-alpha-hint = Plugins sind experimentell. Das Format kann sich ändern, und ein Plugin kann nach einem Update nicht mehr funktionieren.
remote-copy = Adresse kopieren

## Updates

update-title = UPDATES
update-state-idle = Version { $version }
update-state-current = die neueste Version ist installiert: { $version }
update-state-skipped = Version { $version } übersprungen: JETZT PRÜFEN bietet sie wieder an
update-state-checking = Suche nach Updates...
update-state-available = Version { $version } ist verfügbar
update-state-downloading = { $version } wird heruntergeladen...
update-state-ready = { $version } ist bereit: Impulse neu starten, um sie zu installieren
update-check-failed = der Release-Feed ist nicht erreichbar
update-install-failed = das Update konnte nicht installiert werden
update-check = JETZT PRÜFEN
update-changelog = ÄNDERUNGEN
update-channel = UPDATE-KANAL
update-channel-hint = Vorabversionen kommen früher und können instabil sein; der Wechsel zurück zu Stabil installiert nie eine ältere Version
update-channel-stable = STABIL
update-channel-pre = VORABVERSION
update-auto = NACH UPDATES SUCHEN
update-auto-hint = beim Start und alle paar Stunden; abgerufen wird nur die Release-Liste, über dich wird nichts gesendet
update-title-available = UPDATE VERFÜGBAR
update-title-new = NEU IN { $version }
update-from-to = von { $current } auf { $latest }
update-later = SPÄTER
update-skip = DIESE VERSION ÜBERSPRINGEN
update-restart = JETZT NEU STARTEN
update-close = SCHLIESSEN
update-busy = ein Trainer läuft: das Update wird installiert, sobald Impulse beendet wird
update-no-notes = keine Versionshinweise
update-chip-available = UPDATE { $version }
update-chip-downloading = LÄDT...
update-chip-ready = NEUSTART ZUM UPDATE
update-changelog-hint = was sich in jeder Version geändert hat
update-open = ÖFFNEN
