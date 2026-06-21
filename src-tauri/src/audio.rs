//! In-process audio playback (notification chimes + full Adhan). rodio's output
//! stream is `!Send`, so it lives on a dedicated thread driven by a command
//! channel; the handle stored in Tauri state is just the `Sender`.

use std::io::Cursor;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use prayer_core::NotificationSound;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

const ADHAN_MAKKAH: &[u8] = include_bytes!("../audio/adhan-makkah.m4a");
const ADHAN_MADINAH: &[u8] = include_bytes!("../audio/adhan-madinah.m4a");
const TAKBIR: &[u8] = include_bytes!("../audio/takbir.m4a");
const SOFT_CHIME: &[u8] = include_bytes!("../audio/soft-chime.m4a");

/// A clip + whether it's long enough to warrant a Stop control (i.e. Adhan).
struct Clip {
    bytes: &'static [u8],
    long: bool,
}

/// Pick the in-process clip for a sound choice. `None` means don't play in-process —
/// `systemDefault` is handled by the OS notification, `none` is silent.
fn clip_for(sound: NotificationSound) -> Option<Clip> {
    match sound {
        NotificationSound::None | NotificationSound::SystemDefault => None,
        NotificationSound::SoftChime => Some(Clip { bytes: SOFT_CHIME, long: false }),
        NotificationSound::Takbir => Some(Clip { bytes: TAKBIR, long: false }),
        NotificationSound::AdhanMakkah => Some(Clip { bytes: ADHAN_MAKKAH, long: true }),
        NotificationSound::AdhanMadinah => Some(Clip { bytes: ADHAN_MADINAH, long: true }),
    }
}

enum Command {
    Play { sound: NotificationSound },
    PlayFullAdhan { madinah: bool },
    Stop,
}

/// A handle to the audio thread. Cloneable, `Send + Sync` — safe in Tauri state.
#[derive(Clone)]
pub struct Audio {
    tx: Sender<Command>,
}

impl Audio {
    /// Spawn the audio thread. The output device is opened lazily on the thread so
    /// a missing device never breaks app startup — playback just no-ops.
    pub fn spawn() -> Self {
        let (tx, rx) = channel::<Command>();
        thread::spawn(move || {
            // Open the device lazily and keep it: a missing device at startup must
            // not kill the thread (that would close the channel), so we keep
            // draining commands and just no-op playback until a device appears.
            let mut stream: Option<(OutputStream, OutputStreamHandle)> = None;
            let mut sink: Option<Sink> = None;
            while let Ok(cmd) = rx.recv() {
                match cmd {
                    Command::Play { sound } => {
                        let Some(clip) = clip_for(sound) else { continue };
                        play(&mut stream, &mut sink, clip.bytes);
                    }
                    Command::PlayFullAdhan { madinah } => {
                        let clip = if madinah {
                            Clip { bytes: ADHAN_MADINAH, long: true }
                        } else {
                            Clip { bytes: ADHAN_MAKKAH, long: true }
                        };
                        play(&mut stream, &mut sink, clip.bytes);
                    }
                    Command::Stop => {
                        if let Some(s) = sink.take() {
                            s.stop();
                        }
                    }
                }
            }
        });
        Self { tx }
    }

    /// Play the configured notification sound (skips silent / systemDefault).
    /// Returns whether the clip is "long" (i.e. Adhan) so callers can show a Stop UI.
    pub fn play_sound(&self, sound: NotificationSound) -> bool {
        let long = clip_for(sound).map(|c| c.long).unwrap_or(false);
        let _ = self.tx.send(Command::Play { sound });
        long
    }

    /// Play the full Adhan recording (used when `play_full_adhan` is on).
    pub fn play_full_adhan(&self, madinah: bool) {
        let _ = self.tx.send(Command::PlayFullAdhan { madinah });
    }

    pub fn stop(&self) {
        let _ = self.tx.send(Command::Stop);
    }
}

/// Open the device lazily, then start a new sink with `bytes`, stopping any
/// previous sink so back-to-back triggers don't overlap.
fn play(
    stream: &mut Option<(OutputStream, OutputStreamHandle)>,
    sink: &mut Option<Sink>,
    bytes: &'static [u8],
) {
    if let Some(s) = sink.take() {
        s.stop();
    }
    if stream.is_none() {
        match OutputStream::try_default() {
            Ok(s) => *stream = Some(s),
            Err(err) => {
                eprintln!("audio: no output device: {err}");
                return;
            }
        }
    }
    let Some((_stream, handle)) = stream.as_ref() else { return };
    match Sink::try_new(handle) {
        Ok(new_sink) => match Decoder::new(Cursor::new(bytes)) {
            Ok(source) => {
                new_sink.append(source);
                *sink = Some(new_sink);
            }
            Err(err) => eprintln!("audio: decode failed: {err}"),
        },
        Err(err) => eprintln!("audio: sink create failed: {err}"),
    }
}
