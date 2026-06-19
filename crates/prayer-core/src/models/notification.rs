//! Per-prayer notification configuration with default inheritance.

use serde::{Deserialize, Serialize};

use crate::models::notification_sound::NotificationSound;

fn default_sound() -> NotificationSound {
    NotificationSound::Takbir
}

/// App-wide notification defaults, applied to every prayer unless a per-prayer
/// override is set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationDefaults {
    #[serde(default = "default_sound")]
    pub sound: NotificationSound,
    #[serde(default)]
    pub play_full_adhan: bool,
    /// Minutes before the prayer for the early reminder; 0 = off.
    #[serde(default)]
    pub early_reminder_minutes: i32,
    /// Minutes after the prayer for the iqamah alert; 0 = off.
    #[serde(default)]
    pub iqamah_offset_minutes: i32,
}

impl Default for NotificationDefaults {
    fn default() -> Self {
        Self {
            sound: default_sound(),
            play_full_adhan: false,
            early_reminder_minutes: 0,
            iqamah_offset_minutes: 0,
        }
    }
}

/// Per-prayer notification configuration. The three booleans map to the
/// Notify / Adhan / Remind matrix columns; the three optionals are overrides
/// that fall back to [`NotificationDefaults`] when `None`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrayerNotificationConfig {
    #[serde(default = "yes")]
    pub notify: bool,
    #[serde(default)]
    pub play_full_adhan: bool,
    #[serde(default)]
    pub early_reminder_enabled: bool,
    #[serde(default)]
    pub sound_override: Option<NotificationSound>,
    #[serde(default)]
    pub early_lead_minutes_override: Option<i32>,
    #[serde(default)]
    pub iqamah_offset_minutes_override: Option<i32>,
}

fn yes() -> bool {
    true
}

impl Default for PrayerNotificationConfig {
    fn default() -> Self {
        Self {
            notify: true,
            play_full_adhan: false,
            early_reminder_enabled: false,
            sound_override: None,
            early_lead_minutes_override: None,
            iqamah_offset_minutes_override: None,
        }
    }
}

/// The fully-resolved notification behaviour for one prayer, after merging the
/// per-prayer config over the app defaults — what the scheduler consumes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedNotification {
    pub notify: bool,
    pub sound: NotificationSound,
    pub play_full_adhan: bool,
    pub early_reminder_enabled: bool,
    pub early_lead_minutes: i32,
    pub iqamah_offset_minutes: i32,
}
