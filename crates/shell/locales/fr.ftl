## Shared

off = OFF
on = ON
number-group = { "\u00A0" }
number-decimal = ,

## Tray: Windows menu words

tray-open = Ouvrir Impulse
tray-stop = Arrêter le trainer
tray-overlay = Overlay
tray-quit = Quitter

## Failures: a kind's words fit after "It could not be loaded:"

failure-offline = pas de connexion
failure-signed-out = déconnecté
failure-not-found = introuvable
failure-unsupported = pas encore pris en charge
failure-setup-needed = il manque un réglage : voir Paramètres > App
failure-game-not-found = le jeu est introuvable
failure-launch-failed = le launcher n'a pas pu lancer le jeu : voir le log
failure-not-started = le jeu ne s'est pas lancé à temps : vérifiez son launcher, puis réessayez
failure-not-running = le jeu n'a pas été lancé à temps : lancez-le vous-même, puis réessayez
failure-external-launch = ce trainer ne fonctionne qu'avec un jeu que vous lancez vous-même : lancez-le, puis appuyez sur JOUER + TRAINER
failure-game-exited = le jeu s'est fermé avant que le trainer n'y entre
failure-refused = refusé
failure-elevation = le jeu tourne en administrateur : acceptez la demande pour vous connecter
failure-protected = la protection du jeu bloque les trainers
failure-needs-64 = le trainer demande la version 64 bits du jeu : lancez celle-ci
failure-needs-32 = le trainer demande la version 32 bits du jeu : lancez celle-ci
failure-load-failed = le trainer n'a pas pu être chargé dans le jeu : voir le log
failure-trainer-failed = le trainer n'a pas démarré dans le jeu : voir le log
failure-restart = le jeu est déjà lancé : fermez-le, puis lancez-le d'ici
failure-internal = erreur inattendue : voir le log
failure-place = { $why } : voir Paramètres > App

## Page

catalog-failed-tag = [ INDISPO ]
catalog-failed = CATALOGUE INDISPONIBLE
catalog-failed-body = Téléchargement impossible : { $why }. Impulse réessaiera au prochain démarrage.
library-filter = filtrer
picker-filter = nom du jeu
page-search = { $count ->
    [one] rechercher parmi { $trainers } trainer
   *[other] rechercher parmi { $trainers } trainers
}
page-search-none = rechercher un trainer
page-search-key = CTRL K
page-add-game = AJOUTER UN JEU
page-get-on = OBTENIR SUR { $store }
page-add-manually = AJOUTER LE JEU MANUELLEMENT...
page-signed-out = NON CONNECTÉ
page-elevated = Exécuté en administrateur : les jeux lancés en administrateur se connectent sans demande
page-overlay = OVERLAY
page-star = Favori
page-folder = Ouvrir le dossier du jeu
page-launcher = Ouvrir dans { $launcher }
page-source = Options du jeu
page-overlay-default = OVERLAY : COMME DANS LES PARAMÈTRES
page-overlay-in-game = OVERLAY : INTÉGRÉ
page-overlay-over = OVERLAY : PAR-DESSUS
page-custom = PERSO
page-live = ACTIF
page-active = { $active ->
    [one] { $active }/{ $total } ACTIF
   *[other] { $active }/{ $total } ACTIFS
}
page-idle = INACTIF
page-log = LOG
page-binding = TAPEZ LE RACCOURCI · ESC ANNULER · BKSP RÉTABLIR
page-binding-unbind = TAPEZ LE RACCOURCI · ESC ANNULER · BKSP RÉTABLIR · DEL EFFACER
page-hotkeys-on = RACCOURCIS ON
page-hotkeys-off = RACCOURCIS OFF
disclaimer-title = AVANT DE COMMENCER
disclaimer-body = Impulse est un client non officiel pour { $sources }. Il utilise les fichiers de l'appli installée sur ce PC et votre session déjà ouverte pour récupérer vos jeux et trainers auprès du service. Impulse ne vous demande jamais votre mot de passe. En dehors de ces échanges avec le service, les données de votre compte restent sur ce PC. Impulse n'est pas affilié au service et n'a pas son approbation. L'utilisation de trainers peut entraîner un bannissement dans les jeux en ligne : vous les utilisez à vos risques et périls.
disclaimer-accept = J'ACCEPTE
disclaimer-exit = QUITTER
signed-out-title = CONNEXION IMPOSSIBLE
signed-out-body = Impulse n'a pas pu se connecter à { $sources }. Vérifiez que l'appli est installée sur ce PC, que vous êtes connecté à votre compte et que vous avez accès à Internet, puis réessayez. Les trainers ne seront disponibles qu'une fois la connexion rétablie.
signed-out-retry = RÉESSAYER
signed-out-why = Raison : { $why }
signed-out-trying = Connexion…
signed-out-close = FERMER

## Library

library-favorites = FAVORIS
library-games = JEUX
library-no-match = AUCUN RÉSULTAT
library-add = + JEU
library-live = ACTIF
library-scanning-tag = [ ANALYSE ]
library-scanning = RECHERCHE DES JEUX INSTALLÉS
library-scanning-body = Impulse compare le catalogue aux jeux installés via vos lanceurs.
library-empty-tag = [ AUCUN JEU ]
library-empty = AUCUN JEU INSTALLÉ DÉTECTÉ
library-empty-body = Impulse cherche les jeux pour lesquels un trainer est disponible parmi ceux installés via Steam, Epic, GOG, Ubisoft, Xbox, Rockstar ou EA.
library-source-exe = EXE  { $file }
library-source-add = + .EXE
library-source-change = CHANGER .EXE
library-source-remove = RETIRER .EXE
library-source-trainer = CHANGER DE TRAINER...
library-source-own-trainer = SON PROPRE TRAINER

## Picker

picker-title = AJOUTER UN JEU
picker-owned = PLUSIEURS JEUX CORRESPONDENT : LEQUEL ?
picker-unowned = JEU NON IDENTIFIÉ : RECHERCHEZ-LE PAR NOM
picker-in-library = { $store }  EN BIBLIOTHÈQUE
picker-prompt = Choisir le fichier .EXE du jeu
picker-trainer-title = CHOISIR UN TRAINER
picker-trainer = SES AUTRES BOUTIQUES D'ABORD, PUIS PAR NOM : UN TRAINER D'UNE AUTRE VERSION PEUT NE PAS CONVENIR
picker-in-use = UTILISÉ
picker-find-title = TROUVER UN JEU
picker-find = TOUS LES TRAINERS PAR NOM · ↑ ↓ POUR CHOISIR, ENTRÉE POUR OUVRIR

## Cheats

cheats-failed-tag = [ SANS DONNÉES ]
cheats-failed = LISTE DES MODS INDISPONIBLE
cheats-failed-body = Téléchargement impossible : { $why }. Impulse réessaiera la prochaine fois que vous ouvrirez ce jeu.
cheats-filter = filtrer les mods
cheats-pinned = ÉPINGLÉS
cheats-kept = Une fois téléchargée, la liste reste sur votre PC.
cheats-notes = À LIRE AVANT DE JOUER
cheats-no-trainer-tag = [ AUCUN TRAINER ]
cheats-no-trainer = PAS ENCORE DE TRAINER POUR { $game }
cheats-no-trainer-body = Dès qu'un trainer sortira pour ce jeu, Impulse l'indiquera dans votre bibliothèque.
cheats-other-store = Il n'y en a pas encore pour cette boutique, mais la version { $store } du jeu en a un. Les versions peuvent différer d'une boutique à l'autre, il se peut donc qu'il ne convienne pas à la vôtre.
cheats-use-trainer = UTILISER LE TRAINER { $store }
cheats-other-trainer = AUTRE TRAINER...
cheats-not-installed = NON INSTALLÉ · AJOUTEZ LE JEU POUR UTILISER SES MODS
cheats-loading-tag = [ CHARGEMENT ]
cheats-loading = CHARGEMENT DES MODS
cheats-shown = { $count ->
    [one] { $shown } MOD
   *[other] { $shown } MODS
}
cheats-no-match = AUCUN MOD TROUVÉ
cheats-updated = JEU MIS À JOUR
cheats-updated-body = Le jeu est plus récent que le trainer. Certains mods risquent de ne pas fonctionner tant que le trainer n'aura pas été mis à jour.
cheats-set = OK
cheats-set-done = OK
cheats-execute = LANCER
cheats-execute-done = FAIT
cheats-save = GARDER
cheats-saved = { $count ->
    [one] { $saved } GARDÉ
   *[other] { $saved } GARDÉS
}
cheats-save-tip = Les mods encore actifs en fin de session se réactivent à la suivante.
cheats-reset = RESET
cheats-no-collections = AUCUNE COLLECTION
cheats-collect-tip = Enregistre les mods actifs dans une collection.
cheats-edit = MODIFIER
cheats-delete = SUPPRIMER
cheats-name-field = nom
cheats-cancel = ANNULER
cheats-name = NOM
cheats-icon = ICÔNE
cheats-collection-mods = { $count ->
    [one] { $mods } MOD
   *[other] { $mods } MODS
}
cheats-edit-collection = MODIFIER LA COLLECTION
cheats-new-collection = NOUVELLE COLLECTION

## Hotkeys

hotkeys-press-keys = APPUYEZ
hotkeys-off = OFF
hotkeys-in-game = EN JEU
hotkeys-global = GLOBAL
hotkeys-enabled = RACCOURCIS
hotkeys-enabled-hint = active les raccourcis du trainer pendant la session
hotkeys-works-in = CONTEXTE
hotkeys-works-in-hint = en jeu : uniquement quand le jeu est au premier plan
hotkeys-bind-hint = ESC ANNULER · BKSP RÉTABLIR · DEL EFFACER
hotkeys-clash = { $keys } est déjà utilisé par { $name }
hotkeys-clash-overlay = { $keys } est réservé à l'overlay
hotkeys-keep = GARDER LES DEUX
hotkeys-rebind = RÉASSIGNER
hotkeys-shared-tip = un autre mod a aussi ce raccourci : les deux se déclenchent

## Trainer

trainer-none = AUCUN TRAINER
trainer-mods = { $count ->
    [one] { $count } MOD
   *[other] { $count } MODS
}
trainer-from = TRAINER { $store }
trainer-version = V{ $version }
trainer-version-unknown = VERSION INCONNUE
trainer-not-installed = NON INSTALLÉ
trainer-update-pending = MAJ EN ATTENTE
trainer-sessions = { $count ->
    [one] { $count } SESSION
   *[other] { $count } SESSIONS
}
trainer-played-min = { $count } MIN DE JEU
trainer-played-hours = { $hours } H DE JEU

## Run

run-no-cheats = la liste des mods n'est pas chargée
run-preparing = PRÉPARATION DU TRAINER
run-launching = LANCEMENT DU JEU
run-waiting = ATTENTE DU JEU
run-attaching = CHARGEMENT DU TRAINER
run-stopping = ARRÊT DU TRAINER
run-stop = STOP
run-launch = JOUER + TRAINER
run-enter = ENTRÉE
run-session = SESSION
run-last-session = DERNIÈRE SESSION
run-active = ACTIFS
run-hotkeys = RACCOURCIS
run-idle = LANCEZ LE JEU POUR UTILISER LES MODS
run-unconfirmed-title = JEU MIS À JOUR
run-unconfirmed-body = Le jeu est plus récent que le trainer. Certains mods risquent de ne pas fonctionner tant que le trainer n'aura pas été mis à jour. Jouer quand même ?
run-unconfirmed-skip = TOUJOURS JOUER
run-ask-cancel = ANNULER
run-unconfirmed-play = JOUER QUAND MÊME
run-unconfirmed-ask = DEMANDER APRÈS UNE MISE À JOUR
run-unconfirmed-ask-hint = demander avant de lancer quand le jeu est plus récent que la version prise en charge par le trainer
run-outside-title = LANCEZ LE JEU VOUS-MÊME
run-outside-body = Ce trainer ne s'attache qu'à un jeu lancé en dehors d'Impulse, et le jeu n'est pas lancé. Lancez-le, attendez qu'il charge, puis appuyez sur JOUER + TRAINER.
run-outside-start = LANCER LE JEU

## Map

teleport-failed = TÉLÉPORTATION IMPOSSIBLE : { $why }
map-failed-tag = [ PAS DE CARTE ]
map-failed = CARTE INDISPONIBLE
map-failed-body = Chargement impossible : { $why }.
map-title = CARTE
map-none = PAS DE CARTE INTERACTIVE POUR CE JEU
map-link = LIER UNE CARTE
map-unlink = DISSOCIER LA CARTE
map-link-title = LIER UNE CARTE
map-link-caption = QUEL TITRE CORRESPOND À CE JEU ?
map-link-empty = Aucune carte trouvée pour ce titre.
map-refresh = ACTUALISER
map-loading-tag = [ CHARGEMENT ]
map-loading = CHARGEMENT DE LA CARTE
map-loading-body = Au premier chargement, le fond de carte et les repères sont enregistrés sur votre PC.
map-search = rechercher un lieu, un groupe ou un mot-clé
map-unfinished = Carte en cours de création : certains repères peuvent manquer ou changer de place.
map-found = TROUVÉ
map-found-count = TROUVÉS
map-mark-found = JE L'AI TROUVÉ
map-unfound = À TROUVER
map-show-all = TOUT VOIR
map-hide-all = TOUT CACHER
map-teleport = TÉLÉPORTER
map-filter-teleport = AVEC TP
map-teleport-here = TÉLÉPORTER ICI
map-teleport-height = HAUTEUR DE TÉLÉPORTATION
map-teleport-height-hint = S'ajoute à la hauteur d'une téléportation vers les repères et points de la carte, en unités du jeu
map-pin-position = Un repère là où se trouve le joueur
map-confirm = CONFIRMER
map-delete = SUPPRIMER
map-add-pin = NOUVEAU REPÈRE
map-my-pin = MON REPÈRE
map-my-pins = MES REPÈRES
map-pin-name = Mon repère { $number }
map-name-field = nom du repère
map-note-field = note
map-no-pins = Clic droit sur la carte pour en ajouter.
map-regions = RÉGIONS
map-layer-regions = RÉGIONS
map-places = { $count ->
    [one] { $places } LIEU
   *[other] { $places } LIEUX
}
map-no-matches = Aucun résultat. Essayez un groupe ou un mot plus court.
map-nearby = À PROXIMITÉ
map-copy = COPIER
map-copied = COPIÉ
map-hide-group = MASQUER { $group } SUR LA CARTE
map-pin-hide = MASQUER CE REPÈRE
map-pin-unhide = AFFICHER CE REPÈRE
map-pin-move = DÉPLACER
map-add-media = AJOUTER DES MÉDIAS
map-hidden-pins = REPÈRES MASQUÉS
map-pin-drag = ALT + GLISSER LE DÉPLACE
map-video = VIDÉO
map-photo = PHOTO
map-layer-labels = NOMS DES LIEUX
map-layer-clusters = REGROUPER LES REPÈRES
map-layer-mini-clusters = AUSSI SUR LA MINICARTE
map-layer-rises = HAUTEURS À PROXIMITÉ
map-axis-x = X
map-axis-y = Y
map-axis-z = Z

## Overlay

overlay-dock-mods = MODS
overlay-dock-map = CARTE
overlay-dock-session = SESSION
overlay-dock-minimap = MINICARTE
overlay-minimap-tag = [ MINICARTE ]
overlay-no-live-map = SUIVI INACTIF
overlay-no-live-map-hint = Pour que la minicarte suive votre position, le jeu doit avoir une carte et le trainer actif doit pouvoir vous localiser.
overlay-mode = MODE
overlay-mode-hint = intégré, l'overlay est dessiné dans l'image du jeu si le jeu le permet, et le jeu garde le focus
overlay-mode-over = PAR-DESSUS
overlay-mode-in-game = INTÉGRÉ
overlay-hotkey = RACCOURCI
overlay-hotkey-hint = combinez des touches comme Ctrl ou Maj avec une autre touche. Évitez les raccourcis déjà utilisés par vos applis ou vos mods. Actif uniquement quand le jeu est au premier plan
overlay-minimap-hotkey = RACCOURCI MINICARTE
overlay-minimap-hotkey-hint = affiche ou masque la minicarte, uniquement quand le jeu est au premier plan
overlay-zoom-in-hotkey = ZOOM + MINICARTE
overlay-zoom-in-hotkey-hint = zoome sur la minicarte, uniquement quand le jeu est au premier plan
overlay-zoom-out-hotkey = ZOOM − MINICARTE
overlay-zoom-out-hotkey-hint = élargit la vue sur la minicarte, uniquement quand le jeu est au premier plan
overlay-pin-here-hotkey = REPÈRE ICI
overlay-pin-here-hotkey-hint = place un repère là où se trouve le joueur, avec une capture du jeu, uniquement quand le jeu est au premier plan
overlay-exclusive-title = PLEIN ÉCRAN EXCLUSIF
overlay-exclusive-body = En plein écran exclusif, l'overlay risque de ne pas s'afficher et la fenêtre du jeu peut se réduire. Pour éviter ça, choisissez le mode « Fenêtré sans bordure » dans les réglages vidéo du jeu.
overlay-exclusive-chip = PLEIN ÉCRAN EXCLUSIF : PASSEZ EN FENÊTRÉ SANS BORDURE
overlay-exclusive-skip = NE PLUS AFFICHER
overlay-exclusive-ok = OK
overlay-exclusive-ask-hint = demander quand l'overlay s'ouvre sur un jeu en plein écran exclusif

## Settings

settings = PARAMÈTRES
settings-appearance = APPARENCE
settings-overlay = OVERLAY
settings-hotkeys = RACCOURCIS
settings-app = APP
settings-resources = RESSOURCES
settings-plugins = PLUGINS
settings-about = À PROPOS
accent = COULEUR
accent-hint = met uniquement en évidence les éléments actifs
accent-lime = ANIS
accent-amber = AMBRE
accent-cyan = CYAN
accent-white = BLANC
sidebar = BARRE LATÉRALE
sidebar-hint = ce qui s'affiche devant le nom du jeu
sidebar-number = NUMÉRO
sidebar-thumbnail = VIGNETTE
panels = PANNEAUX
panels-hint = place la carte et la session au-dessus ou à côté des mods
panels-top = HAUT
panels-side = CÔTÉ
reduce-motion = RÉDUIRE LES ANIMATIONS
reduce-motion-hint = désactive le clignotement du statut ACTIF et les transitions
language = LANGUE
language-hint = la langue des textes d'Impulse
language-system = SYSTÈME
close-to-tray = MASQUER EN FERMANT
close-to-tray-hint = fermer la fenêtre masque Impulse. Pour quitter, passez par son icône dans la zone de notification
hide-from-capture = EXCLURE DES CAPTURES
hide-from-capture-hint = Impulse n'apparaît ni dans les captures d'écran, ni dans les enregistrements, ni sur les streams
hide-from-capture-inside = l'overlay dans le jeu fait partie de l'image du jeu : seule la fenêtre d'Impulse est masquée, pas ce qu'Impulse dessine dans le jeu
folder-path = chemin du dossier
folder-select = Choisir le dossier
settings-sounds = SONS
sounds-toggle-on = ACTIVATION
sounds-toggle-on-hint = une note montante quand un mod s'active
sounds-toggle-off = DÉSACTIVATION
sounds-toggle-off-hint = une note descendante quand un mod se désactive
sounds-slider = CURSEURS
sounds-slider-hint = un clic à chaque cran quand vous déplacez le curseur
sounds-number = VALEURS
sounds-number-hint = un clic quand une valeur change
sounds-button = BOUTONS
sounds-button-hint = un clic quand vous lancez un mod à l'aide d'un bouton
sounds-selection = CHOIX
sounds-selection-hint = une note à chaque changement d'option
sounds-listen = Écouter
sounds-choose = Remplacer par un fichier WAV
sounds-reset = Rétablir le son par défaut
sounds-refused = fichier non WAV ou de plus de 1 Mo
sounds-prompt = Choisir un fichier WAV

## Remote

remote = TÉLÉCOMMANDE
remote-open = Télécommande : pilotez Impulse depuis votre téléphone
remote-switch = ACCÈS MOBILE
remote-hint = pilotez Impulse depuis un téléphone connecté à ce réseau
remote-address = ADRESSE
remote-address-hint = scannez le code QR avec votre téléphone. Ce code est à usage unique et change toutes les cinq minutes
remote-failed = aucune adresse réseau disponible
remote-devices = APPAREILS
remote-devices-hint = téléphones et navigateurs associés. Si vous retirez un accès, l'appareil se déconnecte aussitôt
remote-no-devices = aucun appareil pour l'instant
remote-device = appareil { $number }
remote-unknown-device = NAVIGATEUR
remote-revoke = RETIRER L'ACCÈS
remote-trust = Connexion HTTP non chiffrée : n'importe qui sur ce réseau peut voir les échanges. N'utilisez cette option que sur un réseau de confiance.
remote-tips = SI LE TÉLÉPHONE NE SE CONNECTE PAS
remote-tip-network = Connectez votre téléphone au même réseau que ce PC, sans passer par les données mobiles, un Wi-Fi invité ni un VPN.
remote-tip-firewall = Le pare-feu Windows peut bloquer Impulse sans vous prévenir, surtout sur un réseau public. Passez le réseau en mode privé ou autorisez Impulse dans Sécurité Windows > Pare-feu et protection du réseau > Autoriser une application via le pare-feu.
remote-tip-isolation = Certains routeurs empêchent les appareils Wi-Fi de communiquer entre eux (isolation AP ou client). Désactivez cette option ou essayez un autre réseau.
remote-note-changed = La liste a changé. Réessayez.
remote-note-not-live = Lancez d'abord le jeu.
remote-note-busy = Une autre session est en cours ou le trainer n'est pas encore prêt.
remote-note-nothing-on = Activez d'abord des mods.
remote-note-name = Nommez d'abord la collection.

## Plugins

plugins-install = INSTALLER
plugins-install-hint = choisissez un dossier ou un fichier .zip contenant manifest.json, puis validez les accès demandés
plugins-folder = DOSSIER
plugins-zip = ZIP
plugins-pick-folder = Choisir le dossier du plugin
plugins-pick-zip = Choisir le fichier .zip du plugin
plugins-none = aucun plugin installé
plugins-approve-title = INSTALLER { $name } { $version } ?
plugins-replaces = remplace la version { $version }
plugins-hosts = ACCÈS RÉSEAU
plugins-links = LIENS EXTERNES
plugins-no-hosts = aucun
plugins-new = NOUVEAU
plugins-sandbox = Le plugin s'exécute dans un environnement isolé et ne peut se connecter qu'aux adresses indiquées. N'installez que des plugins de confiance.
plugins-approve = APPROUVER
plugins-cancel = ANNULER
plugins-version = version { $version } · { $size } sur le disque
plugins-remove = DÉSINSTALLER
plugins-value = valeur

## Resources

media-pictures = IMAGES
media-pictures-hint = visuels des jeux et photos des repères
media-tiles = FONDS DE CARTE
media-tiles-hint = fonds des cartes consultées
media-videos = VIDÉOS
media-videos-hint = vidéos associées aux repères
resources-reading = CALCUL EN COURS...
resources-in-use = ESPACE UTILISÉ
resources-in-use-hint = tout ce qu'Impulse stocke sur ce PC
resources-clear = VIDER
resources-on-restart = AU REDÉMARRAGE
resources-clears-on-restart = VIDÉ AU REDÉMARRAGE
resources-clear-caches = VIDER LES CACHES
resources-clear-caches-hint = les données ci-dessous sont conservées
resources-clear-all = TOUT VIDER
resources-limit = LIMITE DES MÉDIAS
resources-limit-hint = au-delà de cette limite, Impulse supprime d'abord les médias qui n'ont pas servi depuis le plus longtemps
resources-limit-500m = 500 Mo
resources-limit-1g = 1 Go
resources-limit-2g = 2 Go
resources-limit-5g = 5 Go
resources-limit-none = AUCUNE
resources-kept = HORS CACHE
resources-progress = PROGRESSION SUR LES CARTES
resources-progress-hint = repères trouvés et repères ajoutés
resources-hotkeys = RACCOURCIS
resources-hotkeys-hint = les touches que vous avez attribuées
resources-logs = LOGS
resources-logs-hint = deux fichiers maximum, consultables dans le log
resources-bytes = { $size } o
resources-kilobytes = { $size } Ko
resources-megabytes = { $size } Mo
resources-gigabytes = { $size } Go

## Log

log-title = LOG
log-error = ERREUR
log-warn = AVERT.
log-info = INFO
log-debug = DEBUG
log-filter = filtrer le log
log-follow = SUIVRE
log-copy-all = TOUT COPIER
log-copy = COPIER { $count }
log-copied = COPIÉ
log-clear = VIDER
log-folder = DOSSIER
log-detail-empty = CLIQUEZ SUR UNE LIGNE POUR LIRE ET SÉLECTIONNER LE TEXTE COMPLET

## About

about-version = VERSION { $version }
about-tagline = GRATUIT ET OPEN SOURCE
about-intro = Impulse est développé par une seule personne sur son temps libre. Il est gratuit et open source. Une étoile, un rapport de bug ou un café suffisent à le faire vivre.
about-github = GITHUB
about-github-hint = lisez le code, suivez les versions et laissez une étoile pour qu'on le trouve plus facilement
about-star = ÉTOILE
about-discord = DISCORD
about-discord-hint = posez une question, demandez un jeu ou une fonction, discutez avec les autres utilisateurs
about-join = REJOINDRE
about-bug = UN BUG ?
about-bug-hint = ouvrez un ticket avec ce qui s'est passé et ce que vous attendiez. Le LOG de la barre d'état aide
about-report = SIGNALER
about-support = SOUTENIR IMPULSE
about-support-hint = gratuit et open source : un café paie le temps passé sur chaque mise à jour
about-coffee = UN CAFÉ
about-contact = CONTACT
about-contact-hint = pour tout le reste, ou pour un sujet privé : { $address }
about-email = E-MAIL
alpha = ALPHA
plugins-alpha-hint = les plugins sont expérimentaux. Le format peut changer et un plugin peut cesser de fonctionner après une mise à jour.
remote-copy = copier l'adresse

## Updates

update-title = MISES À JOUR
update-state-idle = version { $version }
update-state-current = vous avez la dernière version, { $version }
update-state-skipped = la version { $version } est ignorée : VÉRIFIER la propose de nouveau
update-state-checking = recherche de mises à jour...
update-state-available = la version { $version } est disponible
update-state-downloading = téléchargement de la { $version }...
update-state-ready = la { $version } est prête : redémarrez Impulse pour l'installer
update-check-failed = impossible de joindre la liste des versions
update-install-failed = la mise à jour n'a pas pu être installée
update-check = VÉRIFIER
update-changelog = HISTORIQUE
update-channel = CANAL DE MISE À JOUR
update-channel-hint = les préversions arrivent plus tôt et peuvent être instables ; revenir au canal stable ne rétrograde jamais
update-channel-stable = STABLE
update-channel-pre = PRÉVERSION
update-auto = RECHERCHER LES MISES À JOUR
update-auto-hint = au démarrage et toutes les quelques heures ; seule la liste des versions est demandée, rien sur vous n'est envoyé
update-title-available = MISE À JOUR DISPONIBLE
update-title-new = NOUVEAUTÉS DE LA { $version }
update-from-to = de la { $current } à la { $latest }
update-later = PLUS TARD
update-skip = IGNORER CETTE VERSION
update-restart = REDÉMARRER
update-close = FERMER
update-busy = un trainer est en cours : la mise à jour s'installera quand vous quitterez Impulse
update-no-notes = pas de notes de version
update-chip-available = MISE À JOUR { $version }
update-chip-downloading = TÉLÉCHARGEMENT...
update-chip-ready = REDÉMARRER POUR METTRE À JOUR
update-changelog-hint = ce qui a changé dans chaque version
update-open = OUVRIR
