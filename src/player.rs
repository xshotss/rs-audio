use std::collections::HashMap;
use std::io::Error;
use std::thread;
use std::{sync::mpsc, time::Duration};

use rodio::source::SineWave;
use rodio::{OutputStream, Sink, Source};

use crate::{BPMChoice, Note, WaveForm};

/**
This struct represents a song.<br>
It contains a list of notes and a BPM (beats per minute) setting.<br><br>
## Usage:
```
use rs_audio::*;
let song = Song::default(); // creates a default song with one note (A4, 440Hz, 3 seconds, 0.20 volume, sine wave)
```
*/
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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
    /**
    Generates a default song that is useful for debugging purposes.<br>
    It contains a single sine wave with a frequency of 440 Hz, lasts 3 seconds and has a volume of 0.20.<br>
    It has a BPM of 120 (the default)<br><br>
    # Usage
    ```
    use rs_audio::*;

    let default_song = Song::default();
    ```
    */
    fn default() -> Self {
        Self {
            bpm: BPMChoice::Default,
            notes: vec![Note::default()],
        }
    }
}

impl Song {
    /**
    Saves a song to a JSON file. This can be useful if you want to save your songs somewhere.<br>
    Note that this can return an error if it fails to:<br>
    * Convert the song to JSON
    * Write to the file
      <br><br>
    # Usage
    ```
    use rs_audio::*;

    let song = Song::default();
    match Song::save_to_json(&song, "song.json") {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e.to_string()),
    }
    ```
    */
    pub fn save_to_json(song: &Song, filename: &str) -> Result<(), Error> {
        let json = serde_json::to_string_pretty(song)?;
        std::fs::write(filename, json)?;
        Ok(())
    }

    /**
    Loads a Song struct from a JSON file. This can be useful if you want to load existing songs from JSONs.<br>
    Note that this will return an error if it fails to:<br>
    * Open the file (it may not exist or it could not read it)
    * Read from the file.
      <br><br>
    # Usage
    ```
    use rs_audio::*;

    let loaded_song: Song = match Song::load_from_json("song.json") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        }
    };
    ```
    */
    pub fn load_from_json(filename: &str) -> Result<Song, Error> {
        let json = std::fs::read_to_string(filename)?;
        let song = serde_json::from_str(&json)?;
        Ok(song)
    }
}

/**
This struct manages audio playback.<br>
It allows playing multiple songs simultaneously, stopping them individually or all at once, and adjusting their volumes<br>
It handles everything in a dedicated audio thread to ensure smooth playback and to not disrupt any other tasks.<br><br>
## Usage:
```
use rs_audio::*;


let mut audio_manager = AudioManager::new();

let song = Song::default();

let track_id = audio_manager.play(song); // play the song and get its track ID

audio_manager.set_volume(track_id, 0.5); // set volume for this track
audio_manager.stop(track_id); // stop this specific track
audio_manager.stop_all(); // stop all tracks
```
*/
pub struct AudioManager {
    tx: mpsc::Sender<AudioCommand>,
    next_track_id: usize,
}

#[derive(Debug, Clone)]
pub(crate) enum AudioCommand {
    PlayTrack { id: usize, song: Song },
    StopTrack(usize),
    SetVolume(usize, f32),
    StopAll,
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioManager {
    /**
    Creates a new AudioManager instance and starts the audio thread.<br>
    This thread handles all audio playback and control.<br>
    It uses channels to receive commands from the main thread.<br><br>
    # Usage:
    ```
    use rs_audio::*;
    let mut audio_manager = AudioManager::new();
    ```
    */
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
                                WaveForm::Rest => {
                                    std::thread::sleep(Duration::from_secs_f64(note.dur));
                                    continue;
                                }
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
    /// Plays a song and returns a unique track ID for later control (like stopping or adjusting volume).
    pub fn play(&mut self, song: Song) -> usize {
        let track_id = self.next_track_id;
        self.next_track_id += 1;

        let _ = self.tx.send(AudioCommand::PlayTrack { id: track_id, song });

        track_id // return ID for later control
    }

    /// Stops a specific track using its track ID.
    pub fn stop(&self, track_id: usize) {
        let _ = self.tx.send(AudioCommand::StopTrack(track_id));
    }

    /// Sets the volume for a specific track using its track ID.
    pub fn set_volume(&self, track_id: usize, volume: f32) {
        let _ = self.tx.send(AudioCommand::SetVolume(track_id, volume));
    }

    /// Stops all currently playing tracks.
    pub fn stop_all(&self) {
        let _ = self.tx.send(AudioCommand::StopAll);
    }
}
