//! In-process Adhan playback. rodio's output stream is `!Send`, so it lives on a
//! dedicated thread driven by a command channel; the handle stored in Tauri state
//! is just the `Sender`.

use std::io::Cursor;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

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
            // Open the device lazily and keep it: a missing device at startup must
            // not kill the thread (that would close the channel), so we keep
            // draining commands and just no-op playback until a device appears.
            let mut stream: Option<(OutputStream, OutputStreamHandle)> = None;
            let mut sink: Option<Sink> = None;
            while let Ok(cmd) = rx.recv() {
                match cmd {
                    Command::Play { madinah } => {
                        if let Some(s) = sink.take() {
                            s.stop();
                        }
                        if stream.is_none() {
                            match OutputStream::try_default() {
                                Ok(s) => stream = Some(s),
                                Err(err) => {
                                    eprintln!("audio: no output device: {err}");
                                    continue;
                                }
                            }
                        }
                        let Some((_stream, handle)) = stream.as_ref() else {
                            continue;
                        };
                        let bytes = if madinah { ADHAN_MADINAH } else { ADHAN_MAKKAH };
                        match Sink::try_new(handle) {
                            Ok(new_sink) => match Decoder::new(Cursor::new(bytes)) {
                                Ok(source) => {
                                    new_sink.append(source);
                                    sink = Some(new_sink);
                                }
                                Err(err) => eprintln!("audio: adhan decode failed: {err}"),
                            },
                            Err(err) => eprintln!("audio: sink create failed: {err}"),
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
