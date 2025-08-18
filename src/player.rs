use std::{time::Duration};
use crate::assets::loader::load_asset;
use rodio::{source::SineWave, OutputStream, Sink, Source};
use crate::note::{Note};
use crate::waveform::{WaveForm};


/**
The BPMChoice is an enum for picking the <b>beats per minute</b> for making songs.<br>
Usage:
```
let song = Song::new(vec![
Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Square },
Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Triangle },
], BPMChoice::Default);
```
*/
pub enum BPMChoice {
    Default,
    Custom(u32),
}

impl BPMChoice { fn to_u32(&self) -> u32 {
    match self {
        BPMChoice::Default => 120,
        BPMChoice::Custom(n) => *n,
    }
}}

/**
Songs are collections of Notes. Each song can export to a .wav file.<br>

Example:
```
let song = Song::default();
song.play();

let second_song = Song::new(vec![
Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Square },
Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Triangle },
], BPMChoice::Default);

second_song.play();
second_song.export_to_wav("test.wav".to_string());
```
*/
pub struct Song {
    pub notes: Vec<Note>,

    pub bpm: u32, // beats per minute
}

impl Default for Song {
    /**
    Creates a default song that is useful for debugging purposes.<br><br>
    It contains a single default sine wave.<br>
    Usage:
    ```
    let song = Song::default();
    ```
    */
    fn default() -> Self {
        Song {
            notes: vec![Note::default()],
            bpm: 120,
        }
    }
}

impl Song {
    pub fn new(notes: Vec<Note>, bpm: BPMChoice) -> Self {
        Song {
            notes,
            bpm: bpm.to_u32(),
        }
    }

    pub fn play(&mut self) {
        let mut volume_warning_given: bool = false; /*
        if the volume warning has been given (this is for volume warnings with sine waves)
        */

        // creates stream and sink (audio mixer)
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        // iterates over the notes
        for note in &mut self.notes {
            if !volume_warning_given && note.vol > 0.20 && note.wave == WaveForm::Sine { // issue a warning

                /* loads the warning ascii art */
                println!("{}", load_asset("warning_ascii.txt"));

                /* loads the volume warning text */
                println!("{}", load_asset("warning_volume.txt"));

                let mut input = String::new();

                // wait for input
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("RS-AUDIO: Could not read input!");

                match input.trim() {
                    // handle options
                    n if n.starts_with("c") => {
                        volume_warning_given = true;
                    }, // do nothing

                    n if n.starts_with("a") => { // abort
                        println!("RS-AUDIO: Exiting...");
                        std::process::exit(0);
                        // no need to change volume_warning_given because we exited
                    },

                    n if n.starts_with("d") || n.is_empty() => {
                        /*
                        use the default value

                        this is either "d" or just an empty string (achieved by pressing enter and trimming)
                        */
                        note.vol = 0.20;
                        volume_warning_given = true;
                    },

                    _ => {
                        eprintln!("RS-AUDIO: Input is invalid\nRS-AUDIO: Exiting...");
                        std::process::exit(1);
                        // no need to change volume_warning_given because we exitted
                    }
                }
            }
            let converted = match note.wave {
                WaveForm::Sine => SineWave::new(note.freq as f32),
                _ => unimplemented!("RS-AUDIO: This feature is not implemented! Only sine waves are allowed.\n
                You can still make other waveforms, just with a bit of math.")
            }
                .take_duration(Duration::from_secs_f64(note.dur))
                .amplify(note.vol);
                
            
            sink.append(converted);
        }

        sink.sleep_until_end();
    }


    
}

