//! Calculation-method adapters: translate a location into the numeric
//! parameters the engine consumes. Methods are pure value producers — no state,
//! no I/O. Everything Islam-specific lives here; the engine stays generic.

use crate::coordinates::Coordinates;
use crate::params::CalculationParameters;

mod diyanet;
mod egyptian;
mod hanafi;
mod isna;
mod jakim;
mod karachi;
mod kemenag;
mod manual;
mod moonsighting;
mod mwl;
mod registry;
mod umm_al_qura;

pub use diyanet::DiyanetAdapter;
pub use egyptian::EgyptianAdapter;
pub use hanafi::HanafiAsrModifier;
pub use isna::IsnaAdapter;
pub use jakim::JakimAdapter;
pub use karachi::KarachiAdapter;
pub use kemenag::KemenagAdapter;
pub use manual::ManualAdapter;
pub use moonsighting::MoonsightingCommitteeAdapter;
pub use mwl::MwlAdapter;
pub use registry::MethodRegistry;
pub use umm_al_qura::UmmAlQuraAdapter;

/// A calculation method: translates a location into the numeric parameters the
/// engine consumes.
pub trait CalculationMethodAdapter {
    /// Stable key used for persistence and the registry. Never localized.
    fn id(&self) -> String;
    /// Human-readable name for the UI (localized at the presentation layer).
    fn display_name(&self) -> String;
    /// One-line description of the method's provenance / parameters.
    fn summary(&self) -> String;
    /// Produce engine parameters for the given location.
    fn resolve(&self, coordinates: Coordinates) -> CalculationParameters;
}
