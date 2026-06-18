//! Shared helpers for the prayer-core test suite.
//!
//! Each integration-test file compiles `common` into its own binary, so a helper
//! used by only some suites reads as dead code in the others — hence the
//! module-wide allow rather than per-item attributes (which would conflict in the
//! binary that *does* use the helper).
#![allow(dead_code)]

use chrono::{DateTime, NaiveDate, Timelike};
use chrono_tz::Tz;

/// Parse an IANA timezone identifier, panicking on an unknown zone.
pub fn tz(id: &str) -> Tz {
    id.parse().unwrap_or_else(|_| panic!("Unknown timezone identifier: {id}"))
}

/// Build a Gregorian civil date.
pub fn date(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).expect("valid date")
}

/// Minutes-since-midnight of an instant in its own timezone (truncating seconds).
pub fn minutes_of_day(dt: DateTime<Tz>) -> i64 {
    dt.hour() as i64 * 60 + dt.minute() as i64
}

/// Minutes-since-midnight, rounding to the nearest minute (≥30 s rounds up).
pub fn minutes_of_day_rounded(dt: DateTime<Tz>) -> i64 {
    minutes_of_day(dt) + if dt.second() >= 30 { 1 } else { 0 }
}

/// Parse "HH:MM" to minutes since midnight.
pub fn hm(s: &str) -> i64 {
    let (h, m) = s.split_once(':').expect("HH:MM");
    h.parse::<i64>().unwrap() * 60 + m.parse::<i64>().unwrap()
}
