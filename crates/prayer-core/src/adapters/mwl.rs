use crate::adapters::CalculationMethodAdapter;
use crate::coordinates::Coordinates;
use crate::high_latitude::HighLatitudeRule;
use crate::params::CalculationParameters;

/// Muslim World League. Fajr 18°, Isha 17°, shadow factor 1. The sensible global
/// default and the fallback for unknown countries; uses the angle-based high-lat
/// rule which suits northern Europe.
#[derive(Debug, Default, Clone, Copy)]
pub struct MwlAdapter;

impl CalculationMethodAdapter for MwlAdapter {
    fn id(&self) -> String {
        "mwl".into()
    }
    fn display_name(&self) -> String {
        "Muslim World League".into()
    }
    fn summary(&self) -> String {
        "Fajr 18°, Isha 17°. Global default.".into()
    }
    fn resolve(&self, _coordinates: Coordinates) -> CalculationParameters {
        CalculationParameters {
            isha_angle: Some(17.0),
            high_latitude_rule: HighLatitudeRule::AngleBased,
            ..CalculationParameters::new(18.0)
        }
    }
}
