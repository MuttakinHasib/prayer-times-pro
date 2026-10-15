//! Thin wrapper over `tauri-plugin-autostart` so callers don't have to think
//! about the manager or registration state — they pass a desired bool and we
//! reconcile.

use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;

/// Ensure launch-at-login matches `desired`. No-ops if it's already correct, and
/// any plugin error is logged (not surfaced) so a misconfigured launch agent
/// can't take the rest of the app down.
pub fn sync(app: &AppHandle, desired: bool) {
    let manager = app.autolaunch();
    let current = manager.is_enabled().unwrap_or(false);
    if current == desired {
        return;
    }
    let result = if desired { manager.enable() } else { manager.disable() };
    if let Err(err) = result {
        eprintln!("autostart: failed to {} ({err})", if desired { "enable" } else { "disable" });
    }
}
