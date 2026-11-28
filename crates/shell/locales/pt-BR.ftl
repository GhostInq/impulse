## Shared

off = OFF
on = ON
# A number's marks: between thousands, before decimals.
number-group = .
number-decimal = ,

## Tray: Windows menu words

tray-open = Abrir Impulse
tray-stop = Parar o trainer
tray-overlay = Overlay
tray-quit = Sair

## Failures: a kind's words fit after "It could not be loaded:"

failure-offline = sem conexão
failure-signed-out = sem login
failure-not-found = não encontrado
failure-unsupported = ainda sem suporte
failure-setup-needed = falta configurar o app (Configurações > App)
failure-game-not-found = jogo não encontrado
failure-launch-failed = o launcher não conseguiu iniciar o jogo: o log tem os detalhes
failure-not-started = o jogo não iniciou a tempo: verifique o launcher dele e tente de novo
failure-not-running = o jogo não foi iniciado a tempo: inicie o jogo e tente de novo
failure-external-launch = este trainer só funciona com um jogo que você inicia: inicie o jogo e depois pressione JOGAR + TRAINER
failure-game-exited = o jogo fechou antes de o trainer entrar
failure-refused = recusado
failure-elevation = o jogo está rodando como administrador: aceite o aviso para conectar
failure-protected = a proteção do jogo bloqueia trainers
failure-needs-64 = o trainer precisa da versão de 64 bits do jogo: inicie essa
failure-needs-32 = o trainer precisa da versão de 32 bits do jogo: inicie essa
failure-load-failed = não foi possível carregar o trainer no jogo: o log tem os detalhes
failure-trainer-failed = o trainer não iniciou no jogo: o log tem os detalhes
failure-restart = o jogo já está em execução: feche-o e depois inicie-o por aqui
failure-internal = algo deu errado: o log tem os detalhes
failure-place = { $why } (Configurações > App)

## Page

catalog-failed-tag = [ SEM CATÁLOGO ]
catalog-failed = CATÁLOGO DE JOGOS INDISPONÍVEL
catalog-failed-body = Não deu para baixar o catálogo: { $why }. O Impulse tenta de novo na próxima vez que iniciar.
library-filter = filtrar
picker-filter = nome do jogo
page-search = { $count ->
    [0] buscar { $trainers } trainers
    [one] buscar { $trainers } trainer
   *[other] buscar { $trainers } trainers
}
page-search-none = buscar trainers
page-search-key = CTRL K
page-add-game = ADICIONAR JOGO
page-get-on = OBTER EM { $store }
page-add-manually = ADICIONAR JOGO MANUALMENTE...
page-signed-out = SEM LOGIN
page-elevated = Executando como administrador: jogos executados como administrador conectam sem pedir permissão
page-overlay = OVERLAY
page-star = Favoritar
page-folder = Abrir a pasta do jogo
page-launcher = Abrir em { $launcher }
page-source = Opções do jogo
page-overlay-default = OVERLAY: COMO NAS CONFIGURAÇÕES
page-overlay-in-game = OVERLAY: NO JOGO
page-overlay-over = OVERLAY: SOBRE O JOGO
page-custom = MANUAL
page-live = ATIVO
page-active = { $active ->
    [0] { $active }/{ $total } ATIVOS
    [one] { $active }/{ $total } ATIVO
   *[other] { $active }/{ $total } ATIVOS
}
page-idle = OFF
page-log = LOG
page-binding = TECLE · ESC CANCELA · BKSP PADRÃO
page-binding-unbind = TECLE · ESC CANCELA · BKSP PADRÃO · DEL REMOVE
page-hotkeys-on = ATALHOS ON
page-hotkeys-off = ATALHOS OFF
disclaimer-title = ANTES DE COMEÇAR
disclaimer-body = O Impulse é um cliente não oficial que usa a instalação local de { $sources } e a conta que já está conectada neste PC. Ele lê os arquivos dessa instalação e consulta o serviço em nome da sua conta para buscar jogos e trainers. O Impulse nunca pede a senha, e os dados da conta só saem do PC nessas consultas. O serviço não tem vínculo com o Impulse nem o recomenda. Usar trainers em jogos online pode render banimento. Use por sua conta e risco.
disclaimer-accept = CONCORDO
disclaimer-exit = SAIR
signed-out-title = FALHA NO LOGIN
signed-out-body = O Impulse não conseguiu fazer login em { $sources }. Confira se o app está instalado, se a conta está conectada neste PC e se há conexão com a internet. Depois, tente de novo. Sem login, os trainers não carregam.
signed-out-retry = TENTAR DE NOVO
signed-out-why = Motivo: { $why }
signed-out-trying = Entrando…
signed-out-close = FECHAR

## Library

library-favorites = FAVORITOS
library-games = JOGOS
library-no-match = SEM RESULTADOS
library-add = + JOGO
library-live = ATIVO
library-scanning-tag = [ BUSCANDO ]
library-scanning = BUSCANDO JOGOS INSTALADOS
library-scanning-body = Conferindo o catálogo e as instalações dos launchers.
library-empty-tag = [ SEM JOGOS ]
library-empty = NENHUM JOGO INSTALADO DETECTADO
library-empty-body = O Impulse procura jogos que tenham trainer nas instalações da Steam, Epic, GOG, Ubisoft, Xbox, Rockstar e EA.
library-source-exe = EXE  { $file }
library-source-add = + .EXE
library-source-change = TROCAR .EXE
library-source-remove = REMOVER .EXE
library-source-trainer = TROCAR TRAINER...
library-source-own-trainer = TRAINER PRÓPRIO

## Picker

picker-title = ADICIONAR JOGO
picker-owned = ENCONTRAMOS MAIS DE UM JOGO. QUAL DELES É O CERTO?
picker-unowned = SEM RESULTADOS NO CATÁLOGO. BUSQUE PELO NOME.
picker-in-library = { $store }  NA BIBLIOTECA
picker-prompt = Selecione o .exe do jogo
picker-trainer-title = ESCOLHER UM TRAINER
picker-trainer = PRIMEIRO AS OUTRAS LOJAS, DEPOIS POR NOME: UM TRAINER DE OUTRA VERSÃO PODE NÃO SERVIR
picker-in-use = EM USO
picker-find-title = BUSCAR UM JOGO
picker-find = TODOS OS TRAINERS POR NOME · ↑ ↓ PARA ESCOLHER, ENTER PARA ABRIR

## Cheats

cheats-failed-tag = [ SEM DADOS ]
cheats-failed = LISTA DE MODS NÃO BAIXADA
cheats-failed-body = Não foi possível baixar: { $why }. O Impulse tenta de novo ao abrir o jogo.
cheats-filter = filtrar mods
cheats-pinned = FIXADOS
cheats-kept = Depois do primeiro download, a lista fica salva neste PC.
cheats-notes = LEIA ANTES DE JOGAR
cheats-no-trainer-tag = [ SEM TRAINER ]
cheats-no-trainer = AINDA SEM TRAINER PARA { $game }
cheats-no-trainer-body = Este jogo ainda não tem trainer. Quando sair um, o Impulse avisa na biblioteca.
cheats-other-store = Ainda não há um para esta loja, mas a versão de { $store } tem. As versões podem variar entre lojas, então pode não servir na sua.
cheats-use-trainer = USAR O TRAINER DE { $store }
cheats-other-trainer = OUTRO TRAINER...
cheats-not-installed = NÃO INSTALADO · ADICIONE O JOGO PARA USAR OS MODS
cheats-loading-tag = [ CARREGANDO ]
cheats-loading = CARREGANDO A LISTA DE MODS
cheats-shown = { $count ->
    [0] { $shown } MODS
    [one] { $shown } MOD
   *[other] { $shown } MODS
}
cheats-no-match = NENHUM MOD ENCONTRADO
cheats-updated = JOGO ATUALIZADO
cheats-updated-body = O trainer ainda não é compatível com esta versão do jogo. Alguns mods podem não funcionar até sair uma atualização do trainer.
cheats-set = USAR
cheats-set-done = OK
cheats-execute = ATIVAR
cheats-execute-done = FEITO
cheats-save = SALVAR
cheats-saved = { $count ->
    [one] { $saved } SALVO
   *[other] { $saved } SALVOS
}
cheats-save-tip = Os mods ativos no fim da sessão são reativados na próxima.
cheats-reset = RESETAR
cheats-no-collections = SEM COLEÇÕES
cheats-collect-tip = Salva os mods ativos em uma coleção.
cheats-edit = EDITAR
cheats-delete = EXCLUIR
cheats-name-field = nome
cheats-cancel = CANCELAR
cheats-name = NOME
cheats-icon = ÍCONE
cheats-collection-mods = { $count ->
    [0] { $mods } MODS
    [one] { $mods } MOD
   *[other] { $mods } MODS
}
cheats-edit-collection = EDITAR COLEÇÃO
cheats-new-collection = NOVA COLEÇÃO

## Hotkeys

hotkeys-press-keys = TECLE
hotkeys-off = OFF
hotkeys-in-game = NO JOGO
hotkeys-global = GLOBAL
hotkeys-enabled = ATALHOS
hotkeys-enabled-hint = ativa os atalhos do trainer durante a sessão
hotkeys-works-in = ONDE USAR
hotkeys-works-in-hint = no jogo: só quando o jogo está em primeiro plano
hotkeys-bind-hint = ESC CANCELA · BKSP PADRÃO · DEL REMOVE
hotkeys-clash = { $keys } já é usado por { $name }
hotkeys-clash-overlay = { $keys } é reservado para o overlay
hotkeys-keep = MANTER AMBOS
hotkeys-rebind = REATRIBUIR
hotkeys-shared-tip = outro mod também tem este atalho: os dois são ativados

## Trainer

trainer-none = SEM TRAINER
trainer-mods = { $count ->
    [0] { $count } MODS
    [one] { $count } MOD
   *[other] { $count } MODS
}
trainer-from = TRAINER DE { $store }
trainer-version = V{ $version }
trainer-version-unknown = VERSÃO DESCONHECIDA
trainer-not-installed = NÃO INSTALADO
trainer-update-pending = AGUARDANDO UPDATE
trainer-sessions = { $count ->
    [one] { $count } SESSÃO
   *[other] { $count } SESSÕES
}
trainer-played-min = { $count } MIN DE JOGO
trainer-played-hours = { $hours } H DE JOGO

## Run

run-no-cheats = a lista de mods ainda não carregou
run-preparing = PREPARANDO O TRAINER
run-launching = INICIANDO O JOGO
run-waiting = AGUARDANDO O JOGO
run-attaching = CARREGANDO O TRAINER
run-stopping = PARANDO O TRAINER
run-stop = PARAR
run-launch = JOGAR + TRAINER
run-enter = ENTER
run-session = SESSÃO
run-last-session = ÚLTIMA SESSÃO
run-active = ATIVOS
run-hotkeys = ATALHOS
run-idle = TRAINER PARADO · INICIE O JOGO PARA USAR OS MODS
run-unconfirmed-title = JOGO ATUALIZADO
run-unconfirmed-body = O trainer ainda não é compatível com esta versão do jogo. Alguns mods podem não funcionar até sair uma atualização do trainer. Jogar mesmo assim?
run-unconfirmed-skip = SEMPRE JOGAR
run-ask-cancel = CANCELAR
run-unconfirmed-play = JOGAR MESMO ASSIM
run-unconfirmed-ask = AVISAR QUANDO O JOGO ATUALIZAR
run-unconfirmed-ask-hint = perguntar antes de jogar quando a versão do jogo for mais nova que a suportada pelo trainer
run-outside-title = INICIE O JOGO VOCÊ MESMO
run-outside-body = Este trainer só se conecta a um jogo iniciado fora do Impulse, e o jogo não está aberto. Inicie o jogo, espere carregar e depois pressione JOGAR + TRAINER.
run-outside-start = INICIAR JOGO

## Map

teleport-failed = FALHA NO TELEPORTE: { $why }
map-failed-tag = [ SEM MAPA ]
map-failed = MAPA INDISPONÍVEL
map-failed-body = Não foi possível carregar: { $why }.
map-title = MAPA
map-none = SEM MAPA INTERATIVO PARA ESTE JOGO
map-link = VINCULAR MAPA
map-unlink = DESVINCULAR MAPA
map-link-title = VINCULAR MAPA
map-link-caption = QUAL DESTES MAPAS É DO SEU JOGO?
map-link-empty = Nenhum mapa encontrado com esse nome.
map-refresh = ATUALIZAR
map-loading-tag = [ CARREGANDO ]
map-loading = CARREGANDO O MAPA
map-loading-body = Os marcadores e as imagens do mapa ficam salvos neste PC depois do primeiro carregamento.
map-search = buscar local, grupo ou palavra
map-unfinished = O mapa ainda está em construção. Alguns marcadores podem mudar de lugar ou ainda não aparecer.
map-found = ACHADO
map-found-count = ACHADOS
map-mark-found = JÁ ACHEI
map-unfound = PENDENTES
map-show-all = MOSTRAR TUDO
map-hide-all = OCULTAR TUDO
map-teleport = TELEPORTAR
map-filter-teleport = TELEPORTE
map-teleport-here = TELEPORTAR AQUI
map-teleport-height = ALTURA DO TELEPORTE
map-teleport-height-hint = Somada à altura do teleporte até os marcadores e pontos do mapa, em unidades do jogo
map-pin-position = Um marcador onde o jogador está
map-confirm = CONFIRMAR
map-delete = EXCLUIR
map-add-pin = + MARCADOR
map-my-pin = MEU MARCADOR
map-my-pins = MEUS MARCADORES
map-pin-name = Meu marcador { $number }
map-name-field = nome deste marcador
map-note-field = anotação
map-no-pins = Clique com o botão direito no mapa para adicionar um.
map-regions = REGIÕES
map-layer-regions = REGIÕES
map-places = { $count ->
    [0] { $places } LOCAIS
    [one] { $places } LOCAL
   *[other] { $places } LOCAIS
}
map-no-matches = Nada encontrado. Tente o nome de um grupo ou um termo mais curto.
map-nearby = PRÓXIMOS
map-copy = COPIAR
map-copied = COPIADO
map-hide-group = OCULTAR { $group } NO MAPA
map-pin-hide = OCULTAR ESTE MARCADOR
map-pin-unhide = MOSTRAR ESTE MARCADOR
map-pin-move = MOVER
map-add-media = ADICIONAR MÍDIA
map-hidden-pins = MARCADORES OCULTOS
map-pin-drag = ALT + ARRASTAR MOVE
map-video = VÍDEO
map-photo = FOTO
map-layer-labels = NOMES NO MAPA
map-layer-clusters = AGRUPAR MARCADORES
map-layer-mini-clusters = TAMBÉM NO MINIMAPA
map-layer-rises = ALTURAS PRÓXIMAS
map-axis-x = X
map-axis-y = Y
map-axis-z = Z

## Overlay

overlay-dock-mods = MODS
overlay-dock-map = MAPA
overlay-dock-session = SESSÃO
overlay-dock-minimap = MINIMAPA
overlay-minimap-tag = [ MINIMAPA ]
overlay-no-live-map = SEM RASTREIO
overlay-no-live-map-hint = O minimapa só acompanha a posição do jogador se houver um mapa para o jogo e um trainer ativo que leia essa posição.
overlay-mode = MODO
overlay-mode-hint = no jogo, a sobreposição é desenhada dentro da imagem do jogo quando o jogo permite, e o jogo mantém o foco
overlay-mode-over = SOBRE O JOGO
overlay-mode-in-game = NO JOGO
overlay-hotkey = ATALHO
overlay-hotkey-hint = combine uma tecla com Ctrl, Shift, Alt ou Win; use um atalho que não esteja em uso em outros apps ou mods; só funciona com o jogo em primeiro plano
overlay-minimap-hotkey = ATALHO DO MINIMAPA
overlay-minimap-hotkey-hint = mostra ou oculta o minimapa; só funciona com o jogo em primeiro plano
overlay-zoom-in-hotkey = APROXIMAR MINIMAPA
overlay-zoom-in-hotkey-hint = aproxima o minimapa; só funciona com o jogo em primeiro plano
overlay-zoom-out-hotkey = AFASTAR MINIMAPA
overlay-zoom-out-hotkey-hint = afasta o minimapa; só funciona com o jogo em primeiro plano
overlay-pin-here-hotkey = MARCADOR AQUI
overlay-pin-here-hotkey-hint = coloca um marcador onde o jogador está, com uma captura do jogo; só funciona com o jogo em primeiro plano
overlay-exclusive-title = TELA CHEIA EXCLUSIVA
overlay-exclusive-body = Em tela cheia exclusiva, o overlay pode não aparecer e o jogo pode acabar minimizado. Para evitar isso, escolha Janela sem bordas nas configurações de vídeo do jogo.
overlay-exclusive-chip = TELA CHEIA EXCLUSIVA: USE JANELA SEM BORDAS
overlay-exclusive-skip = NÃO MOSTRAR MAIS
overlay-exclusive-ok = OK
overlay-exclusive-ask-hint = perguntar quando o overlay abre sobre um jogo em tela cheia exclusiva

## Settings

settings = CONFIGURAÇÕES
settings-appearance = APARÊNCIA
settings-overlay = OVERLAY
settings-hotkeys = ATALHOS
settings-app = APP
settings-resources = RECURSOS
settings-plugins = PLUGINS
settings-about = SOBRE
accent = DESTAQUE
accent-hint = cor usada só nos indicadores de atividade
accent-lime = LIMÃO
accent-amber = ÂMBAR
accent-cyan = CIANO
accent-white = BRANCO
sidebar = BARRA LATERAL
sidebar-hint = o que aparece antes do nome do jogo
sidebar-number = NÚMERO
sidebar-thumbnail = MINIATURA
panels = PAINÉIS
panels-hint = posição do mapa e da sessão em relação aos mods
panels-top = ACIMA
panels-side = AO LADO
reduce-motion = REDUZIR MOVIMENTO
reduce-motion-hint = desativa as transições e deixa o indicador ATIVO sem piscar
language = IDIOMA
language-hint = idioma da interface do Impulse
language-system = SISTEMA
close-to-tray = OCULTAR AO FECHAR
close-to-tray-hint = fechar a janela só oculta o Impulse; para sair, use o ícone perto do relógio
hide-from-capture = OCULTAR NA CAPTURA
hide-from-capture-hint = capturas de tela, gravações e transmissões não mostram a janela do Impulse
hide-from-capture-inside = o overlay dentro do jogo faz parte da imagem do jogo: só a janela do Impulse fica oculta, não o que ele desenha dentro do jogo
folder-path = caminho da pasta
folder-select = Selecione a pasta
settings-sounds = SONS
sounds-toggle-on = LIGADO
sounds-toggle-on-hint = um som que fica mais agudo ao ativar um mod
sounds-toggle-off = DESLIGADO
sounds-toggle-off-hint = um som que fica mais grave ao desativar um mod
sounds-slider = SLIDERS
sounds-slider-hint = um clique a cada passo, que se repete ao arrastar
sounds-number = NÚMEROS
sounds-number-hint = um clique quando o valor muda
sounds-button = BOTÕES
sounds-button-hint = um clique ao acionar um mod pelo botão
sounds-selection = OPÇÕES
sounds-selection-hint = um som ao trocar de opção
sounds-listen = Ouvir
sounds-choose = Usar outro som (.WAV)
sounds-reset = Voltar ao som padrão
sounds-refused = o arquivo não é WAV ou passa de 1 MB
sounds-prompt = Selecione um arquivo WAV

## Remote

remote = PAINEL REMOTO
remote-open = Painel remoto: controlar o Impulse pelo celular
remote-switch = ACESSO PELO CELULAR
remote-hint = permite controlar o Impulse pelo celular na mesma rede
remote-address = ENDEREÇO
remote-address-hint = escaneie o QR Code com a câmera do celular; vale para um pareamento e muda a cada cinco minutos
remote-failed = nenhum endereço de rede disponível para o painel
remote-devices = DISPOSITIVOS
remote-devices-hint = celulares e navegadores pareados; remover o acesso desconecta o dispositivo na hora
remote-no-devices = nenhum dispositivo pareado
remote-device = dispositivo { $number }
remote-unknown-device = NAVEGADOR
remote-revoke = REMOVER
remote-trust = A conexão usa HTTP sem criptografia. Quem está na mesma rede pode ver o tráfego. Use só em redes confiáveis.
remote-tips = SE O CELULAR NÃO CONECTAR
remote-tip-network = Conecte o celular à mesma rede do PC, sem usar dados móveis, Wi-Fi de convidados ou VPN.
remote-tip-firewall = O Firewall do Windows pode bloquear o Impulse sem avisar, principalmente em redes públicas. Mude o perfil para uma rede privada ou libere o Impulse em Segurança do Windows > Firewall e proteção de rede > Permitir um aplicativo pelo firewall.
remote-tip-isolation = Alguns roteadores impedem que os dispositivos do Wi-Fi se comuniquem. Desative o isolamento de AP ou de cliente no roteador, ou tente outra rede.
remote-note-changed = A lista mudou. Tente de novo.
remote-note-not-live = Inicie o jogo primeiro.
remote-note-busy = Já tem uma sessão em andamento ou a lista de mods ainda não carregou.
remote-note-nothing-on = Ative alguns mods primeiro.
remote-note-name = Dê um nome à coleção primeiro.

## Plugins

plugins-install = INSTALAR
plugins-install-hint = selecione uma pasta ou um .zip com um manifest.json e confirme as permissões
plugins-folder = PASTA
plugins-zip = ZIP
plugins-pick-folder = Selecione a pasta do plugin
plugins-pick-zip = Selecione o .zip do plugin
plugins-none = nenhum plugin instalado
plugins-approve-title = INSTALAR { $name } { $version }?
plugins-replaces = substitui a versão { $version }
plugins-hosts = ACESSA
plugins-links = ABRE LINKS EM
plugins-no-hosts = nenhum
plugins-new = NOVO
plugins-sandbox = O plugin roda isolado e só pode acessar os endereços da lista. Instale apenas plugins confiáveis.
plugins-approve = APROVAR
plugins-cancel = CANCELAR
plugins-version = versão { $version } · { $size } em disco
plugins-remove = REMOVER
plugins-value = valor

## Resources

media-pictures = IMAGENS
media-pictures-hint = artes dos jogos e imagens dos marcadores
media-tiles = IMAGENS DO MAPA
media-tiles-hint = imagens de fundo dos mapas já abertos
media-videos = VÍDEOS
media-videos-hint = vídeos dos marcadores
resources-reading = CALCULANDO USO...
resources-in-use = EM USO
resources-in-use-hint = tudo o que o Impulse guarda neste PC
resources-clear = LIMPAR
resources-on-restart = AO REINICIAR
resources-clears-on-restart = LIMPEZA AO REINICIAR
resources-clear-caches = LIMPAR TODOS OS CACHES
resources-clear-caches-hint = mantém os dados listados abaixo
resources-clear-all = LIMPAR TUDO
resources-limit = LIMITE DE MÍDIA
resources-limit-hint = ao passar do limite, o Impulse apaga as mídias que estão há mais tempo sem uso
resources-limit-500m = 500 MB
resources-limit-1g = 1 GB
resources-limit-2g = 2 GB
resources-limit-5g = 5 GB
resources-limit-none = NENHUM
resources-kept = PRESERVADOS
resources-progress = PROGRESSO NOS MAPAS
resources-progress-hint = marcadores já encontrados e marcadores pessoais
resources-hotkeys = ATALHOS
resources-hotkeys-hint = combinações de teclas configuradas
resources-logs = LOGS
resources-logs-hint = até dois arquivos, disponíveis na tela de log
resources-bytes = { $size } B
resources-kilobytes = { $size } KB
resources-megabytes = { $size } MB
resources-gigabytes = { $size } GB

## Log

log-title = LOG
log-error = ERRO
log-warn = AVISO
log-info = INFO
log-debug = DEBUG
log-filter = filtrar log
log-follow = SEGUIR
log-copy-all = COPIAR TUDO
log-copy = COPIAR { $count }
log-copied = COPIADO
log-clear = LIMPAR
log-folder = PASTA
log-detail-empty = CLIQUE EM UMA LINHA PARA LER E SELECIONAR O LOG COMPLETO

## About

about-version = VERSÃO { $version }
about-tagline = GRATUITO E DE CÓDIGO ABERTO
about-intro = O Impulse é feito por uma pessoa só, no tempo livre. É gratuito e de código aberto. Uma estrela, um relato de bug ou um café ajudam a manter o projeto vivo.
about-github = GITHUB
about-github-hint = leia o código, acompanhe as versões e deixe uma estrela para mais gente encontrar
about-star = ESTRELA
about-discord = DISCORD
about-discord-hint = tire dúvidas, peça um jogo ou um recurso e converse com outros usuários
about-join = ENTRAR
about-bug = ACHOU UM BUG?
about-bug-hint = abra uma issue com o que aconteceu e o que você esperava; o LOG da barra de status ajuda
about-report = REPORTAR
about-support = APOIAR O IMPULSE
about-support-hint = gratuito e de código aberto; um café paga o tempo de cada atualização
about-coffee = PAGAR UM CAFÉ
about-contact = CONTATO
about-contact-hint = para qualquer outro assunto ou algo privado: { $address }
about-email = E-MAIL
alpha = ALPHA
plugins-alpha-hint = os plugins são experimentais; o formato pode mudar e um plugin pode parar de funcionar após uma atualização
remote-copy = copiar o endereço

## Updates

update-title = ATUALIZAÇÕES
update-state-idle = versão { $version }
update-state-current = você está na versão mais recente, { $version }
update-state-skipped = a versão { $version } foi ignorada: VERIFICAR a oferece de novo
update-state-checking = procurando atualizações...
update-state-available = a versão { $version } está disponível
update-state-downloading = baixando a { $version }...
update-state-ready = a { $version } está pronta: reinicie o Impulse para instalá-la
update-check-failed = não foi possível acessar a lista de versões
update-install-failed = não foi possível instalar a atualização
update-check = VERIFICAR
update-changelog = NOVIDADES
update-channel = CANAL DE ATUALIZAÇÃO
update-channel-hint = as pré-versões chegam antes e podem ser instáveis; voltar para o estável nunca instala uma versão anterior
update-channel-stable = ESTÁVEL
update-channel-pre = PRÉ-VERSÃO
update-auto = PROCURAR ATUALIZAÇÕES
update-auto-hint = ao iniciar e a cada poucas horas; só a lista de versões é pedida, nada sobre você é enviado
update-title-available = ATUALIZAÇÃO DISPONÍVEL
update-title-new = NOVIDADES DA { $version }
update-from-to = da { $current } para a { $latest }
update-later = DEPOIS
update-skip = PULAR ESTA VERSÃO
update-restart = REINICIAR AGORA
update-close = FECHAR
update-busy = um trainer está em execução: a atualização é instalada quando você fechar o Impulse
update-no-notes = sem notas da versão
update-chip-available = ATUALIZAR { $version }
update-chip-downloading = BAIXANDO...
update-chip-ready = REINICIAR PARA ATUALIZAR
update-changelog-hint = o que mudou em cada versão
update-open = ABRIR
