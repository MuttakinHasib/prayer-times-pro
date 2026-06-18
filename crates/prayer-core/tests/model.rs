//! Value-type model logic: next/current prayer selection, ordering, and sound
//! metadata. Ported from PrayerKit `ModelTests` (the `AppSettings` cases land in
//! the M3 settings port).

use std::collections::BTreeMap;

use chrono::{DateTime, Duration, TimeZone};
use chrono_tz::{Tz, UTC};
use prayer_core::{NotificationSound, Prayer, PrayerTimes};

fn sample_times() -> PrayerTimes {
    let base = UTC.timestamp_opt(1_700_000_000, 0).unwrap(); // fixed anchor
    let at = |h: f64| base + Duration::seconds((h * 3600.0) as i64);
    let times = BTreeMap::from([
        (Prayer::Fajr, at(5.0)),
        (Prayer::Sunrise, at(6.5)),
        (Prayer::Dhuhr, at(12.0)),
        (Prayer::Asr, at(15.0)),
        (Prayer::Maghrib, at(18.0)),
        (Prayer::Isha, at(19.5)),
    ]);
    PrayerTimes::new(base, times)
}

fn plus_hours(t: &PrayerTimes, h: f64) -> DateTime<Tz> {
    t.date + Duration::seconds((h * 3600.0) as i64)
}

#[test]
fn next_after_picks_earliest_future_prayer() {
    let t = sample_times();
    let now = plus_hours(&t, 13.0); // between Dhuhr and Asr
    assert_eq!(t.next(now).map(|(p, _)| p), Some(Prayer::Asr));
}

#[test]
fn next_after_returns_none_when_all_past() {
    let t = sample_times();
    let now = plus_hours(&t, 23.0);
    assert!(t.next(now).is_none());
}

#[test]
fn current_at_picks_most_recent_past() {
    let t = sample_times();
    let now = plus_hours(&t, 18.5); // just after Maghrib
    assert_eq!(t.current(now).map(|(p, _)| p), Some(Prayer::Maghrib));
}

#[test]
fn current_at_returns_none_before_fajr() {
    let t = sample_times();
    let now = plus_hours(&t, 3.0);
    assert!(t.current(now).is_none());
}

#[test]
fn ordered_is_chronological() {
    let order: Vec<Prayer> = sample_times().ordered().into_iter().map(|(p, _)| p).collect();
    assert_eq!(
        order,
        vec![
            Prayer::Fajr,
            Prayer::Sunrise,
            Prayer::Dhuhr,
            Prayer::Asr,
            Prayer::Maghrib,
            Prayer::Isha
        ]
    );
}

#[test]
fn obligatory_excludes_sunrise() {
    assert_eq!(
        Prayer::OBLIGATORY,
        [Prayer::Fajr, Prayer::Dhuhr, Prayer::Asr, Prayer::Maghrib, Prayer::Isha]
    );
    assert!(!Prayer::Sunrise.is_obligatory());
    assert!(Prayer::Dhuhr.is_obligatory());
}

#[test]
fn notification_sound_metadata() {
    assert!(NotificationSound::AdhanMakkah.has_full_adhan());
    assert!(!NotificationSound::SoftChime.has_full_adhan());
    assert_eq!(
        NotificationSound::AdhanMakkah.full_adhan_file_name(),
        Some("adhan-makkah.m4a")
    );
    assert_eq!(
        NotificationSound::AdhanMakkah.notification_clip_file_name(),
        Some("takbir.caf")
    );
    assert_eq!(NotificationSound::None.notification_clip_file_name(), None);
}
