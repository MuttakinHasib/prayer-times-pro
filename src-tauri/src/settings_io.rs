//! Persist [`AppSettings`] as JSON under the app config directory.

use std::fs;
use std::path::PathBuf;

use prayer_core::AppSettings;
use tauri::{AppHandle, Manager};

const FILE: &str = "settings.json";

fn settings_path(app: &AppHandle) -> Option<PathBuf> {
    app.path().app_config_dir().ok().map(|dir| dir.join(FILE))
}

/// Load persisted settings, falling back to defaults if missing or unreadable.
pub fn load(app: &AppHandle) -> AppSettings {
    let Some(path) = settings_path(app) else {
        return AppSettings::default();
    };
    match fs::read_to_string(&path) {
        Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
        Err(_) => AppSettings::default(),
    }
}

/// Write settings to disk (best effort; errors are surfaced to the caller).
pub fn save(app: &AppHandle, settings: &AppSettings) -> std::io::Result<()> {
    let Some(path) = settings_path(app) else {
        return Ok(());
    };
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_string_pretty(settings).map_err(std::io::Error::other)?;
    fs::write(path, json)
}
