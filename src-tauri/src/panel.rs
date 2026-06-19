//! The menu-bar dropdown panel and its dismiss backdrop.
//!
//! On macOS the panel is a non-activating `NSPanel` (via `tauri-nspanel`) so it
//! behaves like a native menu-bar popover — it floats above the menu bar, never
//! activates the app, and hides on resign-key. A transparent full-screen backdrop
//! window (level just below the panel, above apps/wallpaper) catches the dismiss
//! click and consumes it, so clicking outside closes the panel without triggering
//! macOS "reveal desktop". Other platforms use a borderless hide-on-blur window.

use tauri::{
    AppHandle, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindowBuilder,
};

#[cfg(not(target_os = "macos"))]
use tauri::WindowEvent;

use crate::BACKDROP_LABEL;
// Panel window constants are only referenced by the non-macOS window path; the
// macOS path imports them inside its own submodule.
#[cfg(not(target_os = "macos"))]
use crate::{PANEL_H, PANEL_LABEL, PANEL_W};

// --- Backdrop (click-catcher) ----------------------------------------------

/// A transparent, borderless, always-on-top window created hidden. While the
/// panel is open it covers the whole virtual screen just below the panel and
/// swallows the first outside click (its webview calls `dismiss_panel`).
pub fn build_backdrop(app: &AppHandle) -> tauri::Result<()> {
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

// --- macOS NSPanel ---------------------------------------------------------

#[cfg(target_os = "macos")]
mod macos {
    // The `panel_event!` macro requires an explicit `-> ()` return type, which
    // clippy would otherwise flag as a needless unit return in the expansion.
    #![allow(clippy::unused_unit)]
    use tauri::{AppHandle, Manager, PhysicalPosition, Size, WebviewUrl};
    use tauri_nspanel::{
        tauri_panel, CollectionBehavior, ManagerExt, PanelBuilder, PanelLevel, StyleMask,
    };

    use crate::{PANEL_H, PANEL_LABEL, PANEL_W};

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
        let panel = PanelBuilder::<_, PrayerPanel>::new(app, PANEL_LABEL)
            .url(WebviewUrl::App("index.html".into()))
            .title("Prayer Times")
            .size(Size::Logical((PANEL_W, PANEL_H).into()))
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

        // Lost key focus (backdrop/another app took it) → dismiss everything.
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
        app.get_webview_panel(PANEL_LABEL)
            .map(|p| p.is_visible())
            .unwrap_or(false)
    }

    pub fn show(app: &AppHandle, anchor: Option<(i32, i32, i32, i32)>) {
        let Ok(panel) = app.get_webview_panel(PANEL_LABEL) else {
            return;
        };
        if let (Some((ix, iy, iw, ih)), Some(win)) = (anchor, app.get_webview_window(PANEL_LABEL)) {
            if let Ok(sz) = win.outer_size() {
                let x = (ix + iw - sz.width as i32).max(0);
                let y = iy + ih;
                let _ = win.set_position(PhysicalPosition { x, y });
            }
        }
        panel.show_and_make_key();
        set_tray_highlighted(app, true);
    }

    pub fn hide(app: &AppHandle) {
        if let Ok(panel) = app.get_webview_panel(PANEL_LABEL) {
            panel.hide();
        }
        set_tray_highlighted(app, false);
    }

    /// Toggle the native menu-bar highlight on our `NSStatusItem`'s button — the
    /// same selected background AppKit draws for the Wi-Fi/Control-Center items
    /// while their popover is open. `tray-icon` keeps the status item private, so
    /// we reach the button through AppKit (it lives inside an `NSStatusBarWindow`
    /// in `NSApp.windows`) and only message a view that responds to `highlight:`,
    /// so an unexpected layout no-ops instead of throwing (which would abort).
    fn set_tray_highlighted(_app: &AppHandle, on: bool) {
        use tauri_nspanel::objc2_app_kit::NSApplication;
        use tauri_nspanel::objc2_foundation::MainThreadMarker;

        let Some(mtm) = MainThreadMarker::new() else {
            return;
        };
        let ns_app = NSApplication::sharedApplication(mtm);
        for window in ns_app.windows().iter() {
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

    /// First view responding to `highlight:` (the status-bar button) gets its
    /// highlight set. Guarded by `respondsToSelector:` so it never throws.
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
            highlight_recursive(&subviews.objectAtIndex(i), on);
        }
    }
}

// --- Other platforms (borderless window + hide-on-blur) --------------------

#[cfg(not(target_os = "macos"))]
fn build_window(app: &AppHandle) -> tauri::Result<()> {
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
fn show_window(app: &AppHandle, anchor: Option<(i32, i32, i32, i32)>) {
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

// --- Platform-neutral API --------------------------------------------------

pub fn build(app: &AppHandle) -> tauri::Result<()> {
    #[cfg(target_os = "macos")]
    {
        macos::build(app)
    }
    #[cfg(not(target_os = "macos"))]
    {
        build_window(app)
    }
}

fn is_visible(app: &AppHandle) -> bool {
    #[cfg(target_os = "macos")]
    {
        macos::is_visible(app)
    }
    #[cfg(not(target_os = "macos"))]
    {
        app.get_webview_window(PANEL_LABEL)
            .map(|w| w.is_visible().unwrap_or(false))
            .unwrap_or(false)
    }
}

/// Open the panel: raise the click-catching backdrop, then show the panel above it.
fn show(app: &AppHandle, anchor: Option<(i32, i32, i32, i32)>) {
    show_backdrop(app);
    #[cfg(target_os = "macos")]
    macos::show(app, anchor);
    #[cfg(not(target_os = "macos"))]
    show_window(app, anchor);
}

/// Hide the panel and the backdrop together.
pub fn hide_all(app: &AppHandle) {
    #[cfg(target_os = "macos")]
    macos::hide(app);
    #[cfg(not(target_os = "macos"))]
    if let Some(w) = app.get_webview_window(PANEL_LABEL) {
        let _ = w.hide();
    }
    hide_backdrop(app);
}

pub fn toggle(app: &AppHandle, anchor: Option<(i32, i32, i32, i32)>) {
    if is_visible(app) {
        hide_all(app);
    } else {
        show(app, anchor);
    }
}
