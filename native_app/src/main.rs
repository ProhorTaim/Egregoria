#![allow(clippy::type_complexity)]

#[macro_use]
extern crate common;

extern crate simulation;

#[allow(unused_imports)]
#[macro_use]
extern crate inline_tweak;

#[macro_use]
mod uiworld;

mod audio;
mod debug_gui;
mod game_loop;
mod gui;
mod i18n;
mod init;
mod inputmap;
mod network;
mod rendering;

fn stabilize_workdir() {
    if std::path::Path::new("assets").is_dir() {
        return;
    }

    let Ok(exe) = std::env::current_exe() else {
        return;
    };

    for dir in exe.ancestors().skip(1).take(8) {
        if dir.join("assets").is_dir() {
            if let Err(e) = std::env::set_current_dir(dir) {
                log::warn!("failed to set current dir to {}: {}", dir.display(), e);
            } else {
                log::info!("working directory set to {}", dir.display());
            }
            return;
        }
    }
}

fn apply_startup_window_flags_from_settings() {
    let Ok(text) = std::fs::read_to_string("world/settings.json") else {
        return;
    };

    let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else {
        return;
    };

    if json
        .pointer("/gfx/fullscreen")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        std::env::set_var("EGREGORIA_START_FULLSCREEN", "1");
    }

    if json
        .get("low_dpi_mode")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        std::env::set_var("EGREGORIA_DISALLOW_HIDPI", "1");
    }
}

fn main() {
    #[cfg(feature = "profile")]
    profiling::tracy_client::Client::start();
    profiling::register_thread!("Main Thread");

    stabilize_workdir();
    apply_startup_window_flags_from_settings();
    engine::framework::init();
    init::init();

    engine::framework::start::<game_loop::State>();
}
