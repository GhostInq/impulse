## Shared

off = 关
on = 开
# A number's marks: between thousands, before decimals.
number-group = ,
number-decimal = .

## Tray: Windows menu words

tray-open = 打开 Impulse
tray-stop = 停止修改器
tray-overlay = 悬浮窗
tray-quit = 退出

## Failures: a kind's words fit after "It could not be loaded:"

failure-offline = 没有网络连接
failure-signed-out = 未登录
failure-not-found = 未找到
failure-unsupported = 暂不支持
failure-setup-needed = 缺少一项设置：见 设置 · 应用
failure-game-not-found = 没有找到游戏
failure-launch-failed = 启动器没能启动游戏：详情见日志
failure-not-started = 游戏没有按时启动：请检查它的启动器，然后重试
failure-not-running = 没有及时启动游戏：请自行启动，然后重试
failure-external-launch = 此修改器只适用于你自行启动的游戏：请先启动游戏，然后点击 启动 + 附加
failure-game-exited = 修改器载入前游戏就关闭了
failure-refused = 被拒绝
failure-elevation = 游戏以管理员身份运行：接受弹窗后才能附加
failure-protected = 游戏的保护机制阻止了修改器
failure-needs-64 = 修改器需要游戏的 64 位版本：请启动那个版本
failure-needs-32 = 修改器需要游戏的 32 位版本：请启动那个版本
failure-load-failed = 修改器无法进入游戏：详情见日志
failure-trainer-failed = 修改器没能在游戏中启动：详情见日志
failure-restart = 游戏已在运行：请关闭它，再从这里启动
failure-internal = 出了点问题：详情见日志
failure-place = { $why }：见 设置 · 应用

## Page

catalog-failed-tag = [ 无目录 ]
catalog-failed = 游戏目录不可用
catalog-failed-body = 下载失败：{ $why }。Impulse 会在下次启动时重试。
library-filter = 筛选
picker-filter = 游戏名称
page-search = 搜索 { $trainers } 个修改器
page-search-none = 搜索修改器
page-search-key = CTRL K
page-add-game = 添加游戏
page-get-on = 到 { $store } 获取
page-add-manually = 手动添加游戏…
page-signed-out = 未登录
page-elevated = 正以管理员身份运行：以管理员身份运行的游戏无需弹窗即可附加
page-overlay = 悬浮窗
page-star = 收藏
page-folder = 打开游戏文件夹
page-launcher = 在 { $launcher } 中打开
page-source = 游戏选项
page-overlay-default = 悬浮窗：跟随设置
page-overlay-in-game = 悬浮窗：游戏内
page-overlay-over = 悬浮窗：覆盖在游戏上
page-custom = 自定义
page-live = 运行中
page-active = { $active }/{ $total } 已开启
page-idle = 空闲
page-log = 日志
page-binding = 按下组合键 · ESC 取消 · 退格 恢复默认
page-binding-unbind = 按下组合键 · ESC 取消 · 退格 恢复默认 · DEL 解除绑定
page-hotkeys-on = 快捷键已开
page-hotkeys-off = 快捷键已关
disclaimer-title = 开始之前
disclaimer-body = Impulse 是为你自己的 { $sources } 账号和本地安装准备的第三方客户端。它通过该应用已在这台电脑上保存的登录状态登录，读取其文件夹里的文件，并代你向它的服务请求游戏和修改器。它从不索取你的密码，除这些请求之外，你的账号数据都留在这台电脑上。Impulse 与该服务的运营方没有隶属或授权关系。在联机游戏中使用修改器可能导致封号：风险自负。
disclaimer-accept = 我同意
disclaimer-exit = 退出
signed-out-title = 登录失败
signed-out-body = Impulse 无法登录 { $sources }。请确认它已安装、已在这台电脑上登录，并且网络正常，然后重试。在登录成功之前，修改器无法加载。
signed-out-retry = 重试
signed-out-why = 原因：{ $why }
signed-out-trying = 正在登录…
signed-out-close = 关闭

## Library

library-favorites = 收藏
library-games = 游戏
library-no-match = 没有匹配的游戏
library-add = + 添加
library-live = 运行中
library-scanning-tag = [ 扫描中 ]
library-scanning = 正在查找已安装的游戏
library-scanning-body = 正在读取游戏目录和你的启动器。
library-empty-tag = [ 无游戏 ]
library-empty = 没有找到已安装的游戏
library-empty-body = Impulse 会在 Steam、Epic、GOG、Ubisoft、Xbox、Rockstar 和 EA 中查找有修改器的游戏。
library-source-exe = EXE  { $file }
library-source-add = + 添加 .EXE
library-source-change = 更换 .EXE
library-source-remove = 移除 .EXE
library-source-trainer = 更换修改器…
library-source-own-trainer = 它自带的修改器

## Picker

picker-title = 添加游戏
picker-owned = 有多个同名游戏：是哪一个？
picker-unowned = 目录里没有匹配的游戏：按名称查找
picker-in-library = { $store }  库中
picker-prompt = 选择游戏的 .exe
picker-trainer-title = 选择修改器
picker-trainer = 先列其他商店的，再按名称排：其他版本的修改器可能不适用
picker-in-use = 使用中
picker-find-title = 查找游戏
picker-find = 按名称查找所有修改器 · ↑ ↓ 选择，回车打开

## Cheats

cheats-failed-tag = [ 无数据 ]
cheats-failed = 修改项列表未能下载
cheats-failed-body = 下载失败：{ $why }。打开游戏时 Impulse 会重试。
cheats-filter = 筛选修改项
cheats-pinned = 已固定
cheats-kept = 首次下载后会保存在本地。
cheats-notes = 开始游戏前请阅读
cheats-no-trainer-tag = [ 无修改器 ]
cheats-no-trainer = 还没有 { $game } 的修改器
cheats-no-trainer-body = 这个游戏目前还没有修改器。一旦发布，Impulse 会在你的库中标记出来。
cheats-other-store = 这个商店还没有，但它的 { $store } 版本有。不同商店的构建可能不同，未必适用于你这一份。
cheats-use-trainer = 使用 { $store } 版修改器
cheats-other-trainer = 其他修改器…
cheats-not-installed = 未安装 · 添加该游戏后才能使用其修改项
cheats-loading-tag = [ 加载中 ]
cheats-loading = 正在加载修改项列表
cheats-shown = { $shown } 个修改项
cheats-no-match = 没有匹配筛选的修改项
cheats-updated = 游戏已更新
cheats-updated-body = 这个版本比修改器支持的更新。在修改器更新之前，修改项可能无法生效。
cheats-set = 设置
cheats-set-done = 确定
cheats-execute = 执行
cheats-execute-done = 已生效
cheats-save = 保存
cheats-saved = 已保存 { $saved }
cheats-save-tip = 一次运行结束后仍开启的修改项，下次运行时会自动恢复。
cheats-reset = 重置
cheats-no-collections = 没有组合
cheats-collect-tip = 把当前开启的修改项保存为一个组合。
cheats-edit = 编辑
cheats-delete = 删除
cheats-name-field = 名称
cheats-cancel = 取消
cheats-name = 名称
cheats-icon = 图标
cheats-collection-mods = { $mods } 个修改项
cheats-edit-collection = 编辑组合
cheats-new-collection = 新建组合

## Hotkeys

hotkeys-press-keys = 按下组合键
hotkeys-off = 关
hotkeys-in-game = 仅游戏中
hotkeys-global = 全局
hotkeys-enabled = 快捷键
hotkeys-enabled-hint = 运行期间的修改器快捷键
hotkeys-works-in = 生效范围
hotkeys-works-in-hint = 仅游戏中：只在游戏位于前台时生效
hotkeys-bind-hint = ESC 取消 · 退格 恢复默认 · DEL 解除绑定
hotkeys-clash = { $keys } 已被 { $name } 使用
hotkeys-clash-overlay = { $keys } 是悬浮窗的专用键
hotkeys-keep = 都保留
hotkeys-rebind = 重新绑定
hotkeys-shared-tip = 另一个修改项也用了这个快捷键：两者都会触发

## Trainer

trainer-none = 无修改器
trainer-mods = { $count } 个修改项
trainer-from = { $store } 版修改器
trainer-version = V{ $version }
trainer-version-unknown = 版本未知
trainer-not-installed = 未安装
trainer-update-pending = 等待更新
trainer-sessions = { $count ->
    *[other] { $count } 次运行
}
trainer-played-min = { $count } 分钟
trainer-played-hours = { $hours } 小时

## Run

run-no-cheats = 修改项列表尚未加载
run-preparing = 正在准备修改器
run-launching = 正在启动游戏
run-waiting = 正在等待游戏
run-attaching = 正在加载修改器
run-stopping = 正在停止修改器
run-stop = 停止
run-launch = 启动 + 附加
run-enter = 进入
run-session = 本次运行
run-last-session = 上次运行
run-active = 运行中
run-hotkeys = 快捷键
run-idle = 修改器空闲 · 启动后才能操作
run-unconfirmed-title = 游戏已更新
run-unconfirmed-body = 这个版本比修改器支持的版本更新，在修改器更新之前，部分修改项可能无效。仍要启动吗？
run-unconfirmed-skip = 总是启动
run-ask-cancel = 取消
run-unconfirmed-play = 仍然启动
run-unconfirmed-ask = 游戏更新时询问
run-unconfirmed-ask-hint = 游戏版本比修改器支持的版本更新时，启动前先询问
run-outside-title = 请自行启动游戏
run-outside-body = 此修改器只会附加到在 Impulse 之外启动的游戏，而游戏当前未运行。请启动游戏，等待加载完成，然后点击 启动 + 附加。
run-outside-start = 启动游戏

## Map

teleport-failed = 传送失败：{ $why }
map-failed-tag = [ 无地图 ]
map-failed = 地图不可用
map-failed-body = 加载失败：{ $why }。
map-title = 地图
map-none = 这个游戏没有交互地图
map-link = 关联地图
map-unlink = 取消关联
map-link-title = 关联一张地图
map-link-caption = 这些条目里哪个是这个游戏？
map-link-empty = 没有地图源有这个名字的条目。
map-refresh = 刷新
map-loading-tag = [ 加载中 ]
map-loading = 正在加载地图
map-loading-body = 首次之后，标记和瓦片都会保存在本地。
map-search = 查找地点、分组或关键字
map-unfinished = 仍在绘制中：标记可能缺失或移动。
map-found = 已找到
map-found-count = 已找到
map-mark-found = 标记为已找到
map-unfound = 未找到
map-show-all = 全部显示
map-hide-all = 全部隐藏
map-teleport = 传送
map-filter-teleport = 传送
map-teleport-here = 传送到此处
map-teleport-height = 传送高度
map-teleport-height-hint = 叠加在传送到地图标记和地点的高度上，单位为游戏单位
map-pin-position = 在玩家所在位置放一个标记
map-confirm = 确认
map-delete = 删除
map-add-pin = 添加我的标记
map-my-pin = 我的标记
map-my-pins = 我的标记
map-pin-name = 我的标记 { $number }
map-name-field = 给这个标记起名
map-note-field = 备注
map-no-pins = 在地图上右键即可添加。
map-regions = 区域
map-layer-regions = 区域
map-places = { $places } 个地点
map-no-matches = 没有匹配项。试试分组或更短的词。
map-nearby = 附近
map-copy = 复制
map-copied = 已复制
map-hide-group = 在地图上隐藏 { $group }
map-pin-hide = 隐藏此标记
map-pin-unhide = 显示此标记
map-pin-move = 移动
map-add-media = 添加媒体
map-hidden-pins = 已隐藏的标记
map-pin-drag = Alt + 拖动即可移动
map-video = 视频
map-photo = 图片
map-layer-labels = 地名
map-layer-clusters = 合并相邻标记
map-layer-mini-clusters = 小地图也适用
map-layer-rises = 附近高低差
map-axis-x = X
map-axis-y = Y
map-axis-z = Z

## Overlay

overlay-dock-mods = 修改项
overlay-dock-map = 地图
overlay-dock-session = 本次运行
overlay-dock-minimap = 小地图
overlay-minimap-tag = [ 小地图 ]
overlay-no-live-map = 没有实时地图
overlay-no-live-map-hint = 当能定位玩家的修改器在有地图的游戏中运行时，它会跟随玩家。
overlay-mode = 模式
overlay-mode-hint = 游戏内：游戏允许时，悬浮窗画在游戏画面里，游戏保持焦点
overlay-mode-over = 覆盖在游戏上
overlay-mode-in-game = 游戏内
overlay-hotkey = 快捷键
overlay-hotkey-hint = 修饰键加一个键，且未被其他程序和修改项占用；只在游戏位于前台时生效
overlay-minimap-hotkey = 小地图快捷键
overlay-minimap-hotkey-hint = 显示或隐藏小地图；只在游戏位于前台时生效
overlay-zoom-in-hotkey = 小地图放大
overlay-zoom-in-hotkey-hint = 拉近小地图；只在游戏位于前台时生效
overlay-zoom-out-hotkey = 小地图缩小
overlay-zoom-out-hotkey-hint = 拉远小地图；只在游戏位于前台时生效
overlay-pin-here-hotkey = 在此放置标记
overlay-pin-here-hotkey-hint = 在玩家所在位置放置标记并附游戏截图；只在游戏位于前台时生效
overlay-exclusive-title = 独占全屏
overlay-exclusive-body = 这个游戏运行在独占全屏下，悬浮窗可能无法显示，游戏也可能被最小化。为获得最佳效果，请在游戏的画面设置中改为无边框窗口。
overlay-exclusive-chip = 独占全屏：请把游戏改为无边框窗口
overlay-exclusive-skip = 不再显示
overlay-exclusive-ok = 好
overlay-exclusive-ask-hint = 悬浮窗覆盖在独占全屏的游戏上时询问

## Settings

settings = 设置
settings-appearance = 外观
settings-overlay = 悬浮窗
settings-hotkeys = 快捷键
settings-app = 应用
settings-resources = 资源
settings-plugins = 插件
settings-about = 关于
accent = 强调色
accent-hint = 仅用于标记运行状态
accent-lime = 青柠
accent-amber = 琥珀
accent-cyan = 青色
accent-white = 白色
sidebar = 侧栏
sidebar-hint = 游戏行的首列内容
sidebar-number = 序号
sidebar-thumbnail = 缩略图
panels = 面板
panels-hint = 地图和运行信息放在修改项上方，或放在旁边
panels-top = 上方
panels-side = 侧边
reduce-motion = 减少动效
reduce-motion-hint = 不闪动的运行状态，无过渡动画
language = 语言
language-hint = Impulse 自身文字的语言
language-system = 跟随系统
close-to-tray = 关闭到托盘
close-to-tray-hint = 关闭按钮只是隐藏 Impulse；要从托盘图标退出
hide-from-capture = 屏幕捕获时隐藏
hide-from-capture-hint = 截图、录屏和直播中都看不到 Impulse 窗口
hide-from-capture-inside = 游戏内悬浮窗是游戏画面的一部分：只隐藏 Impulse 的窗口，不隐藏它在游戏里画的内容
folder-path = 文件夹路径
folder-select = 选择文件夹
settings-sounds = 音效
sounds-toggle-on = 开启时
sounds-toggle-on-hint = 修改项开启时播放一个上扬的音
sounds-toggle-off = 关闭时
sounds-toggle-off-hint = 修改项关闭时播放一个下落的音
sounds-slider = 滑块
sounds-slider-hint = 每一格一个咔哒声；拖动时会频繁播放
sounds-number = 数值
sounds-number-hint = 数值变化时一个咔哒声
sounds-button = 按钮
sounds-button-hint = 按下按钮类修改项时一个点击声
sounds-selection = 选项
sounds-selection-hint = 选择另一个选项时一个音
sounds-listen = 试听
sounds-choose = 选择要替换的 WAV 文件
sounds-reset = 换回内置音效
sounds-refused = 不是 WAV 文件，或超过 1 MB
sounds-prompt = 选择 WAV 文件

## Remote

remote = 手机面板
remote-open = 手机面板：用手机控制 Impulse
remote-switch = 手机访问
remote-hint = 在同一网络上用手机控制 Impulse
remote-address = 地址
remote-address-hint = 用手机相机扫描：配对一次，二维码每五分钟更新
remote-failed = 没有可监听的网络地址
remote-devices = 设备
remote-devices-hint = 已配对的手机和浏览器；撤销后立即登出
remote-no-devices = 还没有设备
remote-device = 设备 { $number }
remote-unknown-device = 浏览器
remote-revoke = 撤销
remote-trust = 明文 HTTP：同一网络上的任何人都能看到。只在可信网络中使用。
remote-tips = 手机连不上时
remote-tip-network = 手机必须与这台电脑在同一网络：不能用移动数据、访客 Wi-Fi 或 VPN。
remote-tip-firewall = Windows 防火墙可能不询问就拦下 Impulse，在公用网络上尤其常见：把这个网络设为专用，或在 Windows 安全中心 > 防火墙和网络保护 > 允许应用通过防火墙 中放行 Impulse。
remote-tip-isolation = 有些路由器会把 Wi-Fi 设备互相隔离（AP 隔离或客户端隔离）：关掉它，或换一个网络。
remote-note-changed = 列表已变化：请重试。
remote-note-not-live = 请先启动游戏。
remote-note-busy = 已有一次运行在进行，或修改器还没准备好。
remote-note-nothing-on = 请先开启一些修改项。
remote-note-name = 请先给组合起名。

## Plugins

plugins-install = 安装
plugins-install-hint = 含 manifest.json 的文件夹或 .zip；先确认它被允许访问什么
plugins-folder = 文件夹
plugins-zip = ZIP
plugins-pick-folder = 选择插件文件夹
plugins-pick-zip = 选择插件的 .zip
plugins-none = 未安装任何插件
plugins-approve-title = 安装 { $name } { $version }？
plugins-replaces = 会替换版本 { $version }
plugins-hosts = 请求自
plugins-links = 打开链接到
plugins-no-hosts = 无
plugins-new = 新增
plugins-sandbox = 它在沙箱中运行，只能访问这些主机。请只安装你信任的插件。
plugins-approve = 批准
plugins-cancel = 取消
plugins-version = 版本 { $version }，占用 { $size }
plugins-remove = 移除
plugins-value = 值

## Resources

media-pictures = 图片
media-pictures-hint = 游戏封面和标记图片
media-tiles = 地图瓦片
media-tiles-hint = 你打开过的地图的地面
media-videos = 视频
media-videos-hint = 标记片段
resources-reading = 正在读取大小…
resources-in-use = 使用中
resources-in-use-hint = Impulse 在这台电脑上保存的全部内容
resources-clear = 清除
resources-on-restart = 重启后
resources-clears-on-restart = 重启后清除
resources-clear-caches = 清除所有缓存
resources-clear-caches-hint = 下面的个人数据会保留
resources-clear-all = 全部清除
resources-limit = 媒体上限
resources-limit-hint = 超过后，最久未使用的媒体会被清除
resources-limit-500m = 500 MB
resources-limit-1g = 1 GB
resources-limit-2g = 2 GB
resources-limit-5g = 5 GB
resources-limit-none = 不限
resources-kept = 保留
resources-progress = 地图进度
resources-progress-hint = 已找到的标记和你自己的标记
resources-hotkeys = 快捷键
resources-hotkeys-hint = 你绑定的按键
resources-logs = 日志
resources-logs-hint = 最多两个文件，由 日志 读取
resources-bytes = { $size } B
resources-kilobytes = { $size } KB
resources-megabytes = { $size } MB
resources-gigabytes = { $size } GB

## Log

log-title = 日志
log-error = 错误
log-warn = 警告
log-info = 信息
log-debug = 调试
log-filter = 筛选日志
log-follow = 跟随
log-copy-all = 全部复制
log-copy = 复制 { $count }
log-copied = 已复制
log-clear = 清空
log-folder = 文件夹
log-detail-empty = 点击一行可查看并选中完整记录

## About

about-version = 版本 { $version }
about-tagline = 免费且开源
about-intro = Impulse 由一个人利用业余时间开发。免费且开源。一个收藏、一个 bug 报告、一杯咖啡，就是它继续下去的动力。
about-github = GITHUB
about-github-hint = 阅读代码、跟进发布、点个收藏让更多人找到它
about-star = 收藏
about-discord = DISCORD
about-discord-hint = 提问、请求游戏或功能、和其他用户交流
about-join = 加入
about-bug = 遇到 bug？
about-bug-hint = 提交问题时说明发生了什么、你期望什么；状态栏的日志会有帮助
about-report = 报告
about-support = 支持 IMPULSE
about-support-hint = 免费且开源；一杯咖啡为每次更新背后的时间买单
about-coffee = 请我喝咖啡
about-contact = 联系
about-contact-hint = 其他任何事，或私密内容：{ $address }
about-email = 邮件
alpha = 测试版
plugins-alpha-hint = 插件功能仍在试验阶段。格式可能变化，插件可能在更新后失效。
remote-copy = 复制地址

## Updates

update-title = 更新
update-state-idle = 版本 { $version }
update-state-current = 已是最新版本 { $version }
update-state-skipped = 已跳过版本 { $version }：点 立即检查 可再次获取
update-state-checking = 正在检查更新…
update-state-available = 有新版本 { $version }
update-state-downloading = 正在下载 { $version }…
update-state-ready = { $version } 已就绪：重启 Impulse 即可安装
update-check-failed = 无法访问发布源
update-install-failed = 更新未能安装
update-check = 立即检查
update-changelog = 更新日志
update-channel = 更新通道
update-channel-hint = 预发布版来得更早，可能不稳定；切回稳定版不会降级
update-channel-stable = 稳定版
update-channel-pre = 预发布版
update-auto = 检查更新
update-auto-hint = 启动时以及每隔几小时检查一次；只请求发布列表，不发送任何关于你的信息
update-title-available = 有可用更新
update-title-new = { $version } 的新内容
update-from-to = 从 { $current } 到 { $latest }
update-later = 稍后
update-skip = 跳过此版本
update-restart = 立即重启
update-close = 关闭
update-busy = 修改器正在运行：退出 Impulse 时会安装更新
update-no-notes = 没有更新说明
update-chip-available = 更新 { $version }
update-chip-downloading = 正在下载…
update-chip-ready = 重启以更新
update-changelog-hint = 每个版本都有哪些变化
update-open = 打开
