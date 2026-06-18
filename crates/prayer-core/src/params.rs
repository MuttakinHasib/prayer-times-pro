//! The complete numeric contract the engine consumes.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::high_latitude::HighLatitudeRule;
use crate::models::prayer::Prayer;

/// The complete numeric contract the engine consumes. Everything Islam-specific
/// (twilight angles, shadow factors, method offsets) is expressed here so the
/// engine itself stays a pure astronomical calculator. Adapters produce these.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculationParameters {
    /// Sun depression below the horizon for Fajr, in degrees (e.g. 18.0).
    pub fajr_angle: f64,

    /// Sun depression below the horizon for Isha, in degrees. `None` when the
    /// method defines Isha as a fixed offset after Maghrib instead.
    #[serde(default)]
    pub isha_angle: Option<f64>,

    /// Fixed minutes after Maghrib for Isha (e.g. Umm al-Qura = 90). Mutually
    /// exclusive with `isha_angle`; when both are set, the fixed offset wins.
    #[serde(default)]
    pub isha_fixed_minutes: Option<i32>,

    /// Apparent solar altitude at sunrise/sunset, in degrees (negative = below
    /// horizon). Standard atmospheric refraction is −0.833; Diyanet uses −1.9.
    #[serde(default = "default_sunrise_angle")]
    pub sunrise_angle: f64,

    /// Asr shadow length factor: 1.0 = Shafi/Maliki/Hanbali/Diyanet, 2.0 = Hanafi.
    #[serde(default = "default_asr_shadow_factor")]
    pub asr_shadow_factor: f64,

    /// Minutes added to solar transit for Dhuhr (Diyanet ihtiyat = +5, others 0).
    #[serde(default)]
    pub dhuhr_offset_minutes: i32,

    /// Minutes added to the computed Asr time (Diyanet = +4, others 0).
    #[serde(default)]
    pub asr_offset_minutes: i32,

    /// Signed per-prayer fine-tuning in minutes, applied last. Absent keys = 0.
    #[serde(default)]
    pub manual_offsets: BTreeMap<Prayer, i32>,

    /// High-latitude resolution strategy for Fajr/Isha.
    #[serde(default = "default_high_latitude_rule")]
    pub high_latitude_rule: HighLatitudeRule,
}

fn default_sunrise_angle() -> f64 {
    -0.833
}
fn default_asr_shadow_factor() -> f64 {
    1.0
}
fn default_high_latitude_rule() -> HighLatitudeRule {
    HighLatitudeRule::None
}

impl CalculationParameters {
    /// Builder mirroring the Swift memberwise init with the same defaults.
    pub fn new(fajr_angle: f64) -> Self {
        Self {
            fajr_angle,
            isha_angle: None,
            isha_fixed_minutes: None,
            sunrise_angle: default_sunrise_angle(),
            asr_shadow_factor: default_asr_shadow_factor(),
            dhuhr_offset_minutes: 0,
            asr_offset_minutes: 0,
            manual_offsets: BTreeMap::new(),
            high_latitude_rule: default_high_latitude_rule(),
        }
    }
}
