use crate::adapters::CalculationMethodAdapter;
use crate::coordinates::Coordinates;
use crate::high_latitude::HighLatitudeRule;
use crate::params::CalculationParameters;

/// Türkiye Diyanet İşleri Başkanlığı. The validated reference method: Fajr 18°,
/// Isha 17°, a −1.9° sunrise/maghrib horizon, shadow factor 1, and the official
/// ihtiyat (precaution) offsets of +5 min on Dhuhr and +4 min on Asr.
/// Reproduces the official Diyanet tables to within ±1 minute.
#[derive(Debug, Default, Clone, Copy)]
pub struct DiyanetAdapter;

impl CalculationMethodAdapter for DiyanetAdapter {
    fn id(&self) -> String {
        "diyanet".into()
    }
    fn display_name(&self) -> String {
        "Diyanet İşleri (Türkiye)".into()
    }
    fn summary(&self) -> String {
        "Fajr 18°, Isha 17°, horizon −1.9°, +5 min Dhuhr, +4 min Asr.".into()
    }
    fn resolve(&self, _coordinates: Coordinates) -> CalculationParameters {
        CalculationParameters {
            fajr_angle: 18.0,
            isha_angle: Some(17.0),
            sunrise_angle: -1.9,
            asr_shadow_factor: 1.0,
            dhuhr_offset_minutes: 5,
            asr_offset_minutes: 4,
            high_latitude_rule: HighLatitudeRule::None,
            ..CalculationParameters::new(18.0)
        }
    }
}
