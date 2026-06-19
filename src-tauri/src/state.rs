//! Live prayer-time state: holds the runtime config + today/tomorrow times,
//! recomputes on day rollover, and derives the tray label + the frontend DTO.
//!
//! M2 uses a hardcoded [`AppConfig`]; M3 replaces it with the persisted
//! `AppSettings` loaded from `tauri-plugin-store`.

use std::sync::Mutex;

use chrono::{DateTime, Duration, NaiveDate, Utc};
use chrono_tz::Tz;
use prayer_core::{
    calculate, CalculationMethodAdapter, Coordinates, CurrentWaqt, MethodRegistry, Prayer,
    PrayerTimes,
};
use serde::Serialize;

/// Stable lowercase key for a prayer, matching the `prayer-core` serde encoding
/// and the frontend's expectations.
fn prayer_key(p: Prayer) -> &'static str {
    match p {
        Prayer::Fajr => "fajr",
        Prayer::Sunrise => "sunrise",
        Prayer::Dhuhr => "dhuhr",
        Prayer::Asr => "asr",
        Prayer::Maghrib => "maghrib",
        Prayer::Isha => "isha",
    }
}

/// English display name (M6 moves naming to frontend i18n).
fn prayer_name(p: Prayer) -> &'static str {
    match p {
        Prayer::Fajr => "Fajr",
        Prayer::Sunrise => "Sunrise",
        Prayer::Dhuhr => "Dhuhr",
        Prayer::Asr => "Asr",
        Prayer::Maghrib => "Maghrib",
        Prayer::Isha => "Isha",
    }
}

/// Compact relative countdown for the tray label: "3h 25m", "25m", or "45s".
fn short_countdown(seconds: i64) -> String {
    let total = seconds.max(0);
    let (h, m, s) = (total / 3600, (total % 3600) / 60, total % 60);
    if h > 0 {
        format!("{h}h {m}m")
    } else if m > 0 {
        format!("{m}m")
    } else {
        format!("{s}s")
    }
}

/// Runtime configuration for the engine + display. M2 placeholder; M3 sources
/// this from persisted `AppSettings`.
#[derive(Clone)]
pub struct AppConfig {
    pub method_id: String,
    pub hanafi_asr: bool,
    pub coordinates: Coordinates,
    pub tz: Tz,
    pub show_ishraq: bool,
    pub show_hijri: bool,
    pub hijri_adjustment: i32,
}

impl Default for AppConfig {
    fn default() -> Self {
        // System timezone, falling back to UTC. A sensible default location/method
        // (Dhaka, Karachi + Hanafi) so the panel is meaningful before settings and
        // location land in M3/M5.
        let tz: Tz = iana_time_zone::get_timezone()
            .ok()
            .and_then(|id| id.parse().ok())
            .unwrap_or(chrono_tz::UTC);
        Self {
            method_id: "karachi".into(),
            hanafi_asr: true,
            coordinates: Coordinates::new(23.8103, 90.4125),
            tz,
            show_ishraq: false,
            show_hijri: true,
            hijri_adjustment: 0,
        }
    }
}

/// Serializable snapshot the frontend renders. All instants are epoch
/// milliseconds; the frontend formats clocks/dates with `Intl` in `tz` and runs
/// its own 1 Hz countdown from `next.at_ms`.
#[derive(Serialize, Clone)]
pub struct PrayerState {
    pub tz: String,
    pub now_ms: i64,
    pub method_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub show_ishraq: bool,
    pub show_hijri: bool,
    pub hijri_adjustment: i32,
    pub ishraq_ms: Option<i64>,
    pub next: Option<PrayerInstant>,
    pub current_waqt: Option<WaqtDto>,
    pub times: Vec<PrayerInstant>,
}

#[derive(Serialize, Clone)]
pub struct PrayerInstant {
    pub prayer: &'static str,
    pub at_ms: i64,
}

#[derive(Serialize, Clone)]
pub struct WaqtDto {
    pub prayer: &'static str,
    pub end_ms: i64,
    pub is_obligatory: bool,
}

/// The live clock: today/tomorrow times,
/// recompute on rollover, derived next/countdown.
pub struct Clock {
    pub config: AppConfig,
    today: PrayerTimes,
    tomorrow: PrayerTimes,
    now: DateTime<Tz>,
    last_day: NaiveDate,
    /// Last signature pushed to the frontend, to throttle `state-changed` events.
    emitted: Option<(NaiveDate, &'static str)>,
}

impl Clock {
    pub fn new(config: AppConfig) -> Self {
        let now = Utc::now().with_timezone(&config.tz);
        let today = Self::compute(&config, now, 0);
        let tomorrow = Self::compute(&config, now, 1);
        Self {
            last_day: now.date_naive(),
            config,
            today,
            tomorrow,
            now,
            emitted: None,
        }
    }

    fn compute(config: &AppConfig, now: DateTime<Tz>, offset_days: i64) -> PrayerTimes {
        let params = MethodRegistry::resolve(&config.method_id, config.hanafi_asr, None)
            .map(|a| a.resolve(config.coordinates))
            .unwrap_or_else(|| prayer_core::MwlAdapter.resolve(config.coordinates));
        let day = (now + Duration::days(offset_days)).date_naive();
        calculate(day, config.coordinates, &params, config.tz)
    }

    fn method_name(&self) -> String {
        MethodRegistry::resolve(&self.config.method_id, self.config.hanafi_asr, None)
            .map(|a| a.display_name())
            .unwrap_or_else(|| "Muslim World League".into())
    }

    /// Advance to `now_utc`, recomputing today/tomorrow on a civil-day rollover.
    /// Returns `true` when the schedule was recomputed.
    pub fn tick(&mut self, now_utc: DateTime<Utc>) -> bool {
        self.now = now_utc.with_timezone(&self.config.tz);
        let day = self.now.date_naive();
        if day != self.last_day {
            self.last_day = day;
            self.today = Self::compute(&self.config, self.now, 0);
            self.tomorrow = Self::compute(&self.config, self.now, 1);
            true
        } else {
            false
        }
    }

    /// Recompute immediately (e.g. after a config change). Wired in M3 when
    /// settings can change at runtime; defined now so the clock API is complete.
    #[allow(dead_code)]
    pub fn recompute(&mut self) {
        self.today = Self::compute(&self.config, self.now, 0);
        self.tomorrow = Self::compute(&self.config, self.now, 1);
        self.last_day = self.now.date_naive();
        // Force the next `should_emit` to push: a config change can leave the
        // (day, next-prayer) signature unchanged while the actual times moved.
        self.emitted = None;
    }

    /// The upcoming prayer: next today, else tomorrow's Fajr.
    fn next_event(&self) -> Option<(Prayer, DateTime<Tz>)> {
        self.today.next(self.now).or_else(|| self.tomorrow.next(self.now))
    }

    fn seconds_until_next(&self) -> i64 {
        self.next_event()
            .map(|(_, t)| (t - self.now).num_seconds().max(0))
            .unwrap_or(0)
    }

    /// Compact tray label, e.g. "Fajr in 5h 38m". (Tray icon carries the glyph.)
    pub fn tray_label(&self) -> String {
        match self.next_event() {
            Some((p, _)) => format!("{} in {}", prayer_name(p), short_countdown(self.seconds_until_next())),
            None => "Prayer Times".into(),
        }
    }

    /// A signature that changes when the rendered state must refresh (day rolled
    /// over or the next prayer changed).
    fn signature(&self) -> (NaiveDate, &'static str) {
        let next = self.next_event().map(|(p, _)| prayer_key(p)).unwrap_or("none");
        (self.last_day, next)
    }

    /// `true` when the rendered state changed since the last emit (and records
    /// the new signature). Used by the tick loop to throttle `state-changed`.
    pub fn should_emit(&mut self) -> bool {
        let sig = self.signature();
        if self.emitted == Some(sig) {
            false
        } else {
            self.emitted = Some(sig);
            true
        }
    }

    /// Build the frontend DTO.
    pub fn snapshot(&self) -> PrayerState {
        let next = self
            .next_event()
            .map(|(p, t)| PrayerInstant { prayer: prayer_key(p), at_ms: t.timestamp_millis() });
        let current_waqt = CurrentWaqt::resolve(self.now, &self.today, &self.tomorrow).map(|w| {
            WaqtDto {
                prayer: prayer_key(w.prayer),
                end_ms: w.end.timestamp_millis(),
                is_obligatory: w.is_obligatory(),
            }
        });
        let times = self
            .today
            .ordered()
            .into_iter()
            .map(|(p, t)| PrayerInstant { prayer: prayer_key(p), at_ms: t.timestamp_millis() })
            .collect();
        PrayerState {
            tz: self.config.tz.name().to_string(),
            now_ms: self.now.timestamp_millis(),
            method_name: self.method_name(),
            latitude: self.config.coordinates.latitude,
            longitude: self.config.coordinates.longitude,
            show_ishraq: self.config.show_ishraq,
            show_hijri: self.config.show_hijri,
            hijri_adjustment: self.config.hijri_adjustment,
            ishraq_ms: self.today.ishraq(15).map(|t| t.timestamp_millis()),
            next,
            current_waqt,
            times,
        }
    }
}

/// Shared clock guarded for cross-thread access from the tick loop and commands.
pub type SharedClock = Mutex<Clock>;
