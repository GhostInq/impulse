## Shared

off = ВЫКЛ
on = ВКЛ
# A number's marks: between thousands, before decimals.
number-group = { "\u00A0" }
number-decimal = ,

## Tray: Windows menu words

tray-open = Открыть Impulse
tray-stop = Остановить трейнер
tray-overlay = Оверлей
tray-quit = Выйти

## Failures: a kind's words fit after "It could not be loaded:"

failure-offline = нет соединения
failure-signed-out = вы не вошли в аккаунт
failure-not-found = не найдено
failure-unsupported = пока не поддерживается
failure-setup-needed = не всё настроено. Откройте раздел «Общие» в настройках
failure-game-not-found = игра не найдена
failure-launch-failed = лаунчер не смог запустить игру: подробности в логе
failure-not-started = игра не запустилась вовремя: проверьте её лаунчер и попробуйте снова
failure-not-running = игру не запустили вовремя: запустите её сами и попробуйте снова
failure-external-launch = этот трейнер работает только с игрой, запущенной вами: запустите её, затем нажмите ИГРА + ТРЕЙНЕР
failure-game-exited = игра закрылась раньше, чем трейнер подключился
failure-refused = запрос отклонён
failure-elevation = игра запущена от администратора: подтвердите запрос, чтобы подключиться
failure-protected = защита игры не пускает трейнеры
failure-needs-64 = трейнеру нужна 64-битная сборка игры: запустите её
failure-needs-32 = трейнеру нужна 32-битная сборка игры: запустите её
failure-load-failed = не удалось загрузить трейнер в игру: подробности в логе
failure-trainer-failed = трейнер не запустился в игре: подробности в логе
failure-restart = игра уже запущена: закройте её, затем запустите отсюда
failure-internal = что-то пошло не так: подробности в логе
failure-place = { $why }. Проверьте раздел «Общие» в настройках

## Page

catalog-failed-tag = [ НЕТ КАТАЛОГА ]
catalog-failed = КАТАЛОГ ИГР НЕДОСТУПЕН
catalog-failed-body = Не удалось скачать: { $why }. Impulse повторит попытку при следующем запуске.
library-filter = поиск игр
picker-filter = название игры
page-search = { $count ->
    [one] поиск по { $trainers } трейнеру
    [few] поиск по { $trainers } трейнерам
    [many] поиск по { $trainers } трейнерам
   *[other] поиск по { $trainers } трейнера
}
page-search-none = поиск трейнеров
page-search-key = CTRL K
page-add-game = ДОБАВИТЬ ИГРУ
page-get-on = ПОЛУЧИТЬ В { $store }
page-add-manually = ДОБАВИТЬ ИГРУ ВРУЧНУЮ...
page-signed-out = ВЫ НЕ ВОШЛИ
page-elevated = Запущено от имени администратора: игры с правами администратора подключаются без запроса
page-overlay = ОВЕРЛЕЙ
page-star = Избранное
page-folder = Открыть папку игры
page-launcher = Открыть в { $launcher }
page-source = Параметры игры
page-overlay-default = ОВЕРЛЕЙ: КАК В НАСТРОЙКАХ
page-overlay-in-game = ОВЕРЛЕЙ: В ИГРЕ
page-overlay-over = ОВЕРЛЕЙ: ПОВЕРХ ИГРЫ
page-custom = .EXE
page-live = В ИГРЕ
page-active = { $active }/{ $total } АКТИВНО
page-idle = ВЫКЛ
page-log = ЛОГ
page-binding = НАЖМИТЕ КЛАВИШИ · ESC ОТМЕНА · BKSP СБРОС
page-binding-unbind = НАЖМИТЕ КЛАВИШИ · ESC ОТМЕНА · BKSP СБРОС · DEL СНЯТЬ
page-hotkeys-on = ХОТКЕИ ВКЛ
page-hotkeys-off = ХОТКЕИ ВЫКЛ
disclaimer-title = ПЕРЕД НАЧАЛОМ
disclaimer-body = Impulse — неофициальный клиент, который работает с вашим аккаунтом в { $sources }. Он использует открытую сессию и файлы приложения на этом ПК, чтобы запрашивать список игр и трейнеры от вашего имени. Impulse не просит пароль. Данные аккаунта остаются на этом ПК, кроме тех, что нужны для запросов к сервису. Сервис не связан с Impulse и не одобрял его. За трейнеры в онлайн-играх могут забанить. Используйте их на свой риск.
disclaimer-accept = СОГЛАСЕН
disclaimer-exit = ВЫЙТИ
signed-out-title = НЕ УДАЛОСЬ ВОЙТИ
signed-out-body = Impulse не смог войти в { $sources }. Проверьте, что на этом ПК есть интернет, приложение установлено и вы вошли в аккаунт. Затем попробуйте снова: без входа трейнеры не загрузятся.
signed-out-retry = ПОВТОРИТЬ
signed-out-why = Причина: { $why }
signed-out-trying = Вход…
signed-out-close = ЗАКРЫТЬ

## Library

library-favorites = ИЗБРАННОЕ
library-games = ИГРЫ
library-no-match = ИГР НЕ НАЙДЕНО
library-add = + ИГРА
library-live = В ИГРЕ
library-scanning-tag = [ ПОИСК ]
library-scanning = ПОИСК УСТАНОВЛЕННЫХ ИГР
library-scanning-body = Сверяем игры из ваших лаунчеров с каталогом.
library-empty-tag = [ НЕТ ИГР ]
library-empty = УСТАНОВЛЕННЫЕ ИГРЫ НЕ НАЙДЕНЫ
library-empty-body = Impulse ищет в Steam, Epic, GOG, Ubisoft, Xbox, Rockstar и EA только игры с трейнерами.
library-source-exe = EXE  { $file }
library-source-add = + .EXE
library-source-change = СМЕНИТЬ .EXE
library-source-remove = УБРАТЬ .EXE
library-source-trainer = СМЕНИТЬ ТРЕЙНЕР...
library-source-own-trainer = СВОЙ ТРЕЙНЕР

## Picker

picker-title = ДОБАВИТЬ ИГРУ
picker-owned = НАШЛОСЬ НЕСКОЛЬКО ИГР. ВЫБЕРИТЕ СВОЮ
picker-unowned = ИГРА НЕ НАШЛАСЬ В КАТАЛОГЕ. ВВЕДИТЕ НАЗВАНИЕ
picker-in-library = { $store }  В БИБЛИОТЕКЕ
picker-prompt = Выберите .exe игры
picker-trainer-title = ВЫБЕРИТЕ ТРЕЙНЕР
picker-trainer = СНАЧАЛА ДРУГИЕ МАГАЗИНЫ, ПОТОМ ПО НАЗВАНИЮ: ТРЕЙНЕР ДЛЯ ДРУГОЙ ВЕРСИИ МОЖЕТ НЕ ПОДОЙТИ
picker-in-use = ИСПОЛЬЗУЕТСЯ
picker-find-title = НАЙТИ ИГРУ
picker-find = ВСЕ ТРЕЙНЕРЫ ПО НАЗВАНИЮ · ↑ ↓ ВЫБРАТЬ, ENTER ОТКРЫТЬ

## Cheats

cheats-failed-tag = [ НЕТ ДАННЫХ ]
cheats-failed = НЕ УДАЛОСЬ ЗАГРУЗИТЬ СПИСОК ЧИТОВ
cheats-failed-body = Не удалось скачать: { $why }. Impulse попробует снова, когда вы откроете игру.
cheats-filter = поиск модов
cheats-pinned = ЗАКРЕПЛЁННЫЕ
cheats-kept = После первой загрузки список сохранится на диске.
cheats-notes = ПРОЧТИТЕ ПЕРЕД ИГРОЙ
cheats-no-trainer-tag = [ НЕТ ТРЕЙНЕРА ]
cheats-no-trainer = ТРЕЙНЕРА ДЛЯ { $game } ПОКА НЕТ
cheats-no-trainer-body = Для этой игры пока нет трейнера. Когда он появится, Impulse отметит игру в библиотеке.
cheats-other-store = Для этого магазина трейнера пока нет, но он есть у версии из { $store }. Версии в разных магазинах могут отличаться, поэтому он может не подойти.
cheats-use-trainer = ИСПОЛЬЗОВАТЬ ТРЕЙНЕР { $store }
cheats-other-trainer = ДРУГОЙ ТРЕЙНЕР...
cheats-not-installed = НЕ УСТАНОВЛЕНА · ДОБАВЬТЕ ИГРУ, ЧТОБЫ ВКЛЮЧАТЬ МОДЫ
cheats-loading-tag = [ ЗАГРУЗКА ]
cheats-loading = ЗАГРУЗКА СПИСКА ЧИТОВ
cheats-shown = { $count ->
    [one] { $shown } МОД
    [few] { $shown } МОДА
    [many] { $shown } МОДОВ
   *[other] { $shown } МОДА
}
cheats-no-match = МОДЫ НЕ НАЙДЕНЫ
cheats-updated = ИГРА ОБНОВЛЕНА
cheats-updated-body = Трейнер пока не поддерживает эту версию игры. Некоторые моды могут не работать до обновления трейнера.
cheats-set = ВВОД
cheats-set-done = OK
cheats-execute = ПРИМЕНИТЬ
cheats-execute-done = ГОТОВО
cheats-save = СОХРАНЯТЬ
cheats-saved = СОХРАНЕНО { $saved }
cheats-save-tip = Моды, которые вы оставите включёнными, снова заработают при следующем запуске трейнера.
cheats-reset = СБРОС
cheats-no-collections = НЕТ КОЛЛЕКЦИЙ
cheats-collect-tip = В коллекцию попадут все включённые моды.
cheats-edit = ИЗМЕНИТЬ
cheats-delete = УДАЛИТЬ
cheats-name-field = название
cheats-cancel = ОТМЕНА
cheats-name = НАЗВАНИЕ
cheats-icon = ИКОНКА
cheats-collection-mods = { $count ->
    [one] { $mods } МОД
    [few] { $mods } МОДА
    [many] { $mods } МОДОВ
   *[other] { $mods } МОДА
}
cheats-edit-collection = ИЗМЕНИТЬ КОЛЛЕКЦИЮ
cheats-new-collection = НОВАЯ КОЛЛЕКЦИЯ

## Hotkeys

hotkeys-press-keys = НАЖМИТЕ КЛАВИШИ
hotkeys-off = ВЫКЛ
hotkeys-in-game = В ИГРЕ
hotkeys-global = ВЕЗДЕ
hotkeys-enabled = ХОТКЕИ
hotkeys-enabled-hint = хоткеи модов, пока запущен трейнер
hotkeys-works-in = ГДЕ РАБОТАЮТ
hotkeys-works-in-hint = «В игре» — только пока окно игры на переднем плане
hotkeys-bind-hint = ESC ОТМЕНА · BKSP СБРОС · DEL СНЯТЬ
hotkeys-clash = { $keys } уже занято: { $name }
hotkeys-clash-overlay = { $keys } зарезервировано для оверлея
hotkeys-keep = ОСТАВИТЬ ОБА
hotkeys-rebind = ПЕРЕНАЗНАЧИТЬ
hotkeys-shared-tip = у другого мода такой же хоткей: сработают оба

## Trainer

trainer-none = НЕТ ТРЕЙНЕРА
trainer-mods = { $count ->
    [one] { $count } МОД
    [few] { $count } МОДА
    [many] { $count } МОДОВ
   *[other] { $count } МОДА
}
trainer-from = ТРЕЙНЕР { $store }
trainer-version = V{ $version }
trainer-version-unknown = ВЕРСИЯ НЕИЗВЕСТНА
trainer-not-installed = НЕ УСТАНОВЛЕНА
trainer-update-pending = НУЖНО ОБНОВЛЕНИЕ
trainer-sessions = { $count ->
    [one] { $count } СЕССИЯ
    [few] { $count } СЕССИИ
    [many] { $count } СЕССИЙ
   *[other] { $count } СЕССИИ
}
trainer-played-min = { $count } МИН В ИГРЕ
trainer-played-hours = { $hours } Ч В ИГРЕ

## Run

run-no-cheats = список читов не загружен
run-preparing = ПОДГОТОВКА ТРЕЙНЕРА
run-launching = ЗАПУСК ИГРЫ
run-waiting = ОЖИДАНИЕ ИГРЫ
run-attaching = ПОДКЛЮЧЕНИЕ ТРЕЙНЕРА
run-stopping = ОСТАНОВКА ТРЕЙНЕРА
run-stop = СТОП
run-launch = ИГРА + ТРЕЙНЕР
run-enter = ENTER
run-session = СЕССИЯ
run-last-session = ПРОШЛАЯ СЕССИЯ
run-active = АКТИВНО
run-hotkeys = ХОТКЕИ
run-idle = МОДЫ СТАНУТ ДОСТУПНЫ ПОСЛЕ ЗАПУСКА ТРЕЙНЕРА
run-unconfirmed-title = ИГРА ОБНОВЛЕНА
run-unconfirmed-body = Трейнер пока не поддерживает эту версию игры. Некоторые моды могут не работать до обновления трейнера. Всё равно запустить?
run-unconfirmed-skip = ЗАПУСКАТЬ ВСЕГДА
run-ask-cancel = ОТМЕНА
run-unconfirmed-play = ВСЁ РАВНО ЗАПУСТИТЬ
run-unconfirmed-ask = СПРАШИВАТЬ ПОСЛЕ ОБНОВЛЕНИЯ ИГРЫ
run-unconfirmed-ask-hint = спрашивать перед запуском, если версия игры новее той, что поддерживает трейнер
run-outside-title = ЗАПУСТИТЕ ИГРУ САМИ
run-outside-body = Этот трейнер подключается только к игре, запущенной вне Impulse, а игра не запущена. Запустите её, дождитесь загрузки, затем нажмите ИГРА + ТРЕЙНЕР.
run-outside-start = ЗАПУСТИТЬ ИГРУ

## Map

teleport-failed = СБОЙ ТЕЛЕПОРТА: { $why }
map-failed-tag = [ НЕТ КАРТЫ ]
map-failed = КАРТА НЕДОСТУПНА
map-failed-body = Не удалось загрузить: { $why }.
map-title = КАРТА
map-none = ДЛЯ ЭТОЙ ИГРЫ НЕТ ИНТЕРАКТИВНОЙ КАРТЫ
map-link = ПРИВЯЗАТЬ КАРТУ
map-unlink = ОТВЯЗАТЬ КАРТУ
map-link-title = ПРИВЯЗАТЬ КАРТУ
map-link-caption = ВЫБЕРИТЕ СВОЮ ИГРУ ИЗ СПИСКА
map-link-empty = Карт для игры с таким названием нет.
map-refresh = ОБНОВИТЬ
map-loading-tag = [ ЗАГРУЗКА ]
map-loading = ЗАГРУЗКА КАРТЫ
map-loading-body = После первой загрузки точки и тайлы сохранятся на диске.
map-search = место, группа или ключевое слово
map-unfinished = Карту ещё дополняют: некоторых точек пока нет, а положение других может измениться.
map-found = НАЙДЕНО
map-found-count = НАЙДЕНО
map-mark-found = ОТМЕТИТЬ
map-unfound = НЕ НАЙДЕНО
map-show-all = ПОКАЗАТЬ ВСЁ
map-hide-all = СКРЫТЬ ВСЁ
map-teleport = ТЕЛЕПОРТ
map-filter-teleport = ТЕЛЕПОРТ
map-teleport-here = ТЕЛЕПОРТ СЮДА
map-teleport-height = ВЫСОТА ТЕЛЕПОРТА
map-teleport-height-hint = Добавляется к высоте телепорта к точкам и местам карты, в единицах игры
map-pin-position = Точка там, где стоит игрок
map-confirm = ПОДТВЕРДИТЬ
map-delete = УДАЛИТЬ
map-add-pin = ПОСТАВИТЬ ТОЧКУ
map-my-pin = МОЯ ТОЧКА
map-my-pins = МОИ ТОЧКИ
map-pin-name = Моя точка { $number }
map-name-field = название точки
map-note-field = заметка
map-no-pins = Щёлкните правой кнопкой по карте, чтобы поставить точку.
map-regions = РЕГИОНЫ
map-layer-regions = РЕГИОНЫ
map-places = { $count ->
    [one] { $places } ТОЧКА
    [few] { $places } ТОЧКИ
    [many] { $places } ТОЧЕК
   *[other] { $places } ТОЧКИ
}
map-no-matches = Ничего не найдено. Попробуйте поискать по группе или сократить запрос.
map-nearby = РЯДОМ
map-copy = КОПИРОВАТЬ
map-copied = СКОПИРОВАНО
map-hide-group = СКРЫТЬ { $group } НА КАРТЕ
map-pin-hide = СКРЫТЬ ЭТУ ТОЧКУ
map-pin-unhide = ПОКАЗАТЬ ЭТУ ТОЧКУ
map-pin-move = ПЕРЕНЕСТИ
map-add-media = ДОБАВИТЬ МЕДИА
map-hidden-pins = СКРЫТЫЕ ТОЧКИ
map-pin-drag = ПЕРЕНОС: ALT + ПЕРЕТАЩИТЬ
map-video = ВИДЕО
map-photo = ФОТО
map-layer-labels = НАЗВАНИЯ МЕСТ
map-layer-clusters = ГРУППЫ БЛИЗКИХ ТОЧЕК
map-layer-mini-clusters = ТАКЖЕ НА МИНИКАРТЕ
map-layer-rises = ВЫСОТЫ РЯДОМ
map-axis-x = X
map-axis-y = Y
map-axis-z = Z

## Overlay

overlay-dock-mods = МОДЫ
overlay-dock-map = КАРТА
overlay-dock-session = СЕССИЯ
overlay-dock-minimap = МИНИКАРТА
overlay-minimap-tag = [ МИНИКАРТА ]
overlay-no-live-map = НЕТ МИНИКАРТЫ
overlay-no-live-map-hint = Миникарта следует за вами, если у игры есть карта, а запущенный трейнер умеет отслеживать ваше положение.
overlay-mode = РЕЖИМ
overlay-mode-hint = в режиме «В игре» оверлей рисуется прямо в изображении игры, если игра это позволяет, и фокус остаётся у игры
overlay-mode-over = ПОВЕРХ ИГРЫ
overlay-mode-in-game = В ИГРЕ
overlay-hotkey = ХОТКЕЙ
overlay-hotkey-hint = сочетание CTRL, SHIFT, ALT или WIN с одной клавишей, не занятое другими приложениями или модами; работает, только пока окно игры на переднем плане
overlay-minimap-hotkey = ХОТКЕЙ МИНИКАРТЫ
overlay-minimap-hotkey-hint = показывает и скрывает миникарту; работает, только пока окно игры на переднем плане
overlay-zoom-in-hotkey = ПРИБЛИЗИТЬ
overlay-zoom-in-hotkey-hint = приближает миникарту; работает, только пока окно игры на переднем плане
overlay-zoom-out-hotkey = ОТДАЛИТЬ
overlay-zoom-out-hotkey-hint = отдаляет миникарту; работает, только пока окно игры на переднем плане
overlay-pin-here-hotkey = ТОЧКА ЗДЕСЬ
overlay-pin-here-hotkey-hint = ставит точку там, где стоит игрок, со снимком игры; работает, только пока окно игры на переднем плане
overlay-exclusive-title = ПОЛНЫЙ ЭКРАН
overlay-exclusive-body = В режиме «Полный экран» оверлей может не появиться, а игра — свернуться. Выберите «Окно без рамки» в настройках графики игры.
overlay-exclusive-chip = ПОЛНЫЙ ЭКРАН: ВЫБЕРИТЕ ОКНО БЕЗ РАМКИ
overlay-exclusive-skip = БОЛЬШЕ НЕ ПОКАЗЫВАТЬ
overlay-exclusive-ok = OK
overlay-exclusive-ask-hint = спрашивать, когда оверлей открывается поверх игры в режиме «Полный экран»

## Settings

settings = НАСТРОЙКИ
settings-appearance = ОФОРМЛЕНИЕ
settings-overlay = ОВЕРЛЕЙ
settings-hotkeys = ХОТКЕИ
settings-app = ОБЩИЕ
settings-resources = РЕСУРСЫ
settings-plugins = ПЛАГИНЫ
settings-about = О ПРОГРАММЕ
accent = АКЦЕНТ
accent-hint = выделяет активный трейнер
accent-lime = ЛАЙМ
accent-amber = ЯНТАРНЫЙ
accent-cyan = БИРЮЗА
accent-white = БЕЛЫЙ
sidebar = БОКОВАЯ ПАНЕЛЬ
sidebar-hint = что показывать слева от названия игры
sidebar-number = НОМЕР
sidebar-thumbnail = МИНИАТЮРА
panels = ПАНЕЛИ
panels-hint = карта и сессия над модами или рядом с ними
panels-top = СВЕРХУ
panels-side = СБОКУ
reduce-motion = МЕНЬШЕ АНИМАЦИИ
reduce-motion-hint = без мигания «В игре» и анимированных переходов
language = ЯЗЫК
language-hint = язык интерфейса Impulse
language-system = СИСТЕМНЫЙ
close-to-tray = СВОРАЧИВАТЬ В ТРЕЙ
close-to-tray-hint = крестик скроет окно; чтобы выйти, воспользуйтесь значком в трее
hide-from-capture = СКРЫТЬ ПРИ ЗАХВАТЕ
hide-from-capture-hint = окно Impulse не попадёт на скриншоты, записи и трансляции
hide-from-capture-inside = оверлей внутри игры — часть картинки игры: скрывается только окно Impulse, а не то, что оно рисует внутри игры
folder-path = путь к папке
folder-select = Выберите папку
settings-sounds = ЗВУКИ
sounds-toggle-on = ВКЛЮЧЕНИЕ
sounds-toggle-on-hint = звук с повышением тона при включении мода
sounds-toggle-off = ВЫКЛЮЧЕНИЕ
sounds-toggle-off-hint = звук с понижением тона при выключении мода
sounds-slider = ПОЛЗУНКИ
sounds-slider-hint = щелчок на каждом делении, пока вы двигаете ползунок
sounds-number = ЧИСЛА
sounds-number-hint = щелчок при изменении значения
sounds-button = КНОПКИ
sounds-button-hint = щелчок при нажатии кнопки мода
sounds-selection = ВЫБОР
sounds-selection-hint = звук при смене варианта
sounds-listen = Прослушать
sounds-choose = Выбрать свой WAV-файл
sounds-reset = Вернуть встроенный звук
sounds-refused = не WAV-файл или больше 1 МБ
sounds-prompt = Выберите WAV-файл

## Remote

remote = ПУЛЬТ УПРАВЛЕНИЯ
remote-open = Пульт управления Impulse с телефона
remote-switch = ДОСТУП С ТЕЛЕФОНА
remote-hint = телефон и ПК должны быть в одной сети
remote-address = АДРЕС
remote-address-hint = наведите камеру телефона на QR-код: он одноразовый и обновляется каждые пять минут
remote-failed = не удалось найти сетевой адрес для подключения
remote-devices = УСТРОЙСТВА
remote-devices-hint = телефоны и браузеры с доступом к пульту; удаление сразу отключит устройство
remote-no-devices = устройств пока нет
remote-device = устройство { $number }
remote-unknown-device = БРАУЗЕР
remote-revoke = УДАЛИТЬ
remote-trust = HTTP не шифрует данные: их может увидеть любой в этой сети. Подключайтесь только в сетях, которым доверяете.
remote-tips = ЕСЛИ ТЕЛЕФОН НЕ ПОДКЛЮЧАЕТСЯ
remote-tip-network = Подключите телефон к той же сети, что и ПК. Не используйте мобильный интернет, гостевую сеть Wi-Fi или VPN.
remote-tip-firewall = Брандмауэр Windows может заблокировать Impulse без предупреждения, особенно в общедоступной сети. Сделайте сеть частной или добавьте Impulse в список разрешённых приложений: «Безопасность Windows» → «Брандмауэр и защита сети» → «Разрешить работу с приложением через брандмауэр».
remote-tip-isolation = Некоторые роутеры не дают устройствам в Wi-Fi видеть друг друга. Отключите изоляцию клиентов (AP Isolation) в настройках роутера или попробуйте другую сеть.
remote-note-changed = Список изменился. Попробуйте ещё раз.
remote-note-not-live = Сначала запустите игру.
remote-note-busy = Уже идёт сессия или трейнер ещё не готов.
remote-note-nothing-on = Сначала включите хотя бы один мод.
remote-note-name = Сначала назовите коллекцию.

## Plugins

plugins-install = УСТАНОВИТЬ
plugins-install-hint = выберите папку или .zip с manifest.json и подтвердите доступ плагина к сайтам
plugins-folder = ПАПКА
plugins-zip = ZIP
plugins-pick-folder = Выберите папку плагина
plugins-pick-zip = Выберите .zip плагина
plugins-none = плагинов нет
plugins-approve-title = УСТАНОВИТЬ { $name } { $version }?
plugins-replaces = заменяет версию { $version }
plugins-hosts = ЗАГРУЗКИ С
plugins-links = ССЫЛКИ НА
plugins-no-hosts = нет адресов
plugins-new = НОВЫЙ
plugins-sandbox = Плагин работает в песочнице: доступ есть только к адресам из списка. Устанавливайте только плагины, которым доверяете.
plugins-approve = РАЗРЕШИТЬ
plugins-cancel = ОТМЕНА
plugins-version = версия { $version }, занимает { $size }
plugins-remove = УДАЛИТЬ
plugins-value = значение

## Resources

media-pictures = ИЗОБРАЖЕНИЯ
media-pictures-hint = обложки игр и картинки к точкам на карте
media-tiles = ТАЙЛЫ КАРТ
media-tiles-hint = фрагменты карт, которые вы открывали
media-videos = ВИДЕО
media-videos-hint = ролики для точек на карте
resources-reading = СЧИТАЕМ ОБЪЁМ...
resources-in-use = ЗАНЯТО
resources-in-use-hint = всё, что Impulse хранит на этом ПК
resources-clear = ОЧИСТИТЬ
resources-on-restart = ПРИ ПЕРЕЗАПУСКЕ
resources-clears-on-restart = ОЧИСТИТСЯ ПРИ ПЕРЕЗАПУСКЕ
resources-clear-caches = ОЧИСТИТЬ ВЕСЬ КЭШ
resources-clear-caches-hint = данные из списка ниже останутся
resources-clear-all = ОЧИСТИТЬ ВСЁ
resources-limit = ЛИМИТ МЕДИА
resources-limit-hint = при превышении лимита первыми удалятся медиафайлы, которые дольше всего не использовались
resources-limit-500m = 500 МБ
resources-limit-1g = 1 ГБ
resources-limit-2g = 2 ГБ
resources-limit-5g = 5 ГБ
resources-limit-none = НЕТ
resources-kept = ВНЕ КЭША
resources-progress = ПРОГРЕСС НА КАРТАХ
resources-progress-hint = какие точки вы нашли и какие добавили сами
resources-hotkeys = ХОТКЕИ
resources-hotkeys-hint = назначенные вами сочетания клавиш
resources-logs = ЛОГИ
resources-logs-hint = до двух файлов; их можно открыть в разделе «Лог»
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
log-filter = поиск по логу
log-follow = АВТОПРОКРУТКА
log-copy-all = КОПИРОВАТЬ ВСЁ
log-copy = КОПИРОВАТЬ { $count }
log-copied = СКОПИРОВАНО
log-clear = ОЧИСТИТЬ
log-folder = ПАПКА
log-detail-empty = НАЖМИТЕ НА СТРОКУ, ЧТОБЫ ПРОЧИТАТЬ И ВЫДЕЛИТЬ ВСЮ ЗАПИСЬ

## About

about-version = ВЕРСИЯ { $version }
about-tagline = БЕСПЛАТНО И С ОТКРЫТЫМ КОДОМ
about-intro = Impulse делает один человек в свободное время. Он бесплатный и с открытым исходным кодом. Звезда на GitHub, отчёт об ошибке или чашка кофе помогают проекту жить.
about-github = GITHUB
about-github-hint = читайте код, следите за релизами и поставьте звезду, чтобы проект находили чаще
about-star = ЗВЕЗДА
about-discord = DISCORD
about-discord-hint = задайте вопрос, попросите игру или функцию, пообщайтесь с другими пользователями
about-join = ВОЙТИ
about-bug = НАШЛИ ОШИБКУ?
about-bug-hint = создайте issue: что случилось и что ожидалось. Поможет LOG из строки состояния
about-report = СООБЩИТЬ
about-support = ПОДДЕРЖАТЬ IMPULSE
about-support-hint = бесплатно и с открытым кодом: чашка кофе оплачивает время, которое уходит на каждое обновление
about-coffee = УГОСТИТЬ КОФЕ
about-contact = КОНТАКТ
about-contact-hint = всё остальное или личный вопрос: { $address }
about-email = ПОЧТА
alpha = АЛЬФА
plugins-alpha-hint = плагины экспериментальные: формат может измениться, а плагин может перестать работать после обновления
remote-copy = скопировать адрес

## Updates

update-title = ОБНОВЛЕНИЯ
update-state-idle = версия { $version }
update-state-current = у вас последняя версия, { $version }
update-state-skipped = версия { $version } пропущена: ПРОВЕРИТЬ предложит её снова
update-state-checking = проверяем обновления...
update-state-available = доступна версия { $version }
update-state-downloading = скачиваем { $version }...
update-state-ready = версия { $version } готова: перезапустите Impulse, чтобы установить её
update-check-failed = не удалось связаться со списком релизов
update-install-failed = не удалось установить обновление
update-check = ПРОВЕРИТЬ
update-changelog = ЧТО НОВОГО
update-channel = КАНАЛ ОБНОВЛЕНИЙ
update-channel-hint = пре-релизы приходят раньше и могут быть нестабильны; возврат на стабильный канал никогда не понижает версию
update-channel-stable = СТАБИЛЬНЫЙ
update-channel-pre = ПРЕ-РЕЛИЗ
update-auto = ПРОВЕРЯТЬ ОБНОВЛЕНИЯ
update-auto-hint = при запуске и раз в несколько часов; запрашивается только список релизов, ничего о вас не отправляется
update-title-available = ДОСТУПНО ОБНОВЛЕНИЕ
update-title-new = ЧТО НОВОГО В { $version }
update-from-to = с { $current } на { $latest }
update-later = ПОЗЖЕ
update-skip = ПРОПУСТИТЬ ЭТУ ВЕРСИЮ
update-restart = ПЕРЕЗАПУСТИТЬ
update-close = ЗАКРЫТЬ
update-busy = работает трейнер: обновление установится, когда вы закроете Impulse
update-no-notes = описания релиза нет
update-chip-available = ОБНОВЛЕНИЕ { $version }
update-chip-downloading = СКАЧИВАНИЕ...
update-chip-ready = ПЕРЕЗАПУСК ДЛЯ ОБНОВЛЕНИЯ
update-changelog-hint = что изменилось в каждой версии
update-open = ОТКРЫТЬ
