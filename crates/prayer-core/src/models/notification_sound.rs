//! Selectable sound for a notification slot.

use serde::{Deserialize, Serialize};

/// Selectable sound for a notification slot. The `adhan*` cases imply a *short*
/// notification clip plus optional full-file playback via the in-process audio
/// path in the resident app.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NotificationSound {
    None,
    SystemDefault,
    SoftChime,
    Takbir,
    AdhanMakkah,
    AdhanMadinah,
}

impl NotificationSound {
    /// Whether this selection has an associated full-length Adhan file that the
    /// in-process audio path can play.
    pub fn has_full_adhan(self) -> bool {
        matches!(self, NotificationSound::AdhanMakkah | NotificationSound::AdhanMadinah)
    }

    /// Bundled short clip filename used for the notification sound, or `None`
    /// for `None`/`SystemDefault`.
    ///
    /// NOTE: these `.caf` names are placeholders; the M4 audio work re-encodes the
    /// clips to a portable format — update them when the asset pipeline lands.
    pub fn notification_clip_file_name(self) -> Option<&'static str> {
        match self {
            NotificationSound::None | NotificationSound::SystemDefault => None,
            NotificationSound::SoftChime => Some("soft-chime.caf"),
            NotificationSound::Takbir => Some("takbir.caf"),
            NotificationSound::AdhanMakkah | NotificationSound::AdhanMadinah => Some("takbir.caf"),
        }
    }

    /// Bundled full-length Adhan filename for the in-process player, if any.
    pub fn full_adhan_file_name(self) -> Option<&'static str> {
        match self {
            NotificationSound::AdhanMakkah => Some("adhan-makkah.m4a"),
            NotificationSound::AdhanMadinah => Some("adhan-madinah.m4a"),
            _ => None,
        }
    }
}
