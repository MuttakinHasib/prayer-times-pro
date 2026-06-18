//! Central catalog of calculation methods. Resolves persisted ids back to
//! adapters, applies the Hanafi Asr modifier, and maps ISO country codes to a
//! default method for the optional auto-detect feature.

use crate::adapters::{
    CalculationMethodAdapter, DiyanetAdapter, EgyptianAdapter, HanafiAsrModifier, IsnaAdapter,
    JakimAdapter, KarachiAdapter, KemenagAdapter, ManualAdapter, MoonsightingCommitteeAdapter,
    MwlAdapter, UmmAlQuraAdapter,
};
use crate::params::CalculationParameters;

pub struct MethodRegistry;

impl MethodRegistry {
    /// All selectable built-in methods, in display order. `ManualAdapter` is
    /// excluded because it requires user-supplied parameters; construct it
    /// directly when the "Manual" method is chosen.
    pub fn built_in() -> Vec<Box<dyn CalculationMethodAdapter>> {
        vec![
            Box::new(DiyanetAdapter),
            Box::new(MwlAdapter),
            Box::new(IsnaAdapter),
            Box::new(UmmAlQuraAdapter),
            Box::new(EgyptianAdapter),
            Box::new(KarachiAdapter),
            Box::new(JakimAdapter),
            Box::new(KemenagAdapter),
            Box::new(MoonsightingCommitteeAdapter),
        ]
    }

    /// Look up a base method by its stable id (without the `.hanafi` suffix).
    pub fn adapter(for_id: &str) -> Option<Box<dyn CalculationMethodAdapter>> {
        Self::built_in().into_iter().find(|a| a.id() == for_id)
    }

    /// Resolve a persisted selection into a ready-to-use adapter.
    ///
    /// - `method_id`: a built-in id, or `"manual"`.
    /// - `hanafi_asr`: wraps the result in `HanafiAsrModifier` when `true`.
    /// - `manual_parameters`: required when `method_id == "manual"`.
    ///
    /// Returns `None` if the id is unknown / manual params missing.
    pub fn resolve(
        method_id: &str,
        hanafi_asr: bool,
        manual_parameters: Option<CalculationParameters>,
    ) -> Option<Box<dyn CalculationMethodAdapter>> {
        let base: Box<dyn CalculationMethodAdapter> = if method_id == "manual" {
            Box::new(ManualAdapter::new(manual_parameters?))
        } else {
            Self::adapter(method_id)?
        };
        if hanafi_asr {
            Some(Box::new(HanafiAsrModifier::new(base)))
        } else {
            Some(base)
        }
    }

    /// Default method id for a country code; `"mwl"` when unmapped or `None`.
    /// ISO 3166-1 alpha-2; case-insensitive. Extend as coverage grows.
    pub fn method_id_for_country(code: Option<&str>) -> String {
        let Some(code) = code else {
            return "mwl".into();
        };
        let id = match code.to_uppercase().as_str() {
            "TR" => "diyanet",
            "US" | "CA" => "isna",
            "SA" => "ummalqura",
            "EG" => "egyptian",
            "PK" | "IN" | "BD" | "AF" => "karachi",
            // Malaysia → JAKIM, calibrated to e-Solat. Neighbours (Singapore/MUIS,
            // Brunei) run their own authorities and aren't mapped here.
            "MY" => "jakim",
            // Indonesia → Kemenag (Kementerian Agama RI).
            "ID" => "kemenag",
            // GB + Northern Europe lean MWL with angle-based high-lat.
            "GB" | "IE" | "NO" | "SE" | "FI" | "DK" | "IS" => "mwl",
            _ => "mwl",
        };
        id.into()
    }
}
