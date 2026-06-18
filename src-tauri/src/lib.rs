//! Tauri app entry. M2: menu-bar tray with a live countdown title, a dropdown
//! panel, and a 1 Hz Rust tick loop that owns the clock and emits
//! `prayer://state-changed` when the rendered state changes.
//!
//! On macOS the panel is a non-activating `NSPanel` (via `tauri-nspanel`) so it
//! behaves like a native menu-bar popover — it floats, never activates the app,
//! and vanishes the moment it loses key focus. A transparent full-screen
//! backdrop window sits below the panel and above everything else; it captures
//! the dismiss click so it never reaches the wallpaper (no "reveal desktop"),
//! exactly like the system menu-bar popovers. Other platforms use a borderless
//! always-on-top window with hide-on-blur + the same backdrop.

mod state;

use std::sync::Mutex;

use chrono::Utc;
use state::{AppConfig, Clock, PrayerState, SharedClock};
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, LogicalSize, Manager, PhysicalPosition, PhysicalSize, WebviewUrl,
    WebviewWindowBuilder,
};

#[cfg(not(target_os = "macos"))]
use tauri::WindowEvent;

const PANEL_LABEL: &str = "panel";
const BACKDROP_LABEL: &str = "backdrop";
const TRAY_ID: &str = "tray";
const STATE_EVENT: &str = "prayer://state-changed";
const PANEL_W: f64 = 312.0;
const PANEL_H: f64 = 482.0;

/// The panel's measured content size (logical px), reported by the webview so the
/// window can be sized to its content (so the shadow hugs the card, no empty rect).
#[derive(Default)]
struct ContentSize(Mutex<Option<(f64, f64)>>);

// --- Commands --------------------------------------------------------------

#[tauri::command]
fn get_prayer_state(clock: tauri::State<'_, SharedClock>) -> PrayerState {
    clock.lock().expect("clock lock").snapshot()
}

/// Hide the panel (e.g. after a footer action) — also clears the backdrop.
#[tauri::command]
fn hide_panel(app: AppHandle) {
    hide_all(&app);
}

/// Clicked on the transparent backdrop (outside the panel) — dismiss everything.
#[tauri::command]
fn dismiss_panel(app: AppHandle) {
    hide_all(&app);
}

#[tauri::command]
fn quit_app(app: AppHandle) {
    app.exit(0);
}

/// The webview reports its measured content size (logical px). We size the panel
/// window to it while hidden, so re-opens never resize a visible window.
#[tauri::command]
fn report_content_size(app: AppHandle, size: tauri::State<'_, ContentSize>, width: f64, height: f64) {
    *size.0.lock().expect("content-size lock") = Some((width, height));
    if let Some(win) = app.get_webview_window(PANEL_LABEL) {
        if !win.is_visible().unwrap_or(false) {
            let _ = win.set_size(LogicalSize::new(width, height));
        }
    }
}

/// M2 stubs — fleshed out in later milestones (M3 settings, M9 updater).
#[tauri::command]
fn open_settings(app: AppHandle) {
    let _ = app.emit("prayer://open-settings-requested", ());
}

#[tauri::command]
fn check_for_updates(app: AppHandle) {
    let _ = app.emit("prayer://check-updates-requested", ());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_opener::init());

    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(tauri_nspanel::init());
    }

    builder
        .manage(SharedClock::new(Clock::new(AppConfig::default())))
        .manage(ContentSize::default())
        .invoke_handler(tauri::generate_handler![
            get_prayer_state,
            hide_panel,
            dismiss_panel,
            quit_app,
            report_content_size,
            open_settings,
            check_for_updates
        ])
        .setup(|app| {
            // Menu-bar agent: no Dock icon on macOS.
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            build_panel(app.handle())?;
            build_backdrop(app.handle())?;
            build_tray(app.handle())?;
            spawn_tick_loop(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// --- Backdrop (click-catcher) ----------------------------------------------

/// A transparent, borderless, always-on-top window created hidden. While the
/// panel is open it covers the whole virtual screen just below the panel and
/// swallows the first outside click (its webview calls `dismiss_panel`), so that
/// click never reaches the wallpaper or another app.
fn build_backdrop(app: &AppHandle) -> tauri::Result<()> {
    WebviewWindowBuilder::new(app, BACKDROP_LABEL, WebviewUrl::App("index.html".into()))
        .title("")
        .decorations(false)
        .transparent(true)
        .shadow(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .focused(false)
        .visible(false)
        .build()?;
    Ok(())
}

fn show_backdrop(app: &AppHandle) {
    let Some(bd) = app.get_webview_window(BACKDROP_LABEL) else {
        return;
    };
    // Cover the union of all monitors so a click on any screen is caught.
    if let Ok(monitors) = app.available_monitors() {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        for m in &monitors {
            let p = m.position();
            let s = m.size();
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x + s.width as i32);
            max_y = max_y.max(p.y + s.height as i32);
        }
        if max_x > min_x && max_y > min_y {
            let _ = bd.set_position(PhysicalPosition { x: min_x, y: min_y });
            let _ = bd.set_size(PhysicalSize {
                width: (max_x - min_x) as u32,
                height: (max_y - min_y) as u32,
            });
        }
    }
    let _ = bd.show();
}

fn hide_backdrop(app: &AppHandle) {
    if let Some(bd) = app.get_webview_window(BACKDROP_LABEL) {
        let _ = bd.hide();
    }
}

// --- Panel — macOS NSPanel -------------------------------------------------

#[cfg(target_os = "macos")]
mod macos_panel {
    // The `panel_event!` macro requires an explicit `-> ()` return type, which
    // clippy would otherwise flag as a needless unit return in the expansion.
    #![allow(clippy::unused_unit)]
    use tauri::{AppHandle, Manager, PhysicalPosition, Size, WebviewUrl};
    use tauri_nspanel::{
        tauri_panel, CollectionBehavior, ManagerExt, PanelBuilder, PanelLevel, StyleMask,
    };

    tauri_panel! {
        panel!(PrayerPanel {
            config: {
                can_become_key_window: true,
                can_become_main_window: false,
                is_floating_panel: true
            }
        })

        panel_event!(PrayerPanelEvents {
            window_did_resign_key(notification: &NSNotification) -> ()
        })
    }

    pub fn build(app: &AppHandle) -> tauri::Result<()> {
        let panel = PanelBuilder::<_, PrayerPanel>::new(app, super::PANEL_LABEL)
            .url(WebviewUrl::App("index.html".into()))
            .title("Prayer Times")
            .size(Size::Logical((super::PANEL_W, super::PANEL_H).into()))
            // Well above the floating-level backdrop so panel clicks hit the panel,
            // not the click-catcher (and above the menu bar so it's never clipped).
            .level(PanelLevel::PopUpMenu)
            .has_shadow(true)
            .collection_behavior(CollectionBehavior::new().can_join_all_spaces().stationary())
            .hides_on_deactivate(false)
            .with_window(|w| w.decorations(false).transparent(true).visible(false))
            .style_mask(StyleMask::empty().nonactivating_panel())
            .no_activate(true)
            .build()?;

        // No show/hide animation — appear/vanish instantly like a native popover.
        use tauri_nspanel::objc2_app_kit::NSWindowAnimationBehavior;
        panel
            .as_panel()
            .setAnimationBehavior(NSWindowAnimationBehavior::None);

        // Lost key focus (e.g. backdrop/another app took it) → dismiss everything.
        let handle = app.clone();
        let handler = PrayerPanelEvents::new();
        handler.window_did_resign_key(move |_notification| {
            super::hide_all(&handle);
        });
        panel.set_event_handler(Some(handler.as_ref()));
        std::mem::forget(handler); // keep the delegate alive for the app's lifetime

        panel.hide();
        Ok(())
    }

    pub fn is_visible(app: &AppHandle) -> bool {
        app.get_webview_panel(super::PANEL_LABEL)
            .map(|p| p.is_visible())
            .unwrap_or(false)
    }

    pub fn show(app: &AppHandle, anchor: Option<(i32, i32, i32, i32)>) {
        let Ok(panel) = app.get_webview_panel(super::PANEL_LABEL) else {
            return;
        };
        if let (Some((ix, iy, iw, ih)), Some(win)) =
            (anchor, app.get_webview_window(super::PANEL_LABEL))
        {
            if let Ok(sz) = win.outer_size() {
                let x = (ix + iw - sz.width as i32).max(0);
                let y = iy + ih;
                let _ = win.set_position(PhysicalPosition { x, y });
            }
        }
        panel.show_and_make_key();
    }

    pub fn hide(app: &AppHandle) {
        if let Ok(panel) = app.get_webview_panel(super::PANEL_LABEL) {
            panel.hide();
        }
    }

    /// Toggle the native menu-bar highlight on our `NSStatusItem`'s button — the
    /// same selected background AppKit draws for the Wi-Fi/Control-Center items
    /// while their popover is open. `tray-icon` keeps the status item private, so
    /// we reach the button through AppKit (it lives inside an `NSStatusBarWindow`
    /// in `NSApp.windows`) and only message a view that actually responds to
    /// `highlight:`, so a layout we don't expect no-ops instead of throwing an
    /// Objective-C exception (which would abort the process).
    pub fn set_tray_highlighted(on: bool) {
        use tauri_nspanel::objc2_app_kit::NSApplication;
        use tauri_nspanel::objc2_foundation::MainThreadMarker;

        let Some(mtm) = MainThreadMarker::new() else {
            return;
        };
        let app = NSApplication::sharedApplication(mtm);
        for window in app.windows().iter() {
            let is_status_bar = window
                .class()
                .name()
                .to_str()
                .is_ok_and(|n| n.contains("StatusBar"));
            if is_status_bar {
                if let Some(view) = window.contentView() {
                    highlight_recursive(&view, on);
                }
            }
        }
    }

    /// Find the first view responding to `highlight:` (the status-bar button) and
    /// set its highlight. Guarded by `respondsToSelector:` so it never throws.
    fn highlight_recursive(view: &tauri_nspanel::objc2_app_kit::NSView, on: bool) {
        use tauri_nspanel::objc2::runtime::Bool;
        use tauri_nspanel::objc2::{msg_send, sel};

        let responds: Bool = unsafe { msg_send![view, respondsToSelector: sel!(highlight:)] };
        if responds.as_bool() {
            unsafe {
                let _: () = msg_send![view, highlight: Bool::new(on)];
            }
            return;
        }
        let subviews = view.subviews();
        for i in 0..subviews.count() {
            let sub = subviews.objectAtIndex(i);
            highlight_recursive(&sub, on);
        }
    }
}

// --- Panel — other platforms (borderless window + hide-on-blur) ------------

#[cfg(not(target_os = "macos"))]
fn build_panel_window(app: &AppHandle) -> tauri::Result<()> {
    let win = WebviewWindowBuilder::new(app, PANEL_LABEL, WebviewUrl::App("index.html".into()))
        .title("Prayer Times")
        .inner_size(PANEL_W, PANEL_H)
        .resizable(false)
        .minimizable(false)
        .maximizable(false)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .visible(false)
        .build()?;

    let app = app.clone();
    win.on_window_event(move |event| {
        if let WindowEvent::Focused(false) = event {
            hide_all(&app);
        }
    });
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn show_window_panel(app: &AppHandle, anchor: Option<(i32, i32, i32, i32)>) {
    let Some(panel) = app.get_webview_window(PANEL_LABEL) else {
        return;
    };
    if let Some((ix, iy, iw, ih)) = anchor {
        if let Ok(sz) = panel.outer_size() {
            let x = (ix + iw - sz.width as i32).max(0);
            let y = iy + ih;
            let _ = panel.set_position(PhysicalPosition { x, y });
        }
    }
    let _ = panel.show();
    let _ = panel.set_focus();
}

// --- Platform-neutral entry points -----------------------------------------

fn build_panel(app: &AppHandle) -> tauri::Result<()> {
    #[cfg(target_os = "macos")]
    {
        macos_panel::build(app)
    }
    #[cfg(not(target_os = "macos"))]
    {
        build_panel_window(app)
    }
}

fn panel_visible(app: &AppHandle) -> bool {
    #[cfg(target_os = "macos")]
    {
        macos_panel::is_visible(app)
    }
    #[cfg(not(target_os = "macos"))]
    {
        app.get_webview_window(PANEL_LABEL)
            .map(|w| w.is_visible().unwrap_or(false))
            .unwrap_or(false)
    }
}

/// Open the panel: raise the click-catching backdrop, then show the panel above it.
fn show_panel(app: &AppHandle, anchor: Option<(i32, i32, i32, i32)>) {
    show_backdrop(app);
    #[cfg(target_os = "macos")]
    {
        macos_panel::show(app, anchor);
        macos_panel::set_tray_highlighted(true);
    }
    #[cfg(not(target_os = "macos"))]
    show_window_panel(app, anchor);
}

/// Hide the panel and the backdrop together.
fn hide_all(app: &AppHandle) {
    #[cfg(target_os = "macos")]
    {
        macos_panel::hide(app);
        macos_panel::set_tray_highlighted(false);
    }
    #[cfg(not(target_os = "macos"))]
    if let Some(w) = app.get_webview_window(PANEL_LABEL) {
        let _ = w.hide();
    }
    hide_backdrop(app);
}

fn toggle(app: &AppHandle, anchor: Option<(i32, i32, i32, i32)>) {
    if panel_visible(app) {
        hide_all(app);
    } else {
        show_panel(app, anchor);
    }
}

// --- Tray ------------------------------------------------------------------

fn build_tray(app: &AppHandle) -> tauri::Result<()> {
    let mut builder = TrayIconBuilder::with_id(TRAY_ID)
        .title("Prayer Times")
        .tooltip("Prayer Times");

    // Mosque glyph (from the macOS app's asset), embedded at compile time and
    // rendered as a template so macOS tints it for the light/dark menu bar. Falls
    // back to the default icon if the embedded PNG ever fails to decode.
    match tauri::image::Image::from_bytes(include_bytes!("../icons/tray-mosque@2x.png")) {
        Ok(icon) => builder = builder.icon(icon).icon_as_template(true),
        Err(err) => {
            eprintln!("tray: mosque icon decode failed ({err}); using default icon");
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
                toggle(tray.app_handle(), anchor);
            }
        })
        .build(app)?;
    Ok(())
}

/// 1 Hz tick on a background thread, doing its UI work on the main thread:
/// advance the clock, refresh the tray title, and emit `state-changed` when the
/// rendered state actually changed.
fn spawn_tick_loop(app: AppHandle) {
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
