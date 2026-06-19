//! IPC commands invoked from the webview.

use std::sync::PoisonError;

use prayer_core::AppSettings;
use tauri::{AppHandle, Emitter, LogicalSize, Manager};

use crate::state::{PrayerState, SharedClock};
use crate::{panel, settings_io, PANEL_LABEL, STATE_EVENT, TRAY_ID};

/// Current rendered state — called by the panel on mount.
#[tauri::command]
pub fn get_prayer_state(clock: tauri::State<'_, SharedClock>) -> PrayerState {
    clock.lock().unwrap_or_else(PoisonError::into_inner).snapshot()
}

/// Hide the panel (e.g. after a footer action) — also clears the backdrop.
#[tauri::command]
pub fn hide_panel(app: AppHandle) {
    panel::hide_all(&app);
}

/// Clicked the transparent backdrop (outside the panel) — dismiss everything.
#[tauri::command]
pub fn dismiss_panel(app: AppHandle) {
    panel::hide_all(&app);
}

#[tauri::command]
pub fn quit_app(app: AppHandle) {
    app.exit(0);
}

/// The webview reports its measured content size (logical px). Size the panel
/// window to it while hidden, so re-opens never resize a visible window.
#[tauri::command]
pub fn report_content_size(app: AppHandle, width: f64, height: f64) {
    if let Some(win) = app.get_webview_window(PANEL_LABEL) {
        if !win.is_visible().unwrap_or(false) {
            let _ = win.set_size(LogicalSize::new(width, height));
        }
    }
}

/// The current persisted settings — called by the settings UI on mount.
#[tauri::command]
pub fn get_settings(clock: tauri::State<'_, SharedClock>) -> AppSettings {
    clock.lock().unwrap_or_else(PoisonError::into_inner).settings()
}

/// Persist new settings, apply them to the live clock, and re-render: refresh the
/// tray title and push the recomputed state to any open window.
#[tauri::command]
pub fn apply_settings(app: AppHandle, clock: tauri::State<'_, SharedClock>, settings: AppSettings) {
    if let Err(err) = settings_io::save(&app, &settings) {
        eprintln!("settings: failed to persist ({err})");
    }
    let (label, snapshot) = {
        let mut c = clock.lock().unwrap_or_else(PoisonError::into_inner);
        c.set_settings(settings);
        (c.tray_label(), c.snapshot())
    };
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        let _ = tray.set_title(Some(label));
    }
    let _ = app.emit(STATE_EVENT, snapshot);
}

/// Open (or focus) the settings window, dismissing the panel first.
#[tauri::command]
pub fn open_settings(app: AppHandle) {
    panel::hide_all(&app);
    if let Some(win) = app.get_webview_window(crate::SETTINGS_LABEL) {
        let _ = win.show();
        let _ = win.unminimize();
        let _ = win.set_focus();
    }
}

#[tauri::command]
pub fn check_for_updates(app: AppHandle) {
    let _ = app.emit("prayer://check-updates-requested", ());
}

/// Stop any Adhan currently playing in-process.
#[tauri::command]
pub fn stop_adhan(app: AppHandle, audio: tauri::State<'_, crate::audio::Audio>) {
    audio.stop();
    let _ = app.emit(crate::scheduler::ADHAN_EVENT, false);
}
