//! Location detection — IP-based (cross-platform). Resolves coordinates, the ISO
//! country code, and the IANA timezone from the caller's IP. (Native macOS
//! CoreLocation was evaluated but needs a signed bundle to authorize; IP gives
//! city-level coordinates that are accurate for prayer-time calculation.)

use prayer_core::Coordinates;

mod ip;

/// What a detection yields: coordinates plus hints used to pick a method/timezone.
pub struct Detected {
    pub coords: Coordinates,
    pub country_code: Option<String>,
    pub tz: Option<String>,
}

/// Resolve the current location from the public IP.
pub async fn detect() -> Result<Detected, String> {
    ip::detect().await
}
