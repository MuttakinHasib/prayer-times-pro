//! Live prayer-time state: holds the settings + today/tomorrow times, recomputes
//! on day rollover or a settings change, and derives the tray label + frontend DTO.

use std::sync::Mutex;

use chrono::{DateTime, Duration, NaiveDate, Timelike, Utc};
use chrono_tz::Tz;
use prayer_core::{
    calculate, AppSettings, CalculationMethodAdapter, CalculationMode, Coordinates, CurrentWaqt,
    HighLatitudeRule, MenuBarCountdownMode, MenuBarStyle, MethodRegistry, MwlAdapter, Prayer,
    PrayerTimes,
};
use serde::Serialize;

/// Fallback location until the user sets one (Location & Time tab) / auto-detect (M5).
const DEFAULT_COORDINATES: Coordinates = Coordinates {
    latitude: 23.8103,
    longitude: 90.4125,
    elevation: 0.0,
};

fn system_tz() -> Tz {
    iana_time_zone::get_timezone()
        .ok()
        .and_then(|id| id.parse().ok())
        .unwrap_or(chrono_tz::UTC)
}

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

/// Short wall-clock for the tray label, e.g. "4:35 AM".
fn clock_label(dt: DateTime<Tz>) -> String {
    let (pm, hour12) = dt.hour12();
    format!("{hour12}:{:02} {}", dt.minute(), if pm { "PM" } else { "AM" })
}

/// Serializable snapshot the frontend renders. All instants are epoch
/// milliseconds; the frontend formats clocks/dates with `Intl` in `tz`.
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

/// The live clock: today/tomorrow times, recompute on rollover, derived next/countdown.
pub struct Clock {
    settings: AppSettings,
    today: PrayerTimes,
    tomorrow: PrayerTimes,
    now: DateTime<Tz>,
    last_day: NaiveDate,
    emitted: Option<(NaiveDate, &'static str)>,
}

impl Clock {
    pub fn new(settings: AppSettings) -> Self {
        let tz = resolved_tz(&settings);
        let now = Utc::now().with_timezone(&tz);
        let today = compute(&settings, now, 0);
        let tomorrow = compute(&settings, now, 1);
        Self {
            last_day: now.date_naive(),
            settings,
            today,
            tomorrow,
            now,
            emitted: None,
        }
    }

    pub fn settings(&self) -> AppSettings {
        self.settings.clone()
    }

    /// Replace the settings and recompute. Returns nothing; callers re-emit state.
    pub fn set_settings(&mut self, settings: AppSettings) {
        self.settings = settings;
        self.now = Utc::now().with_timezone(&resolved_tz(&self.settings));
        self.recompute();
        self.emitted = None; // force the next emit after a settings change
    }

    fn recompute(&mut self) {
        self.today = compute(&self.settings, self.now, 0);
        self.tomorrow = compute(&self.settings, self.now, 1);
        self.last_day = self.now.date_naive();
    }

    fn method_name(&self) -> String {
        MethodRegistry::resolve(&self.settings.method_id, self.settings.hanafi_asr, None)
            .map(|a| a.display_name())
            .unwrap_or_else(|| "Muslim World League".into())
    }

    /// Advance to `now_utc`, recomputing on a civil-day rollover.
    pub fn tick(&mut self, now_utc: DateTime<Utc>) {
        self.now = now_utc.with_timezone(&resolved_tz(&self.settings));
        let day = self.now.date_naive();
        if day != self.last_day {
            self.recompute();
        }
    }

    fn next_event(&self) -> Option<(Prayer, DateTime<Tz>)> {
        self.today.next(self.now).or_else(|| self.tomorrow.next(self.now))
    }

    fn seconds_until_next(&self) -> i64 {
        self.next_event()
            .map(|(_, t)| (t - self.now).num_seconds().max(0))
            .unwrap_or(0)
    }

    /// Tray label per the configured [`MenuBarStyle`] / countdown mode.
    pub fn tray_label(&self) -> String {
        let Some((prayer, time)) = self.next_event() else {
            return "Prayer Times".into();
        };
        let style = self.settings.menu_bar_style;
        let name = prayer_name(prayer);

        // The icon is the tray image; here we only build the text portion.
        let value = match style {
            MenuBarStyle::IconOnly => String::new(),
            MenuBarStyle::NextPrayerClock | MenuBarStyle::IconNameClock => clock_label(time),
            _ => self.countdown_text(prayer),
        };
        let shows_name = matches!(
            style,
            MenuBarStyle::NextPrayerCountdown
                | MenuBarStyle::IconNameCountdown
                | MenuBarStyle::NextPrayerClock
                | MenuBarStyle::IconNameClock
        );
        match (shows_name, value.is_empty()) {
            (_, true) if shows_name => name.to_string(),
            (true, _) => format!("{name} {value}"),
            (false, true) => String::new(),
            (false, _) => value,
        }
    }

    /// "in 1h 24m", or "40m left" in current-waqt mode while an obligatory prayer runs.
    fn countdown_text(&self, next: Prayer) -> String {
        if self.settings.menu_bar_countdown_mode == MenuBarCountdownMode::CurrentWaqt {
            if let Some(waqt) = CurrentWaqt::resolve(self.now, &self.today, &self.tomorrow) {
                if waqt.is_obligatory() {
                    let left = (waqt.end - self.now).num_seconds().max(0);
                    return format!("{} left", short_countdown(left));
                }
            }
        }
        let _ = next;
        format!("in {}", short_countdown(self.seconds_until_next()))
    }

    pub fn signature(&self) -> (NaiveDate, &'static str) {
        let next = self.next_event().map(|(p, _)| prayer_key(p)).unwrap_or("none");
        (self.last_day, next)
    }

    pub fn should_emit(&mut self) -> bool {
        let sig = self.signature();
        if self.emitted == Some(sig) {
            false
        } else {
            self.emitted = Some(sig);
            true
        }
    }

    pub fn snapshot(&self) -> PrayerState {
        let next = self
            .next_event()
            .map(|(p, t)| PrayerInstant { prayer: prayer_key(p), at_ms: t.timestamp_millis() });
        let current_waqt =
            CurrentWaqt::resolve(self.now, &self.today, &self.tomorrow).map(|w| WaqtDto {
                prayer: prayer_key(w.prayer),
                end_ms: w.end.timestamp_millis(),
                is_obligatory: w.is_obligatory(),
            });
        let times = self
            .today
            .ordered()
            .into_iter()
            .map(|(p, t)| PrayerInstant { prayer: prayer_key(p), at_ms: t.timestamp_millis() })
            .collect();
        PrayerState {
            tz: resolved_tz(&self.settings).name().to_string(),
            now_ms: self.now.timestamp_millis(),
            method_name: self.method_name(),
            latitude: resolved_coordinates(&self.settings).latitude,
            longitude: resolved_coordinates(&self.settings).longitude,
            show_ishraq: self.settings.show_ishraq_time,
            show_hijri: self.settings.show_hijri_date,
            hijri_adjustment: self.settings.hijri_day_adjustment,
            ishraq_ms: self.today.ishraq(15).map(|t| t.timestamp_millis()),
            next,
            current_waqt,
            times,
        }
    }
}

fn resolved_coordinates(settings: &AppSettings) -> Coordinates {
    // Auto-detected location lands in M5; until then use the manual coordinates.
    settings.manual_coordinates.unwrap_or(DEFAULT_COORDINATES)
}

fn resolved_tz(settings: &AppSettings) -> Tz {
    settings
        .timezone_override
        .as_deref()
        .and_then(|id| id.parse().ok())
        .unwrap_or_else(system_tz)
}

fn resolved_params(settings: &AppSettings, coords: Coordinates) -> prayer_core::CalculationParameters {
    let mut params = MethodRegistry::resolve(
        &settings.method_id,
        settings.hanafi_asr,
        settings.manual_parameters.clone(),
    )
    .map(|a| a.resolve(coords))
    .unwrap_or_else(|| MwlAdapter.resolve(coords));

    // The user's explicit high-latitude rule wins; `Automatic` keeps the method's
    // recommended rule (the engine never sees `Automatic`).
    if settings.high_latitude_rule != HighLatitudeRule::Automatic {
        params.high_latitude_rule = settings.high_latitude_rule;
    }
    params
}

fn compute(settings: &AppSettings, now: DateTime<Tz>, offset_days: i64) -> PrayerTimes {
    let tz = resolved_tz(settings);
    let coords = resolved_coordinates(settings);
    let params = resolved_params(settings, coords);
    let day = (now + Duration::days(offset_days)).date_naive();
    let astronomical = calculate(day, coords, &params, tz);

    if settings.calculation_mode != CalculationMode::Manual {
        return astronomical;
    }
    apply_jamaat(astronomical, settings, day, tz)
}

/// Replace the five obligatory times with the fixed jamaat schedule (minutes since
/// local midnight), keeping astronomical Sunrise/Ishraq.
fn apply_jamaat(
    astronomical: PrayerTimes,
    settings: &AppSettings,
    day: NaiveDate,
    tz: Tz,
) -> PrayerTimes {
    use chrono::TimeZone;
    let midnight = tz
        .from_local_datetime(&day.and_hms_opt(0, 0, 0).expect("valid midnight"))
        .earliest();
    let Some(midnight) = midnight else {
        return astronomical;
    };

    let mut times = astronomical.times.clone();
    for prayer in Prayer::OBLIGATORY {
        if let Some(&minutes) = settings.jamaat_times.get(&prayer) {
            times.insert(prayer, midnight + Duration::minutes(minutes as i64));
        }
    }
    PrayerTimes::new(astronomical.date, times)
}

/// Shared clock guarded for cross-thread access from the tick loop and commands.
pub type SharedClock = Mutex<Clock>;
