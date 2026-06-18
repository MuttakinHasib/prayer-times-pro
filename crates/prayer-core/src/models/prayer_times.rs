//! The computed absolute times for one calendar day at one location.

use std::collections::BTreeMap;

use chrono::{DateTime, Duration};
use chrono_tz::Tz;

use crate::models::prayer::Prayer;

/// The computed absolute times for one calendar day at one location, in the
/// timezone the engine was asked to use.
#[derive(Debug, Clone, PartialEq)]
pub struct PrayerTimes {
    /// The calendar day these times belong to (midnight, in the engine timezone).
    pub date: DateTime<Tz>,

    /// Absolute instant for each prayer. All six keys are present unless a polar
    /// edge case leaves Fajr/Isha undefined under `HighLatitudeRule::None`.
    pub times: BTreeMap<Prayer, DateTime<Tz>>,
}

impl PrayerTimes {
    pub fn new(date: DateTime<Tz>, times: BTreeMap<Prayer, DateTime<Tz>>) -> Self {
        Self { date, times }
    }

    /// Absolute instant for `prayer`, if present.
    pub fn get(&self, prayer: Prayer) -> Option<DateTime<Tz>> {
        self.times.get(&prayer).copied()
    }

    /// The next prayer strictly after `now`, scanning this day's times in
    /// chronological order. Returns `None` if every time today is in the past
    /// (caller should then consult tomorrow's `PrayerTimes`).
    pub fn next(&self, now: DateTime<Tz>) -> Option<(Prayer, DateTime<Tz>)> {
        self.times
            .iter()
            .filter(|(_, &t)| t > now)
            .min_by_key(|(_, &t)| t)
            .map(|(&p, &t)| (p, t))
    }

    /// The most recent prayer at or before `now` today, or `None` if `now`
    /// precedes the day's first time.
    pub fn current(&self, now: DateTime<Tz>) -> Option<(Prayer, DateTime<Tz>)> {
        self.times
            .iter()
            .filter(|(_, &t)| t <= now)
            .max_by_key(|(_, &t)| t)
            .map(|(&p, &t)| (p, t))
    }

    /// Times in chronological order, e.g. for rendering the panel list.
    pub fn ordered(&self) -> Vec<(Prayer, DateTime<Tz>)> {
        let mut v: Vec<(Prayer, DateTime<Tz>)> = self.times.iter().map(|(&p, &t)| (p, t)).collect();
        v.sort_by_key(|(_, t)| *t);
        v
    }

    /// Start of the Ishraq/Duha window: a fixed offset after sunrise (roughly the
    /// time the sun has risen "a spear's length" above the horizon). Ishraq is a
    /// voluntary prayer, so this is a derived display value, not one of the six
    /// computed `times`. Returns `None` if sunrise is undefined.
    pub fn ishraq(&self, offset_minutes: i64) -> Option<DateTime<Tz>> {
        self.get(Prayer::Sunrise)
            .map(|t| t + Duration::minutes(offset_minutes))
    }
}
