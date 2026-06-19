//! Focus Mode: a borderless, transparent, always-on-top overlay shown over every
//! display at prayer time. One window per monitor, pre-built hidden so each is
//! already listening when it engages. A discipline aid, not a lock — "I've prayed"
//! always dismisses, and Esc dismisses when emergency exit is enabled.

use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, WindowEvent};

use crate::scheduler::FocusCue;
use crate::PANEL_LABEL;

pub const FOCUS_LABEL: &str = "focus";
/// Carries the [`FocusCue`] to every overlay webview when Focus Mode engages.
pub const ENGAGE_EVENT: &str = "focus://engage";
/// Tells all overlays to clear their countdowns when one of them is dismissed.
pub const DISMISS_EVENT: &str = "focus://dismiss";

/// Monitors, queried through the panel window (windows are the monitor source).
fn monitors(app: &AppHandle) -> Vec<tauri::Monitor> {
    app.get_webview_window(PANEL_LABEL)
        .and_then(|w| w.available_monitors().ok())
        .unwrap_or_default()
}

/// Pre-build one hidden overlay per monitor. New monitors plugged in after launch
/// won't get an overlay until restart (acceptable; the common case is covered).
pub fn build_all(app: &AppHandle) -> tauri::Result<()> {
    let count = monitors(app).len().max(1);
    for i in 0..count {
        build_overlay(app, &format!("{FOCUS_LABEL}-{i}"))?;
    }
    Ok(())
}

fn build_overlay(app: &AppHandle, label: &str) -> tauri::Result<()> {
    let win = WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
        .title("Focus")
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .visible_on_all_workspaces(true)
        .skip_taskbar(true)
        .visible(false)
        .build()?;

    #[cfg(target_os = "macos")]
    cover_all(&win);

    let w = win.clone();
    win.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            let _ = w.hide();
        }
    });
    Ok(())
}

/// Size each overlay to its monitor, hand all of them the cue, and show them.
pub fn engage(app: &AppHandle, cue: &FocusCue) {
    let mons = monitors(app);
    for (i, mon) in mons.iter().enumerate() {
        if let Some(win) = app.get_webview_window(&format!("{FOCUS_LABEL}-{i}")) {
            let _ = win.set_position(*mon.position());
            let _ = win.set_size(*mon.size());
            let _ = win.show();
            let _ = win.set_focus();
        }
    }
    if mons.is_empty() {
        if let Some(win) = app.get_webview_window(&format!("{FOCUS_LABEL}-0")) {
            let _ = win.show();
            let _ = win.set_focus();
        }
    }
    let _ = app.emit(ENGAGE_EVENT, cue.clone());
}

/// Hide every overlay and tell them to clear their countdowns.
pub fn dismiss(app: &AppHandle) {
    for (label, win) in app.webview_windows() {
        if label.starts_with(FOCUS_LABEL) {
            let _ = win.hide();
        }
    }
    let _ = app.emit(DISMISS_EVENT, ());
}

/// Raise an overlay to screen-saver level and span every Space, so it covers the
/// screen (menu bar + Dock included) above all other windows.
#[cfg(target_os = "macos")]
fn cover_all(win: &WebviewWindow) {
    use tauri_nspanel::objc2_app_kit::{NSWindow, NSWindowCollectionBehavior};

    const SCREEN_SAVER_LEVEL: isize = 1000;

    let Ok(ptr) = win.ns_window() else {
        return;
    };
    unsafe {
        let ns: &NSWindow = &*(ptr as *const NSWindow);
        ns.setLevel(SCREEN_SAVER_LEVEL);
        ns.setCollectionBehavior(
            NSWindowCollectionBehavior::CanJoinAllSpaces
                | NSWindowCollectionBehavior::FullScreenAuxiliary
                | NSWindowCollectionBehavior::Stationary,
        );
    }
}
