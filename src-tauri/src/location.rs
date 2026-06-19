//! Location detection for the "Detect" action. IP-based (cross-platform) — it
//! resolves coordinates, the ISO country code, and the IANA timezone from the
//! caller's IP. Precise macOS CoreLocation is a planned enhancement.

use prayer_core::Coordinates;
use serde::Deserialize;

/// What a detection yields: coordinates plus hints used to pick a method/timezone.
pub struct Detected {
    pub coords: Coordinates,
    pub country_code: Option<String>,
    pub tz: Option<String>,
}

#[derive(Deserialize)]
struct Timezone {
    id: Option<String>,
}

#[derive(Deserialize)]
struct IpWho {
    #[serde(default)]
    success: bool,
    #[serde(default)]
    message: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    country_code: Option<String>,
    timezone: Option<Timezone>,
}

/// Resolve the current location from the public IP. Errors are returned as
/// human-readable strings for the frontend to surface.
pub async fn detect() -> Result<Detected, String> {
    let resp = reqwest::Client::new()
        .get("https://ipwho.is/")
        .header("User-Agent", "prayer-times-pro")
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    let info: IpWho = resp.json().await.map_err(|e| format!("Could not read location: {e}"))?;
    if !info.success {
        return Err(info.message.unwrap_or_else(|| "Location lookup failed".into()));
    }
    let (Some(latitude), Some(longitude)) = (info.latitude, info.longitude) else {
        return Err("Location response was missing coordinates".into());
    };

    Ok(Detected {
        coords: Coordinates { latitude, longitude, elevation: 0.0 },
        country_code: info.country_code,
        tz: info.timezone.and_then(|t| t.id),
    })
}
