//! In-process Adhan playback. rodio's output stream is `!Send`, so it lives on a
//! dedicated thread driven by a command channel; the handle stored in Tauri state
//! is just the `Sender`.

use std::io::Cursor;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use rodio::{Decoder, OutputStream, Sink};

const ADHAN_MAKKAH: &[u8] = include_bytes!("../audio/adhan-makkah.m4a");
const ADHAN_MADINAH: &[u8] = include_bytes!("../audio/adhan-madinah.m4a");

enum Command {
    Play { madinah: bool },
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
            let Ok((_stream, handle)) = OutputStream::try_default() else {
                return; // no audio device; drain commands as no-ops
            };
            let mut sink: Option<Sink> = None;
            while let Ok(cmd) = rx.recv() {
                match cmd {
                    Command::Play { madinah } => {
                        if let Some(s) = sink.take() {
                            s.stop();
                        }
                        let bytes = if madinah { ADHAN_MADINAH } else { ADHAN_MAKKAH };
                        if let Ok(new_sink) = Sink::try_new(&handle) {
                            match Decoder::new(Cursor::new(bytes)) {
                                Ok(source) => {
                                    new_sink.append(source);
                                    sink = Some(new_sink);
                                }
                                Err(err) => eprintln!("audio: adhan decode failed: {err}"),
                            }
                        }
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

    pub fn play_adhan(&self, madinah: bool) {
        let _ = self.tx.send(Command::Play { madinah });
    }

    pub fn stop(&self) {
        let _ = self.tx.send(Command::Stop);
    }
}
