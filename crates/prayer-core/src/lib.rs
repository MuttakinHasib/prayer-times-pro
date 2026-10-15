//! Pure Islamic prayer-time calculation core.
//!
//! Design rule (do not violate): this crate is a *generic astronomical
//! calculator*. Everything Islam-specific (twilight angles, shadow factors,
//! method offsets) lives in [`adapters`] that produce [`CalculationParameters`].
//! Madhab is a *modifier* ([`adapters::HanafiAsrModifier`]) over a method, not a
//! separate method. No UI, no I/O — the golden-table accuracy gate lives in
//! `tests/` and must stay green (±1 minute vs official Diyanet/JAKIM/Kemenag
//! tables).

mod degree_math;
mod engine;
mod hour_angle;
mod solar;

pub mod adapters;
pub mod coordinates;
pub mod high_latitude;
pub mod models;
pub mod params;

pub use coordinates::Coordinates;
pub use engine::calculate;
pub use high_latitude::HighLatitudeRule;
pub use params::CalculationParameters;

pub use adapters::{
    CalculationMethodAdapter, DiyanetAdapter, EgyptianAdapter, HanafiAsrModifier, IsnaAdapter,
    JakimAdapter, KarachiAdapter, KemenagAdapter, ManualAdapter, MethodRegistry,
    MoonsightingCommitteeAdapter, MwlAdapter, UmmAlQuraAdapter,
};
pub use models::{
    current_waqt::CurrentWaqt,
    notification::{NotificationDefaults, PrayerNotificationConfig, ResolvedNotification},
    notification_sound::NotificationSound,
    prayer::Prayer,
    prayer_times::PrayerTimes,
    settings::{
        default_jamaat_times, AppSettings, AppearanceTheme, CalculationMode, FocusBlurIntensity,
        FocusTrigger, LocationMode, MenuBarCountdownMode, MenuBarStyle,
        FALLBACK_EARLY_LEAD_MINUTES,
    },
};
