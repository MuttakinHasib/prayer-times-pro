use crate::adapters::CalculationMethodAdapter;
use crate::coordinates::Coordinates;
use crate::params::CalculationParameters;

/// Madhab is not a method — it is a modifier that overrides only the Asr shadow
/// factor on top of any official method. Wrapping `MwlAdapter` in this yields
/// "MWL, Hanafi Asr". Standard (Shafi/Maliki/Hanbali) Asr needs no wrapper.
pub struct HanafiAsrModifier {
    base: Box<dyn CalculationMethodAdapter>,
}

impl HanafiAsrModifier {
    pub fn new(base: Box<dyn CalculationMethodAdapter>) -> Self {
        Self { base }
    }
}

impl CalculationMethodAdapter for HanafiAsrModifier {
    fn id(&self) -> String {
        format!("{}.hanafi", self.base.id())
    }
    fn display_name(&self) -> String {
        format!("{} (Hanafi)", self.base.display_name())
    }
    fn summary(&self) -> String {
        format!("{} Hanafi Asr (shadow ×2).", self.base.summary())
    }
    fn resolve(&self, coordinates: Coordinates) -> CalculationParameters {
        let mut p = self.base.resolve(coordinates);
        p.asr_shadow_factor = 2.0;
        p
    }
}
