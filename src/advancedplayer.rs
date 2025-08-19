use std::{sync::mpsc, time::Duration};
use std::thread;

use rodio::source::SineWave;
use rodio::{OutputStream, Sink, Source};

use crate::{Note, WaveForm};



pub struct AudioManager {
    tx: mpsc::Sender<AudioCommand>,
}

pub enum AudioCommand {
    Play(Vec<Note>), // send notes to play
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
                    AudioCommand::Play(notes) => {
                        sink.stop(); // Stop current playback
                        for note in notes {
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
    
    pub fn play(&self, notes: Vec<Note>) {
        let _ = self.tx.send(AudioCommand::Play(notes));
    }
    
    pub fn stop(&self) {
        let _ = self.tx.send(AudioCommand::Stop);
    }
}