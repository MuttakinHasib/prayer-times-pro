//! Tauri app entry. M2: menu-bar tray with a live countdown title, a borderless
//! dropdown panel webview, and a 1 Hz Rust tick loop that owns the clock and
//! emits `prayer://state-changed` when the rendered state changes.

mod state;

use chrono::Utc;
use state::{AppConfig, Clock, PrayerState, SharedClock};
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent,
};

const PANEL_LABEL: &str = "panel";
const TRAY_ID: &str = "tray";
const STATE_EVENT: &str = "prayer://state-changed";

/// Current rendered state — called by the panel on mount and after navigation.
#[tauri::command]
fn get_prayer_state(clock: tauri::State<'_, SharedClock>) -> PrayerState {
    clock.lock().expect("clock lock").snapshot()
}

/// Hide the dropdown panel (e.g. after a footer action).
#[tauri::command]
fn hide_panel(app: tauri::AppHandle) {
    if let Some(w) = app.get_webview_window(PANEL_LABEL) {
        let _ = w.hide();
    }
}

/// Quit the whole app (tray agent has no window-close path).
#[tauri::command]
fn quit_app(app: tauri::AppHandle) {
    app.exit(0);
}

/// M2 stubs — fleshed out in later milestones (M3 settings, M9 updater).
#[tauri::command]
fn open_settings(app: tauri::AppHandle) {
    let _ = app.emit("prayer://open-settings-requested", ());
}

#[tauri::command]
fn check_for_updates(app: tauri::AppHandle) {
    let _ = app.emit("prayer://check-updates-requested", ());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(SharedClock::new(Clock::new(AppConfig::default())))
        .invoke_handler(tauri::generate_handler![
            get_prayer_state,
            hide_panel,
            quit_app,
            open_settings,
            check_for_updates
        ])
        .setup(|app| {
            // Menu-bar agent: no Dock icon on macOS.
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            build_panel_window(app.handle())?;
            build_tray(app.handle())?;
            spawn_tick_loop(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// The borderless, transparent, always-on-top dropdown panel. Created hidden and
/// shown under the tray icon on click; hidden again on blur.
fn build_panel_window(app: &tauri::AppHandle) -> tauri::Result<()> {
    let win = WebviewWindowBuilder::new(app, PANEL_LABEL, WebviewUrl::App("index.html".into()))
        .title("Prayer Times")
        .inner_size(360.0, 560.0)
        .resizable(false)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .visible(false)
        .build()?;

    // Hide when the panel loses focus, matching a native menu-bar popover.
    win.clone().on_window_event(move |event| {
        if let WindowEvent::Focused(false) = event {
            let _ = win.hide();
        }
    });
    Ok(())
}

/// Build the tray icon. The icon carries the glyph; `set_title` shows the live
/// countdown text (macOS). Left-click toggles the panel.
fn build_tray(app: &tauri::AppHandle) -> tauri::Result<()> {
    // Mosque glyph (from the macOS app's asset), embedded at compile time and
    // rendered as a template so macOS tints it for the light/dark menu bar.
    let mosque = tauri::image::Image::from_bytes(include_bytes!("../icons/tray-mosque@2x.png"))?;

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(mosque)
        .icon_as_template(true)
        .title("Prayer Times")
        .tooltip("Prayer Times")
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                rect,
                ..
            } = event
            {
                // Extract the icon's physical screen rect (x, y, width, height) so
                // the helper need not name the private `Rect` type.
                let anchor = match (rect.position, rect.size) {
                    (tauri::Position::Physical(p), tauri::Size::Physical(s)) => {
                        Some((p.x, p.y, s.width as i32, s.height as i32))
                    }
                    _ => None,
                };
                toggle_panel(tray.app_handle(), anchor);
            }
        })
        .build(app)?;
    Ok(())
}

/// Show the panel under the tray icon, or hide it if already visible. `anchor` is
/// the icon's physical screen rect `(x, y, width, height)`, when available.
fn toggle_panel(app: &tauri::AppHandle, anchor: Option<(i32, i32, i32, i32)>) {
    let Some(panel) = app.get_webview_window(PANEL_LABEL) else {
        return;
    };
    if panel.is_visible().unwrap_or(false) {
        let _ = panel.hide();
        return;
    }

    // Position the panel's top edge just below the tray icon, right-aligned to it
    // (matching a native menu-bar popover dropping down from the status item).
    if let Some((ix, iy, iw, ih)) = anchor {
        if let Ok(win_size) = panel.outer_size() {
            let x = ix + iw - (win_size.width as i32);
            let y = iy + ih;
            let _ = panel.set_position(tauri::PhysicalPosition { x: x.max(0), y });
        }
    }
    let _ = panel.show();
    let _ = panel.set_focus();
}

/// 1 Hz tick on a background thread, doing its UI work on the main thread:
/// advance the clock, refresh the tray title, and emit `state-changed` when the
/// rendered state actually changed.
fn spawn_tick_loop(app: tauri::AppHandle) {
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let app = app.clone();
        let scheduled = app.clone().run_on_main_thread(move || {
            let clock_state = app.state::<SharedClock>();
            let mut clock = clock_state.lock().expect("clock lock");
            clock.tick(Utc::now());
            if let Some(tray) = app.tray_by_id(TRAY_ID) {
                let _ = tray.set_title(Some(clock.tray_label()));
            }
            if clock.should_emit() {
                let snapshot = clock.snapshot();
                let _ = app.emit(STATE_EVENT, snapshot);
            }
        });
        // The main event loop is gone (app exiting) — stop, don't strand the thread.
        if scheduled.is_err() {
            break;
        }
    });
}
