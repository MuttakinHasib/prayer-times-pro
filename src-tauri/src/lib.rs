//! Tauri app entry: a menu-bar tray with a live countdown title, a dropdown
//! panel, and a 1 Hz Rust tick loop that owns the clock. Wiring lives in
//! focused modules: [`commands`] (IPC), [`panel`] (the popover + backdrop),
//! [`tray`] (status item + tick), and [`state`] (the prayer clock).

mod commands;
mod panel;
mod state;
mod tray;

use state::{AppConfig, Clock, SharedClock};

pub(crate) const PANEL_LABEL: &str = "panel";
pub(crate) const BACKDROP_LABEL: &str = "backdrop";
pub(crate) const TRAY_ID: &str = "tray";
pub(crate) const STATE_EVENT: &str = "prayer://state-changed";
pub(crate) const PANEL_W: f64 = 312.0;
pub(crate) const PANEL_H: f64 = 482.0;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_opener::init());

    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(tauri_nspanel::init());
    }

    builder
        .manage(SharedClock::new(Clock::new(AppConfig::default())))
        .invoke_handler(tauri::generate_handler![
            commands::get_prayer_state,
            commands::hide_panel,
            commands::dismiss_panel,
            commands::quit_app,
            commands::report_content_size,
            commands::open_settings,
            commands::check_for_updates,
        ])
        .setup(|app| {
            // Menu-bar agent: no Dock icon on macOS.
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            panel::build(app.handle())?;
            panel::build_backdrop(app.handle())?;
            tray::build(app.handle())?;
            tray::spawn_tick_loop(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
