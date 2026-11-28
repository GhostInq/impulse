## Shared

off = OFF
on = ON
# A number's marks: between thousands, before decimals.
number-group = .
number-decimal = ,

## Tray: Windows menu words

tray-open = Apri Impulse
tray-stop = Ferma il trainer
tray-overlay = Overlay
tray-quit = Esci

## Failures: a kind's words fit after "It could not be loaded:"

failure-offline = nessuna connessione
failure-signed-out = accesso non effettuato
failure-not-found = non trovato
failure-unsupported = non ancora supportato
failure-setup-needed = configurazione incompleta: vedi Impostazioni > APP
failure-game-not-found = gioco non trovato
failure-launch-failed = il launcher non è riuscito ad avviare il gioco: i dettagli sono nel log
failure-not-started = il gioco non si è avviato in tempo: controlla il suo launcher e riprova
failure-not-running = il gioco non è stato avviato in tempo: avvialo tu e riprova
failure-external-launch = questo trainer funziona solo con un gioco avviato da te: avvialo, poi premi GIOCA CON TRAINER
failure-game-exited = il gioco si è chiuso prima che il trainer entrasse
failure-refused = richiesta respinta
failure-elevation = il gioco è in esecuzione come amministratore: accetta la richiesta per collegarti
failure-protected = la protezione del gioco blocca i trainer
failure-needs-64 = il trainer richiede la versione a 64 bit del gioco: avvia quella
failure-needs-32 = il trainer richiede la versione a 32 bit del gioco: avvia quella
failure-load-failed = non è stato possibile caricare il trainer nel gioco: i dettagli sono nel log
failure-trainer-failed = il trainer non si è avviato nel gioco: i dettagli sono nel log
failure-restart = il gioco è già in esecuzione: chiudilo, poi avvialo da qui
failure-internal = qualcosa è andato storto: i dettagli sono nel log
failure-place = { $why }: vedi Impostazioni > APP

## Page

catalog-failed-tag = [ NESSUN CATALOGO ]
catalog-failed = IL CATALOGO DEI GIOCHI NON È DISPONIBILE
catalog-failed-body = Impossibile scaricare il catalogo: { $why }. Impulse riprova al prossimo avvio.
library-filter = filtra
picker-filter = nome del gioco
page-search = cerca in { $trainers } trainer
page-search-none = cerca trainer
page-search-key = CTRL K
page-add-game = AGGIUNGI GIOCO
page-get-on = OTTIENI SU { $store }
page-add-manually = AGGIUNGI GIOCO MANUALMENTE...
page-signed-out = SENZA ACCESSO
page-elevated = In esecuzione come amministratore: i giochi avviati come amministratore si collegano senza richiesta
page-overlay = OVERLAY
page-star = Preferito
page-folder = Apri la cartella del gioco
page-launcher = Apri in { $launcher }
page-source = Opzioni del gioco
page-overlay-default = OVERLAY: COME NELLE IMPOSTAZIONI
page-overlay-in-game = OVERLAY: IN GIOCO
page-overlay-over = OVERLAY: SOPRA IL GIOCO
page-custom = MANUALE
page-live = LIVE
page-active = { $active ->
    [one] { $active }/{ $total } ATTIVA
   *[other] { $active }/{ $total } ATTIVE
}
page-idle = FERMO
page-log = LOG
page-binding = PREMI I TASTI · ESC ANNULLA · BKSP RIPRISTINA
page-binding-unbind = PREMI I TASTI · ESC ANNULLA · BKSP RIPRISTINA · DEL RIMUOVI
page-hotkeys-on = HOTKEY ON
page-hotkeys-off = HOTKEY OFF
disclaimer-title = PRIMA DI INIZIARE
disclaimer-body = Impulse è un client non ufficiale che usa l'account { $sources } e l'installazione già presenti su questo PC. Usa la sessione aperta nell'app e i file della sua cartella per recuperare dal servizio i tuoi giochi e trainer. Non ti chiede mai la password: a parte le richieste al servizio, i dati dell'account restano sul PC. Impulse non ha legami con il servizio e non ha la sua approvazione. Usare i trainer nei giochi online può costarti un ban: usali a tuo rischio.
disclaimer-accept = ACCETTO
disclaimer-exit = ESCI
signed-out-title = ACCESSO NON RIUSCITO
signed-out-body = Impulse non è riuscito ad accedere a { $sources }. Assicurati che l'app sia installata su questo PC. Aprila, accedi al tuo account e verifica di essere online. Poi riprova: senza accesso, Impulse non può caricare i trainer.
signed-out-retry = RIPROVA
signed-out-why = Motivo: { $why }
signed-out-trying = Accesso in corso…
signed-out-close = CHIUDI

## Library

library-favorites = PREFERITI
library-games = GIOCHI
library-no-match = NESSUN RISULTATO
library-add = + GIOCO
library-live = LIVE
library-scanning-tag = [ RICERCA ]
library-scanning = RICERCA DEI GIOCHI INSTALLATI
library-scanning-body = Impulse controlla il catalogo dei giochi e i tuoi launcher.
library-empty-tag = [ NESSUN GIOCO ]
library-empty = NESSUN GIOCO RILEVATO
library-empty-body = Impulse cerca i giochi con un trainer tra quelli installati da Steam, Epic, GOG, Ubisoft, Xbox, Rockstar ed EA.
library-source-exe = EXE  { $file }
library-source-add = + FILE .EXE
library-source-change = CAMBIA .EXE
library-source-remove = RIMUOVI .EXE
library-source-trainer = CAMBIA TRAINER...
library-source-own-trainer = IL SUO TRAINER

## Picker

picker-title = AGGIUNGI UN GIOCO
picker-owned = PIÙ RISULTATI: QUAL È IL GIOCO?
picker-unowned = GIOCO NON RICONOSCIUTO: CERCALO PER NOME
picker-in-library = { $store }  IN LIBRERIA
picker-prompt = Seleziona il file .exe del gioco
picker-trainer-title = SCEGLI UN TRAINER
picker-trainer = PRIMA GLI ALTRI STORE, POI PER NOME: UN TRAINER PER UN'ALTRA VERSIONE POTREBBE NON ADATTARSI
picker-in-use = IN USO
picker-find-title = CERCA UN GIOCO
picker-find = TUTTI I TRAINER PER NOME · ↑ ↓ PER SCEGLIERE, INVIO PER APRIRE

## Cheats

cheats-failed-tag = [ NESSUN DATO ]
cheats-failed = ELENCO MOD NON DISPONIBILE
cheats-failed-body = Impossibile scaricare l'elenco: { $why }. Impulse riprova quando riapri il gioco.
cheats-filter = filtra mod
cheats-pinned = FISSATE
cheats-kept = Dopo il primo download, l'elenco resta sul disco.
cheats-notes = DA LEGGERE PRIMA DI GIOCARE
cheats-no-trainer-tag = [ NESSUN TRAINER ]
cheats-no-trainer = NESSUN TRAINER PER { $game }
cheats-no-trainer-body = Non c'è ancora un trainer per questo gioco. Appena ne esce uno, Impulse te lo segnala in libreria.
cheats-other-store = Per questo store non ce n'è ancora uno, ma la versione { $store } ne ha uno. Le versioni possono differire da uno store all'altro, quindi potrebbe non adattarsi alla tua.
cheats-use-trainer = USA IL TRAINER { $store }
cheats-other-trainer = ALTRO TRAINER...
cheats-not-installed = NON INSTALLATO · AGGIUNGI IL GIOCO PER USARE LE SUE MOD
cheats-loading-tag = [ CARICAMENTO ]
cheats-loading = CARICAMENTO ELENCO MOD
cheats-shown = { $shown } MOD
cheats-no-match = NESSUNA MOD CORRISPONDE AL FILTRO
cheats-updated = GIOCO AGGIORNATO
cheats-updated-body = Il gioco è più recente della versione supportata dal trainer. Le mod potrebbero non funzionare fino al prossimo aggiornamento del trainer.
cheats-set = USA
cheats-set-done = OK
cheats-execute = ESEGUI
cheats-execute-done = FATTO
cheats-save = SALVA
cheats-saved = { $count ->
    [one] { $saved } SALVATA
   *[other] { $saved } SALVATE
}
cheats-save-tip = Le mod che lasci attive si riattivano alla prossima sessione.
cheats-reset = RESET
cheats-no-collections = NESSUNA COLLEZIONE
cheats-collect-tip = Salva le mod attive in una collezione.
cheats-edit = MODIFICA
cheats-delete = ELIMINA
cheats-name-field = nome
cheats-cancel = ANNULLA
cheats-name = NOME
cheats-icon = ICONA
cheats-collection-mods = { $mods } MOD
cheats-edit-collection = MODIFICA COLLEZIONE
cheats-new-collection = NUOVA COLLEZIONE

## Hotkeys

hotkeys-press-keys = PREMI I TASTI
hotkeys-off = OFF
hotkeys-in-game = IN GIOCO
hotkeys-global = GLOBALE
hotkeys-enabled = HOTKEY
hotkeys-enabled-hint = hotkey del trainer durante una sessione
hotkeys-works-in = DOVE
hotkeys-works-in-hint = in gioco: solo con il gioco in primo piano
hotkeys-bind-hint = ESC ANNULLA · BKSP RIPRISTINA · DEL RIMUOVI
hotkeys-clash = { $keys } è già usato da { $name }
hotkeys-clash-overlay = { $keys } è riservato all'overlay
hotkeys-keep = TIENI ENTRAMBI
hotkeys-rebind = RIASSEGNA
hotkeys-shared-tip = anche un altro mod ha questa hotkey: scattano entrambi

## Trainer

trainer-none = NESSUN TRAINER
trainer-mods = { $count } MOD
trainer-from = TRAINER { $store }
trainer-version = V{ $version }
trainer-version-unknown = VERSIONE SCONOSCIUTA
trainer-not-installed = NON INSTALLATO
trainer-update-pending = IN ATTESA DI PATCH
trainer-sessions = { $count ->
    [one] { $count } SESSIONE
   *[other] { $count } SESSIONI
}
trainer-played-min = { $count } MIN DI GIOCO
trainer-played-hours = { $hours } H DI GIOCO

## Run

run-no-cheats = elenco mod non caricato
run-preparing = PREPARAZIONE TRAINER
run-launching = AVVIO DEL GIOCO
run-waiting = IN ATTESA DEL GIOCO
run-attaching = CARICAMENTO TRAINER
run-stopping = CHIUSURA TRAINER
run-stop = STOP
run-launch = GIOCA CON TRAINER
run-enter = INVIO
run-session = SESSIONE
run-last-session = ULTIMA SESSIONE
run-active = ATTIVE
run-hotkeys = HOTKEY
run-idle = TRAINER FERMO · COMANDI DISPONIBILI DOPO L'AVVIO
run-unconfirmed-title = GIOCO AGGIORNATO
run-unconfirmed-body = Il gioco è più recente della versione supportata dal trainer. Le mod potrebbero non funzionare fino al prossimo aggiornamento del trainer. Giocare comunque?
run-unconfirmed-skip = GIOCA SEMPRE
run-ask-cancel = ANNULLA
run-unconfirmed-play = GIOCA COMUNQUE
run-unconfirmed-ask = CHIEDI SE IL GIOCO SI AGGIORNA
run-unconfirmed-ask-hint = chiedi prima di avviare quando il gioco è più recente della versione supportata dal trainer
run-outside-title = AVVIA TU IL GIOCO
run-outside-body = Questo trainer si collega solo a un gioco avviato fuori da Impulse, e il gioco non è in esecuzione. Avvialo, attendi che si carichi, poi premi GIOCA CON TRAINER.
run-outside-start = AVVIA GIOCO

## Map

teleport-failed = TELETRASPORTO NON RIUSCITO: { $why }
map-failed-tag = [ NESSUNA MAPPA ]
map-failed = LA MAPPA NON È DISPONIBILE
map-failed-body = Impossibile caricare la mappa: { $why }.
map-title = MAPPA
map-none = NESSUNA MAPPA INTERATTIVA PER QUESTO GIOCO
map-link = COLLEGA MAPPA
map-unlink = SCOLLEGA MAPPA
map-link-title = COLLEGA MAPPA
map-link-caption = A QUALE TITOLO CORRISPONDE IL GIOCO?
map-link-empty = Nessuna mappa trovata con questo titolo.
map-refresh = AGGIORNA
map-loading-tag = [ CARICAMENTO ]
map-loading = CARICAMENTO DELLA MAPPA
map-loading-body = Al primo caricamento, i pin e i riquadri della mappa vengono salvati sul disco.
map-search = cerca per luogo, gruppo o parola
map-unfinished = Mappatura ancora in corso: alcuni pin potrebbero mancare o spostarsi.
map-found = TROVATO
map-found-count = TROVATI
map-mark-found = L'HO TROVATO
map-unfound = DA TROVARE
map-show-all = MOSTRA TUTTO
map-hide-all = NASCONDI TUTTO
map-teleport = TELETRASPORTO
map-filter-teleport = CON TP
map-teleport-here = TELETRASPORTA QUI
map-teleport-height = ALTEZZA DEL TELETRASPORTO
map-teleport-height-hint = Si aggiunge all'altezza del teletrasporto verso i pin e i punti della mappa, in unità di gioco
map-pin-position = Un pin dove si trova il giocatore
map-confirm = CONFERMA
map-delete = ELIMINA
map-add-pin = AGGIUNGI PIN
map-my-pin = IL MIO PIN
map-my-pins = I MIEI PIN
map-pin-name = Pin { $number }
map-name-field = nome del pin
map-note-field = nota
map-no-pins = Clic destro sulla mappa per aggiungerne uno.
map-regions = REGIONI
map-layer-regions = REGIONI
map-places = { $count ->
    [one] { $places } LUOGO
   *[other] { $places } LUOGHI
}
map-no-matches = Nessun risultato. Prova con un gruppo o una parola più breve.
map-nearby = VICINI
map-copy = COPIA
map-copied = COPIATO
map-hide-group = NASCONDI { $group } SULLA MAPPA
map-pin-hide = NASCONDI QUESTO PIN
map-pin-unhide = MOSTRA QUESTO PIN
map-pin-move = SPOSTA
map-add-media = AGGIUNGI MEDIA
map-hidden-pins = PIN NASCOSTI
map-pin-drag = ALT + TRASCINA PER SPOSTARLO
map-video = VIDEO
map-photo = FOTO
map-layer-labels = NOMI DEI LUOGHI
map-layer-clusters = RAGGRUPPA PIN VICINI
map-layer-mini-clusters = ANCHE SULLA MINIMAPPA
map-layer-rises = ALTEZZE VICINE
map-axis-x = X
map-axis-y = Y
map-axis-z = Z

## Overlay

overlay-dock-mods = MOD
overlay-dock-map = MAPPA
overlay-dock-session = SESSIONE
overlay-dock-minimap = MINIMAPPA
overlay-minimap-tag = [ MINIMAPPA ]
overlay-no-live-map = SENZA MAPPA LIVE
overlay-no-live-map-hint = La minimappa segue i tuoi movimenti se il gioco ha una mappa e il trainer attivo può rilevare la tua posizione.
overlay-mode = MODALITÀ
overlay-mode-hint = in gioco, l'overlay viene disegnato dentro l'immagine del gioco se il gioco lo consente, e il gioco mantiene il focus
overlay-mode-over = SOPRA IL GIOCO
overlay-mode-in-game = IN GIOCO
overlay-hotkey = HOTKEY
overlay-hotkey-hint = scegli una combinazione non usata da altre app o mod: modificatori come CTRL e un solo altro tasto; attiva solo con il gioco in primo piano
overlay-minimap-hotkey = HOTKEY MINIMAPPA
overlay-minimap-hotkey-hint = mostra o nasconde la minimappa solo con il gioco in primo piano
overlay-zoom-in-hotkey = ZOOM + MINIMAPPA
overlay-zoom-in-hotkey-hint = aumenta lo zoom della minimappa solo con il gioco in primo piano
overlay-zoom-out-hotkey = ZOOM - MINIMAPPA
overlay-zoom-out-hotkey-hint = riduce lo zoom della minimappa solo con il gioco in primo piano
overlay-pin-here-hotkey = PIN QUI
overlay-pin-here-hotkey-hint = un pin dove si trova il giocatore, con uno screenshot del gioco; attivo solo con il gioco in primo piano
overlay-exclusive-title = SCHERMO INTERO ESCLUSIVO
overlay-exclusive-body = In modalità schermo intero esclusivo, l'overlay potrebbe non comparire e il gioco potrebbe ridursi a icona. Meglio usare la modalità Finestra senza bordi nelle impostazioni video del gioco.
overlay-exclusive-chip = SCHERMO INTERO ESCLUSIVO: USA FINESTRA SENZA BORDI
overlay-exclusive-skip = NON MOSTRARE PIÙ
overlay-exclusive-ok = OK
overlay-exclusive-ask-hint = chiedi quando l'overlay si apre su un gioco a schermo intero esclusivo

## Settings

settings = IMPOSTAZIONI
settings-appearance = ASPETTO
settings-overlay = OVERLAY
settings-hotkeys = HOTKEY
settings-app = APP
settings-resources = RISORSE
settings-plugins = PLUGIN
settings-about = INFO
accent = COLORE
accent-hint = evidenzia solo lo stato live
accent-lime = LIME
accent-amber = AMBRA
accent-cyan = CIANO
accent-white = BIANCO
sidebar = BARRA LATERALE
sidebar-hint = cosa mostrare all'inizio di ogni riga della libreria
sidebar-number = NUMERO
sidebar-thumbnail = MINIATURA
panels = PANNELLI
panels-hint = mostra la mappa e la sessione sopra o accanto alle mod
panels-top = SOPRA
panels-side = A LATO
reduce-motion = RIDUCI ANIMAZIONI
reduce-motion-hint = niente lampeggi sul badge LIVE né transizioni
language = LINGUA
language-hint = la lingua dei testi di Impulse
language-system = SISTEMA
close-to-tray = CHIUDI E NASCONDI
close-to-tray-hint = il pulsante di chiusura nasconde Impulse; per uscire, usa l'icona nell'area di notifica
hide-from-capture = ESCLUDI DA CATTURE
hide-from-capture-hint = nasconde le finestre di Impulse da screenshot, registrazioni e stream
hide-from-capture-inside = l'overlay nel gioco fa parte dell'immagine del gioco: viene nascosta solo la finestra di Impulse, non ciò che disegna dentro il gioco
folder-path = percorso della cartella
folder-select = Seleziona la cartella
settings-sounds = SUONI
sounds-toggle-on = ATTIVAZIONE
sounds-toggle-on-hint = un suono che sale quando attivi una mod
sounds-toggle-off = DISATTIVAZIONE
sounds-toggle-off-hint = un suono che scende quando disattivi una mod
sounds-slider = CURSORI
sounds-slider-hint = un tic a ogni scatto, anche mentre trascini il cursore
sounds-number = NUMERI
sounds-number-hint = un tic quando un valore cambia
sounds-button = PULSANTI
sounds-button-hint = un clic quando esegui una mod tramite il suo pulsante
sounds-selection = OPZIONI
sounds-selection-hint = un suono quando cambi opzione
sounds-listen = Ascolta
sounds-choose = Scegli un file WAV come suono personalizzato
sounds-reset = Ripristina il suono predefinito
sounds-refused = il file non è in formato WAV o supera 1 MB
sounds-prompt = Scegli un file WAV

## Remote

remote = PANNELLO REMOTO
remote-open = Pannello remoto: controlla Impulse dal telefono
remote-switch = ACCESSO DAL TELEFONO
remote-hint = controlla Impulse dal telefono sulla stessa rete
remote-address = INDIRIZZO
remote-address-hint = inquadra il QR con la fotocamera del telefono. Il codice è monouso e cambia ogni cinque minuti
remote-failed = nessun indirizzo di rete disponibile
remote-devices = DISPOSITIVI
remote-devices-hint = telefoni e browser associati. Revocare l'accesso disconnette subito il dispositivo
remote-no-devices = ancora nessun dispositivo
remote-device = dispositivo { $number }
remote-unknown-device = BROWSER
remote-revoke = REVOCA
remote-trust = La connessione HTTP non è protetta: chiunque sulla rete può vedere i dati. Usa solo reti fidate.
remote-tips = SE IL TELEFONO NON SI CONNETTE
remote-tip-network = Collega il telefono alla stessa rete del PC, senza usare i dati mobili, il Wi-Fi per ospiti o una VPN.
remote-tip-firewall = Il firewall di Windows può bloccare Impulse senza avvisarti, soprattutto sulle reti pubbliche. Imposta la rete come privata oppure autorizza Impulse da Sicurezza di Windows > Firewall e protezione della rete > Consenti app tramite firewall.
remote-tip-isolation = Alcuni router impediscono ai dispositivi Wi-Fi di comunicare tra loro (isolamento AP o client). Disattiva questa opzione o prova un'altra rete.
remote-note-changed = L'elenco è cambiato. Riprova.
remote-note-not-live = Avvia prima il gioco.
remote-note-busy = C'è un'altra sessione in corso o il trainer non è ancora pronto.
remote-note-nothing-on = Attiva almeno una mod.
remote-note-name = Dai prima un nome alla collezione.

## Plugins

plugins-install = INSTALLA
plugins-install-hint = scegli una cartella o un file .zip con il manifest.json: prima di installarlo puoi verificare e approvare gli accessi del plugin
plugins-folder = CARTELLA
plugins-zip = ZIP
plugins-pick-folder = Seleziona la cartella del plugin
plugins-pick-zip = Seleziona il file .zip del plugin
plugins-none = nessun plugin installato
plugins-approve-title = INSTALLARE { $name } { $version }?
plugins-replaces = sostituisce la versione { $version }
plugins-hosts = SCARICA DA
plugins-links = APRE LINK VERSO
plugins-no-hosts = nessuno
plugins-new = NUOVO
plugins-sandbox = Il plugin gira in una sandbox e può contattare solo gli host elencati. Installa solo plugin di cui ti fidi.
plugins-approve = APPROVA
plugins-cancel = ANNULLA
plugins-version = versione { $version } · { $size } su disco
plugins-remove = RIMUOVI
plugins-value = valore

## Resources

media-pictures = IMMAGINI
media-pictures-hint = copertine dei giochi e immagini dei pin
media-tiles = RIQUADRI MAPPA
media-tiles-hint = riquadri delle mappe già aperte
media-videos = VIDEO
media-videos-hint = clip dei pin
resources-reading = CALCOLO SPAZIO...
resources-in-use = IN USO
resources-in-use-hint = tutto ciò che Impulse conserva su questo PC
resources-clear = SVUOTA
resources-on-restart = AL RIAVVIO
resources-clears-on-restart = SI SVUOTA AL RIAVVIO
resources-clear-caches = SVUOTA TUTTE LE CACHE
resources-clear-caches-hint = i dati elencati qui sotto non vengono cancellati
resources-clear-all = SVUOTA TUTTO
resources-limit = LIMITE MEDIA
resources-limit-hint = oltre il limite, Impulse elimina per primi i file multimediali usati meno di recente
resources-limit-500m = 500 MB
resources-limit-1g = 1 GB
resources-limit-2g = 2 GB
resources-limit-5g = 5 GB
resources-limit-none = NESSUNO
resources-kept = CONSERVATO
resources-progress = PROGRESSO MAPPE
resources-progress-hint = pin trovati e pin personali
resources-hotkeys = HOTKEY
resources-hotkeys-hint = i tasti che hai assegnato
resources-logs = LOG
resources-logs-hint = fino a due file, consultabili dal pannello LOG
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
log-filter = filtra il log
log-follow = SEGUI
log-copy-all = COPIA TUTTO
log-copy = COPIA { $count }
log-copied = COPIATO
log-clear = SVUOTA
log-folder = CARTELLA
log-detail-empty = CLICCA SU UNA RIGA PER LEGGERE E SELEZIONARE IL MESSAGGIO COMPLETO

## About

about-version = VERSIONE { $version }
about-tagline = GRATUITO E OPEN SOURCE
about-intro = Impulse è sviluppato da una sola persona nel tempo libero. È gratuito e open source. Una stella, una segnalazione di bug o un caffè lo tengono in vita.
about-github = GITHUB
about-github-hint = leggi il codice, segui le versioni e lascia una stella perché altri lo trovino
about-star = STELLA
about-discord = DISCORD
about-discord-hint = fai una domanda, chiedi un gioco o una funzione, chiacchiera con gli altri utenti
about-join = ENTRA
about-bug = HAI TROVATO UN BUG?
about-bug-hint = apri una issue con cosa è successo e cosa ti aspettavi. Il LOG nella barra di stato aiuta
about-report = SEGNALA
about-support = SOSTIENI IMPULSE
about-support-hint = gratuito e open source: un caffè paga il tempo dedicato a ogni aggiornamento
about-coffee = OFFRI UN CAFFÈ
about-contact = CONTATTI
about-contact-hint = per tutto il resto o per qualcosa di privato: { $address }
about-email = E-MAIL
alpha = ALPHA
plugins-alpha-hint = i plugin sono sperimentali. Il formato può cambiare e un plugin può smettere di funzionare dopo un aggiornamento
remote-copy = copia l'indirizzo

## Updates

update-title = AGGIORNAMENTI
update-state-idle = versione { $version }
update-state-current = hai l'ultima versione, { $version }
update-state-skipped = la versione { $version } è saltata: CONTROLLA la propone di nuovo
update-state-checking = ricerca di aggiornamenti...
update-state-available = la versione { $version } è disponibile
update-state-downloading = download della { $version }...
update-state-ready = la { $version } è pronta: riavvia Impulse per installarla
update-check-failed = impossibile raggiungere l'elenco delle versioni
update-install-failed = non è stato possibile installare l'aggiornamento
update-check = CONTROLLA
update-changelog = NOVITÀ
update-channel = CANALE DI AGGIORNAMENTO
update-channel-hint = le versioni preliminari arrivano prima e possono essere instabili; tornare a stabile non installa mai una versione precedente
update-channel-stable = STABILE
update-channel-pre = PRELIMINARE
update-auto = CERCA AGGIORNAMENTI
update-auto-hint = all'avvio e ogni poche ore; viene richiesto solo l'elenco delle versioni, non viene inviato nulla su di te
update-title-available = AGGIORNAMENTO DISPONIBILE
update-title-new = NOVITÀ DELLA { $version }
update-from-to = dalla { $current } alla { $latest }
update-later = DOPO
update-skip = SALTA QUESTA VERSIONE
update-restart = RIAVVIA ORA
update-close = CHIUDI
update-busy = un trainer è in esecuzione: l'aggiornamento si installa quando chiudi Impulse
update-no-notes = nessuna nota di rilascio
update-chip-available = AGGIORNA { $version }
update-chip-downloading = DOWNLOAD...
update-chip-ready = RIAVVIA PER AGGIORNARE
update-changelog-hint = cosa è cambiato in ogni versione
update-open = APRI
