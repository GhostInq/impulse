## Shared

off = WYŁ.
on = WŁ.
# A number's marks: between thousands, before decimals.
number-group = { "\u00A0" }
number-decimal = ,

## Tray: Windows menu words

tray-open = Otwórz Impulse
tray-stop = Zatrzymaj trainer
tray-overlay = Nakładka
tray-quit = Zakończ

## Failures: a kind's words fit after "It could not be loaded:"

failure-offline = brak połączenia
failure-signed-out = wylogowano
failure-not-found = nie znaleziono
failure-unsupported = na razie brak obsługi
failure-setup-needed = wymagana konfiguracja (Ustawienia → Ogólne)
failure-game-not-found = nie znaleziono gry
failure-launch-failed = launcher nie mógł uruchomić gry: szczegóły w logu
failure-not-started = gra nie uruchomiła się na czas: sprawdź jej launcher i spróbuj ponownie
failure-not-running = gra nie została uruchomiona na czas: uruchom ją i spróbuj ponownie
failure-external-launch = ten trainer działa tylko z grą uruchomioną przez ciebie: uruchom ją, potem naciśnij GRA + TRAINER
failure-game-exited = gra została zamknięta, zanim trainer się do niej dostał
failure-refused = odrzucono
failure-elevation = gra działa jako administrator: zaakceptuj monit, aby się połączyć
failure-protected = zabezpieczenia gry blokują trainery
failure-needs-64 = trainer wymaga 64-bitowej wersji gry: uruchom ją
failure-needs-32 = trainer wymaga 32-bitowej wersji gry: uruchom ją
failure-load-failed = nie udało się załadować trainera do gry: szczegóły w logu
failure-trainer-failed = trainer nie uruchomił się w grze: szczegóły w logu
failure-restart = gra jest już uruchomiona: zamknij ją, a potem uruchom stąd
failure-internal = coś poszło nie tak: szczegóły w logu
failure-place = { $why } (Ustawienia → Ogólne)

## Page

catalog-failed-tag = [ BRAK KATALOGU ]
catalog-failed = KATALOG GIER NIEDOSTĘPNY
catalog-failed-body = Nie udało się pobrać katalogu: { $why }. Impulse spróbuje ponownie przy następnym uruchomieniu.
library-filter = filtruj gry
picker-filter = nazwa gry
page-search = { $count ->
    [one] szukaj · { $trainers } trainer
    [few] szukaj · { $trainers } trainery
    [many] szukaj · { $trainers } trainerów
   *[other] szukaj · { $trainers } trainera
}
page-search-none = szukaj trainerów
page-search-key = CTRL K
page-add-game = DODAJ GRĘ
page-get-on = ZDOBĄDŹ NA { $store }
page-add-manually = DODAJ GRĘ RĘCZNIE...
page-signed-out = WYLOGOWANO
page-elevated = Uruchomiono jako administrator: gry uruchomione jako administrator łączą się bez pytania
page-overlay = NAKŁADKA
page-star = Ulubione
page-folder = Otwórz folder gry
page-launcher = Otwórz w { $launcher }
page-source = Opcje gry
page-overlay-default = NAKŁADKA: JAK W USTAWIENIACH
page-overlay-in-game = NAKŁADKA: W GRZE
page-overlay-over = NAKŁADKA: NA WIERZCHU
page-custom = .EXE
page-live = W GRZE
page-active = { $active }/{ $total } WŁ.
page-idle = WYŁ.
page-log = LOG
page-binding = NACIŚNIJ KLAWISZE · ESC ANULUJ · BKSP PRZYWRÓĆ
page-binding-unbind = NACIŚNIJ KLAWISZE · ESC ANULUJ · BKSP PRZYWRÓĆ · DEL USUŃ
page-hotkeys-on = SKRÓTY WŁ.
page-hotkeys-off = SKRÓTY WYŁ.
disclaimer-title = ZANIM ZACZNIESZ
disclaimer-body = Impulse to nieoficjalny klient { $sources }. Korzysta z plików aplikacji zainstalowanej na tym komputerze i z jej aktywnej sesji, by pobrać z serwisu gry i trainery przypisane do konta. Impulse nie prosi o hasło. Dane konta nie opuszczają komputera poza zapytaniami do serwisu. Impulse nie jest powiązany z serwisem i nie ma jego poparcia. Za używanie trainerów w grach online można dostać bana. Korzystanie z nich jest na własne ryzyko.
disclaimer-accept = ZGADZAM SIĘ
disclaimer-exit = WYJDŹ
signed-out-title = BŁĄD LOGOWANIA
signed-out-body = Nie udało się zalogować do { $sources }. Przed ponowną próbą sprawdź połączenie z internetem i upewnij się, że aplikacja jest zainstalowana na tym komputerze i ma aktywną sesję. Bez logowania Impulse nie może wczytać trainerów.
signed-out-retry = PONÓW PRÓBĘ
signed-out-why = Powód: { $why }
signed-out-trying = Logowanie…
signed-out-close = ZAMKNIJ

## Library

library-favorites = ULUBIONE
library-games = GRY
library-no-match = BRAK WYNIKÓW
library-add = + DODAJ
library-live = W GRZE
library-scanning-tag = [ SZUKANIE ]
library-scanning = SZUKANIE ZAINSTALOWANYCH GIER
library-scanning-body = Impulse sprawdza katalog gier i zainstalowane launchery.
library-empty-tag = [ BRAK GIER ]
library-empty = NIE ZNALEZIONO ZAINSTALOWANYCH GIER
library-empty-body = Impulse szuka w Steam, Epic, GOG, Ubisoft, Xbox, Rockstar i EA gier, do których ma trainery.
library-source-exe = EXE  { $file }
library-source-add = + DODAJ .EXE
library-source-change = ZMIEŃ .EXE
library-source-remove = USUŃ .EXE
library-source-trainer = ZMIEŃ TRAINER...
library-source-own-trainer = WŁASNY TRAINER

## Picker

picker-title = DODAJ GRĘ
picker-owned = PASUJE KILKA GIER: KTÓRA TO?
picker-unowned = NIE ROZPOZNANO GRY. SZUKAJ W KATALOGU PO NAZWIE.
picker-in-library = { $store }  W BIBLIOTECE
picker-prompt = Wybierz plik .exe gry
picker-trainer-title = WYBIERZ TRAINER
picker-trainer = NAJPIERW INNE SKLEPY, POTEM WG NAZWY: TRAINER DO INNEJ WERSJI MOŻE NIE PASOWAĆ
picker-in-use = W UŻYCIU
picker-find-title = ZNAJDŹ GRĘ
picker-find = KAŻDY TRAINER WG NAZWY · ↑ ↓ ABY WYBRAĆ, ENTER ABY OTWORZYĆ

## Cheats

cheats-failed-tag = [ BRAK DANYCH ]
cheats-failed = NIE POBRANO LISTY MODÓW
cheats-failed-body = Nie udało się pobrać listy modów: { $why }. Impulse spróbuje ponownie przy kolejnym otwarciu gry.
cheats-filter = filtruj mody
cheats-pinned = PRZYPIĘTE
cheats-kept = Pobrana lista modów zostaje na dysku.
cheats-notes = PRZECZYTAJ PRZED GRĄ
cheats-no-trainer-tag = [ BRAK TRAINERA ]
cheats-no-trainer = BRAK TRAINERA: { $game }
cheats-no-trainer-body = Do tej gry nie ma jeszcze trainera. Gdy się pojawi, Impulse oznaczy grę w bibliotece.
cheats-other-store = Dla tego sklepu jeszcze go nie ma, ale wersja z { $store } ma trainer. Wersje mogą się różnić między sklepami, więc może nie pasować do twojej.
cheats-use-trainer = UŻYJ TRAINERA Z { $store }
cheats-other-trainer = INNY TRAINER...
cheats-not-installed = NIEZAINSTALOWANA · DODAJ GRĘ, ABY UŻYWAĆ JEJ MODÓW
cheats-loading-tag = [ ŁADOWANIE ]
cheats-loading = ŁADOWANIE LISTY MODÓW
cheats-shown = { $count ->
    [one] { $shown } MOD
    [few] { $shown } MODY
    [many] { $shown } MODÓW
   *[other] { $shown } MODA
}
cheats-no-match = BRAK WYNIKÓW
cheats-updated = NOWA WERSJA GRY
cheats-updated-body = Trainer nie obsługuje jeszcze tej wersji gry. Do czasu aktualizacji trainera mody mogą nie działać.
cheats-set = USTAW
cheats-set-done = OK
cheats-execute = WYKONAJ
cheats-execute-done = OK
cheats-save = ZAPISZ
cheats-saved = ZAPISANO { $saved }
cheats-save-tip = Mody aktywne pod koniec sesji włączą się ponownie na początku następnej.
cheats-reset = RESET
cheats-no-collections = BRAK KOLEKCJI
cheats-collect-tip = Do kolekcji trafią tylko włączone mody.
cheats-edit = EDYTUJ
cheats-delete = USUŃ
cheats-name-field = nazwa
cheats-cancel = ANULUJ
cheats-name = NAZWA
cheats-icon = IKONA
cheats-collection-mods = { $count ->
    [one] { $mods } MOD
    [few] { $mods } MODY
    [many] { $mods } MODÓW
   *[other] { $mods } MODA
}
cheats-edit-collection = EDYTUJ KOLEKCJĘ
cheats-new-collection = NOWA KOLEKCJA

## Hotkeys

hotkeys-press-keys = USTAW SKRÓT
hotkeys-off = WYŁ.
hotkeys-in-game = W GRZE
hotkeys-global = WSZĘDZIE
hotkeys-enabled = SKRÓTY
hotkeys-enabled-hint = skróty klawiszowe trainera w trakcie sesji
hotkeys-works-in = GDZIE DZIAŁAJĄ
hotkeys-works-in-hint = „W grze” oznacza, że skróty działają tylko w aktywnym oknie gry
hotkeys-bind-hint = ESC ANULUJ · BKSP PRZYWRÓĆ · DEL USUŃ
hotkeys-clash = { $keys } jest już przypisany do: { $name }
hotkeys-clash-overlay = { $keys } jest zarezerwowany dla nakładki
hotkeys-keep = ZOSTAW OBA
hotkeys-rebind = PRZYPISZ PONOWNIE
hotkeys-shared-tip = inny mod ma ten sam skrót: zadziałają oba

## Trainer

trainer-none = BRAK TRAINERA
trainer-mods = { $count ->
    [one] { $count } MOD
    [few] { $count } MODY
    [many] { $count } MODÓW
   *[other] { $count } MODA
}
trainer-from = TRAINER { $store }
trainer-version = V{ $version }
trainer-version-unknown = NIEZNANA WERSJA
trainer-not-installed = NIEZAINSTALOWANA
trainer-update-pending = DO AKTUALIZACJI
trainer-sessions = { $count ->
    [one] { $count } SESJA
    [few] { $count } SESJE
    [many] { $count } SESJI
   *[other] { $count } SESJI
}
trainer-played-min = { $count } MIN GRY
trainer-played-hours = { $hours } H GRY

## Run

run-no-cheats = nie wczytano listy modów
run-preparing = PRZYGOTOWANIE TRAINERA
run-launching = URUCHAMIANIE GRY
run-waiting = CZEKANIE NA GRĘ
run-attaching = ŁADOWANIE TRAINERA
run-stopping = ZATRZYMYWANIE TRAINERA
run-stop = STOP
run-launch = GRA + TRAINER
run-enter = ENTER
run-session = SESJA
run-last-session = OSTATNIA SESJA
run-active = AKTYWNE
run-hotkeys = SKRÓTY
run-idle = TRAINER WYŁ. · MODY DOSTĘPNE PO STARCIE
run-unconfirmed-title = NOWA WERSJA GRY
run-unconfirmed-body = Trainer nie obsługuje jeszcze tej wersji gry. Do czasu aktualizacji trainera mody mogą nie działać. Uruchomić mimo to?
run-unconfirmed-skip = ZAWSZE URUCHAMIAJ
run-ask-cancel = ANULUJ
run-unconfirmed-play = URUCHOM MIMO TO
run-unconfirmed-ask = PYTAJ PO AKTUALIZACJI GRY
run-unconfirmed-ask-hint = pytaj przed uruchomieniem, gdy wersja gry jest nowsza niż obsługiwana przez trainer
run-outside-title = URUCHOM GRĘ SAM
run-outside-body = Ten trainer łączy się tylko z grą uruchomioną poza Impulse, a gra nie jest uruchomiona. Uruchom ją, poczekaj, aż się wczyta, potem naciśnij GRA + TRAINER.
run-outside-start = URUCHOM GRĘ

## Map

teleport-failed = BŁĄD TELEPORTU: { $why }
map-failed-tag = [ BRAK MAPY ]
map-failed = MAPA NIEDOSTĘPNA
map-failed-body = Nie udało się jej wczytać: { $why }.
map-title = MAPA
map-none = BRAK INTERAKTYWNEJ MAPY TEJ GRY
map-link = POWIĄŻ MAPĘ
map-unlink = ODŁĄCZ MAPĘ
map-link-title = POWIĄŻ MAPĘ
map-link-caption = KTÓRY TYTUŁ PASUJE DO TEJ GRY?
map-link-empty = Nie znaleziono gry o tej nazwie w katalogach map.
map-refresh = ODŚWIEŻ
map-loading-tag = [ ŁADOWANIE ]
map-loading = ŁADOWANIE MAPY
map-loading-body = Po pierwszym wczytaniu pinezki i kafelki mapy zostają na dysku.
map-search = szukaj miejsca, grupy lub słowa
map-unfinished = Mapa wciąż powstaje. Niektórych pinezek może brakować, a ich położenie może się zmienić.
map-found = ZNALEZIONA
map-found-count = ZNALEZIONE
map-mark-found = ODHACZ
map-unfound = NIEZNALEZIONE
map-show-all = POKAŻ WSZYSTKO
map-hide-all = UKRYJ WSZYSTKO
map-teleport = TELEPORTUJ
map-filter-teleport = TELEPORT
map-teleport-here = TELEPORTUJ TUTAJ
map-teleport-height = WYSOKOŚĆ TELEPORTU
map-teleport-height-hint = Dodawana do wysokości teleportu do pinezek i punktów mapy, w jednostkach gry
map-pin-position = Pinezka tam, gdzie stoi gracz
map-confirm = POTWIERDŹ
map-delete = USUŃ
map-add-pin = DODAJ PINEZKĘ
map-my-pin = MOJA PINEZKA
map-my-pins = MOJE PINEZKI
map-pin-name = Moja pinezka { $number }
map-name-field = nazwij pinezkę
map-note-field = notatka
map-no-pins = Aby dodać pinezkę, kliknij mapę prawym przyciskiem myszy.
map-regions = REGIONY
map-layer-regions = REGIONY
map-places = { $count ->
    [one] { $places } MIEJSCE
    [few] { $places } MIEJSCA
    [many] { $places } MIEJSC
   *[other] { $places } MIEJSCA
}
map-no-matches = Brak wyników. Spróbuj wpisać nazwę grupy lub krótsze słowo.
map-nearby = W POBLIŻU
map-copy = KOPIUJ
map-copied = SKOPIOWANO
map-hide-group = UKRYJ { $group } NA MAPIE
map-pin-hide = UKRYJ TĘ PINEZKĘ
map-pin-unhide = POKAŻ TĘ PINEZKĘ
map-pin-move = PRZENIEŚ
map-add-media = DODAJ MEDIA
map-hidden-pins = UKRYTE PINEZKI
map-pin-drag = ALT + PRZECIĄGNIJ, BY PRZENIEŚĆ
map-video = WIDEO
map-photo = ZDJĘCIE
map-layer-labels = NAZWY MIEJSC
map-layer-clusters = ŁĄCZ SĄSIEDNIE PINEZKI
map-layer-mini-clusters = TAKŻE NA MINIMAPIE
map-layer-rises = WYSOKOŚCI W POBLIŻU
map-axis-x = X
map-axis-y = Y
map-axis-z = Z

## Overlay

overlay-dock-mods = MODY
overlay-dock-map = MAPA
overlay-dock-session = SESJA
overlay-dock-minimap = MINIMAPA
overlay-minimap-tag = [ MINIMAPA ]
overlay-no-live-map = BRAK POZYCJI
overlay-no-live-map-hint = Minimapa śledzi pozycję gracza w grach z mapą, jeśli uruchomiony trainer potrafi odczytać tę pozycję.
overlay-mode = TRYB
overlay-mode-hint = w grze nakładka jest rysowana w obrazie gry, jeśli gra na to pozwala, a gra zachowuje fokus
overlay-mode-over = NA WIERZCHU
overlay-mode-in-game = W GRZE
overlay-hotkey = SKRÓT
overlay-hotkey-hint = połączenie klawiszy takich jak CTRL, SHIFT lub ALT z jednym innym klawiszem, nieużywane przez inne aplikacje ani mody; skrót jest zajęty tylko w aktywnym oknie gry
overlay-minimap-hotkey = SKRÓT MINIMAPY
overlay-minimap-hotkey-hint = pokazuje lub ukrywa minimapę; skrót jest zajęty tylko w aktywnym oknie gry
overlay-zoom-in-hotkey = PRZYBLIŻ MINIMAPĘ
overlay-zoom-in-hotkey-hint = przybliża minimapę; skrót jest zajęty tylko w aktywnym oknie gry
overlay-zoom-out-hotkey = ODDAL MINIMAPĘ
overlay-zoom-out-hotkey-hint = oddala minimapę; skrót jest zajęty tylko w aktywnym oknie gry
overlay-pin-here-hotkey = PINEZKA TUTAJ
overlay-pin-here-hotkey-hint = stawia pinezkę tam, gdzie stoi gracz, ze zrzutem gry; skrót jest zajęty tylko w aktywnym oknie gry
overlay-exclusive-title = WYŁĄCZNY PEŁNY EKRAN
overlay-exclusive-body = W trybie wyłącznego pełnego ekranu nakładka może się nie pojawić, a gra może się zminimalizować. Nakładka działa najlepiej w trybie okna bez ramek. Można go włączyć w ustawieniach obrazu w grze.
overlay-exclusive-chip = WYŁĄCZNY PEŁNY EKRAN: WYBIERZ OKNO BEZ RAMEK
overlay-exclusive-skip = NIE POKAZUJ WIĘCEJ
overlay-exclusive-ok = OK
overlay-exclusive-ask-hint = pytaj, gdy nakładka otwiera się nad grą w trybie wyłącznego pełnego ekranu

## Settings

settings = USTAWIENIA
settings-appearance = WYGLĄD
settings-overlay = NAKŁADKA
settings-hotkeys = SKRÓTY
settings-app = OGÓLNE
settings-resources = ZASOBY
settings-plugins = WTYCZKI
settings-about = INFO
accent = AKCENT
accent-hint = kolor oznaczeń aktywnej sesji
accent-lime = LIMONKA
accent-amber = BURSZTYN
accent-cyan = CYJAN
accent-white = BIEL
sidebar = PASEK BOCZNY
sidebar-hint = co wyświetlać obok nazwy gry
sidebar-number = NUMER
sidebar-thumbnail = MINIATURA
panels = PANELE
panels-hint = mapa i sesja nad modami lub obok nich
panels-top = GÓRA
panels-side = BOK
reduce-motion = MNIEJ ANIMACJI
reduce-motion-hint = bez migania etykiety „W grze” i bez animowanych przejść
language = JĘZYK
language-hint = język interfejsu Impulse
language-system = SYSTEMOWY
close-to-tray = DO ZASOBNIKA
close-to-tray-hint = zamknięcie okna nie wyłącza Impulse; opcja „Zakończ” jest w menu ikony obok zegara
hide-from-capture = UKRYJ W NAGRANIACH
hide-from-capture-hint = okna Impulse nie widać na zrzutach ekranu, nagraniach ani streamach
hide-from-capture-inside = nakładka w grze jest częścią obrazu gry: ukryte jest tylko okno Impulse, a nie to, co rysuje w grze
folder-path = ścieżka folderu
folder-select = Wybierz folder
settings-sounds = DŹWIĘKI
sounds-toggle-on = WŁĄCZENIE
sounds-toggle-on-hint = dźwięk o rosnącej wysokości po włączeniu moda
sounds-toggle-off = WYŁĄCZENIE
sounds-toggle-off-hint = dźwięk o opadającej wysokości po wyłączeniu moda
sounds-slider = SUWAKI
sounds-slider-hint = krótki klik przy każdym kroku suwaka, także podczas przeciągania
sounds-number = LICZBY
sounds-number-hint = krótki klik przy zmianie wartości
sounds-button = PRZYCISKI
sounds-button-hint = klik po użyciu moda uruchamianego przyciskiem
sounds-selection = WYBÓR
sounds-selection-hint = dźwięk po wybraniu innej opcji
sounds-listen = Odsłuchaj
sounds-choose = Ustaw własny dźwięk z pliku WAV
sounds-reset = Przywróć domyślny dźwięk
sounds-refused = to nie jest plik WAV albo jest większy niż 1 MB
sounds-prompt = Wybierz plik WAV

## Remote

remote = PANEL ZDALNY
remote-open = Panel zdalny: sterowanie Impulse z telefonu
remote-switch = DOSTĘP Z TELEFONU
remote-hint = sterowanie Impulse z telefonu w tej samej sieci
remote-address = ADRES
remote-address-hint = wystarczy zeskanować kod aparatem telefonu; kod jest jednorazowy i zmienia się co pięć minut
remote-failed = brak adresu sieciowego dla panelu
remote-devices = URZĄDZENIA
remote-devices-hint = sparowane telefony i przeglądarki; po cofnięciu dostępu urządzenie od razu się rozłącza
remote-no-devices = brak sparowanych urządzeń
remote-device = urządzenie { $number }
remote-unknown-device = PRZEGLĄDARKA
remote-revoke = COFNIJ
remote-trust = Panel działa przez nieszyfrowane HTTP. Każdy w tej sieci może podejrzeć przesyłane dane, dlatego z panelu należy korzystać tylko w zaufanej sieci.
remote-tips = GDY TELEFON SIĘ NIE ŁĄCZY
remote-tip-network = Telefon i komputer muszą być w tej samej sieci. Połączenie nie zadziała przez dane komórkowe, sieć Wi-Fi dla gości ani VPN.
remote-tip-firewall = Zapora Windows może zablokować Impulse bez ostrzeżenia, zwłaszcza w sieci publicznej. Zmień profil sieci na prywatny albo dodaj Impulse do wyjątków: Zabezpieczenia Windows → Zapora i ochrona sieci → Zezwalaj aplikacji na dostęp przez zaporę.
remote-tip-isolation = Niektóre routery blokują połączenia między urządzeniami Wi-Fi (izolacja AP lub klientów). Wyłącz tę opcję albo użyj innej sieci.
remote-note-changed = Lista się zmieniła. Spróbuj ponownie.
remote-note-not-live = Najpierw uruchom grę.
remote-note-busy = Trwa inna sesja albo trainer jeszcze nie działa.
remote-note-nothing-on = Najpierw włącz co najmniej jeden mod.
remote-note-name = Najpierw nazwij kolekcję.

## Plugins

plugins-install = INSTALUJ
plugins-install-hint = folder lub archiwum .zip z plikiem manifest.json; przed instalacją trzeba zatwierdzić listę dozwolonych adresów
plugins-folder = FOLDER
plugins-zip = ZIP
plugins-pick-folder = Wybierz folder wtyczki
plugins-pick-zip = Wybierz .zip wtyczki
plugins-none = brak zainstalowanych wtyczek
plugins-approve-title = ZAINSTALOWAĆ { $name } { $version }?
plugins-replaces = zastępuje wersję { $version }
plugins-hosts = POBIERA Z
plugins-links = OTWIERA LINKI DO
plugins-no-hosts = brak
plugins-new = NOWY
plugins-sandbox = Wtyczka działa w izolacji i ma dostęp tylko do wymienionych adresów. Warto instalować wyłącznie zaufane wtyczki.
plugins-approve = ZATWIERDŹ
plugins-cancel = ANULUJ
plugins-version = wersja { $version }, zajmuje { $size }
plugins-remove = USUŃ
plugins-value = wartość

## Resources

media-pictures = OBRAZY
media-pictures-hint = grafiki gier i zdjęcia przypisane do pinezek
media-tiles = KAFELKI MAP
media-tiles-hint = podkłady wczytanych map
media-videos = WIDEO
media-videos-hint = nagrania przypisane do pinezek
resources-reading = SPRAWDZANIE ROZMIARU…
resources-in-use = ZAJĘTE MIEJSCE
resources-in-use-hint = wszystkie dane zapisane przez Impulse na tym komputerze
resources-clear = WYCZYŚĆ
resources-on-restart = PO RESTARCIE
resources-clears-on-restart = DO USUNIĘCIA PO RESTARCIE
resources-clear-caches = WYCZYŚĆ CACHE
resources-clear-caches-hint = dane z sekcji poniżej nie zostaną usunięte
resources-clear-all = WYCZYŚĆ WSZYSTKO
resources-limit = LIMIT MEDIÓW
resources-limit-hint = po przekroczeniu limitu najpierw znikają pliki, których najdłużej nie używano
resources-limit-500m = 500 MB
resources-limit-1g = 1 GB
resources-limit-2g = 2 GB
resources-limit-5g = 5 GB
resources-limit-none = BRAK
resources-kept = BEZ ZMIAN
resources-progress = POSTĘP NA MAPACH
resources-progress-hint = znalezione i własne pinezki
resources-hotkeys = SKRÓTY
resources-hotkeys-hint = przypisane skróty klawiszowe
resources-logs = LOGI
resources-logs-hint = do dwóch plików; ich zawartość widać w panelu „LOG”
resources-bytes = { $size } B
resources-kilobytes = { $size } KB
resources-megabytes = { $size } MB
resources-gigabytes = { $size } GB

## Log

log-title = LOG
log-error = BŁĄD
log-warn = UWAGA
log-info = INFO
log-debug = DEBUG
log-filter = filtruj log
log-follow = ŚLEDŹ
log-copy-all = KOPIUJ WSZYSTKO
log-copy = KOPIUJ { $count }
log-copied = SKOPIOWANO
log-clear = WYCZYŚĆ
log-folder = FOLDER
log-detail-empty = KLIKNIJ WPIS, ABY ZOBACZYĆ I ZAZNACZYĆ CAŁĄ TREŚĆ

## About

about-version = WERSJA { $version }
about-tagline = DARMOWY I OPEN SOURCE
about-intro = Impulse tworzy jedna osoba w wolnym czasie. Jest darmowy i ma otwarty kod. Gwiazdka, zgłoszenie błędu albo kawa pomagają go rozwijać.
about-github = GITHUB
about-github-hint = czytaj kod, śledź wydania i zostaw gwiazdkę, żeby inni łatwiej go znaleźli
about-star = GWIAZDKA
about-discord = DISCORD
about-discord-hint = zadaj pytanie, poproś o grę lub funkcję, porozmawiaj z innymi użytkownikami
about-join = DOŁĄCZ
about-bug = ZNALEZIONO BŁĄD?
about-bug-hint = otwórz zgłoszenie i opisz, co się stało i co miało się stać; pomaga LOG z paska stanu
about-report = ZGŁOŚ
about-support = WESPRZYJ IMPULSE
about-support-hint = darmowy i z otwartym kodem; kawa płaci za czas poświęcony każdej aktualizacji
about-coffee = POSTAW KAWĘ
about-contact = KONTAKT
about-contact-hint = w każdej innej sprawie lub w prywatnej: { $address }
about-email = E-MAIL
alpha = ALPHA
plugins-alpha-hint = wtyczki są eksperymentalne; format może się zmienić, a wtyczka może przestać działać po aktualizacji
remote-copy = kopiuj adres

## Updates

update-title = AKTUALIZACJE
update-state-idle = wersja { $version }
update-state-current = masz najnowszą wersję, { $version }
update-state-skipped = wersja { $version } jest pominięta: SPRAWDŹ pokaże ją ponownie
update-state-checking = sprawdzanie aktualizacji...
update-state-available = dostępna jest wersja { $version }
update-state-downloading = pobieranie wersji { $version }...
update-state-ready = wersja { $version } jest gotowa: uruchom Impulse ponownie, aby ją zainstalować
update-check-failed = nie udało się połączyć z listą wydań
update-install-failed = nie udało się zainstalować aktualizacji
update-check = SPRAWDŹ
update-changelog = ZMIANY
update-channel = KANAŁ AKTUALIZACJI
update-channel-hint = wersje wstępne pojawiają się wcześniej i bywają niestabilne; powrót do stabilnych nigdy nie cofa wersji
update-channel-stable = STABILNY
update-channel-pre = WSTĘPNY
update-auto = SPRAWDZAJ AKTUALIZACJE
update-auto-hint = przy starcie i co kilka godzin; pobierana jest tylko lista wydań, nic o tobie nie jest wysyłane
update-title-available = DOSTĘPNA AKTUALIZACJA
update-title-new = CO NOWEGO W { $version }
update-from-to = z { $current } na { $latest }
update-later = PÓŹNIEJ
update-skip = POMIŃ TĘ WERSJĘ
update-restart = URUCHOM PONOWNIE
update-close = ZAMKNIJ
update-busy = trainer jest uruchomiony: aktualizacja zainstaluje się po zamknięciu Impulse
update-no-notes = brak opisu wydania
update-chip-available = AKTUALIZACJA { $version }
update-chip-downloading = POBIERANIE...
update-chip-ready = URUCHOM PONOWNIE
update-changelog-hint = co zmieniło się w każdej wersji
update-open = OTWÓRZ
