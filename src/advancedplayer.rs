use std::{sync::mpsc, time::Duration};
use std::thread;

use rodio::source::SineWave;
use rodio::{OutputStream, Sink, Source};

use crate::{Song, WaveForm};



pub struct AudioManager {
    tx: mpsc::Sender<AudioCommand>,
}

pub enum AudioCommand {
    Play(Song), // send notes to play
    Stop,
    SetVolume(f32),
}

impl AudioManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        
        // spawn the dedicated audio thread
        thread::spawn(move || {
            let (_stream, handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&handle).unwrap();
            
            // audio thread loop
            while let Ok(command) = rx.recv() {
                match command {
                    AudioCommand::Play(song) => {
                        sink.stop(); // stop current playback
                        for note in song.notes {
                            let source = match note.wave {
                                WaveForm::Sine => Box::new(SineWave::new(note.freq as f32)),
                                _ => Box::new(note.to_approx_sine()),
                            };
                            sink.append(source
                                .take_duration(Duration::from_secs_f64(note.dur))
                                .amplify(note.vol));
                        }
                    }
                    AudioCommand::Stop => {
                        sink.stop();
                    }
                    AudioCommand::SetVolume(vol) => {
                        sink.set_volume(vol);
                    }
                }
            }
        });
        
        AudioManager { tx }
    }
    
    pub fn play(&self, song: Song) {
        let _ = self.tx.send(AudioCommand::Play(song));
    }
    
    pub fn stop(&self) {
        let _ = self.tx.send(AudioCommand::Stop);
    }
}