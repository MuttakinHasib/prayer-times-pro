//! High-latitude resolution for Fajr/Isha where the sun may never reach the
//! required twilight depression angle. Clamps the time to a portion of the night
//! relative to sunrise/sunset. Operates on local clock-hour values; because it
//! works on *differences* from sunrise/sunset it is invariant to the timezone
//! offset applied later.

use serde::{Deserialize, Serialize};

/// Strategies for resolving Fajr/Isha (and the night-portion guards) at high
/// latitudes.
///
/// - `Automatic`: defer to the calculation method's own recommendation. This is
///   the user-facing default; the engine never receives it (the app resolves it
///   to a concrete rule first), and if it ever does it behaves like `None`.
/// - `None`: use the raw computed angle times; may be invalid in summer.
/// - `MiddleOfNight`: clamp Fajr/Isha to at least the night midpoint.
/// - `SeventhOfNight`: Fajr ≥ sunrise − night/7, Isha ≤ sunset + night/7.
/// - `AngleBased`: portion of night proportional to the twilight angle / 60.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HighLatitudeRule {
    Automatic,
    None,
    MiddleOfNight,
    SeventhOfNight,
    AngleBased,
}

impl HighLatitudeRule {
    /// Fraction of the night allotted to the twilight phase for `angle`.
    fn night_portion(self, angle: f64) -> f64 {
        match self {
            HighLatitudeRule::Automatic | HighLatitudeRule::None => 0.0,
            HighLatitudeRule::MiddleOfNight => 1.0 / 2.0,
            HighLatitudeRule::SeventhOfNight => 1.0 / 7.0,
            // Clamp to a whole night: a misconfigured (e.g. manual) angle above
            // 60° would otherwise push the clamp boundary past the actual night.
            // No-op for real twilight angles (15–20° → ≤0.33).
            HighLatitudeRule::AngleBased => (angle / 60.0).clamp(0.0, 1.0),
        }
    }

    /// Clamp `time` so it is no further than the allotted night portion from
    /// `base` (sunrise for Fajr, sunset for Isha). `before == true` for events
    /// that precede `base` (Fajr). A `NaN` input (angle never reached) is forced
    /// to the clamp boundary.
    pub(crate) fn clamp(self, time: f64, base: f64, angle: f64, night: f64, before: bool) -> f64 {
        // `Automatic` must be resolved to a concrete rule by the app layer before
        // reaching the engine. Surface that contract violation in dev/tests; fall
        // back to the safe `None` behaviour (return the raw time) in release.
        debug_assert!(
            self != HighLatitudeRule::Automatic,
            "Automatic high-latitude rule must be resolved to a concrete rule before the engine"
        );
        if self == HighLatitudeRule::None || self == HighLatitudeRule::Automatic {
            return time;
        }
        let portion = self.night_portion(angle) * night;
        let diff = if before { base - time } else { time - base };
        if time.is_nan() || diff > portion {
            return if before { base - portion } else { base + portion };
        }
        time
    }
}
