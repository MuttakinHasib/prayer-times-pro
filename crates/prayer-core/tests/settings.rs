//! AppSettings defaults, resilient JSON round-trip, and notification resolution.

use prayer_core::{
    AppSettings, NotificationDefaults, NotificationSound, Prayer, PrayerNotificationConfig,
};

#[test]
fn round_trips_through_json() {
    let settings = AppSettings {
        method_id: "diyanet".into(),
        hanafi_asr: true,
        timezone_override: Some("Europe/Istanbul".into()),
        language_override: Some("tr".into()),
        ..AppSettings::default()
    };

    let json = serde_json::to_string(&settings).unwrap();
    let decoded: AppSettings = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, settings);
}

#[test]
fn partial_blob_falls_back_to_defaults() {
    // An older/partial blob must load without resetting the user.
    let decoded: AppSettings = serde_json::from_str(r#"{"methodId":"isna"}"#).unwrap();
    assert_eq!(decoded.method_id, "isna");
    assert_eq!(decoded, AppSettings { method_id: "isna".into(), ..AppSettings::default() });
}

#[test]
fn default_notifications_match_product_examples() {
    let s = AppSettings::default();
    assert_eq!(s.notifications.len(), 6);
    assert_eq!(s.notifications[&Prayer::Dhuhr].early_lead_minutes_override, Some(20));
    assert!(s.notifications[&Prayer::Dhuhr].early_reminder_enabled);
    assert_eq!(s.notifications[&Prayer::Maghrib].early_lead_minutes_override, Some(10));
    assert!(!s.notifications[&Prayer::Sunrise].notify);
}

#[test]
fn resolved_notification_inherits_defaults() {
    let mut s = AppSettings {
        notification_defaults: NotificationDefaults {
            sound: NotificationSound::AdhanMakkah,
            early_reminder_minutes: 10,
            iqamah_offset_minutes: 5,
            play_full_adhan: false,
        },
        ..AppSettings::default()
    };
    // Asr inherits everything (only the reminder toggle is set).
    s.notifications.insert(
        Prayer::Asr,
        PrayerNotificationConfig { early_reminder_enabled: true, ..Default::default() },
    );
    let asr = s.resolved_notification(Prayer::Asr);
    assert_eq!(asr.sound, NotificationSound::AdhanMakkah);
    assert_eq!(asr.early_lead_minutes, 10);
    assert_eq!(asr.iqamah_offset_minutes, 5);
    assert!(asr.early_reminder_enabled);

    // Fajr overrides the sound and lead; iqamah still inherits.
    s.notifications.insert(
        Prayer::Fajr,
        PrayerNotificationConfig {
            early_reminder_enabled: true,
            sound_override: Some(NotificationSound::Takbir),
            early_lead_minutes_override: Some(30),
            ..Default::default()
        },
    );
    let fajr = s.resolved_notification(Prayer::Fajr);
    assert_eq!(fajr.sound, NotificationSound::Takbir);
    assert_eq!(fajr.early_lead_minutes, 30);
    assert_eq!(fajr.iqamah_offset_minutes, 5);

    // Sunrise never carries Adhan or iqamah, even if asked.
    s.notifications.insert(
        Prayer::Sunrise,
        PrayerNotificationConfig { play_full_adhan: true, ..Default::default() },
    );
    let sunrise = s.resolved_notification(Prayer::Sunrise);
    assert!(!sunrise.play_full_adhan);
    assert_eq!(sunrise.iqamah_offset_minutes, 0);
}
