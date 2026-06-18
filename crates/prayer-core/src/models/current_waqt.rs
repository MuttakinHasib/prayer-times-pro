//! The prayer window currently in progress (for the "time left" countdown).

use chrono::DateTime;
use chrono_tz::Tz;

use crate::models::prayer::Prayer;
use crate::models::prayer_times::PrayerTimes;

/// The prayer window that is currently in progress, used for the "time left in
/// the current prayer" countdown (e.g. "Asr · 40m left") as opposed to the
/// "next prayer in X" countdown.
///
/// Each obligatory prayer's window runs until the next event in the day; Isha
/// runs until the following day's Fajr. The interval between `Sunrise` and
/// `Dhuhr` carries no obligatory prayer (it is the Duha/Ishraq period) — it is
/// represented with `prayer == Sunrise` and `is_obligatory() == false`, so
/// callers can fall back to a next-prayer display there.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurrentWaqt {
    /// The prayer whose window is active. `Sunrise` marks the post-sunrise gap
    /// (Duha/Ishraq), which has no obligatory prayer in progress.
    pub prayer: Prayer,
    /// The instant this window closes (the start of the next event).
    pub end: DateTime<Tz>,
}

impl CurrentWaqt {
    /// `false` during the sunrise→Dhuhr gap; `true` while an obligatory prayer's
    /// window is in progress.
    pub fn is_obligatory(&self) -> bool {
        self.prayer.is_obligatory()
    }

    /// Resolve the window containing `now`, spanning the day boundary so that
    /// Isha (until the next Fajr) and the after-midnight pre-Fajr stretch both
    /// resolve correctly. Returns `None` only when the required times are missing
    /// (e.g. a polar edge case left Fajr undefined).
    pub fn resolve(now: DateTime<Tz>, today: &PrayerTimes, tomorrow: &PrayerTimes) -> Option<CurrentWaqt> {
        // The day's boundaries in chronological order, each tagged with the
        // prayer whose window begins there.
        let order = [
            Prayer::Fajr,
            Prayer::Sunrise,
            Prayer::Dhuhr,
            Prayer::Asr,
            Prayer::Maghrib,
            Prayer::Isha,
        ];
        let boundaries: Vec<(Prayer, DateTime<Tz>)> =
            order.iter().filter_map(|&p| today.get(p).map(|t| (p, t))).collect();
        let first_fajr = today.get(Prayer::Fajr)?;

        // Before today's Fajr we are still inside yesterday's Isha, which ends at
        // today's Fajr. (We don't need yesterday's times — only the window end.)
        if now < first_fajr {
            return Some(CurrentWaqt {
                prayer: Prayer::Isha,
                end: first_fajr,
            });
        }

        // The last boundary at or before `now` owns the active window.
        let idx = boundaries.iter().rposition(|(_, t)| *t <= now)?;
        let active = boundaries[idx].0;

        // The window ends at the next boundary, or — for Isha — at tomorrow's Fajr.
        let end = if idx + 1 < boundaries.len() {
            boundaries[idx + 1].1
        } else {
            tomorrow.get(Prayer::Fajr)?
        };
        Some(CurrentWaqt { prayer: active, end })
    }
}
