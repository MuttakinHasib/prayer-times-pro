use crate::adapters::CalculationMethodAdapter;
use crate::coordinates::Coordinates;
use crate::high_latitude::HighLatitudeRule;
use crate::params::CalculationParameters;

/// Egyptian General Authority of Survey. Fajr 19.5°, Isha 17.5°, shadow factor 1.
#[derive(Debug, Default, Clone, Copy)]
pub struct EgyptianAdapter;

impl CalculationMethodAdapter for EgyptianAdapter {
    fn id(&self) -> String {
        "egyptian".into()
    }
    fn display_name(&self) -> String {
        "Egyptian General Authority of Survey".into()
    }
    fn summary(&self) -> String {
        "Fajr 19.5°, Isha 17.5°.".into()
    }
    fn resolve(&self, _coordinates: Coordinates) -> CalculationParameters {
        CalculationParameters {
            isha_angle: Some(17.5),
            high_latitude_rule: HighLatitudeRule::AngleBased,
            ..CalculationParameters::new(19.5)
        }
    }
}
