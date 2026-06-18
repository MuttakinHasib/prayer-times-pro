//! The six daily events the app tracks.

use serde::{Deserialize, Serialize};

/// The six daily events the app tracks. `Sunrise` is included because it bounds
/// the Fajr window and is shown in the panel, but it is not an obligatory prayer
/// (no iqamah, no congregation).
///
/// Variants are declared in chronological `dayOrder`, so the derived `Ord`
/// matches the order through the day (used by `BTreeMap` and "next prayer" logic).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Prayer {
    Fajr,
    Sunrise,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
}

impl Prayer {
    /// All six events in chronological order.
    pub const ALL: [Prayer; 6] = [
        Prayer::Fajr,
        Prayer::Sunrise,
        Prayer::Dhuhr,
        Prayer::Asr,
        Prayer::Maghrib,
        Prayer::Isha,
    ];

    /// The five obligatory prayers, excluding `Sunrise`. Iqamah/congregation
    /// concepts apply only to these.
    pub const OBLIGATORY: [Prayer; 5] = [
        Prayer::Fajr,
        Prayer::Dhuhr,
        Prayer::Asr,
        Prayer::Maghrib,
        Prayer::Isha,
    ];

    /// `true` for the five obligatory prayers; `false` for `Sunrise`.
    pub fn is_obligatory(self) -> bool {
        self != Prayer::Sunrise
    }

    /// Stable ordering through the day, used for "next prayer" logic.
    pub fn day_order(self) -> u8 {
        match self {
            Prayer::Fajr => 0,
            Prayer::Sunrise => 1,
            Prayer::Dhuhr => 2,
            Prayer::Asr => 3,
            Prayer::Maghrib => 4,
            Prayer::Isha => 5,
        }
    }
}
