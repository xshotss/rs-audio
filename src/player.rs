use std::collections::HashMap;
use std::thread;
use std::{sync::mpsc, time::Duration};

use rodio::source::SineWave;
use rodio::{OutputStream, Sink, Source};

use crate::{BPMChoice, Note, WaveForm};

#[derive(Debug, Clone)]
pub struct Song {
    pub bpm: BPMChoice,
    pub notes: Vec<Note>,
}

impl Song {
    pub fn new(notes: Vec<Note>, bpm: BPMChoice) -> Self {
        Self { bpm, notes }
    }
}

impl Default for Song {
    fn default() -> Self {
        Self {
            bpm: BPMChoice::Default,
            notes: vec![Note::default()],
        }
    }
}

pub struct AudioManager {
    tx: mpsc::Sender<AudioCommand>,
    next_track_id: usize,
}

#[derive(Debug, Clone)]
pub enum AudioCommand {
    PlayTrack { id: usize, song: Song },
    StopTrack(usize),
    SetVolume(usize, f32),
    StopAll,
}

impl AudioManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let mut _next_track_id = 0;

        // Spawn the dedicated audio thread
        thread::spawn(move || {
            let (_stream, handle) = OutputStream::try_default().unwrap();
            let mut sinks: HashMap<usize, Sink> = HashMap::new();

            // audio thread loop
            while let Ok(command) = rx.recv() {
                match command {
                    AudioCommand::PlayTrack { id, song } => {
                        // create a new sink for this track
                        let sink = Sink::try_new(&handle).unwrap();

                        for note in song.notes {
                            let source = match note.wave {
                                WaveForm::Sine => Box::new(SineWave::new(note.freq as f32)),
                                _ => Box::new(note.to_approx_sine()),
                            };
                            sink.append(
                                source
                                    .take_duration(Duration::from_secs_f64(note.dur))
                                    .amplify(note.vol),
                            );
                        }

                        // Store the sink for potential later control
                        sinks.insert(id, sink);
                    }
                    AudioCommand::StopTrack(id) => {
                        if let Some(sink) = sinks.get(&id) {
                            sink.stop(); // Stop only this specific track
                            sinks.remove(&id);
                        }
                    }
                    AudioCommand::SetVolume(id, volume) => {
                        if let Some(sink) = sinks.get(&id) {
                            sink.set_volume(volume);
                        }
                    }
                    AudioCommand::StopAll => {
                        for sink in sinks.values() {
                            sink.stop();
                        }
                        sinks.clear();
                    }
                }
            }
        });

        AudioManager {
            tx,
            next_track_id: 0,
        }
    }

    pub fn play(&mut self, song: Song) -> usize {
        let track_id = self.next_track_id;
        self.next_track_id += 1;

        let _ = self.tx.send(AudioCommand::PlayTrack { id: track_id, song });

        track_id // return ID for later control
    }

    pub fn stop(&self, track_id: usize) {
        let _ = self.tx.send(AudioCommand::StopTrack(track_id));
    }

    pub fn set_volume(&self, track_id: usize, volume: f32) {
        let _ = self.tx.send(AudioCommand::SetVolume(track_id, volume));
    }

    pub fn stop_all(&self) {
        let _ = self.tx.send(AudioCommand::StopAll);
    }
}
