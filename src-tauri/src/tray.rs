//! Menu-bar tray icon and the 1 Hz clock tick.

use std::sync::PoisonError;

use chrono::Utc;
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager,
};

use crate::panel;
use crate::state::SharedClock;
use crate::{STATE_EVENT, TRAY_ID};

/// Build the tray icon. The icon carries the mihrab glyph; `set_title` shows the
/// live countdown text (macOS). Left-click toggles the panel.
pub fn build(app: &AppHandle) -> tauri::Result<()> {
    let mut builder = TrayIconBuilder::with_id(TRAY_ID)
        .title("Prayer Times")
        .tooltip("Prayer Times");

    // Mihrab glyph embedded at compile time and rendered as a template so macOS
    // tints it for the light/dark menu bar. Falls back to the default icon if the
    // embedded PNG ever fails to decode.
    match tauri::image::Image::from_bytes(include_bytes!("../icons/tray-mihrab@2x.png")) {
        Ok(icon) => builder = builder.icon(icon).icon_as_template(true),
        Err(err) => {
            eprintln!("tray: mihrab icon decode failed ({err}); using default icon");
            if let Some(icon) = app.default_window_icon().cloned() {
                builder = builder.icon(icon);
            }
        }
    }

    builder
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                rect,
                ..
            } = event
            {
                let anchor = match (rect.position, rect.size) {
                    (tauri::Position::Physical(p), tauri::Size::Physical(s)) => {
                        Some((p.x, p.y, s.width as i32, s.height as i32))
                    }
                    _ => None,
                };
                panel::toggle(tray.app_handle(), anchor);
            }
        })
        .build(app)?;
    Ok(())
}

/// 1 Hz tick on a background thread, doing its UI work on the main thread:
/// advance the clock, refresh the tray title, and emit `state-changed` when the
/// rendered state actually changed.
pub fn spawn_tick_loop(app: AppHandle) {
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let app = app.clone();
        let scheduled = app.clone().run_on_main_thread(move || {
            let clock_state = app.state::<SharedClock>();
            let mut clock = clock_state.lock().unwrap_or_else(PoisonError::into_inner);
            clock.tick(Utc::now());
            if let Some(tray) = app.tray_by_id(TRAY_ID) {
                let _ = tray.set_title(Some(clock.tray_label()));
            }
            if clock.should_emit() {
                let snapshot = clock.snapshot();
                let _ = app.emit(STATE_EVENT, snapshot);
            }
            let due = clock.due_notifications();
            if !due.is_empty() {
                let now_ms = clock.now_ms();
                let audio = app.state::<crate::audio::Audio>();
                crate::scheduler::fire(&app, audio.inner(), &due, now_ms);
            }
        });
        // The main event loop is gone (app exiting) — stop, don't strand the thread.
        if scheduled.is_err() {
            break;
        }
    });
}
