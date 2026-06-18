//! A geographic location used by the calculation engine.

use serde::{Deserialize, Serialize};

/// A geographic location used by the calculation engine. Elevation is optional
/// and only affects the sunrise/sunset horizon dip; defaults to sea level.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(default)]
    pub elevation: f64,
}

impl Coordinates {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            elevation: 0.0,
        }
    }

    pub fn with_elevation(latitude: f64, longitude: f64, elevation: f64) -> Self {
        Self {
            latitude,
            longitude,
            elevation,
        }
    }
}
