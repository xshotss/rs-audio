use crate::assets::loader::load_asset;
use crate::BPMChoice;
use crate::note::Note;
use crate::waveform::WaveForm;
use rodio::{source::SineWave, OutputStream, Sink, Source};
use std::{io::Error, time::Duration};



/**
# Deprecated
This feature has been deprecated in the latest update. Please use the new player, not the legacy one.<br><br>

Basic songs are collections of Notes. Each song can export to a .wav file.<br>

Example:
```
use rs_audio::{BasicSong, Note, BPMChoice};

let mut song = BasicSong::default();
song.play().unwrap();

let mut second_song = BasicSong::new(vec![
Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Square },
Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Triangle },
], BPMChoice::Default);

second_song.play().unwrap(); // Uses the main thread.
second_song.export_to_wav("test.wav".to_string());
```
*/
pub struct BasicSong {
    pub notes: Vec<Note>,

    pub bpm: BPMChoice, // beats per minute
}

impl Default for BasicSong {
    /**
    # Deprecated
    This feature has been deprecated in the latest update. Please use the new player, not the legacy one.<br><br>


    Creates a default song that is useful for debugging purposes.<br><br>
    It contains a single default sine wave.<br>
    Usage:
    ```
    use rs_audio::{BasicSong, Note, BPMChoice};
    let song = BasicSong::default();
    ```
    */
    fn default() -> Self {
        BasicSong {
            notes: vec![Note::default()],
            bpm: BPMChoice::Default,
        }
    }
}

impl BasicSong {
    pub fn new(notes: Vec<Note>, bpm: BPMChoice) -> Self {
        BasicSong {
            notes,
            bpm,
        }
    }

    pub fn play(&mut self) -> Result<(), Error> {
        let mut volume_warning_given: bool = false;
        // if the volume warning has been given (this is for volume warnings with sine waves) 

        // creates stream and sink (audio mixer)
        let (_stream, handle) = match OutputStream::try_default() {
            Ok(e) => e,
            Err(e) => return Err(Error::other(e.to_string())),
        };

        let sink = match Sink::try_new(&handle) {
            Ok(e) => e,
            Err(e) => return Err(Error::other(e.to_string())),
        };

        // iterates over the notes
        for note in &mut self.notes {
            if !volume_warning_given && note.vol > 0.20 && note.wave == WaveForm::Sine {
                // issue a warning

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
                    } // do nothing

                    n if n.starts_with("a") => {
                        // abort
                        println!("RS-AUDIO: Exiting...");
                        std::process::exit(0);
                        // no need to change volume_warning_given because we exited
                    }

                    n if n.starts_with("d") || n.is_empty() => {
                        /*
                        use the default value

                        this is either "d" or just an empty string (achieved by pressing enter and trimming)
                        */
                        note.vol = 0.20;
                        volume_warning_given = true;
                    }

                    _ => {
                        eprintln!("RS-AUDIO: Input is invalid\nRS-AUDIO: Exiting...");
                        std::process::exit(1);
                        // no need to change volume_warning_given because we exitted
                    }
                }
            }
            let converted = match note.wave {
                WaveForm::Sine => SineWave::new(note.freq as f32),
                _ => note.to_approx_sine(),
            }
            .take_duration(Duration::from_secs_f64(note.dur))
            .amplify(note.vol);

            sink.append(converted);
        }

        sink.sleep_until_end();
        Ok(())
    }
}
