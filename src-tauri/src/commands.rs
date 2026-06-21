//! IPC commands invoked from the webview.

use std::sync::PoisonError;

use prayer_core::{AppSettings, MethodRegistry};
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

/// Engage Focus Mode. `prayer` names the prayer (Snooze re-engages the same one);
/// `None` uses the next prayer (the settings "Try it" preview).
#[tauri::command]
pub fn engage_focus(app: AppHandle, clock: tauri::State<'_, SharedClock>, prayer: Option<String>) {
    let cue = {
        let c = clock.lock().unwrap_or_else(PoisonError::into_inner);
        let s = c.settings();
        crate::scheduler::FocusCue {
            prayer: prayer.unwrap_or_else(|| c.next_prayer_label()),
            duration_minutes: s.focus_duration_minutes,
            blur: s.focus_blur_intensity,
            emergency_exit: s.focus_emergency_exit_enabled,
        }
    };
    crate::focus::engage(&app, &cue);
}

/// Dismiss the Focus Mode overlay ("I've prayed", Esc, or the duration elapsed).
#[tauri::command]
pub fn dismiss_focus(app: AppHandle) {
    crate::focus::dismiss(&app);
}

/// Re-open the onboarding wizard ("Run setup again" in Settings).
#[tauri::command]
pub fn open_onboarding(app: AppHandle) {
    panel::hide_all(&app);
    if let Some(win) = app.get_webview_window(crate::ONBOARDING_LABEL) {
        let _ = win.show();
        let _ = win.set_focus();
    } else if let Err(err) = crate::build_onboarding_window(&app) {
        eprintln!("onboarding: failed to build ({err})");
    }
}

/// Mark onboarding complete, persist, and close the wizard for good.
#[tauri::command]
pub fn complete_onboarding(app: AppHandle, clock: tauri::State<'_, SharedClock>) {
    let settings = {
        let mut c = clock.lock().unwrap_or_else(PoisonError::into_inner);
        let mut s = c.settings();
        s.did_complete_onboarding = true;
        c.set_settings(s.clone());
        s
    };
    if let Err(err) = settings_io::save(&app, &settings) {
        eprintln!("settings: failed to persist ({err})");
    }
    if let Some(win) = app.get_webview_window(crate::ONBOARDING_LABEL) {
        let _ = win.destroy();
    }
}

/// Detect the location from IP, fill coordinates + timezone (+ method when
/// auto-detect is on), persist, recompute, and return the updated settings.
#[tauri::command]
pub async fn detect_location(
    app: AppHandle,
    clock: tauri::State<'_, SharedClock>,
) -> Result<AppSettings, String> {
    // Network call first — no lock held across the await.
    let detected = crate::location::detect().await?;

    let (settings, label, snapshot) = {
        let mut c = clock.lock().unwrap_or_else(PoisonError::into_inner);
        let mut s = c.settings();
        s.manual_coordinates = Some(detected.coords);
        if let Some(tz) = detected.tz {
            s.timezone_override = Some(tz);
        }
        if s.auto_detect_method {
            s.method_id = MethodRegistry::method_id_for_country(detected.country_code.as_deref());
        }
        c.set_settings(s.clone());
        (s, c.tray_label(), c.snapshot())
    };

    if let Err(err) = settings_io::save(&app, &settings) {
        eprintln!("settings: failed to persist ({err})");
    }
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        let _ = tray.set_title(Some(label));
    }
    let _ = app.emit(STATE_EVENT, snapshot);
    Ok(settings)
}
