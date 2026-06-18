use std::collections::BTreeMap;

use crate::adapters::CalculationMethodAdapter;
use crate::coordinates::Coordinates;
use crate::high_latitude::HighLatitudeRule;
use crate::models::prayer::Prayer;
use crate::params::CalculationParameters;

/// Jabatan Kemajuan Islam Malaysia (Department of Islamic Development Malaysia) —
/// the official Malaysian method, as published by the JAKIM e-Solat service.
///
/// The parameters here are calibrated to reproduce e-Solat's own output, not the
/// "Fajr 20°/Isha 18°" preset other apps label "JAKIM" — that preset runs Fajr
/// ~11 minutes early against the real tables. Empirically (KL/WLY01, validated
/// across the solstices and equinoxes) e-Solat's Subuh behaves as a 17.5°
/// depression, Isyak as 18°, plus JAKIM's *ihtiyati* (safety) minutes added to
/// the post-noon prayers: Zohor +3, Asar +2, Maghrib +2, Isyak +2.
#[derive(Debug, Default, Clone, Copy)]
pub struct JakimAdapter;

impl CalculationMethodAdapter for JakimAdapter {
    fn id(&self) -> String {
        "jakim".into()
    }
    fn display_name(&self) -> String {
        "JAKIM (Malaysia)".into()
    }
    fn summary(&self) -> String {
        "Fajr 17.5°, Isha 18°, JAKIM ihtiyati. Matches e-Solat.".into()
    }
    fn resolve(&self, _coordinates: Coordinates) -> CalculationParameters {
        // No dedicated Maghrib/Isha offset field, so JAKIM's ihtiyati for the
        // evening prayers rides on manual_offsets. Safe: the per-prayer offset
        // editor is only exposed for the Manual method, never for built-ins.
        let manual_offsets = BTreeMap::from([(Prayer::Maghrib, 2), (Prayer::Isha, 2)]);
        CalculationParameters {
            isha_angle: Some(18.0),
            dhuhr_offset_minutes: 3,
            asr_offset_minutes: 2,
            manual_offsets,
            high_latitude_rule: HighLatitudeRule::AngleBased,
            ..CalculationParameters::new(17.5)
        }
    }
}
