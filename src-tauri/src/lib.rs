//! Tauri app entry: a menu-bar tray with a live countdown title, a dropdown
//! panel, and a 1 Hz Rust tick loop that owns the clock. Wiring lives in
//! focused modules: [`commands`] (IPC), [`panel`] (the popover + backdrop),
//! [`tray`] (status item + tick), and [`state`] (the prayer clock).

mod audio;
mod commands;
mod focus;
mod location;
mod panel;
mod scheduler;
mod settings_io;
mod state;
mod tray;

use prayer_core::AppSettings;
use state::{Clock, SharedClock};
use tauri::Manager;

pub(crate) const PANEL_LABEL: &str = "panel";
pub(crate) const BACKDROP_LABEL: &str = "backdrop";
pub(crate) const SETTINGS_LABEL: &str = "settings";
pub(crate) const ONBOARDING_LABEL: &str = "onboarding";
pub(crate) const TRAY_ID: &str = "tray";
pub(crate) const STATE_EVENT: &str = "prayer://state-changed";
pub(crate) const PANEL_W: f64 = 336.0;
pub(crate) const PANEL_H: f64 = 482.0;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init());

    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(tauri_nspanel::init());
    }

    builder
        .manage(SharedClock::new(Clock::new(AppSettings::default())))
        .manage(audio::Audio::spawn())
        .invoke_handler(tauri::generate_handler![
            commands::get_prayer_state,
            commands::hide_panel,
            commands::dismiss_panel,
            commands::quit_app,
            commands::report_content_size,
            commands::get_settings,
            commands::apply_settings,
            commands::open_settings,
            commands::check_for_updates,
            commands::stop_adhan,
            commands::detect_location,
            commands::engage_focus,
            commands::dismiss_focus,
            commands::complete_onboarding,
        ])
        .setup(|app| {
            // Menu-bar agent: no Dock icon on macOS.
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // Ask once for notification permission (no-op if already decided).
            use tauri_plugin_notification::{NotificationExt, PermissionState};
            match app.notification().permission_state() {
                Ok(PermissionState::Granted) => {}
                Ok(_) => {
                    let _ = app.notification().request_permission();
                }
                Err(err) => eprintln!("notification: permission check failed: {err}"),
            }

            // Load persisted settings into the live clock.
            let settings = settings_io::load(app.handle());
            let onboarded = settings.did_complete_onboarding;
            app.state::<SharedClock>()
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .set_settings(settings);

            panel::build(app.handle())?;
            panel::build_backdrop(app.handle())?;
            build_settings_window(app.handle())?;
            focus::build_all(app.handle())?;
            tray::build(app.handle())?;
            tray::spawn_tick_loop(app.handle().clone());

            // First launch: run the onboarding wizard.
            if !onboarded {
                build_onboarding_window(app.handle())?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// The standard (decorated) settings window. Created hidden; shown by the
/// `open_settings` command. Closing hides it rather than destroying it, so it can
/// be reopened.
fn build_settings_window(app: &tauri::AppHandle) -> tauri::Result<()> {
    use tauri::{WebviewUrl, WebviewWindowBuilder, WindowEvent};

    let win = WebviewWindowBuilder::new(app, SETTINGS_LABEL, WebviewUrl::App("index.html".into()))
        .title("Prayer Times")
        .inner_size(600.0, 660.0)
        .resizable(false)
        .maximizable(false)
        .visible(false)
        .build()?;

    win.clone().on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            let _ = win.hide();
        }
    });
    Ok(())
}

/// The first-launch onboarding wizard. Shown only when onboarding isn't complete;
/// closed for good by `complete_onboarding`.
fn build_onboarding_window(app: &tauri::AppHandle) -> tauri::Result<()> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    WebviewWindowBuilder::new(app, ONBOARDING_LABEL, WebviewUrl::App("index.html".into()))
        .title("Welcome — Prayer Times")
        .inner_size(720.0, 580.0)
        .resizable(false)
        .maximizable(false)
        .center()
        .build()?;
    Ok(())
}

