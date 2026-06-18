//! Verifies the "current prayer window" resolver that drives the time-left
//! countdown. Ported from PrayerKit `CurrentWaqtTests`.

use std::collections::BTreeMap;

use chrono::{DateTime, TimeZone};
use chrono_tz::{Tz, UTC};
use prayer_core::{CurrentWaqt, Prayer, PrayerTimes};

/// A fixed instant on 2026-06-08 (`day` 0) or a following day, in UTC.
fn at(hour: u32, minute: u32, day: u32) -> DateTime<Tz> {
    UTC.with_ymd_and_hms(2026, 6, 8 + day, hour, minute, 0).unwrap()
}

fn day(offset: u32) -> PrayerTimes {
    let times = BTreeMap::from([
        (Prayer::Fajr, at(5, 0, offset)),
        (Prayer::Sunrise, at(6, 30, offset)),
        (Prayer::Dhuhr, at(12, 0, offset)),
        (Prayer::Asr, at(15, 30, offset)),
        (Prayer::Maghrib, at(18, 0, offset)),
        (Prayer::Isha, at(19, 30, offset)),
    ]);
    PrayerTimes::new(at(0, 0, offset), times)
}

fn waqt(now: DateTime<Tz>) -> Option<CurrentWaqt> {
    CurrentWaqt::resolve(now, &day(0), &day(1))
}

#[test]
fn during_fajr_ends_at_sunrise() {
    let w = waqt(at(5, 30, 0)).unwrap();
    assert_eq!(w.prayer, Prayer::Fajr);
    assert_eq!(w.end, at(6, 30, 0));
    assert!(w.is_obligatory());
}

#[test]
fn sunrise_to_dhuhr_is_non_obligatory_gap() {
    let w = waqt(at(9, 0, 0)).unwrap();
    assert_eq!(w.prayer, Prayer::Sunrise);
    assert!(!w.is_obligatory());
    assert_eq!(w.end, at(12, 0, 0));
}

#[test]
fn during_dhuhr_ends_at_asr() {
    let w = waqt(at(13, 0, 0)).unwrap();
    assert_eq!(w.prayer, Prayer::Dhuhr);
    assert_eq!(w.end, at(15, 30, 0));
}

#[test]
fn during_asr_ends_at_maghrib() {
    let w = waqt(at(16, 0, 0)).unwrap();
    assert_eq!(w.prayer, Prayer::Asr);
    assert_eq!(w.end, at(18, 0, 0));
}

#[test]
fn during_maghrib_ends_at_isha() {
    let w = waqt(at(18, 30, 0)).unwrap();
    assert_eq!(w.prayer, Prayer::Maghrib);
    assert_eq!(w.end, at(19, 30, 0));
}

#[test]
fn during_isha_ends_at_tomorrow_fajr() {
    let w = waqt(at(21, 0, 0)).unwrap();
    assert_eq!(w.prayer, Prayer::Isha);
    assert_eq!(w.end, at(5, 0, 1));
}

#[test]
fn after_midnight_before_fajr_still_isha() {
    let w = waqt(at(2, 0, 0)).unwrap();
    assert_eq!(w.prayer, Prayer::Isha);
    assert_eq!(w.end, at(5, 0, 0));
}

#[test]
fn exact_boundary_belongs_to_starting_prayer() {
    let w = waqt(at(15, 30, 0)).unwrap();
    assert_eq!(w.prayer, Prayer::Asr);
    assert_eq!(w.end, at(18, 0, 0));
}

#[test]
fn ishraq_is_sunrise_plus_offset() {
    assert_eq!(day(0).ishraq(15), Some(at(6, 45, 0)));
    assert_eq!(day(0).ishraq(20), Some(at(6, 50, 0)));
}
