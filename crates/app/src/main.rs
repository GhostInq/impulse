#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod plugins;

use std::{env, path::PathBuf, sync::Arc};

use client::{
    Capabilities, Registry,
    storage::{self, Paths},
};
use gpui_kit::{AppContext as _, Bounds, TitlebarOptions, WindowBounds, WindowOptions, px, size};
use model::SourceId;
use shell::Root;
use wand::Wand;

use self::plugins::Host;

fn main() {
    // A copy started elevated for a game run as administrator: it loads the DLL and exits.
    if let Some(code) = agent::serve_elevated(Wand::PUBLISHERS) {
        std::process::exit(code);
    }
    // Steam's launch options on Linux put Impulse in the game's place (`--game <exe> <args>`): Proton and Steam wait
    // on this process, so it starts the game and lasts as long as the game.
    let game = wine::command();
    let wine = wine::version().map(|version| (version, prepare_for_wine()));
    // One Impulse at a time: a second start shows the first, which may sit in the tray. An update's restart
    // waits for the one it replaces instead.
    if !tray::claim(updater::restarted()) {
        // The Impulse already running serves the game; this copy only keeps Steam waiting for it.
        if let Some(game) = game {
            std::process::exit(game.start().map_or(1, |game| exit_code(game.wait())));
        }
        return;
    }
    let data_dir = data_dir();
    journal::init(&data_dir);
    if let Some((version, prepared)) = wine {
        log::info!("under Wine {version}");
        if let Err(e) = prepared {
            log::warn!("{e:#}");
        }
    }
    let game = match game.as_ref().map(wine::Command::start).transpose() {
        Ok(game) => game.map(Arc::new),
        Err(e) => {
            log::error!("the game Steam asked for did not start: {e:#}");
            journal::closing();
            std::process::exit(1);
        }
    };
    if let Some(game) = &game {
        log::info!("started the game for Proton, process {}{}", game.pid(), if game.followed() { "" } else { ", its first process only" });
    }
    if let Ok(exe) = env::current_exe() {
        updater::cleanup(&exe);
    }
    let paths = Paths::new(data_dir.clone());
    // The v1 files have unqualified ids from the original single source.
    storage::migrate(&paths, &SourceId::new(Wand::SOURCE).expect("a valid source id"));
    // Built before the window: its browser bridge takes a moment to come up.
    let wand = Arc::new(Wand::new(data_dir.clone()));
    let registry = Arc::new(Registry::default());
    // The title bar marks it.
    let elevated = agent::elevated();
    registry.add(Capabilities {
        accounts: Some(wand.clone()),
        games: Some(wand.clone()),
        trainers: Some(wand.clone()),
        maps: Some(wand.clone()),
        folders: Some(wand.clone()),
        caches: Some(wand),
        ..Capabilities::default()
    });
    // The crate, not this module of the same name.
    let store = ::plugins::Store::new(paths.plugins(), paths.plugin_lock(), paths.plugin_cache());
    let plugins = Arc::new(client::plugins::Plugins::new(Arc::new(Host(store)), registry.clone()));
    plugins.start();
    let waited = game.clone();
    gpui_kit::application().with_assets(shell::Assets).run(move |cx| {
        gpui_kit::init(cx);
        shell::init(&data_dir, registry.clone(), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(None, size(px(1280.), px(800.)), cx))),
            titlebar: Some(TitlebarOptions { title: Some("Impulse".into()), appears_transparent: true, ..Default::default() }),
            window_min_size: Some(size(px(1024.), px(640.))),
            ..Default::default()
        };
        // The remote panel's server on the LAN address.
        #[cfg(feature = "remote")]
        let serve: Option<shell::Serve> = Some(|side| {
            let server = remote::Server::bind()?;
            Ok((server.address(), Box::pin(server.serve(side))))
        });
        #[cfg(not(feature = "remote"))]
        let serve = None;
        gpui_kit::open_window(options, cx, |window, cx| cx.new(|cx| Root::new(registry, plugins, data_dir, serve, elevated, window, cx)))
            .expect("failed to open window");
        // gpui-pre-windows keeps running after the last window closes.
        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                journal::closing();
                cx.quit();
            }
        })
        .detach();
        // Steam counts the game as running while this process lives: Impulse quits with the game.
        if let Some(game) = waited {
            cx.spawn(async move |cx| {
                let code = blocking::unblock(move || game.wait()).await;
                log::info!("the game ended with code {code}: Impulse quits with it");
                journal::closing();
                cx.update(|cx| cx.quit());
            })
            .detach();
        }
        cx.activate(true);
    });
    // Impulse was closed while the game runs: Proton and Steam still wait on this process until the game ends.
    if let Some(game) = game {
        std::process::exit(exit_code(game.wait()));
    }
}

/// `%LOCALAPPDATA%\Impulse`, or `IMPULSE_DATA_DIR`: the Linux launcher gives every game's prefix the same folder, so
/// the library, hotkeys and settings follow the user from one game to the next. A Unix path is taken through Wine.
fn data_dir() -> PathBuf {
    if let Some(dir) = env::var("IMPULSE_DATA_DIR").ok().filter(|dir| !dir.is_empty()) {
        return wine::dos_path(&dir).unwrap_or_else(|| dir.into());
    }
    PathBuf::from(env::var_os("LOCALAPPDATA").expect("LOCALAPPDATA is set on Windows")).join("Impulse")
}

/// What differs under Wine, set before GPUI and the first window. The error says what could not be set.
fn prepare_for_wine() -> anyhow::Result<()> {
    // Wine has no DirectComposition: GPUI then presents through a plain swap chain.
    // SAFETY: changing a variable is always sound on Windows, and no other thread runs yet.
    unsafe { env::set_var("GPUI_DISABLE_DIRECT_COMPOSITION", "1") };
    let exe = env::current_exe()?;
    let name = exe.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_else(|| "Impulse.exe".into());
    wine::own_title_bar(&name)
}

/// Windows exit codes are unsigned; the process's is signed.
fn exit_code(code: u32) -> i32 {
    code as i32
}
