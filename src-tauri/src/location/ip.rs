//! IP geolocation via ipwho.is (rustls, no key). Supplies coordinates, ISO
//! country code, and IANA timezone. `ipapi.co` sits behind a Cloudflare
//! bot-challenge, so it's unusable for a headless request.

use std::sync::OnceLock;
use std::time::Duration;

use prayer_core::Coordinates;
use serde::Deserialize;

use super::Detected;

/// One shared client with a bounded timeout, so a slow network can't hang the UI.
fn client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(8))
            .user_agent("prayer-times-pro")
            .build()
            .unwrap_or_default()
    })
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

/// Resolve the location from the public IP. Errors are human-readable strings.
pub async fn detect() -> Result<Detected, String> {
    let resp = client()
        .get("https://ipwho.is/")
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
