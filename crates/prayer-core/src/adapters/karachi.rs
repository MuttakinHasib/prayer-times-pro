use crate::adapters::CalculationMethodAdapter;
use crate::coordinates::Coordinates;
use crate::high_latitude::HighLatitudeRule;
use crate::params::CalculationParameters;

/// University of Islamic Sciences, Karachi. Fajr 18°, Isha 18°, shadow factor 1.
/// Common across Pakistan, India, Bangladesh, and Afghanistan.
#[derive(Debug, Default, Clone, Copy)]
pub struct KarachiAdapter;

impl CalculationMethodAdapter for KarachiAdapter {
    fn id(&self) -> String {
        "karachi".into()
    }
    fn display_name(&self) -> String {
        "University of Islamic Sciences, Karachi".into()
    }
    fn summary(&self) -> String {
        "Fajr 18°, Isha 18°.".into()
    }
    fn resolve(&self, _coordinates: Coordinates) -> CalculationParameters {
        CalculationParameters {
            isha_angle: Some(18.0),
            high_latitude_rule: HighLatitudeRule::AngleBased,
            ..CalculationParameters::new(18.0)
        }
    }
}
