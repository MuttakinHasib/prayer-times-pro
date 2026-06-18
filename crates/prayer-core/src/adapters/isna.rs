use crate::adapters::CalculationMethodAdapter;
use crate::coordinates::Coordinates;
use crate::high_latitude::HighLatitudeRule;
use crate::params::CalculationParameters;

/// Islamic Society of North America. Fajr 15°, Isha 15°, shadow factor 1. Common
/// across the US and Canada.
#[derive(Debug, Default, Clone, Copy)]
pub struct IsnaAdapter;

impl CalculationMethodAdapter for IsnaAdapter {
    fn id(&self) -> String {
        "isna".into()
    }
    fn display_name(&self) -> String {
        "Islamic Society of North America".into()
    }
    fn summary(&self) -> String {
        "Fajr 15°, Isha 15°. North America.".into()
    }
    fn resolve(&self, _coordinates: Coordinates) -> CalculationParameters {
        CalculationParameters {
            isha_angle: Some(15.0),
            high_latitude_rule: HighLatitudeRule::AngleBased,
            ..CalculationParameters::new(15.0)
        }
    }
}
