//! The persisted application configuration and its display/notification enums.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::coordinates::Coordinates;
use crate::high_latitude::HighLatitudeRule;
use crate::models::notification::{
    NotificationDefaults, PrayerNotificationConfig, ResolvedNotification,
};
use crate::models::prayer::Prayer;
use crate::params::CalculationParameters;

/// Menu-bar label content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MenuBarStyle {
    IconOnly,
    CountdownOnly,
    IconCountdown,
    NextPrayerCountdown,
    IconNameCountdown,
    NextPrayerClock,
    IconNameClock,
}

/// What the menu-bar countdown counts toward.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MenuBarCountdownMode {
    NextPrayer,
    CurrentWaqt,
}

/// Strength of the Focus Mode backdrop blur.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FocusBlurIntensity {
    Low,
    Medium,
    High,
    Opaque,
}

/// Which prayers engage Focus Mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FocusTrigger {
    Obligatory,
    All,
    FajrIsha,
}

impl FocusTrigger {
    /// Whether `prayer` should trigger Focus Mode under this rule.
    pub fn includes(self, prayer: Prayer) -> bool {
        match self {
            FocusTrigger::Obligatory => prayer.is_obligatory(),
            FocusTrigger::All => true,
            FocusTrigger::FajrIsha => prayer == Prayer::Fajr || prayer == Prayer::Isha,
        }
    }
}

/// How the daily times are sourced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CalculationMode {
    Calculated,
    Manual,
}

/// How the observer location is determined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LocationMode {
    Automatic,
    Manual,
}

/// UI theme preference. `Auto` follows the system's prefers-color-scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AppearanceTheme {
    Dark,
    Light,
    Auto,
}

/// Sensible fallback early-reminder lead (minutes) when a reminder is enabled but
/// neither the per-prayer override nor the default supplies a concrete lead.
pub const FALLBACK_EARLY_LEAD_MINUTES: i32 = 15;

/// The full persisted configuration. Decodes resiliently — every field is
/// `#[serde(default)]`, so an older stored blob never fails to load and missing
/// keys fall back to [`AppSettings::default`] rather than resetting the user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AppSettings {
    pub method_id: String,
    pub manual_parameters: Option<CalculationParameters>,
    pub hanafi_asr: bool,
    pub high_latitude_rule: HighLatitudeRule,
    pub location_mode: LocationMode,
    pub manual_coordinates: Option<Coordinates>,
    /// IANA timezone id; `None` follows the system zone.
    pub timezone_override: Option<String>,
    pub auto_detect_method: bool,

    pub calculation_mode: CalculationMode,
    /// Minutes before each jamaat time the azan reminder fires (0 = at jamaat).
    pub azan_before_jamaat: i32,
    /// Keep astronomical Sunrise & non-jamaat windows while in manual mode.
    pub manual_keep_waqt: bool,
    /// Fixed jamaat times for the five obligatory prayers, minutes since midnight.
    pub jamaat_times: BTreeMap<Prayer, i32>,

    pub menu_bar_style: MenuBarStyle,
    pub menu_bar_countdown_mode: MenuBarCountdownMode,
    pub show_ishraq_time: bool,
    pub show_hijri_date: bool,
    /// Whole-day correction applied to the displayed Hijri date.
    pub hijri_day_adjustment: i32,

    pub focus_mode_enabled: bool,
    pub focus_duration_minutes: i32,
    pub focus_blur_intensity: FocusBlurIntensity,
    pub focus_trigger: FocusTrigger,
    pub focus_emergency_exit_enabled: bool,

    pub appearance: AppearanceTheme,
    pub launch_at_login: bool,
    /// BCP-47 language tag; `None` follows the system locale.
    pub language_override: Option<String>,
    pub master_notifications_enabled: bool,
    pub notification_defaults: NotificationDefaults,
    pub notifications: BTreeMap<Prayer, PrayerNotificationConfig>,
    pub auto_update_enabled: bool,
    /// Whether the first-launch setup wizard has been completed (or skipped).
    pub did_complete_onboarding: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            method_id: "mwl".into(),
            manual_parameters: None,
            hanafi_asr: false,
            high_latitude_rule: HighLatitudeRule::Automatic,
            location_mode: LocationMode::Automatic,
            manual_coordinates: None,
            timezone_override: None,
            auto_detect_method: false,
            calculation_mode: CalculationMode::Calculated,
            azan_before_jamaat: 15,
            manual_keep_waqt: true,
            jamaat_times: default_jamaat_times(),
            menu_bar_style: MenuBarStyle::IconNameCountdown,
            menu_bar_countdown_mode: MenuBarCountdownMode::NextPrayer,
            show_ishraq_time: false,
            show_hijri_date: true,
            hijri_day_adjustment: 0,
            focus_mode_enabled: false,
            focus_duration_minutes: 15,
            focus_blur_intensity: FocusBlurIntensity::Medium,
            focus_trigger: FocusTrigger::Obligatory,
            focus_emergency_exit_enabled: true,
            appearance: AppearanceTheme::Auto,
            launch_at_login: false,
            language_override: None,
            master_notifications_enabled: true,
            notification_defaults: NotificationDefaults::default(),
            notifications: default_notifications(),
            auto_update_enabled: true,
            did_complete_onboarding: false,
        }
    }
}

impl AppSettings {
    /// Effective early-reminder lead for `prayer`: the per-prayer override, else
    /// the global default. May be 0 ("no lead set").
    pub fn early_lead_minutes(&self, prayer: Prayer) -> i32 {
        self.notifications
            .get(&prayer)
            .and_then(|c| c.early_lead_minutes_override)
            .unwrap_or(self.notification_defaults.early_reminder_minutes)
    }

    /// Merge a prayer's per-prayer config over the app defaults into the concrete
    /// values the scheduler consumes. A reminder fires only when enabled *and*
    /// it resolves to a positive lead. Sunrise never carries Adhan or iqamah.
    pub fn resolved_notification(&self, prayer: Prayer) -> ResolvedNotification {
        let cfg = self.notifications.get(&prayer).cloned().unwrap_or_default();
        // A reminder stays on whenever the user enabled it: a non-positive lead
        // (unset/zero/negative) means "no concrete lead set", so fall back to
        // FALLBACK_EARLY_LEAD_MINUTES rather than silently dropping the reminder.
        let lead = match self.early_lead_minutes(prayer) {
            n if n > 0 => n,
            _ => FALLBACK_EARLY_LEAD_MINUTES,
        };
        let iqamah = if prayer.is_obligatory() {
            cfg.iqamah_offset_minutes_override
                .unwrap_or(self.notification_defaults.iqamah_offset_minutes)
        } else {
            0
        };
        ResolvedNotification {
            notify: cfg.notify,
            sound: cfg.sound_override.unwrap_or(self.notification_defaults.sound),
            play_full_adhan: prayer.is_obligatory() && cfg.play_full_adhan,
            early_reminder_enabled: cfg.early_reminder_enabled,
            early_lead_minutes: lead,
            iqamah_offset_minutes: iqamah.max(0),
        }
    }
}

/// Placeholder jamaat schedule (minutes since midnight), seeded for Manual mode.
pub fn default_jamaat_times() -> BTreeMap<Prayer, i32> {
    BTreeMap::from([
        (Prayer::Fajr, 5 * 60),
        (Prayer::Dhuhr, 13 * 60 + 30),
        (Prayer::Asr, 17 * 60),
        (Prayer::Maghrib, 18 * 60 + 30),
        (Prayer::Isha, 20 * 60),
    ])
}

/// Sensible per-prayer defaults: Sunrise quiet, Dhuhr 20-min reminder, Maghrib 10.
pub fn default_notifications() -> BTreeMap<Prayer, PrayerNotificationConfig> {
    let mut configs = BTreeMap::new();
    for prayer in Prayer::ALL {
        let cfg = match prayer {
            Prayer::Sunrise => PrayerNotificationConfig {
                notify: false,
                ..Default::default()
            },
            Prayer::Dhuhr => PrayerNotificationConfig {
                early_reminder_enabled: true,
                early_lead_minutes_override: Some(20),
                ..Default::default()
            },
            Prayer::Maghrib => PrayerNotificationConfig {
                early_reminder_enabled: true,
                early_lead_minutes_override: Some(10),
                ..Default::default()
            },
            _ => PrayerNotificationConfig::default(),
        };
        configs.insert(prayer, cfg);
    }
    configs
}
