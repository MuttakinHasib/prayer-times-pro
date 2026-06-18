use crate::adapters::CalculationMethodAdapter;
use crate::coordinates::Coordinates;
use crate::high_latitude::HighLatitudeRule;
use crate::params::CalculationParameters;

/// Umm al-Qura University, Makkah. Fajr 18.5°, and Isha as a fixed 90 minutes
/// after Maghrib (not an angle). Standard across Saudi Arabia.
#[derive(Debug, Default, Clone, Copy)]
pub struct UmmAlQuraAdapter;

impl CalculationMethodAdapter for UmmAlQuraAdapter {
    fn id(&self) -> String {
        "ummalqura".into()
    }
    fn display_name(&self) -> String {
        "Umm al-Qura (Makkah)".into()
    }
    fn summary(&self) -> String {
        "Fajr 18.5°, Isha = Maghrib + 90 min.".into()
    }
    fn resolve(&self, _coordinates: Coordinates) -> CalculationParameters {
        CalculationParameters {
            isha_angle: None,
            isha_fixed_minutes: Some(90),
            high_latitude_rule: HighLatitudeRule::None,
            ..CalculationParameters::new(18.5)
        }
    }
}
