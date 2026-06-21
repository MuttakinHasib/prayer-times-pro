//! Prayer notifications + Adhan. The Rust clock owns the schedule: on each tick
//! the clock returns the [`NotifEvent`]s that just came due, and [`fire`] delivers
//! them. The user's chosen sound plays in-process for non-`systemDefault` sounds
//! (so the banner is sent silent to avoid double audio); `systemDefault` lets the
//! OS chime; `none` is a silent banner. A sleep/wake catch-up guard skips audio
//! that's already stale.

use prayer_core::{FocusBlurIntensity, NotificationSound};
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tauri_plugin_notification::NotificationExt;

use crate::audio::Audio;

/// Emitted with `true` when an Adhan starts playing, `false` when stopped, so the
/// panel can offer a Stop control.
pub const ADHAN_EVENT: &str = "prayer://adhan-state";

/// Don't blast audio for a sound we only reached late (e.g. after sleep); the
/// banner still informs.
const AUDIO_STALE_MS: i64 = 10_000;

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
    /// Sound to play at this event (per the resolved per-prayer config).
    pub sound: NotificationSound,
    /// Play the full Adhan recording at this event (athan events for opted-in prayers).
    pub play_full_adhan: bool,
    /// Which Adhan recording to use (when `play_full_adhan` is true).
    pub madinah: bool,
    /// Present on athan events that should engage Focus Mode.
    pub focus: Option<FocusCue>,
}

/// Deliver the due events: an OS banner for each, audio for events that aren't
/// stale, and Focus Mode for the matching athan event.
pub fn fire(app: &AppHandle, audio: &Audio, events: &[NotifEvent], now_ms: i64) {
    for ev in events {
        let stale = now_ms - ev.fire_ms > AUDIO_STALE_MS;
        let in_process = !stale && (ev.play_full_adhan || in_process_sound(ev.sound));

        // Send the banner. If we'll play in-process audio, keep the banner silent to
        // avoid double sound; otherwise let the OS chime if the user picked Default.
        let mut builder = app.notification().builder().title(&ev.title).body(&ev.body);
        if in_process {
            builder = builder.sound(""); // suppress OS sound
        } else if ev.sound == NotificationSound::SystemDefault {
            builder = builder.sound("default");
        }
        if let Err(err) = builder.show() {
            eprintln!("notification show failed: {err}");
        }

        if in_process {
            let long = if ev.play_full_adhan {
                audio.play_full_adhan(ev.madinah);
                true
            } else {
                audio.play_sound(ev.sound)
            };
            if long {
                let _ = app.emit(ADHAN_EVENT, true);
            }
        }

        if let Some(cue) = &ev.focus {
            crate::focus::engage(app, cue);
        }
    }
}

/// Whether this sound plays in-process via rodio (rather than the OS / silent).
fn in_process_sound(sound: NotificationSound) -> bool {
    !matches!(sound, NotificationSound::None | NotificationSound::SystemDefault)
}
