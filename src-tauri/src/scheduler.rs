//! Prayer notifications + Adhan. The Rust clock owns the schedule: on each tick
//! the clock returns the [`NotifEvent`]s that just came due, and [`fire`] delivers
//! them — an OS banner for every event, plus in-process Adhan audio at the prayer
//! instant. A sleep/wake catch-up guard skips Adhan that's already stale.

use prayer_core::FocusBlurIntensity;
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tauri_plugin_notification::NotificationExt;

use crate::audio::Audio;

/// Emitted with `true` when an Adhan starts playing, `false` when stopped, so the
/// panel can offer a Stop control.
pub const ADHAN_EVENT: &str = "prayer://adhan-state";

/// Don't blast a full Adhan that we only reached late (e.g. waking from sleep
/// well past the prayer instant); the banner still informs, but the audio is skipped.
const ADHAN_STALE_MS: i64 = 10_000;

/// What an athan event tells Focus Mode to display when it engages.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusCue {
    pub prayer: String,
    pub duration_minutes: i32,
    pub blur: FocusBlurIntensity,
    pub emergency_exit: bool,
}

/// A single due notification produced by the clock.
#[derive(Clone)]
pub struct NotifEvent {
    pub fire_ms: i64,
    pub title: String,
    pub body: String,
    /// Play the full Adhan in-process at this event (athan events for opted-in prayers).
    pub play_adhan: bool,
    /// Which Adhan recording to use.
    pub madinah: bool,
    /// Present on athan events that should engage Focus Mode.
    pub focus: Option<FocusCue>,
}

/// Deliver the due events: an OS banner for each, and Adhan audio for athan events
/// that aren't stale.
pub fn fire(app: &AppHandle, audio: &Audio, events: &[NotifEvent], now_ms: i64) {
    for ev in events {
        if let Err(err) = app
            .notification()
            .builder()
            .title(&ev.title)
            .body(&ev.body)
            .show()
        {
            eprintln!("notification show failed: {err}");
        }
        if ev.play_adhan && now_ms - ev.fire_ms <= ADHAN_STALE_MS {
            audio.play_adhan(ev.madinah);
            let _ = app.emit(ADHAN_EVENT, true);
        }
        if let Some(cue) = &ev.focus {
            crate::focus::engage(app, cue);
        }
    }
}
