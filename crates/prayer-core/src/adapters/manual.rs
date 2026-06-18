use crate::adapters::CalculationMethodAdapter;
use crate::coordinates::Coordinates;
use crate::params::CalculationParameters;

/// Fully user-supplied parameters. Backs the "Manual" method in settings and
/// doubles as the debugging tool (drive the engine with arbitrary angles,
/// offsets, and shadow factor). Carries its parameters verbatim.
#[derive(Debug, Clone)]
pub struct ManualAdapter {
    pub parameters: CalculationParameters,
}

impl ManualAdapter {
    pub fn new(parameters: CalculationParameters) -> Self {
        Self { parameters }
    }
}

impl CalculationMethodAdapter for ManualAdapter {
    fn id(&self) -> String {
        "manual".into()
    }
    fn display_name(&self) -> String {
        "Manual".into()
    }
    fn summary(&self) -> String {
        "User-supplied angles, shadow factor, and offsets.".into()
    }
    fn resolve(&self, _coordinates: Coordinates) -> CalculationParameters {
        self.parameters.clone()
    }
}
