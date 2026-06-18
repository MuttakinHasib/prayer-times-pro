//! Low-precision solar position (declination + equation of time) good to well
//! under the ±1-minute accuracy this app requires. Based on the U.S. Naval
//! Observatory "Approximate Solar Coordinates" algorithm, the same one used by
//! PrayTimes.org. Inputs and outputs are in degrees / hours.

use crate::degree_math as dm;

/// Solar position at a given Julian Date.
pub(crate) struct Position {
    /// Sun declination, degrees.
    pub declination: f64,
    /// Equation of time, hours (apparent − mean solar time).
    pub equation_of_time: f64,
}

/// Solar position at the given Julian Date.
pub(crate) fn position(julian_date: f64) -> Position {
    let d = julian_date - 2451545.0; // days since J2000.0
    let g = dm::fix_angle(357.529 + 0.98560028 * d); // mean anomaly
    let q = dm::fix_angle(280.459 + 0.98564736 * d); // mean longitude
    let l = dm::fix_angle(q + 1.915 * dm::sin(g) + 0.020 * dm::sin(2.0 * g)); // apparent longitude
    let e = 23.439 - 0.00000036 * d; // obliquity of ecliptic

    let declination = dm::asin(dm::sin(e) * dm::sin(l));
    let right_ascension = dm::fix_hour(dm::atan2(dm::cos(e) * dm::sin(l), dm::cos(l)) / 15.0);
    let equation_of_time = q / 15.0 - right_ascension;
    Position {
        declination,
        equation_of_time,
    }
}

/// Julian Date for a Gregorian calendar day at 00:00 UTC.
pub(crate) fn julian_date(year: i32, month: i32, day: f64) -> f64 {
    let mut y = year;
    let mut m = month;
    if m <= 2 {
        y -= 1;
        m += 12;
    }
    let a = (y as f64 / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();
    (365.25 * (y as f64 + 4716.0)).floor()
        + (30.6001 * (m as f64 + 1.0)).floor()
        + day
        + b
        - 1524.5
}
