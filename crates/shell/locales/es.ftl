## Shared

off = OFF
on = ON
# A number's marks: between thousands, before decimals.
number-group = .
number-decimal = ,

## Tray: Windows menu words

tray-open = Abrir Impulse
tray-stop = Parar el trainer
tray-overlay = Overlay
tray-quit = Salir

## Failures: a kind's words fit after "It could not be loaded:"

failure-offline = sin conexión
failure-signed-out = sin iniciar sesión
failure-not-found = no encontrado
failure-unsupported = todavía no es compatible
failure-setup-needed = falta un ajuste: revisa Ajustes > APP
failure-game-not-found = no se encontró el juego
failure-launch-failed = el launcher no pudo iniciar el juego: consulta el log para ver los detalles
failure-not-started = el juego no se inició a tiempo: revisa su launcher y vuelve a intentarlo
failure-not-running = el juego no fue iniciado a tiempo: inícialo tú y vuelve a intentarlo
failure-external-launch = este trainer solo funciona con un juego que inicias tú: inícialo y pulsa JUGAR + TRAINER
failure-game-exited = el juego se cerró antes de que entrara el trainer
failure-refused = solicitud rechazada
failure-elevation = el juego se ejecuta como administrador: acepta el aviso para conectar
failure-protected = la protección del juego no admite trainers
failure-needs-64 = el trainer necesita la versión de 64 bits del juego: inicia esa
failure-needs-32 = el trainer necesita la versión de 32 bits del juego: inicia esa
failure-load-failed = no se pudo cargar el trainer en el juego: consulta el log para ver los detalles
failure-trainer-failed = el trainer no arrancó en el juego: consulta el log para ver los detalles
failure-restart = el juego ya se está ejecutando: ciérralo y luego inícialo desde aquí
failure-internal = algo salió mal: consulta el log para ver los detalles
failure-place = { $why }: revisa Ajustes > APP

## Page

catalog-failed-tag = [ SIN CATÁLOGO ]
catalog-failed = EL CATÁLOGO DE JUEGOS NO ESTÁ DISPONIBLE
catalog-failed-body = No se pudo descargar: { $why }. Impulse volverá a intentarlo la próxima vez que se abra.
library-filter = filtrar
picker-filter = nombre del juego
page-search = { $count ->
    [one] buscar en { $trainers } trainer
   *[other] buscar entre { $trainers } trainers
}
page-search-none = buscar trainers
page-search-key = CTRL K
page-add-game = AÑADIR JUEGO
page-get-on = CONSEGUIR EN { $store }
page-add-manually = AÑADIR JUEGO MANUALMENTE...
page-signed-out = SIN SESIÓN
page-elevated = Ejecutándose como administrador: los juegos con permisos de administrador se conectan sin preguntar
page-overlay = OVERLAY
page-star = Favorito
page-folder = Abrir la carpeta del juego
page-launcher = Abrir en { $launcher }
page-source = Opciones del juego
page-overlay-default = OVERLAY: COMO EN AJUSTES
page-overlay-in-game = OVERLAY: EN JUEGO
page-overlay-over = OVERLAY: SOBRE EL JUEGO
page-custom = MANUAL
page-live = ACTIVO
page-active = { $active ->
    [one] { $active }/{ $total } ACTIVO
   *[other] { $active }/{ $total } ACTIVOS
}
page-idle = INACTIVO
page-log = LOG
page-binding = PULSA TECLAS · ESC CANCELAR · BKSP RESTAURAR
page-binding-unbind = PULSA TECLAS · ESC CANCELAR · BKSP RESTAURAR · DEL QUITAR
page-hotkeys-on = ATAJOS ON
page-hotkeys-off = ATAJOS OFF
disclaimer-title = ANTES DE EMPEZAR
disclaimer-body = Impulse es un cliente no oficial que funciona con tu cuenta de { $sources } y la app que ya tienes instalada en este PC. Aprovecha la sesión abierta, lee los archivos de la app y consulta al servicio qué juegos y trainers tienes. Impulse nunca te pide la contraseña y los datos de tu cuenta se quedan en este PC, salvo lo necesario para esas consultas. Impulse no tiene relación con el servicio ni cuenta con su respaldo. Usar trainers en juegos online puede acabar en un baneo: úsalos por tu cuenta y riesgo.
disclaimer-accept = ACEPTO
disclaimer-exit = SALIR
signed-out-title = ERROR DE INICIO DE SESIÓN
signed-out-body = Impulse no pudo iniciar sesión en { $sources }. Comprueba que la app esté instalada en este PC, que hayas iniciado sesión en ella y que tengas conexión. Después, vuelve a intentarlo. Sin sesión no se pueden cargar los trainers.
signed-out-retry = REINTENTAR
signed-out-why = Motivo: { $why }
signed-out-trying = Iniciando sesión…
signed-out-close = CERRAR

## Library

library-favorites = FAVORITOS
library-games = JUEGOS
library-no-match = SIN RESULTADOS
library-add = + AÑADIR
library-live = ACTIVO
library-scanning-tag = [ BUSCANDO ]
library-scanning = BUSCANDO JUEGOS INSTALADOS
library-scanning-body = Consultando el catálogo y tus launchers.
library-empty-tag = [ SIN JUEGOS ]
library-empty = NO SE ENCONTRARON JUEGOS INSTALADOS
library-empty-body = Impulse busca en Steam, Epic, GOG, Ubisoft, Xbox, Rockstar y EA los juegos para los que tiene trainers.
library-source-exe = EXE  { $file }
library-source-add = + AÑADIR .EXE
library-source-change = CAMBIAR .EXE
library-source-remove = QUITAR .EXE
library-source-trainer = CAMBIAR TRAINER...
library-source-own-trainer = SU PROPIO TRAINER

## Picker

picker-title = AÑADIR UN JUEGO
picker-owned = ¿CUÁL DE ESTOS JUEGOS QUIERES AÑADIR?
picker-unowned = SIN COINCIDENCIAS EN EL CATÁLOGO: BUSCA POR NOMBRE
picker-in-library = { $store }  EN BIBLIOTECA
picker-prompt = Selecciona el archivo .exe del juego
picker-trainer-title = ELEGIR UN TRAINER
picker-trainer = PRIMERO SUS OTRAS TIENDAS, LUEGO POR NOMBRE: UN TRAINER DE OTRA VERSIÓN PUEDE NO ENCAJAR
picker-in-use = EN USO
picker-find-title = BUSCAR UN JUEGO
picker-find = TODOS LOS TRAINERS POR NOMBRE · ↑ ↓ PARA ELEGIR, ENTER PARA ABRIR

## Cheats

cheats-failed-tag = [ SIN DATOS ]
cheats-failed = LISTA DE MODS NO DISPONIBLE
cheats-failed-body = No se pudo descargar: { $why }. Impulse volverá a intentarlo cuando abras el juego.
cheats-filter = filtrar mods
cheats-pinned = FIJADOS
cheats-kept = La lista se guarda en el disco con la primera descarga.
cheats-notes = LEER ANTES DE JUGAR
cheats-no-trainer-tag = [ SIN TRAINER ]
cheats-no-trainer = AÚN NO HAY TRAINER PARA { $game }
cheats-no-trainer-body = Aún no hay trainer para este juego. Impulse te lo indicará en la biblioteca en cuanto salga uno.
cheats-other-store = Aún no hay ninguno para esta tienda, pero la versión de { $store } sí tiene. Las versiones pueden variar entre tiendas, así que puede que no encaje con la tuya.
cheats-use-trainer = USAR EL TRAINER DE { $store }
cheats-other-trainer = OTRO TRAINER...
cheats-not-installed = NO INSTALADO · AÑADE EL JUEGO PARA USAR SUS MODS
cheats-loading-tag = [ CARGANDO ]
cheats-loading = CARGANDO MODS
cheats-shown = { $count ->
    [one] { $shown } MOD
   *[other] { $shown } MODS
}
cheats-no-match = NINGÚN MOD COINCIDE CON EL FILTRO
cheats-updated = JUEGO ACTUALIZADO
cheats-updated-body = El trainer aún no es compatible con esta versión del juego. Puede que algunos mods no funcionen hasta que se actualice el trainer.
cheats-set = FIJAR
cheats-set-done = OK
cheats-execute = ACTIVAR
cheats-execute-done = HECHO
cheats-save = GUARDAR
cheats-saved = { $count ->
    [one] GUARDADO { $saved }
   *[other] GUARDADOS { $saved }
}
cheats-save-tip = Los mods que dejes activos al cerrar la sesión se reactivan en la siguiente.
cheats-reset = RESET
cheats-no-collections = SIN COLECCIONES
cheats-collect-tip = Guarda los mods activos en una colección.
cheats-edit = EDITAR
cheats-delete = ELIMINAR
cheats-name-field = nombre
cheats-cancel = CANCELAR
cheats-name = NOMBRE
cheats-icon = ICONO
cheats-collection-mods = { $count ->
    [one] { $mods } MOD
   *[other] { $mods } MODS
}
cheats-edit-collection = EDITAR COLECCIÓN
cheats-new-collection = NUEVA COLECCIÓN

## Hotkeys

hotkeys-press-keys = PULSA TECLAS
hotkeys-off = OFF
hotkeys-in-game = EN JUEGO
hotkeys-global = GLOBAL
hotkeys-enabled = ATAJOS
hotkeys-enabled-hint = atajos del trainer durante la sesión
hotkeys-works-in = ALCANCE
hotkeys-works-in-hint = en juego: solo con el juego en primer plano
hotkeys-bind-hint = ESC CANCELAR · BKSP RESTAURAR · DEL QUITAR
hotkeys-clash = { $keys } ya está en uso en { $name }
hotkeys-clash-overlay = { $keys } está reservado para el overlay
hotkeys-keep = MANTENER AMBOS
hotkeys-rebind = REASIGNAR
hotkeys-shared-tip = otro mod tiene también este atajo: se activan los dos

## Trainer

trainer-none = SIN TRAINER
trainer-mods = { $count ->
    [one] { $count } MOD
   *[other] { $count } MODS
}
trainer-from = TRAINER DE { $store }
trainer-version = V{ $version }
trainer-version-unknown = VERSIÓN DESCONOCIDA
trainer-not-installed = NO INSTALADO
trainer-update-pending = ACTUALIZACIÓN PENDIENTE
trainer-sessions = { $count ->
    [one] { $count } SESIÓN
   *[other] { $count } SESIONES
}
trainer-played-min = { $count } MIN DE JUEGO
trainer-played-hours = { $hours } H DE JUEGO

## Run

run-no-cheats = aún no se ha cargado la lista de mods
run-preparing = PREPARANDO EL TRAINER
run-launching = INICIANDO EL JUEGO
run-waiting = A LA ESPERA DEL JUEGO
run-attaching = CARGANDO EL TRAINER
run-stopping = PARANDO EL TRAINER
run-stop = PARAR
run-launch = JUGAR + TRAINER
run-enter = ENTER
run-session = SESIÓN
run-last-session = ÚLTIMA SESIÓN
run-active = ACTIVOS
run-hotkeys = ATAJOS
run-idle = TRAINER INACTIVO · INICIA EL JUEGO PARA USAR LOS MODS
run-unconfirmed-title = JUEGO ACTUALIZADO
run-unconfirmed-body = El trainer aún no es compatible con esta versión del juego. Puede que algunos mods no funcionen hasta que se actualice el trainer. ¿Jugar de todos modos?
run-unconfirmed-skip = JUGAR SIEMPRE
run-ask-cancel = CANCELAR
run-unconfirmed-play = JUGAR IGUALMENTE
run-unconfirmed-ask = AVISAR SI EL JUEGO SE ACTUALIZA
run-unconfirmed-ask-hint = preguntar antes de jugar si la versión del juego es más nueva que la que admite el trainer
run-outside-title = INICIA EL JUEGO TÚ
run-outside-body = Este trainer solo se conecta a un juego iniciado fuera de Impulse, y el juego no está abierto. Inícialo, espera a que cargue y pulsa JUGAR + TRAINER.
run-outside-start = INICIAR JUEGO

## Map

teleport-failed = FALLÓ EL TELETRANSPORTE: { $why }
map-failed-tag = [ SIN MAPA ]
map-failed = EL MAPA NO ESTÁ DISPONIBLE
map-failed-body = No se pudo cargar: { $why }.
map-title = MAPA
map-none = NO HAY MAPA INTERACTIVO PARA ESTE JUEGO
map-link = VINCULAR MAPA
map-unlink = DESVINCULAR MAPA
map-link-title = VINCULAR UN MAPA
map-link-caption = ¿CUÁL ES EL MAPA DE ESTE JUEGO?
map-link-empty = No hay mapas para ese nombre.
map-refresh = ACTUALIZAR
map-loading-tag = [ CARGANDO ]
map-loading = CARGANDO EL MAPA
map-loading-body = La primera vez que abres el mapa, sus marcadores y su fondo se guardan en el disco.
map-search = buscar por lugar, grupo o palabra
map-unfinished = El mapa aún está en desarrollo: puede que falten marcadores o que cambien de sitio.
map-found = ENCONTRADO
map-found-count = ENCONTRADOS
map-mark-found = YA LO ENCONTRÉ
map-unfound = PENDIENTES
map-show-all = MOSTRAR TODO
map-hide-all = OCULTAR TODO
map-teleport = TELETRANSPORTAR
map-filter-teleport = TELETRANSPORTE
map-teleport-here = TELETRANSPORTAR AQUÍ
map-teleport-height = ALTURA DEL TELETRANSPORTE
map-teleport-height-hint = Se suma a la altura del teletransporte a los marcadores y puntos del mapa, en unidades del juego
map-pin-position = Un marcador donde está el jugador
map-confirm = CONFIRMAR
map-delete = ELIMINAR
map-add-pin = AÑADIR MARCADOR
map-my-pin = MI MARCADOR
map-my-pins = MIS MARCADORES
map-pin-name = Mi marcador { $number }
map-name-field = nombre del marcador
map-note-field = nota
map-no-pins = Haz clic derecho en el mapa para añadir un marcador.
map-regions = REGIONES
map-layer-regions = REGIONES
map-places = { $count ->
    [one] { $places } LUGAR
   *[other] { $places } LUGARES
}
map-no-matches = Sin resultados. Prueba con un grupo o una palabra más corta.
map-nearby = CERCANOS
map-copy = COPIAR
map-copied = COPIADO
map-hide-group = OCULTAR { $group } EN EL MAPA
map-pin-hide = OCULTAR ESTE MARCADOR
map-pin-unhide = MOSTRAR ESTE MARCADOR
map-pin-move = MOVER
map-add-media = AÑADIR MULTIMEDIA
map-hidden-pins = MARCADORES OCULTOS
map-pin-drag = ALT + ARRASTRAR LO MUEVE
map-video = VIDEO
map-photo = FOTO
map-layer-labels = NOMBRES DE LUGARES
map-layer-clusters = AGRUPAR POR CERCANÍA
map-layer-mini-clusters = TAMBIÉN EN EL MINIMAPA
map-layer-rises = ALTURAS CERCANAS
map-axis-x = X
map-axis-y = Y
map-axis-z = Z

## Overlay

overlay-dock-mods = MODS
overlay-dock-map = MAPA
overlay-dock-session = SESIÓN
overlay-dock-minimap = MINIMAPA
overlay-minimap-tag = [ MINIMAPA ]
overlay-no-live-map = SIN SEGUIMIENTO
overlay-no-live-map-hint = El minimapa sigue al jugador si el juego tiene mapa y el trainer activo puede detectar su posición.
overlay-mode = MODO
overlay-mode-hint = en juego, la superposición se dibuja dentro de la imagen del juego si el juego lo permite, y el juego conserva el foco
overlay-mode-over = SOBRE EL JUEGO
overlay-mode-in-game = EN JUEGO
overlay-hotkey = ATAJO
overlay-hotkey-hint = combina una tecla con uno o más modificadores; elige un atajo que no usen otras apps o mods; solo funciona con el juego en primer plano
overlay-minimap-hotkey = VER MINIMAPA
overlay-minimap-hotkey-hint = muestra u oculta el minimapa; solo funciona con el juego en primer plano
overlay-zoom-in-hotkey = ACERCAR MINIMAPA
overlay-zoom-in-hotkey-hint = acerca el minimapa; solo funciona con el juego en primer plano
overlay-zoom-out-hotkey = ALEJAR MINIMAPA
overlay-zoom-out-hotkey-hint = aleja el minimapa; solo funciona con el juego en primer plano
overlay-pin-here-hotkey = MARCADOR AQUÍ
overlay-pin-here-hotkey-hint = pone un marcador donde está el jugador, con una captura del juego; solo funciona con el juego en primer plano
overlay-exclusive-title = PANTALLA COMPLETA EXCLUSIVA
overlay-exclusive-body = En pantalla completa exclusiva, el overlay puede no aparecer y el juego puede minimizarse. El overlay funciona mejor en modo ventana sin bordes. Puedes activarlo en los ajustes de video del juego.
overlay-exclusive-chip = PANTALLA COMPLETA EXCLUSIVA: USA VENTANA SIN BORDES
overlay-exclusive-skip = NO VOLVER A MOSTRAR
overlay-exclusive-ok = OK
overlay-exclusive-ask-hint = preguntar cuando el overlay se abre sobre un juego en pantalla completa exclusiva.

## Settings

settings = AJUSTES
settings-appearance = APARIENCIA
settings-overlay = OVERLAY
settings-hotkeys = ATAJOS
settings-app = APP
settings-resources = RECURSOS
settings-plugins = PLUGINS
settings-about = INFO
accent = COLOR
accent-hint = se usa solo para indicar que el trainer está activo
accent-lime = LIMA
accent-amber = ÁMBAR
accent-cyan = CIAN
accent-white = BLANCO
sidebar = BARRA LATERAL
sidebar-hint = qué se muestra junto al nombre de cada juego
sidebar-number = NÚMERO
sidebar-thumbnail = MINIATURA
panels = PANELES
panels-hint = coloca el mapa y la sesión encima de los mods o al lado
panels-top = ARRIBA
panels-side = AL LADO
reduce-motion = REDUCIR MOVIMIENTO
reduce-motion-hint = sin parpadeo en la etiqueta ACTIVO ni animaciones de transición
language = IDIOMA
language-hint = el idioma de la interfaz de Impulse
language-system = SISTEMA
close-to-tray = CERRAR SIN SALIR
close-to-tray-hint = al cerrar la ventana, Impulse se queda en la bandeja del sistema; usa ese icono para salir
hide-from-capture = OCULTAR EN CAPTURAS
hide-from-capture-hint = las capturas de pantalla, grabaciones y streams no muestran la ventana de Impulse
hide-from-capture-inside = el overlay dentro del juego forma parte de la imagen del juego: solo se oculta la ventana de Impulse, no lo que dibuja dentro del juego
folder-path = ruta de la carpeta
folder-select = Selecciona la carpeta
settings-sounds = SONIDOS
sounds-toggle-on = ACTIVADO
sounds-toggle-on-hint = un tono ascendente al activar un mod
sounds-toggle-off = DESACTIVADO
sounds-toggle-off-hint = un tono descendente al desactivar un mod
sounds-slider = DESLIZADORES
sounds-slider-hint = un tic por cada cambio de valor al arrastrar el control
sounds-number = NÚMEROS
sounds-number-hint = un tic cuando cambia un valor
sounds-button = BOTONES
sounds-button-hint = un clic al activar un mod con botón
sounds-selection = OPCIONES
sounds-selection-hint = un tono al elegir otra opción
sounds-listen = Escuchar
sounds-choose = Elegir otro sonido (WAV)
sounds-reset = Restaurar el sonido original
sounds-refused = no es un archivo WAV o pesa más de 1 MB
sounds-prompt = Elegir un archivo WAV

## Remote

remote = PANEL REMOTO
remote-open = Panel remoto: controlar Impulse desde un teléfono
remote-switch = ACCESO MÓVIL
remote-hint = controla Impulse desde un teléfono conectado a esta red
remote-address = DIRECCIÓN
remote-address-hint = escanea el QR con la cámara del teléfono para vincularlo. El código es de un solo uso y cambia cada cinco minutos.
remote-failed = no hay una dirección de red disponible para el panel remoto
remote-devices = DISPOSITIVOS
remote-devices-hint = teléfonos y navegadores vinculados. Si quitas uno, se desconecta al instante.
remote-no-devices = aún no hay dispositivos
remote-device = dispositivo { $number }
remote-unknown-device = NAVEGADOR
remote-revoke = QUITAR
remote-trust = HTTP sin cifrar: cualquiera en esta red puede ver el tráfico. Usa el panel solo en redes de confianza.
remote-tips = SI EL TELÉFONO NO SE CONECTA
remote-tip-network = Conecta el teléfono a la misma red que este PC, sin datos móviles, Wi-Fi de invitados ni VPN.
remote-tip-firewall = El Firewall de Windows puede bloquear Impulse sin avisar, sobre todo en redes públicas. Configura la red como privada o permite el acceso a Impulse en Seguridad de Windows > Firewall y protección de red > Permitir una aplicación a través del firewall.
remote-tip-isolation = Algunos routers impiden que los dispositivos de la Wi-Fi se comuniquen (aislamiento de AP o de clientes). Desactiva esa opción o prueba con otra red.
remote-note-changed = La lista cambió: inténtalo de nuevo.
remote-note-not-live = Inicia el juego primero.
remote-note-busy = Hay otra sesión en marcha o el trainer aún no está listo.
remote-note-nothing-on = Activa algún mod primero.
remote-note-name = Ponle un nombre a la colección.

## Plugins

plugins-install = INSTALAR
plugins-install-hint = elige una carpeta o un .zip que contenga un manifest.json. Revisa los permisos antes de instalar.
plugins-folder = CARPETA
plugins-zip = ZIP
plugins-pick-folder = Selecciona la carpeta del plugin
plugins-pick-zip = Selecciona el .zip del plugin
plugins-none = ningún plugin instalado
plugins-approve-title = ¿INSTALAR { $name } { $version }?
plugins-replaces = reemplaza la versión { $version }
plugins-hosts = DESCARGA DE
plugins-links = ENLACES A
plugins-no-hosts = ninguno
plugins-new = NUEVO
plugins-sandbox = El sandbox limita el acceso a los sitios de la lista. Instala solo plugins de confianza.
plugins-approve = APROBAR
plugins-cancel = CANCELAR
plugins-version = versión { $version } · ocupa { $size }
plugins-remove = QUITAR
plugins-value = valor

## Resources

media-pictures = IMÁGENES
media-pictures-hint = imágenes de juegos y de marcadores
media-tiles = FONDOS DE MAPA
media-tiles-hint = imágenes que forman el fondo de los mapas que has abierto
media-videos = VIDEOS
media-videos-hint = videos de los marcadores
resources-reading = CALCULANDO ESPACIO...
resources-in-use = EN USO
resources-in-use-hint = todo lo que Impulse guarda en este PC
resources-clear = VACIAR
resources-on-restart = AL REINICIAR
resources-clears-on-restart = SE VACÍA AL REINICIAR
resources-clear-caches = VACIAR CACHÉS
resources-clear-caches-hint = no borra los datos de la lista de abajo
resources-clear-all = VACIAR TODO
resources-limit = LÍMITE MULTIMEDIA
resources-limit-hint = al superarlo, se borra lo que lleva más tiempo sin usarse
resources-limit-500m = 500 MB
resources-limit-1g = 1 GB
resources-limit-2g = 2 GB
resources-limit-5g = 5 GB
resources-limit-none = SIN LÍMITE
resources-kept = SE CONSERVA
resources-progress = PROGRESO DEL MAPA
resources-progress-hint = tus marcadores y los que has marcado como encontrados
resources-hotkeys = ATAJOS
resources-hotkeys-hint = las combinaciones de teclas que has asignado
resources-logs = LOGS
resources-logs-hint = hasta dos archivos, disponibles en el panel LOG
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
log-filter = filtrar el log
log-follow = SEGUIR
log-copy-all = COPIAR TODO
log-copy = COPIAR { $count }
log-copied = COPIADO
log-clear = VACIAR
log-folder = CARPETA
log-detail-empty = HAZ CLIC EN UNA FILA PARA LEER Y SELECCIONAR EL TEXTO COMPLETO

## About

about-version = VERSIÓN { $version }
about-tagline = GRATIS Y DE CÓDIGO ABIERTO
about-intro = Impulse lo desarrolla una sola persona en su tiempo libre. Es gratis y de código abierto. Una estrella, un reporte de error o un café lo mantienen vivo.
about-github = GITHUB
about-github-hint = lee el código, sigue las versiones y deja una estrella para que más gente lo encuentre.
about-star = ESTRELLA
about-discord = DISCORD
about-discord-hint = haz una pregunta, pide un juego o una función y charla con otros usuarios.
about-join = UNIRSE
about-bug = ¿ENCONTRASTE UN ERROR?
about-bug-hint = abre un issue con lo que pasó y lo que esperabas. El LOG de la barra de estado ayuda.
about-report = REPORTAR
about-support = APOYAR IMPULSE
about-support-hint = es gratis y de código abierto; un café paga el tiempo que lleva cada actualización.
about-coffee = INVITAR UN CAFÉ
about-contact = CONTACTO
about-contact-hint = para cualquier otra cosa o algo privado: { $address }
about-email = CORREO
alpha = ALPHA
plugins-alpha-hint = los plugins son experimentales. El formato puede cambiar y un plugin puede dejar de funcionar tras una actualización.
remote-copy = copiar la dirección

## Updates

update-title = ACTUALIZACIONES
update-state-idle = versión { $version }
update-state-current = tienes la última versión, { $version }
update-state-skipped = la versión { $version } está omitida: COMPROBAR la ofrece de nuevo
update-state-checking = buscando actualizaciones...
update-state-available = la versión { $version } está disponible
update-state-downloading = descargando la { $version }...
update-state-ready = la { $version } está lista: reinicia Impulse para instalarla
update-check-failed = no se pudo conectar con la lista de versiones
update-install-failed = no se pudo instalar la actualización
update-check = COMPROBAR
update-changelog = CAMBIOS
update-channel = CANAL DE ACTUALIZACIÓN
update-channel-hint = las versiones previas llegan antes y pueden ser inestables; volver a estable nunca instala una versión anterior
update-channel-stable = ESTABLE
update-channel-pre = VERSIÓN PREVIA
update-auto = BUSCAR ACTUALIZACIONES
update-auto-hint = al iniciar y cada pocas horas; solo se pide la lista de versiones, no se envía nada sobre ti
update-title-available = ACTUALIZACIÓN DISPONIBLE
update-title-new = NOVEDADES DE LA { $version }
update-from-to = de la { $current } a la { $latest }
update-later = MÁS TARDE
update-skip = OMITIR ESTA VERSIÓN
update-restart = REINICIAR AHORA
update-close = CERRAR
update-busy = hay un trainer en marcha: la actualización se instala al cerrar Impulse
update-no-notes = sin notas de la versión
update-chip-available = ACTUALIZAR { $version }
update-chip-downloading = DESCARGANDO...
update-chip-ready = REINICIAR PARA ACTUALIZAR
update-changelog-hint = qué cambió en cada versión
update-open = ABRIR
