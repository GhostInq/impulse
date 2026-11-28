## Shared

off = 關
on = 開
# A number's marks: between thousands, before decimals.
number-group = ,
number-decimal = .

## Tray: Windows menu words

tray-open = 開啟 Impulse
tray-stop = 停止修改器
tray-overlay = 重疊介面
tray-quit = 結束

## Failures: a kind's words fit after "It could not be loaded:"

failure-offline = 無網路連線
failure-signed-out = 未登入
failure-not-found = 找不到
failure-unsupported = 尚未支援
failure-setup-needed = 缺少必要設定：請見「設定 · 應用程式」
failure-game-not-found = 找不到遊戲
failure-launch-failed = 啟動器無法啟動遊戲：詳情請見日誌
failure-not-started = 遊戲未在時限內啟動：請檢查啟動器後重試
failure-not-running = 遊戲未在時限內啟動：請自行啟動後重試
failure-external-launch = 此修改器僅適用於您自行啟動的遊戲：請先啟動遊戲，然後按下 啟動 + 附加
failure-game-exited = 遊戲在修改器載入前就已關閉
failure-refused = 遭到拒絕
failure-elevation = 遊戲以系統管理員身分執行：請同意權限提示才能附加
failure-protected = 遊戲的保護機制阻擋了修改器
failure-needs-64 = 修改器需要 64 位元版本的遊戲：請啟動該版本
failure-needs-32 = 修改器需要 32 位元版本的遊戲：請啟動該版本
failure-load-failed = 修改器無法載入遊戲：詳情請見日誌
failure-trainer-failed = 修改器無法在遊戲中啟動：詳情請見日誌
failure-restart = 遊戲已在執行中：請先關閉遊戲，再從這裡啟動
failure-internal = 發生錯誤：詳情請見日誌
failure-place = { $why }：請見「設定 · 應用程式」

## Page

catalog-failed-tag = [ 無目錄 ]
catalog-failed = 遊戲目錄無法使用
catalog-failed-body = 無法下載：{ $why }。Impulse 會在下次啟動時重試。
library-filter = 篩選
picker-filter = 遊戲名稱
page-search = 搜尋 { $trainers } 個修改器
page-search-none = 搜尋修改器
page-search-key = CTRL K
page-add-game = 新增遊戲
page-get-on = 前往 { $store } 取得
page-add-manually = 手動新增遊戲…
page-signed-out = 未登入
page-elevated = 正以系統管理員身分執行：附加以系統管理員身分執行的遊戲時不會再跳出確認提示
page-overlay = 重疊介面
page-star = 加入最愛
page-folder = 開啟遊戲資料夾
page-launcher = 在 { $launcher } 中開啟
page-source = 遊戲選項
page-overlay-default = 重疊介面：依照設定
page-overlay-in-game = 重疊介面：遊戲內嵌
page-overlay-over = 重疊介面：覆蓋於遊戲上方
page-custom = 自訂
page-live = 執行中
page-active = { $active }/{ $total } 已開啟
page-idle = 待機中
page-log = 日誌
page-binding = 請按下按鍵 · ESC 取消 · BKSP 恢復預設
page-binding-unbind = 請按下按鍵 · ESC 取消 · BKSP 恢復預設 · DEL 解除綁定
page-hotkeys-on = 快捷鍵已開
page-hotkeys-off = 快捷鍵已關
disclaimer-title = 開始使用前
disclaimer-body = Impulse 是專為您自己的 { $sources } 帳號與本機安裝環境打造的第三方用戶端。它會透過該程式已在這台電腦上保存的登入狀態進行登入，讀取其資料夾內的檔案，並代表您向該服務請求遊戲與修改器資料。它絕不會索取您的密碼，除了這些必要請求外，您的帳號資料都會留在本機電腦上。Impulse 與該服務沒有任何隸屬或授權關係。在線上遊戲中使用修改器可能導致帳號遭封鎖：請自行承擔風險。
disclaimer-accept = 我同意
disclaimer-exit = 結束
signed-out-title = 登入失敗
signed-out-body = Impulse 無法登入 { $sources }。請確認它已安裝、已在這台電腦上登入且網路連線正常，然後再試一次。登入成功前無法載入修改器。
signed-out-retry = 重試
signed-out-why = 原因：{ $why }
signed-out-trying = 登入中…
signed-out-close = 關閉

## Library

library-favorites = 最愛
library-games = 遊戲
library-no-match = 沒有符合的遊戲
library-add = + 新增
library-live = 執行中
library-scanning-tag = [ 掃描中 ]
library-scanning = 正在尋找已安裝的遊戲
library-scanning-body = 正在讀取遊戲目錄與您的各平台啟動器。
library-empty-tag = [ 無遊戲 ]
library-empty = 找不到已安裝的遊戲
library-empty-body = Impulse 會掃描 Steam、Epic、GOG、Ubisoft、Xbox、Rockstar 與 EA 中支援修改器的遊戲。
library-source-exe = EXE  { $file }
library-source-add = + 新增 .EXE
library-source-change = 更換 .EXE
library-source-remove = 移除 .EXE
library-source-trainer = 更換修改器…
library-source-own-trainer = 專屬修改器

## Picker

picker-title = 新增遊戲
picker-owned = 找到多個符合的遊戲：請問是哪一款？
picker-unowned = 目錄中沒有相符的遊戲：請依名稱搜尋
picker-in-library = { $store }  收藏庫中
picker-prompt = 選擇遊戲的 .exe 檔案
picker-trainer-title = 選擇修改器
picker-trainer = 優先顯示其他商店版本，再依名稱排序：其他版本的修改器可能不相容
picker-in-use = 使用中
picker-find-title = 尋找遊戲
picker-find = 依名稱搜尋所有修改器 · ↑ ↓ 選擇，Enter 開啟

## Cheats

cheats-failed-tag = [ 無資料 ]
cheats-failed = 修改項目清單尚未下載
cheats-failed-body = 無法下載：{ $why }。開啟遊戲時 Impulse 會再重試。
cheats-filter = 篩選修改項目
cheats-pinned = 已釘選
cheats-kept = 首次下載後會儲存在本機硬碟。
cheats-notes = 遊玩前請詳閱
cheats-no-trainer-tag = [ 無修改器 ]
cheats-no-trainer = 尚無 { $game } 的修改器
cheats-no-trainer-body = 這款遊戲目前還沒有修改器。一旦推出，Impulse 會在您的收藏庫中標示出來。
cheats-other-store = 此商店版本尚未推出修改器，但 { $store } 版本已有。不同商店的遊戲版本可能有所差異，不一定相容。
cheats-use-trainer = 使用 { $store } 版修改器
cheats-other-trainer = 其他修改器…
cheats-not-installed = 未安裝 · 新增遊戲後即可使用修改項目
cheats-loading-tag = [ 載入中 ]
cheats-loading = 正在載入修改項目清單
cheats-shown = { $shown } 個修改項目
cheats-no-match = 沒有符合篩選條件的修改項目
cheats-updated = 遊戲已更新
cheats-updated-body = 此遊戲版本比修改器支援的版本更新。在修改器更新前，部分修改項目可能無法生效。
cheats-set = 設定
cheats-set-done = 確定
cheats-execute = 執行
cheats-execute-done = 已套用
cheats-save = 儲存
cheats-saved = 已儲存 { $saved }
cheats-save-tip = 結束時仍保持開啟的修改項目，會在下次執行時自動恢復開啟。
cheats-reset = 重設
cheats-no-collections = 尚無組合
cheats-collect-tip = 將目前開啟的修改項目儲存為一個組合。
cheats-edit = 編輯
cheats-delete = 刪除
cheats-name-field = 名稱
cheats-cancel = 取消
cheats-name = 名稱
cheats-icon = 圖示
cheats-collection-mods = { $mods } 個修改項目
cheats-edit-collection = 編輯組合
cheats-new-collection = 新增組合

## Hotkeys

hotkeys-press-keys = 請按下按鍵
hotkeys-off = 關
hotkeys-in-game = 僅遊戲中
hotkeys-global = 全域
hotkeys-enabled = 快捷鍵
hotkeys-enabled-hint = 遊戲執行期間的修改器快捷鍵
hotkeys-works-in = 生效範圍
hotkeys-works-in-hint = 僅遊戲中：只在遊戲視窗位於前景時生效
hotkeys-bind-hint = ESC 取消 · BKSP 恢復預設 · DEL 解除綁定
hotkeys-clash = { $keys } 已被 { $name } 使用
hotkeys-clash-overlay = { $keys } 已保留給重疊介面使用
hotkeys-keep = 兩者保留
hotkeys-rebind = 重新綁定
hotkeys-shared-tip = 另一個修改項目也使用此快捷鍵：兩者會同時觸發

## Trainer

trainer-none = 無修改器
trainer-mods = { $count } 個修改項目
trainer-from = { $store } 版修改器
trainer-version = V{ $version }
trainer-version-unknown = 版本未知
trainer-not-installed = 未安裝
trainer-update-pending = 等待更新
trainer-sessions = { $count ->
    *[other] { $count } 次執行
}
trainer-played-min = 已遊玩 { $count } 分鐘
trainer-played-hours = 已遊玩 { $hours } 小時

## Run

run-no-cheats = 修改項目清單尚未載入
run-preparing = 正在準備修改器
run-launching = 正在啟動遊戲
run-waiting = 正在等待遊戲啟動
run-attaching = 正在載入修改器
run-stopping = 正在停止修改器
run-stop = 停止
run-launch = 啟動 + 附加
run-enter = 進入
run-session = 本次執行
run-last-session = 上次執行
run-active = 執行中
run-hotkeys = 快捷鍵
run-idle = 修改器待機中 · 啟動遊戲後即可操作
run-unconfirmed-title = 遊戲已更新
run-unconfirmed-body = 此遊戲版本比修改器支援的版本更新，在修改器更新前，部分修改項目可能無法運作。仍要繼續遊玩嗎？
run-unconfirmed-skip = 一律繼續
run-ask-cancel = 取消
run-unconfirmed-play = 仍要遊玩
run-unconfirmed-ask = 遊戲更新時詢問
run-unconfirmed-ask-hint = 當遊戲版本比修改器支援的版本更新時，在啟動前先詢問
run-outside-title = 請自行啟動遊戲
run-outside-body = 此修改器只會附加到在 Impulse 之外啟動的遊戲，而遊戲目前未在執行。請啟動遊戲，等待載入完成，然後按下 啟動 + 附加。
run-outside-start = 啟動遊戲

## Map

teleport-failed = 傳送失敗：{ $why }
map-failed-tag = [ 無地圖 ]
map-failed = 地圖無法使用
map-failed-body = 無法載入：{ $why }。
map-title = 地圖
map-none = 這款遊戲沒有互動式地圖
map-link = 連結地圖
map-unlink = 取消連結地圖
map-link-title = 連結地圖
map-link-caption = 下列哪一個項目對應這款遊戲？
map-link-empty = 地圖來源中找不到該名稱的遊戲。
map-refresh = 重新整理
map-loading-tag = [ 載入中 ]
map-loading = 正在載入地圖
map-loading-body = 首次載入後，標記與圖磚都會儲存在本機硬碟。
map-search = 搜尋地點、分類或關鍵字
map-unfinished = 地圖仍在建置中：標記可能缺少或有所變動。
map-found = 已找到
map-found-count = 已找到
map-mark-found = 標記為已找到
map-unfound = 未找到
map-show-all = 全部顯示
map-hide-all = 全部隱藏
map-teleport = 傳送
map-filter-teleport = 傳送
map-teleport-here = 傳送到這裡
map-teleport-height = 傳送高度
map-teleport-height-hint = 疊加在傳送到地圖標記與地點的高度上，單位為遊戲單位
map-pin-position = 在玩家所在位置放一個標記
map-confirm = 確認
map-delete = 刪除
map-add-pin = 新增我的標記
map-my-pin = 我的標記
map-my-pins = 我的標記
map-pin-name = 我的標記 { $number }
map-name-field = 為標記命名
map-note-field = 備註
map-no-pins = 在地圖上按右鍵即可新增標記。
map-regions = 區域
map-layer-regions = 區域
map-places = { $places } 個地點
map-no-matches = 找不到相符項目，請改試分類名稱或更短的關鍵字。
map-nearby = 附近
map-copy = 複製
map-copied = 已複製
map-hide-group = 在地圖上隱藏 { $group }
map-pin-hide = 隱藏此標記
map-pin-unhide = 顯示此標記
map-pin-move = 移動
map-add-media = 新增媒體
map-hidden-pins = 已隱藏的標記
map-pin-drag = Alt + 拖曳即可移動
map-video = 影片
map-photo = 圖片
map-layer-labels = 地點名稱
map-layer-clusters = 合併相鄰標記
map-layer-mini-clusters = 小地圖也適用
map-layer-rises = 附近高低差
map-axis-x = X
map-axis-y = Y
map-axis-z = Z

## Overlay

overlay-dock-mods = 修改項目
overlay-dock-map = 地圖
overlay-dock-session = 執行狀態
overlay-dock-minimap = 小地圖
overlay-minimap-tag = [ 小地圖 ]
overlay-no-live-map = 無即時地圖
overlay-no-live-map-hint = 當支援定位的修改器在有地圖的遊戲中執行時，小地圖會即時追蹤玩家位置。
overlay-mode = 模式
overlay-mode-hint = 遊戲內嵌：在遊戲允許的情況下，重疊介面會直接繪製於遊戲畫面中，並維持遊戲視窗焦點
overlay-mode-over = 覆蓋於遊戲上方
overlay-mode-in-game = 遊戲內嵌
overlay-hotkey = 快捷鍵
overlay-hotkey-hint = 修飾鍵搭配單一按鍵，且未與其他程式或修改項目衝突；只在遊戲位於前景時生效
overlay-minimap-hotkey = 小地圖快捷鍵
overlay-minimap-hotkey-hint = 顯示或隱藏小地圖；只在遊戲位於前景時生效
overlay-zoom-in-hotkey = 小地圖放大
overlay-zoom-in-hotkey-hint = 拉近小地圖視野；只在遊戲位於前景時生效
overlay-zoom-out-hotkey = 小地圖縮小
overlay-zoom-out-hotkey-hint = 拉遠小地圖視野；只在遊戲位於前景時生效
overlay-pin-here-hotkey = 在此放置標記
overlay-pin-here-hotkey-hint = 在玩家所在位置放置標記並附遊戲截圖；只在遊戲位於前景時生效
overlay-exclusive-title = 獨佔全螢幕
overlay-exclusive-body = 此遊戲正以獨佔全螢幕模式執行，重疊介面可能無法顯示，且遊戲視窗可能會被最小化。為獲得最佳體驗，請在遊戲的影像設定中改為「無邊框視窗化」。
overlay-exclusive-chip = 獨佔全螢幕：請將遊戲改為無邊框視窗化
overlay-exclusive-skip = 不再顯示
overlay-exclusive-ok = 確定
overlay-exclusive-ask-hint = 在獨佔全螢幕的遊戲上開啟重疊介面時發出提醒

## Settings

settings = 設定
settings-appearance = 外觀
settings-overlay = 重疊介面
settings-hotkeys = 快捷鍵
settings-app = 應用程式
settings-resources = 資源
settings-plugins = 擴充套件
settings-about = 關於
accent = 強調色
accent-hint = 僅用於標示執行中狀態
accent-lime = 萊姆綠
accent-amber = 琥珀黃
accent-cyan = 青色
accent-white = 白色
sidebar = 側邊欄
sidebar-hint = 遊戲列左側顯示的項目
sidebar-number = 編號
sidebar-thumbnail = 縮圖
panels = 面板配置
panels-hint = 將地圖與執行狀態置於修改項目上方或側邊
panels-top = 上方
panels-side = 側邊
reduce-motion = 減少動態效果
reduce-motion-hint = 停用執行中閃爍與轉場動畫
language = 語言
language-hint = Impulse 介面文字的顯示語言
language-system = 依系統設定
close-to-tray = 關閉至系統匣
close-to-tray-hint = 按下關閉按鈕時隱藏 Impulse；從系統匣圖示結束程式
hide-from-capture = 螢幕擷取時隱藏
hide-from-capture-hint = 截圖、錄影與直播時不會顯示 Impulse 視窗
hide-from-capture-inside = 遊戲內嵌重疊介面屬於遊戲畫面的一部分：僅隱藏 Impulse 視窗，不會隱藏繪製在遊戲內的畫面
folder-path = 資料夾路徑
folder-select = 選擇資料夾
settings-sounds = 音效
sounds-toggle-on = 開啟時
sounds-toggle-on-hint = 修改項目開啟時播放上揚提示音
sounds-toggle-off = 關閉時
sounds-toggle-off-hint = 修改項目關閉時播放下降提示音
sounds-slider = 滑桿
sounds-slider-hint = 每移動一格播放一次滴答聲；拖曳時會頻繁播放
sounds-number = 數值
sounds-number-hint = 數值變更時播放滴答聲
sounds-button = 按鈕
sounds-button-hint = 套用按鈕類修改項目時播放點擊聲
sounds-selection = 選項
sounds-selection-hint = 切換選項時播放提示音
sounds-listen = 試聽
sounds-choose = 選擇自訂 WAV 音效檔
sounds-reset = 恢復內建音效
sounds-refused = 非 WAV 檔案，或檔案大小超過 1 MB
sounds-prompt = 選擇 WAV 檔案

## Remote

remote = 遠端面板
remote-open = 遠端面板：用手機控制 Impulse
remote-switch = 手機連線
remote-hint = 在同一區域網路下用手機控制 Impulse
remote-address = 連線網址
remote-address-hint = 用手機相機掃描：配對碼僅限使用一次，並每五分鐘更新
remote-failed = 找不到可監聽的網路位址
remote-devices = 裝置
remote-devices-hint = 已配對的手機與瀏覽器；撤銷後會立即登出
remote-no-devices = 尚無配對裝置
remote-device = 裝置 { $number }
remote-unknown-device = 瀏覽器
remote-revoke = 撤銷
remote-trust = 明文 HTTP 傳輸：同網路上的任何人都可檢視，請僅在信任的網路中使用。
remote-tips = 如果手機無法連線
remote-tip-network = 手機必須與這台電腦位於相同網路：請勿使用行動數據、訪客 Wi-Fi 或 VPN。
remote-tip-firewall = Windows 防火牆可能會在未詢問的情況下阻擋 Impulse（尤其在公用網路上）：請將此網路改為私人網路，或在「Windows 安全性 > 防火牆與網路保護 > 允許應用程式通過防火牆」中允許 Impulse。
remote-tip-isolation = 部分路由器會隔離 Wi-Fi 裝置（AP 隔離或用戶端隔離）：請關閉該功能或改用其他網路。
remote-note-changed = 清單已變更：請再試一次。
remote-note-not-live = 請先啟動遊戲。
remote-note-busy = 已有其他遊戲執行中，或修改器尚未就緒。
remote-note-nothing-on = 請先開啟至少一個修改項目。
remote-note-name = 請先為組合命名。

## Plugins

plugins-install = 安裝
plugins-install-hint = 包含 manifest.json 的資料夾或 .zip 壓縮檔；安裝前需先核准其存取權限
plugins-folder = 資料夾
plugins-zip = ZIP
plugins-pick-folder = 選擇擴充套件資料夾
plugins-pick-zip = 選擇擴充套件 .zip 檔案
plugins-none = 未安裝任何擴充套件
plugins-approve-title = 要安裝 { $name } { $version } 嗎？
plugins-replaces = 將取代版本 { $version }
plugins-hosts = 資料存取來源
plugins-links = 可開啟外部連結
plugins-no-hosts = 無
plugins-new = 新增
plugins-sandbox = 此擴充套件在沙盒中執行，僅能存取上述主機。請只安裝您信任的擴充套件。
plugins-approve = 核准
plugins-cancel = 取消
plugins-version = 版本 { $version }，佔用 { $size }
plugins-remove = 移除
plugins-value = 數值

## Resources

media-pictures = 圖片
media-pictures-hint = 遊戲封面與標記圖片
media-tiles = 地圖圖磚
media-tiles-hint = 您開啟過的地圖底圖
media-videos = 影片
media-videos-hint = 標記影片片段
resources-reading = 正在讀取大小…
resources-in-use = 使用中空間
resources-in-use-hint = Impulse 在這台電腦上儲存的所有資料
resources-clear = 清除
resources-on-restart = 重新啟動後
resources-clears-on-restart = 重新啟動後清除
resources-clear-caches = 清除所有快取
resources-clear-caches-hint = 下方的個人資料將會保留
resources-clear-all = 全部清除
resources-limit = 媒體快取上限
resources-limit-hint = 超過上限時，會優先刪除最久未使用的媒體檔案
resources-limit-500m = 500 MB
resources-limit-1g = 1 GB
resources-limit-2g = 2 GB
resources-limit-5g = 5 GB
resources-limit-none = 無上限
resources-kept = 已保留
resources-progress = 地圖進度
resources-progress-hint = 已找到的標記與您的自訂標記
resources-hotkeys = 快捷鍵
resources-hotkeys-hint = 您綁定的按鍵設定
resources-logs = 日誌
resources-logs-hint = 最多保留兩個檔案，供「日誌」讀取
resources-bytes = { $size } B
resources-kilobytes = { $size } KB
resources-megabytes = { $size } MB
resources-gigabytes = { $size } GB

## Log

log-title = 日誌
log-error = 錯誤
log-warn = 警告
log-info = 資訊
log-debug = 除錯
log-filter = 篩選日誌
log-follow = 自動捲動
log-copy-all = 全部複製
log-copy = 複製 { $count } 筆
log-copied = 已複製
log-clear = 清除
log-folder = 資料夾
log-detail-empty = 點選任一列以檢視並選取完整紀錄

## About

about-version = 版本 { $version }
about-tagline = 免費且開源
about-intro = Impulse 由一位開發者利用業餘時間製作，完全免費且開源。給顆星、回報錯誤或贊助一杯咖啡，都是讓專案持續前進的動力。
about-github = GITHUB
about-github-hint = 檢視程式碼、追蹤版本更新，並給顆星讓更多人看見
about-star = 給顆星
about-discord = DISCORD
about-discord-hint = 提問、許願遊戲或功能，以及與其他使用者交流
about-join = 加入
about-bug = 發現 Bug？
about-bug-hint = 提交問題時請說明發生了什麼以及預期結果；狀態列的「日誌」會很有幫助
about-report = 回報
about-support = 支持 IMPULSE
about-support-hint = 免費且開源；一杯咖啡能支持每次更新背後投入的心力
about-coffee = 請喝杯咖啡
about-contact = 聯絡方式
about-contact-hint = 其他事項或私人聯繫：{ $address }
about-email = 電子郵件
alpha = 測試版
plugins-alpha-hint = 擴充套件目前仍為實驗性功能，格式可能會變動，更新後現有擴充套件也可能失效。
remote-copy = 複製網址

## Updates

update-title = 更新
update-state-idle = 版本 { $version }
update-state-current = 目前已是最新版本 { $version }
update-state-skipped = 已略過版本 { $version }：點選「立即檢查」可再次取得
update-state-checking = 正在檢查更新…
update-state-available = 有新版本 { $version } 可用
update-state-downloading = 正在下載 { $version }…
update-state-ready = { $version } 已準備就緒：重新啟動 Impulse 即可安裝
update-check-failed = 無法連線至更新來源
update-install-failed = 無法安裝更新
update-check = 立即檢查
update-changelog = 更新日誌
update-channel = 更新頻道
update-channel-hint = 預覽版能更早體驗新功能，但可能較不穩定；切回穩定版不會降級
update-channel-stable = 穩定版
update-channel-pre = 預覽版
update-auto = 自動檢查更新
update-auto-hint = 啟動時與每隔數小時檢查一次；僅會取得版本清單，不會傳送任何個人資訊
update-title-available = 有可用更新
update-title-new = { $version } 更新內容
update-from-to = 從 { $current } 更新至 { $latest }
update-later = 稍後
update-skip = 略過此版本
update-restart = 立即重新啟動
update-close = 關閉
update-busy = 修改器正在執行中：結束 Impulse 後將會自動安裝更新
update-no-notes = 無更新說明
update-chip-available = 更新 { $version }
update-chip-downloading = 下載中…
update-chip-ready = 重新啟動以更新
update-changelog-hint = 檢視各版本的變更內容
update-open = 開啟
