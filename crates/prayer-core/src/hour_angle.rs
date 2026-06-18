//! Converts a target sun altitude (or Asr shadow factor) into a time offset from
//! solar noon, given latitude and declination. All times are hours; "before
//! noon" callers negate the returned half-arc. Polar cases where the sun never
//! reaches the angle return `None`.

use crate::degree_math as dm;

/// Hours between solar noon and the moment the sun sits at `angle` degrees
/// below the horizon (positive `angle` = below). `None` when the altitude is
/// never reached at this latitude/declination (polar day or night).
pub(crate) fn hour_angle(altitude_below_horizon: f64, latitude: f64, declination: f64) -> Option<f64> {
    let numerator = -dm::sin(altitude_below_horizon) - dm::sin(latitude) * dm::sin(declination);
    let denominator = dm::cos(latitude) * dm::cos(declination);
    let cos_h = numerator / denominator;
    if !(-1.0..=1.0).contains(&cos_h) {
        return None;
    }
    Some(dm::acos(cos_h) / 15.0)
}

/// Hours from solar noon to Asr, for the given shadow factor (1 = Standard,
/// 2 = Hanafi). The Asr altitude is derived from the noon shadow length.
pub(crate) fn asr_hour_angle(shadow_factor: f64, latitude: f64, declination: f64) -> Option<f64> {
    // Sun altitude at Asr (above the horizon, so positive):
    //   α = acot(factor + tan(|lat − decl|)).
    let altitude = dm::acot(shadow_factor + dm::tan((latitude - declination).abs()));
    // The horizon formula takes depression below the horizon = −altitude.
    hour_angle(-altitude, latitude, declination)
}
