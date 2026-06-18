//! The pure calculation core. Given a civil date, a location, fully-resolved
//! `CalculationParameters` (produced by an adapter), and a timezone, it returns
//! the six absolute prayer instants. No UI, no I/O, no Islam-specific constants
//! — every method-specific value arrives via `params`.

use std::collections::BTreeMap;

use chrono::{DateTime, Datelike, Duration, NaiveDate, Offset, TimeZone};
use chrono_tz::Tz;

use crate::coordinates::Coordinates;
use crate::hour_angle;
use crate::models::{prayer::Prayer, prayer_times::PrayerTimes};
use crate::params::CalculationParameters;
use crate::solar;

/// One refinement pass holds the six events as fractional local clock-hours.
#[derive(Clone, Copy)]
struct RawHours {
    fajr: f64,
    sunrise: f64,
    dhuhr: f64,
    asr: f64,
    maghrib: f64,
    isha: f64,
}

/// Compute the day's prayer times.
///
/// - `date`: the civil calendar day (year/month/day) the result belongs to.
/// - `coordinates`: latitude/longitude/elevation of the observer.
/// - `params`: resolved twilight angles, shadow factor, offsets, high-lat rule.
/// - `tz`: the timezone the returned instants and the calendar day use.
pub fn calculate(
    date: NaiveDate,
    coordinates: Coordinates,
    params: &CalculationParameters,
    tz: Tz,
) -> PrayerTimes {
    let year = date.year();
    let month = date.month() as i32;
    let day = date.day() as i32;

    let lat = coordinates.latitude;
    let lng = coordinates.longitude;

    // Midnight (00:00) of the civil date in `tz`, as an absolute instant.
    let naive_midnight = date
        .and_hms_opt(0, 0, 0)
        .expect("00:00 is always valid for a real date");
    let midnight: DateTime<Tz> = tz
        .from_local_datetime(&naive_midnight)
        .earliest()
        .unwrap_or_else(|| tz.from_utc_datetime(&naive_midnight));

    // The UTC offset at local midnight, in hours — matching the Swift engine's
    // `secondsFromGMT(for: midnight)`. Sampling at midnight (not noon) keeps
    // DST-transition behaviour identical to the reference implementation.
    let tz_hours = midnight.offset().fix().local_minus_utc() as f64 / 3600.0;

    let jd0 = solar::julian_date(year, month, day as f64);

    // Sunrise/sunset horizon dip (positive degrees below horizon). `sunrise_angle`
    // is stored as an altitude (negative below horizon), so the dip is its
    // negation. We deliberately do NOT add an elevation term: the official
    // Diyanet horizon (−1.9°) is a flat validated constant, and standard methods
    // specify their dip (−0.833°) directly. Adding 0.0347·√elevation over-lengthens
    // the day at altitude and breaks the ±1-minute golden-table gate.
    let dip = -params.sunrise_angle;

    // Iterate: each event's solar position is evaluated at its own approximate
    // time. Three passes from sensible seeds converge well within a second of arc.
    let mut h = RawHours {
        fajr: 5.0,
        sunrise: 6.0,
        dhuhr: 12.0,
        asr: 13.0,
        maghrib: 18.0,
        isha: 18.0,
    };
    for _ in 0..3 {
        h = compute_pass(&h, jd0, lat, lng, tz_hours, dip, params);
    }

    // High-latitude adjustment (Fajr always; Isha only when angle-based).
    let rule = params.high_latitude_rule;
    if !h.sunrise.is_nan() && !h.maghrib.is_nan() {
        let night = (24.0 - h.maghrib) + h.sunrise; // sunset → next sunrise
        h.fajr = rule.clamp(h.fajr, h.sunrise, params.fajr_angle, night, true);
        if let Some(isha_angle) = params.isha_angle {
            h.isha = rule.clamp(h.isha, h.maghrib, isha_angle, night, false);
        }
    }

    // Method offsets.
    h.dhuhr += params.dhuhr_offset_minutes as f64 / 60.0;
    h.asr += params.asr_offset_minutes as f64 / 60.0;

    // Fixed-offset Isha (e.g. Umm al-Qura: Maghrib + 90).
    if let Some(fixed) = params.isha_fixed_minutes {
        h.isha = h.maghrib + fixed as f64 / 60.0;
    }

    // Per-prayer manual fine-tuning, applied last, then round to the nearest
    // whole minute. Published prayer tables are minute-granular and round rather
    // than truncate; returning the rounded instant keeps the displayed clock, the
    // notification fire time, and the countdown all on the same minute boundary.
    let instant = |hours: f64, prayer: Prayer| -> Option<DateTime<Tz>> {
        let tuned = hours + (*params.manual_offsets.get(&prayer).unwrap_or(&0)) as f64 / 60.0;
        if !tuned.is_finite() {
            return None;
        }
        let minutes = (tuned * 60.0).round();
        Some(midnight + Duration::seconds((minutes * 60.0) as i64))
    };

    let mut times: BTreeMap<Prayer, DateTime<Tz>> = BTreeMap::new();
    for (prayer, hours) in [
        (Prayer::Fajr, h.fajr),
        (Prayer::Sunrise, h.sunrise),
        (Prayer::Dhuhr, h.dhuhr),
        (Prayer::Asr, h.asr),
        (Prayer::Maghrib, h.maghrib),
        (Prayer::Isha, h.isha),
    ] {
        if let Some(t) = instant(hours, prayer) {
            times.insert(prayer, t);
        }
    }

    PrayerTimes::new(midnight, times)
}

/// One refinement pass: recompute every event using the solar position at its
/// current estimated time.
fn compute_pass(
    guess: &RawHours,
    jd0: f64,
    lat: f64,
    lng: f64,
    tz: f64,
    dip: f64,
    params: &CalculationParameters,
) -> RawHours {
    // Local clock noon for an event whose solar position is sampled at `t`.
    let noon = |t: f64| -> f64 {
        let pos = solar::position(jd0 + (t - tz) / 24.0);
        12.0 - lng / 15.0 - pos.equation_of_time + tz
    };
    let declination = |t: f64| -> f64 { solar::position(jd0 + (t - tz) / 24.0).declination };
    // Time of an event at depression `angle`, `before` or after local noon.
    let angle_time = |angle: f64, t: f64, before: bool| -> f64 {
        let n = noon(t);
        match hour_angle::hour_angle(angle, lat, declination(t)) {
            Some(ha) => {
                if before {
                    n - ha
                } else {
                    n + ha
                }
            }
            None => f64::NAN,
        }
    };

    let dhuhr = noon(guess.dhuhr);
    let sunrise = angle_time(dip, guess.sunrise, true);
    let maghrib = angle_time(dip, guess.maghrib, false);
    let fajr = angle_time(params.fajr_angle, guess.fajr, true);
    let isha = match params.isha_angle {
        Some(isha_angle) => angle_time(isha_angle, guess.isha, false),
        None => guess.isha,
    };
    let asr = {
        let n = noon(guess.asr);
        match hour_angle::asr_hour_angle(params.asr_shadow_factor, lat, declination(guess.asr)) {
            Some(ha) => n + ha,
            None => f64::NAN,
        }
    };

    RawHours {
        fajr,
        sunrise,
        dhuhr,
        asr,
        maghrib,
        isha,
    }
}
