//! IPC commands invoked from the webview.

use std::sync::PoisonError;

use tauri::{AppHandle, Emitter, LogicalSize, Manager};

use crate::panel;
use crate::state::{PrayerState, SharedClock};
use crate::PANEL_LABEL;

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

/// M2 stubs — fleshed out in later milestones (M3 settings, M9 updater).
#[tauri::command]
pub fn open_settings(app: AppHandle) {
    let _ = app.emit("prayer://open-settings-requested", ());
}

#[tauri::command]
pub fn check_for_updates(app: AppHandle) {
    let _ = app.emit("prayer://check-updates-requested", ());
}
