use std::collections::BTreeMap;

use crate::adapters::CalculationMethodAdapter;
use crate::coordinates::Coordinates;
use crate::high_latitude::HighLatitudeRule;
use crate::models::prayer::Prayer;
use crate::params::CalculationParameters;

/// Kementerian Agama Republik Indonesia (Kemenag / SIHAT) — the official
/// Indonesian method, as published by Kemenag's prayer-time tables.
///
/// Kemenag uses Fajr (Subuh) 20° and Isha (Isya) 18°, Shafiʿi Asr, plus
/// *ihtiyati* (safety) minutes added to each prayer. Calibrated against
/// Kemenag's published tables for KOTA JAKARTA: Subuh +2, Dzuhur +3, Ashar +2,
/// Maghrib +3, Isya +2 (given the engine's nearest-minute rounding). Kemenag also
/// defines Imsak = Subuh − 10, which this app does not surface.
#[derive(Debug, Default, Clone, Copy)]
pub struct KemenagAdapter;

impl CalculationMethodAdapter for KemenagAdapter {
    fn id(&self) -> String {
        "kemenag".into()
    }
    fn display_name(&self) -> String {
        "Kemenag (Indonesia)".into()
    }
    fn summary(&self) -> String {
        "Fajr 20°, Isha 18°, Kemenag ihtiyati.".into()
    }
    fn resolve(&self, _coordinates: Coordinates) -> CalculationParameters {
        // No dedicated Subuh/Maghrib/Isya offset field, so the remaining ihtiyati
        // rides on manual_offsets. Safe: the per-prayer offset editor is only
        // exposed for the Manual method, never for built-ins.
        let manual_offsets =
            BTreeMap::from([(Prayer::Fajr, 2), (Prayer::Maghrib, 3), (Prayer::Isha, 2)]);
        CalculationParameters {
            isha_angle: Some(18.0),
            dhuhr_offset_minutes: 3,
            asr_offset_minutes: 2,
            manual_offsets,
            high_latitude_rule: HighLatitudeRule::AngleBased,
            ..CalculationParameters::new(20.0)
        }
    }
}
