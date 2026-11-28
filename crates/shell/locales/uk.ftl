## Shared

off = ВИМК
on = УВІМК
# A number's marks: between thousands, before decimals.
number-group = { "\u00A0" }
number-decimal = ,

## Tray: Windows menu words

tray-open = Відкрити Impulse
tray-stop = Зупинити трейнер
tray-overlay = Оверлей
tray-quit = Вийти

## Failures: a kind's words fit after "It could not be loaded:"

failure-offline = немає з'єднання
failure-signed-out = ви не ввійшли
failure-not-found = не знайдено
failure-unsupported = ще не підтримується
failure-setup-needed = не все налаштовано — перевірте вкладку «ПРОГРАМА» в налаштуваннях
failure-game-not-found = гру не знайдено
failure-launch-failed = лаунчер не зміг запустити гру: деталі в розділі «ЛОГ»
failure-not-started = гра не запустилася вчасно: перевірте її лаунчер і спробуйте знову
failure-not-running = гру не запустили вчасно: запустіть її самі й спробуйте знову
failure-external-launch = цей трейнер працює лише з грою, запущеною вами: запустіть її, потім натисніть ГРА + ТРЕЙНЕР
failure-game-exited = гра закрилася раніше, ніж трейнер під'єднався
failure-refused = відмовлено
failure-elevation = гру запущено від адміністратора: підтвердьте запит, щоб під'єднатися
failure-protected = захист гри не пускає трейнери
failure-needs-64 = трейнеру потрібна 64-бітна збірка гри: запустіть її
failure-needs-32 = трейнеру потрібна 32-бітна збірка гри: запустіть її
failure-load-failed = не вдалося завантажити трейнер у гру: деталі в розділі «ЛОГ»
failure-trainer-failed = трейнер не запустився в грі: деталі в розділі «ЛОГ»
failure-restart = гру вже запущено: закрийте її, потім запустіть звідси
failure-internal = щось пішло не так: деталі в розділі «ЛОГ»
failure-place = { $why } — перевірте вкладку «ПРОГРАМА» в налаштуваннях

## Page

catalog-failed-tag = [ БЕЗ КАТАЛОГУ ]
catalog-failed = КАТАЛОГ ІГОР НЕДОСТУПНИЙ
catalog-failed-body = Не вдалося завантажити каталог: { $why }. Impulse спробує ще раз під час наступного запуску.
library-filter = пошук ігор
picker-filter = назва гри
page-search = { $count ->
    [one] пошук серед { $trainers } трейнера
    [few] пошук серед { $trainers } трейнерів
    [many] пошук серед { $trainers } трейнерів
   *[other] пошук серед { $trainers } трейнера
}
page-search-none = пошук трейнерів
page-search-key = CTRL K
page-add-game = ДОДАТИ ГРУ
page-get-on = ОТРИМАТИ В { $store }
page-add-manually = ДОДАТИ ГРУ ВРУЧНУ...
page-signed-out = ВИ НЕ ВВІЙШЛИ
page-elevated = Запущено від імені адміністратора: ігри з правами адміністратора підключаються без запиту
page-overlay = ОВЕРЛЕЙ
page-star = Обране
page-folder = Відкрити папку гри
page-launcher = Відкрити в { $launcher }
page-source = Параметри гри
page-overlay-default = ОВЕРЛЕЙ: ЯК У НАЛАШТУВАННЯХ
page-overlay-in-game = ОВЕРЛЕЙ: У ГРІ
page-overlay-over = ОВЕРЛЕЙ: ПОВЕРХ ГРИ
page-custom = ВРУЧНУ
page-live = У ГРІ
page-active = { $active }/{ $total } АКТИВНО
page-idle = ОЧІКУЄ
page-log = ЛОГ
page-binding = НАТИСНІТЬ КЛАВІШІ · ESC СКАСУВАТИ · BKSP СКИНУТИ
page-binding-unbind = НАТИСНІТЬ КЛАВІШІ · ESC СКАСУВАТИ · BKSP СКИНУТИ · DEL СТЕРТИ
page-hotkeys-on = КЛАВІШІ УВІМК
page-hotkeys-off = КЛАВІШІ ВИМК
disclaimer-title = ПЕРЕД ПОЧАТКОМ
disclaimer-body = Impulse — неофіційний клієнт для { $sources }. Він використовує наявну сесію та файли з папки програми, у яку ви вже ввійшли на цьому ПК. Від вашого імені Impulse запитує в сервісу список ваших ігор і трейнерів. Impulse не просить пароля й не передає дані облікового запису за межі ПК, крім цих запитів. Impulse не пов'язаний із сервісом і не має його схвалення. За трейнери в онлайн-іграх можна отримати бан. Користуйтеся ними на власний ризик.
disclaimer-accept = ПОГОДЖУЮСЯ
disclaimer-exit = ВИЙТИ
signed-out-title = НЕ ВДАЛОСЯ УВІЙТИ
signed-out-body = Impulse не зміг увійти в { $sources }. Поки вхід не вдасться, трейнери не завантажаться. Перевірте, чи цю програму встановлено на ПК, чи ви в неї ввійшли та чи є інтернет. Потім спробуйте ще раз.
signed-out-retry = ЩЕ РАЗ
signed-out-why = Причина: { $why }
signed-out-trying = Вхід…
signed-out-close = ЗАКРИТИ

## Library

library-favorites = ОБРАНЕ
library-games = ІГРИ
library-no-match = ІГОР НЕ ЗНАЙДЕНО
library-add = + ДОДАТИ
library-live = У ГРІ
library-scanning-tag = [ ПОШУК ]
library-scanning = ШУКАЄМО ВСТАНОВЛЕНІ ІГРИ
library-scanning-body = Звіряємо ігри у ваших лаунчерах із каталогом.
library-empty-tag = [ НЕМАЄ ІГОР ]
library-empty = ВСТАНОВЛЕНИХ ІГОР НЕ ЗНАЙДЕНО
library-empty-body = Impulse перевіряє, чи є у вас ігри з трейнерами в Steam, Epic, GOG, Ubisoft, Xbox, Rockstar та EA.
library-source-exe = EXE  { $file }
library-source-add = + ДОДАТИ .EXE
library-source-change = ЗМІНИТИ .EXE
library-source-remove = ВІДВ'ЯЗАТИ .EXE
library-source-trainer = ЗМІНИТИ ТРЕЙНЕР...
library-source-own-trainer = СВІЙ ТРЕЙНЕР

## Picker

picker-title = ДОДАТИ ГРУ
picker-owned = ЗНАЙДЕНО КІЛЬКА ІГОР. ВИБЕРІТЬ СВОЮ
picker-unowned = У КАТАЛОЗІ НЕМАЄ ЗБІГІВ. ПОШУКАЙТЕ ЗА НАЗВОЮ
picker-in-library = { $store }  У БІБЛІОТЕЦІ
picker-prompt = Виберіть .exe гри
picker-trainer-title = ОБЕРІТЬ ТРЕЙНЕР
picker-trainer = СПОЧАТКУ ІНШІ МАГАЗИНИ, ПОТІМ ЗА НАЗВОЮ: ТРЕЙНЕР ДЛЯ ІНШОЇ ВЕРСІЇ МОЖЕ НЕ ПІДІЙТИ
picker-in-use = ВИКОРИСТОВУЄТЬСЯ
picker-find-title = ЗНАЙТИ ГРУ
picker-find = УСІ ТРЕЙНЕРИ ЗА НАЗВОЮ · ↑ ↓ ОБРАТИ, ENTER ВІДКРИТИ

## Cheats

cheats-failed-tag = [ НЕМАЄ ДАНИХ ]
cheats-failed = СПИСОК ЧИТІВ НЕ ЗАВАНТАЖЕНО
cheats-failed-body = Не вдалося завантажити список: { $why }. Impulse спробує ще раз, коли ви знову відкриєте гру.
cheats-filter = пошук модів
cheats-pinned = ЗАКРІПЛЕНІ
cheats-kept = Після першого завантаження список залишиться на диску.
cheats-notes = ПРОЧИТАЙТЕ ПЕРЕД ГРОЮ
cheats-no-trainer-tag = [ БЕЗ ТРЕЙНЕРА ]
cheats-no-trainer = ДЛЯ { $game } ЩЕ НЕМАЄ ТРЕЙНЕРА
cheats-no-trainer-body = Для цієї гри ще немає трейнера. Щойно він з'явиться, Impulse позначить гру в бібліотеці.
cheats-other-store = Для цього магазину трейнера ще немає, але він є у версії з { $store }. Версії в різних магазинах можуть відрізнятися, тож він може не підійти.
cheats-use-trainer = ВИКОРИСТАТИ ТРЕЙНЕР { $store }
cheats-other-trainer = ІНШИЙ ТРЕЙНЕР...
cheats-not-installed = НЕ ВСТАНОВЛЕНА · ДОДАЙТЕ ГРУ, ЩОБ КОРИСТУВАТИСЯ МОДАМИ
cheats-loading-tag = [ ЗАВАНТАЖЕННЯ ]
cheats-loading = ЗАВАНТАЖУЄМО СПИСОК ЧИТІВ
cheats-shown = { $count ->
    [one] { $shown } МОД
    [few] { $shown } МОДИ
    [many] { $shown } МОДІВ
   *[other] { $shown } МОДА
}
cheats-no-match = МОДІВ НЕ ЗНАЙДЕНО
cheats-updated = ГРУ ОНОВЛЕНО
cheats-updated-body = Трейнер ще не підтримує цю версію гри. Деякі моди можуть не працювати, доки трейнер не оновлять.
cheats-set = ЗАДАТИ
cheats-set-done = OK
cheats-execute = ВИКОНАТИ
cheats-execute-done = ГОТОВО
cheats-save = ЗБЕРЕГТИ
cheats-saved = ЗБЕРЕЖЕНО { $saved }
cheats-save-tip = Моди, які залишаться ввімкненими наприкінці сесії, автоматично ввімкнуться наступного разу.
cheats-reset = СКИНУТИ
cheats-no-collections = НЕМАЄ КОЛЕКЦІЙ
cheats-collect-tip = Збережіть у колекцію моди, які зараз увімкнено.
cheats-edit = ЗМІНИТИ
cheats-delete = ВИДАЛИТИ
cheats-name-field = назва
cheats-cancel = СКАСУВАТИ
cheats-name = НАЗВА
cheats-icon = ЗНАЧОК
cheats-collection-mods = { $count ->
    [one] { $mods } МОД
    [few] { $mods } МОДИ
    [many] { $mods } МОДІВ
   *[other] { $mods } МОДА
}
cheats-edit-collection = ЗМІНИТИ КОЛЕКЦІЮ
cheats-new-collection = НОВА КОЛЕКЦІЯ

## Hotkeys

hotkeys-press-keys = НАТИСНІТЬ КЛАВІШІ
hotkeys-off = ВИМК
hotkeys-in-game = У ГРІ
hotkeys-global = УСЮДИ
hotkeys-enabled = ГАРЯЧІ КЛАВІШІ
hotkeys-enabled-hint = гарячі клавіші трейнера під час гри
hotkeys-works-in = ДЕ ДІЮТЬ
hotkeys-works-in-hint = у режимі «У ГРІ» клавіші діють, лише коли вікно гри активне
hotkeys-bind-hint = ESC СКАСУВАТИ · BKSP СКИНУТИ · DEL СТЕРТИ
hotkeys-clash = { $keys } уже зайнято: { $name }
hotkeys-clash-overlay = { $keys } зарезервовано для оверлея
hotkeys-keep = ЗАЛИШИТИ ОБИДВА
hotkeys-rebind = ПЕРЕПРИЗНАЧИТИ
hotkeys-shared-tip = інший мод теж має цю гарячу клавішу: спрацюють обидва

## Trainer

trainer-none = НЕМАЄ ТРЕЙНЕРА
trainer-mods = { $count ->
    [one] { $count } МОД
    [few] { $count } МОДИ
    [many] { $count } МОДІВ
   *[other] { $count } МОДА
}
trainer-from = ТРЕЙНЕР { $store }
trainer-version = V{ $version }
trainer-version-unknown = ВЕРСІЯ НЕВІДОМА
trainer-not-installed = НЕ ВСТАНОВЛЕНА
trainer-update-pending = ОНОВЛЕННЯ ЩЕ НЕМАЄ
trainer-sessions = { $count ->
    [one] { $count } СЕСІЯ
    [few] { $count } СЕСІЇ
    [many] { $count } СЕСІЙ
   *[other] { $count } СЕСІЇ
}
trainer-played-min = { $count } ХВ У ГРІ
trainer-played-hours = { $hours } ГОД У ГРІ

## Run

run-no-cheats = список читів не завантажено
run-preparing = ПІДГОТОВКА ТРЕЙНЕРА
run-launching = ЗАПУСК ГРИ
run-waiting = ЧЕКАЄМО НА ГРУ
run-attaching = ПІДКЛЮЧЕННЯ ТРЕЙНЕРА
run-stopping = ЗУПИНКА ТРЕЙНЕРА
run-stop = СТОП
run-launch = ГРА + ТРЕЙНЕР
run-enter = ENTER
run-session = СЕСІЯ
run-last-session = ОСТАННЯ СЕСІЯ
run-active = АКТИВНО
run-hotkeys = КЛАВІШІ
run-idle = ЗАПУСТІТЬ ТРЕЙНЕР, ЩОБ КОРИСТУВАТИСЯ МОДАМИ
run-unconfirmed-title = ГРУ ОНОВЛЕНО
run-unconfirmed-body = Трейнер ще не підтримує цю версію гри. Деякі моди можуть не працювати, доки трейнер не оновлять. Однаково запустити?
run-unconfirmed-skip = ЗАПУСКАТИ ЗАВЖДИ
run-ask-cancel = СКАСУВАТИ
run-unconfirmed-play = ОДНАКОВО ЗАПУСТИТИ
run-unconfirmed-ask = ПИТАТИ ПІСЛЯ ОНОВЛЕННЯ ГРИ
run-unconfirmed-ask-hint = питати перед запуском, якщо версія гри новіша за ту, що підтримує трейнер
run-outside-title = ЗАПУСТІТЬ ГРУ САМІ
run-outside-body = Цей трейнер підключається лише до гри, запущеної поза Impulse, а гру не запущено. Запустіть її, дочекайтеся завантаження, потім натисніть ГРА + ТРЕЙНЕР.
run-outside-start = ЗАПУСТИТИ ГРУ

## Map

teleport-failed = ТЕЛЕПОРТ НЕ ВДАВСЯ: { $why }
map-failed-tag = [ БЕЗ КАРТИ ]
map-failed = КАРТА НЕДОСТУПНА
map-failed-body = Не вдалося завантажити карту: { $why }.
map-title = КАРТА
map-none = ДЛЯ ЦІЄЇ ГРИ НЕМАЄ ІНТЕРАКТИВНОЇ КАРТИ
map-link = ПРИВ'ЯЗАТИ КАРТУ
map-unlink = ВІДВ'ЯЗАТИ КАРТУ
map-link-title = ПРИВ'ЯЗАТИ КАРТУ
map-link-caption = ВИБЕРІТЬ СВОЮ ГРУ ЗІ СПИСКУ
map-link-empty = Не знайшли карт для гри з такою назвою.
map-refresh = ОНОВИТИ
map-loading-tag = [ ЗАВАНТАЖЕННЯ ]
map-loading = ЗАВАНТАЖУЄМО КАРТУ
map-loading-body = Після першого завантаження мітки й тайли залишаться на диску.
map-search = місце, група або ключове слово
map-unfinished = Карту ще доповнюють: деяких міток може бракувати, а їхнє розташування може змінитися.
map-found = ЗНАЙДЕНО
map-found-count = ЗНАЙДЕНО
map-mark-found = ПОЗНАЧИТИ
map-unfound = НЕЗНАЙДЕНІ
map-show-all = ПОКАЗАТИ ВСЕ
map-hide-all = СХОВАТИ ВСЕ
map-teleport = ТЕЛЕПОРТ
map-filter-teleport = ТЕЛЕПОРТ
map-teleport-here = ТЕЛЕПОРТ СЮДИ
map-teleport-height = ВИСОТА ТЕЛЕПОРТУ
map-teleport-height-hint = Додається до висоти телепорту до міток і місць карти, в одиницях гри
map-pin-position = Мітка там, де стоїть гравець
map-confirm = ПІДТВЕРДИТИ
map-delete = ВИДАЛИТИ
map-add-pin = ДОДАТИ МІТКУ
map-my-pin = МОЯ МІТКА
map-my-pins = МОЇ МІТКИ
map-pin-name = Моя мітка { $number }
map-name-field = назва мітки
map-note-field = нотатка
map-no-pins = Щоб додати мітку, клацніть правою кнопкою миші на карті.
map-regions = РЕГІОНИ
map-layer-regions = РЕГІОНИ
map-places = { $count ->
    [one] { $places } МІСЦЕ
    [few] { $places } МІСЦЯ
    [many] { $places } МІСЦЬ
   *[other] { $places } МІСЦЯ
}
map-no-matches = Нічого не знайдено. Спробуйте ввести назву групи або коротше слово.
map-nearby = ПОРУЧ
map-copy = КОПІЮВАТИ
map-copied = СКОПІЙОВАНО
map-hide-group = СХОВАТИ НА КАРТІ: { $group }
map-pin-hide = СХОВАТИ ЦЮ МІТКУ
map-pin-unhide = ПОКАЗАТИ ЦЮ МІТКУ
map-pin-move = ПЕРЕНЕСТИ
map-add-media = ДОДАТИ МЕДІА
map-hidden-pins = СХОВАНІ МІТКИ
map-pin-drag = ПЕРЕНЕСЕННЯ: ALT + ПЕРЕТЯГНУТИ
map-video = ВІДЕО
map-photo = ФОТО
map-layer-labels = НАЗВИ МІСЦЬ
map-layer-clusters = ОБ'ЄДНУВАТИ МІТКИ ПОРУЧ
map-layer-mini-clusters = ТАКОЖ НА МІНІКАРТІ
map-layer-rises = ВИСОТИ ПОРУЧ
map-axis-x = X
map-axis-y = Y
map-axis-z = Z

## Overlay

overlay-dock-mods = МОДИ
overlay-dock-map = КАРТА
overlay-dock-session = СЕСІЯ
overlay-dock-minimap = МІНІКАРТА
overlay-minimap-tag = [ МІНІКАРТА ]
overlay-no-live-map = БЕЗ МІНІКАРТИ
overlay-no-live-map-hint = Мінікарта показує, де ви, якщо для гри є карта, а запущений трейнер може визначити вашу позицію.
overlay-mode = РЕЖИМ
overlay-mode-hint = у режимі «У грі» оверлей малюється просто в зображенні гри, якщо гра це дозволяє, і фокус лишається у гри
overlay-mode-over = ПОВЕРХ ГРИ
overlay-mode-in-game = У ГРІ
overlay-hotkey = КЛАВІША
overlay-hotkey-hint = комбінація з однієї клавіші та модифікаторів, яку не використовують інші програми чи моди; працює, лише коли вікно гри активне
overlay-minimap-hotkey = КЛАВІША МІНІКАРТИ
overlay-minimap-hotkey-hint = показує або ховає мінікарту, лише коли вікно гри активне
overlay-zoom-in-hotkey = МІНІКАРТА +
overlay-zoom-in-hotkey-hint = збільшує масштаб мінікарти, лише коли вікно гри активне
overlay-zoom-out-hotkey = МІНІКАРТА −
overlay-zoom-out-hotkey-hint = зменшує масштаб мінікарти, лише коли вікно гри активне
overlay-pin-here-hotkey = МІТКА ТУТ
overlay-pin-here-hotkey-hint = ставить мітку там, де стоїть гравець, зі знімком гри; працює, лише коли вікно гри активне
overlay-exclusive-title = ЕКСКЛЮЗИВНИЙ ПОВНИЙ ЕКРАН
overlay-exclusive-body = В ексклюзивному повноекранному режимі оверлей може не з'явитися, а гра може згорнутися. Щоб оверлей працював краще, виберіть віконний режим без рамки в налаштуваннях графіки гри.
overlay-exclusive-chip = ЕКСКЛЮЗИВНИЙ ЕКРАН: ВИБЕРІТЬ РЕЖИМ БЕЗ РАМКИ
overlay-exclusive-skip = БІЛЬШЕ НЕ ПОКАЗУВАТИ
overlay-exclusive-ok = OK
overlay-exclusive-ask-hint = питати, коли оверлей відкривається поверх гри в ексклюзивному повноекранному режимі

## Settings

settings = НАЛАШТУВАННЯ
settings-appearance = ВИГЛЯД
settings-overlay = ОВЕРЛЕЙ
settings-hotkeys = КЛАВІШІ
settings-app = ПРОГРАМА
settings-resources = РЕСУРСИ
settings-plugins = ПЛАГІНИ
settings-about = ПРО IMPULSE
accent = АКЦЕНТ
accent-hint = колір позначок активного трейнера
accent-lime = ЛАЙМ
accent-amber = БУРШТИН
accent-cyan = ЦІАН
accent-white = БІЛИЙ
sidebar = БІЧНА ПАНЕЛЬ
sidebar-hint = що показувати перед назвою гри
sidebar-number = НОМЕР
sidebar-thumbnail = МІНІАТЮРА
panels = ПАНЕЛІ
panels-hint = карта й сесія над модами або поряд із ними
panels-top = ЗВЕРХУ
panels-side = ЗБОКУ
reduce-motion = МЕНШЕ АНІМАЦІЇ
reduce-motion-hint = без блимання позначки «У ГРІ» та анімованих переходів
language = МОВА
language-hint = мова інтерфейсу Impulse
language-system = СИСТЕМНА
close-to-tray = ЗГОРТАТИ В ТРЕЙ
close-to-tray-hint = закриття вікна ховає Impulse; вийти можна через значок у треї
hide-from-capture = НЕ ПОКАЗУВАТИ В ЗАПИСАХ
hide-from-capture-hint = вікна Impulse не буде на знімках екрана, записах і стрімах
hide-from-capture-inside = оверлей усередині гри — частина картинки гри: ховається лише вікно Impulse, а не те, що воно малює всередині гри
folder-path = шлях до папки
folder-select = Виберіть папку
settings-sounds = ЗВУКИ
sounds-toggle-on = УВІМКНЕННЯ
sounds-toggle-on-hint = тон підвищується, коли вмикаєте мод
sounds-toggle-off = ВИМКНЕННЯ
sounds-toggle-off-hint = тон знижується, коли вимикаєте мод
sounds-slider = ПОВЗУНКИ
sounds-slider-hint = клацання на кожній поділці, поки рухаєте повзунок
sounds-number = ЧИСЛА
sounds-number-hint = клацання щоразу, коли змінюється число
sounds-button = КНОПКИ
sounds-button-hint = клацання під час виконання мода з кнопкою
sounds-selection = ВИБІР
sounds-selection-hint = звук, коли вибираєте інший варіант
sounds-listen = Прослухати
sounds-choose = Вибрати свій WAV-файл
sounds-reset = Повернути стандартний звук
sounds-refused = потрібен WAV-файл розміром до 1 МБ
sounds-prompt = Виберіть WAV-файл

## Remote

remote = ПУЛЬТ
remote-open = Пульт: керуйте Impulse з телефона
remote-switch = ДОСТУП З ТЕЛЕФОНА
remote-hint = керуйте Impulse з телефона в тій самій мережі
remote-address = АДРЕСА
remote-address-hint = відскануйте QR-код камерою телефона. Код одноразовий і оновлюється кожні п'ять хвилин
remote-failed = не вдалося знайти мережеву адресу для підключення
remote-devices = ПРИСТРОЇ
remote-devices-hint = прив'язані телефони й браузери. Якщо забрати доступ, пристрій одразу від'єднається
remote-no-devices = пристроїв поки немає
remote-device = пристрій { $number }
remote-unknown-device = БРАУЗЕР
remote-revoke = ЗАБРАТИ ДОСТУП
remote-trust = HTTP без шифрування: будь-хто в цій мережі може бачити, що передається. Користуйтеся пультом лише в мережах, яким довіряєте.
remote-tips = ЯКЩО ТЕЛЕФОН НЕ ПІДКЛЮЧАЄТЬСЯ
remote-tip-network = Підключіть телефон до тієї ж мережі, що й ПК. Не використовуйте мобільний інтернет, гостьовий Wi-Fi чи VPN.
remote-tip-firewall = Брандмауер Windows може заблокувати Impulse без попередження, особливо в загальнодоступній мережі. Змініть тип мережі на приватний або додайте Impulse до дозволених програм: Безпека Windows > Брандмауер і захист мережі > Дозволити роботу з програмою через брандмауер.
remote-tip-isolation = Деякі роутери не дають пристроям у Wi-Fi бачити одне одного. Вимкніть ізоляцію AP або клієнтів у налаштуваннях роутера чи спробуйте іншу мережу.
remote-note-changed = Список змінився. Спробуйте ще раз.
remote-note-not-live = Спершу запустіть гру.
remote-note-busy = Уже запущено іншу сесію або трейнер ще недоступний.
remote-note-nothing-on = Спершу ввімкніть хоча б один мод.
remote-note-name = Спершу назвіть колекцію.

## Plugins

plugins-install = ВСТАНОВИТИ
plugins-install-hint = виберіть папку або .zip із manifest.json і підтвердьте, до яких адрес плагін матиме доступ
plugins-folder = ПАПКА
plugins-zip = ZIP
plugins-pick-folder = Виберіть папку плагіна
plugins-pick-zip = Виберіть .zip плагіна
plugins-none = плагінів немає
plugins-approve-title = ВСТАНОВИТИ { $name } { $version }?
plugins-replaces = замінює версію { $version }
plugins-hosts = ЗАВАНТАЖУЄ З
plugins-links = ВІДКРИВАЄ ПОСИЛАННЯ НА
plugins-no-hosts = немає
plugins-new = НОВИЙ
plugins-sandbox = Плагін працює в пісочниці й має доступ лише до вказаних адрес. Встановлюйте лише плагіни, яким довіряєте.
plugins-approve = ДОЗВОЛИТИ
plugins-cancel = СКАСУВАТИ
plugins-version = версія { $version } · { $size } на диску
plugins-remove = ВИДАЛИТИ
plugins-value = значення

## Resources

media-pictures = ЗОБРАЖЕННЯ
media-pictures-hint = обкладинки ігор і фото на мітках
media-tiles = ТАЙЛИ КАРТ
media-tiles-hint = фрагменти карт, які ви відкривали
media-videos = ВІДЕО
media-videos-hint = відео на мітках
resources-reading = РАХУЄМО ОБСЯГ...
resources-in-use = ЗАЙНЯТО
resources-in-use-hint = усе, що Impulse зберігає на цьому ПК
resources-clear = ОЧИСТИТИ
resources-on-restart = ПІСЛЯ ПЕРЕЗАПУСКУ
resources-clears-on-restart = ОЧИСТИТЬСЯ ПІСЛЯ ПЕРЕЗАПУСКУ
resources-clear-caches = ОЧИСТИТИ ВЕСЬ КЕШ
resources-clear-caches-hint = ваші дані зі списку нижче збережуться
resources-clear-all = ОЧИСТИТИ ВСЕ
resources-limit = ЛІМІТ МЕДІА
resources-limit-hint = якщо медіафайли займають більше місця, ніж дозволено, першими видаляються ті, які найдовше не відкривали
resources-limit-500m = 500 МБ
resources-limit-1g = 1 ГБ
resources-limit-2g = 2 ГБ
resources-limit-5g = 5 ГБ
resources-limit-none = БЕЗ ЛІМІТУ
resources-kept = ВАШІ ДАНІ
resources-progress = ПРОГРЕС НА КАРТАХ
resources-progress-hint = знайдені та власні мітки
resources-hotkeys = ГАРЯЧІ КЛАВІШІ
resources-hotkeys-hint = призначені вами комбінації клавіш
resources-logs = ЛОГИ
resources-logs-hint = до двох файлів; записи можна переглянути в розділі «ЛОГ»
resources-bytes = { $size } Б
resources-kilobytes = { $size } КБ
resources-megabytes = { $size } МБ
resources-gigabytes = { $size } ГБ

## Log

log-title = ЛОГ
log-error = ERROR
log-warn = WARN
log-info = INFO
log-debug = DEBUG
log-filter = пошук у записах
log-follow = АВТОПРОКРУТКА
log-copy-all = КОПІЮВАТИ ВСЕ
log-copy = КОПІЮВАТИ { $count }
log-copied = СКОПІЙОВАНО
log-clear = ОЧИСТИТИ
log-folder = ПАПКА
log-detail-empty = ВИБЕРІТЬ РЯДОК, ЩОБ ПРОЧИТАТИ Й ВИДІЛИТИ ВЕСЬ ЗАПИС

## About

about-version = ВЕРСІЯ { $version }
about-tagline = БЕЗКОШТОВНО І З ВІДКРИТИМ КОДОМ
about-intro = Impulse робить одна людина у вільний час. Він безкоштовний і має відкритий вихідний код. Зірка на GitHub, звіт про помилку чи чашка кави допомагають проєкту жити.
about-github = GITHUB
about-github-hint = читайте код, стежте за релізами й поставте зірку, щоб проєкт знаходили частіше
about-star = ЗІРКА
about-discord = DISCORD
about-discord-hint = поставте запитання, попросіть гру чи функцію, поспілкуйтеся з іншими користувачами
about-join = ПРИЄДНАТИСЯ
about-bug = ЗНАЙШЛИ ПОМИЛКУ?
about-bug-hint = створіть issue: що сталося і чого ви очікували. Допоможе LOG із рядка стану
about-report = ПОВІДОМИТИ
about-support = ПІДТРИМАТИ IMPULSE
about-support-hint = безкоштовно і з відкритим кодом: чашка кави оплачує час, який іде на кожне оновлення
about-coffee = ПОЧАСТУВАТИ КАВОЮ
about-contact = КОНТАКТ
about-contact-hint = усе інше або особисте питання: { $address }
about-email = ПОШТА
alpha = АЛЬФА
plugins-alpha-hint = плагіни експериментальні: формат може змінитися, а плагін може перестати працювати після оновлення
remote-copy = скопіювати адресу

## Updates

update-title = ОНОВЛЕННЯ
update-state-idle = версія { $version }
update-state-current = у вас остання версія, { $version }
update-state-skipped = версія { $version } пропущена: ПЕРЕВІРИТИ запропонує її знову
update-state-checking = перевіряємо оновлення...
update-state-available = доступна версія { $version }
update-state-downloading = завантажуємо { $version }...
update-state-ready = версія { $version } готова: перезапустіть Impulse, щоб встановити її
update-check-failed = не вдалося зв'язатися зі списком релізів
update-install-failed = не вдалося встановити оновлення
update-check = ПЕРЕВІРИТИ
update-changelog = ЩО НОВОГО
update-channel = КАНАЛ ОНОВЛЕНЬ
update-channel-hint = пре-релізи приходять раніше й можуть бути нестабільними; повернення на стабільний канал ніколи не знижує версію
update-channel-stable = СТАБІЛЬНИЙ
update-channel-pre = ПРЕ-РЕЛІЗ
update-auto = ПЕРЕВІРЯТИ ОНОВЛЕННЯ
update-auto-hint = під час запуску й раз на кілька годин; запитується лише список релізів, нічого про вас не надсилається
update-title-available = ДОСТУПНЕ ОНОВЛЕННЯ
update-title-new = ЩО НОВОГО В { $version }
update-from-to = з { $current } на { $latest }
update-later = ПІЗНІШЕ
update-skip = ПРОПУСТИТИ ЦЮ ВЕРСІЮ
update-restart = ПЕРЕЗАПУСТИТИ
update-close = ЗАКРИТИ
update-busy = працює трейнер: оновлення встановиться, коли ви закриєте Impulse
update-no-notes = опису релізу немає
update-chip-available = ОНОВЛЕННЯ { $version }
update-chip-downloading = ЗАВАНТАЖЕННЯ...
update-chip-ready = ПЕРЕЗАПУСК ДЛЯ ОНОВЛЕННЯ
update-changelog-hint = що змінилося в кожній версії
update-open = ВІДКРИТИ
