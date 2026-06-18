use crate::adapters::CalculationMethodAdapter;
use crate::coordinates::Coordinates;
use crate::high_latitude::HighLatitudeRule;
use crate::params::CalculationParameters;

/// Moonsighting Committee Worldwide. Fajr 18°, Isha 18°.
///
/// Note: the canonical Moonsighting method applies a *seasonal* twilight
/// correction that depends on both latitude and day-of-year — which the current
/// `resolve(coordinates)` contract (location only) cannot express. This adapter
/// uses the committee's base angles with an angle-based high-latitude rule as a
/// close approximation. Full seasonal support is tracked as a follow-up.
#[derive(Debug, Default, Clone, Copy)]
pub struct MoonsightingCommitteeAdapter;

impl CalculationMethodAdapter for MoonsightingCommitteeAdapter {
    fn id(&self) -> String {
        "moonsighting".into()
    }
    fn display_name(&self) -> String {
        "Moonsighting Committee Worldwide".into()
    }
    fn summary(&self) -> String {
        "Fajr 18°, Isha 18° (seasonal approximation).".into()
    }
    fn resolve(&self, _coordinates: Coordinates) -> CalculationParameters {
        CalculationParameters {
            isha_angle: Some(18.0),
            high_latitude_rule: HighLatitudeRule::AngleBased,
            ..CalculationParameters::new(18.0)
        }
    }
}
