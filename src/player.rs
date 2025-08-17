use std::time::Duration;
use rodio::{source::SineWave, OutputStream, Sink, Source};
use crate::note::{Note, WaveForm};


pub enum BPMChoice {
    Default,
    Custom(u32),
}

/**
Used for storing multiple notes.
Can be played, resumed, etc...
Example:
```
let song = Song::new(vec![Note::default()]);




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
        let bpmchoice: u32 = match bpm {
            BPMChoice::Default => 120,
            BPMChoice::Custom(n) => n,
        };


        Song {
            notes,
            bpm: bpmchoice,
        }
    }

    pub fn play(&mut self) {
        let mut volume_warning_given: bool = false; // if the volume warning has been given (this is for volume warnings with sine waves)

        // creates stream and sink (audio mixer)
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        // iterates over the notes
        for note in &mut self.notes {
            if !volume_warning_given && note.volume > 0.20 && note.waveform == WaveForm::Sine { // issue a warning
                println!(
                r"
                 \ \        / /\   |  __ \| \ | |_   _| \ | |/ ____|
                  \ \  /\  / /  \  | |__) |  \| | | | |  \| | |  __ 
                   \ \/  \/ / /\ \ |  _  /| . ` | | | | . ` | | |_ |
                    \  /\  / ____ \| | \ \| |\  |_| |_| |\  | |__| |
                     \/  \/_/    \_\_|  \_\_| \_|_____|_| \_|\_____|"
                );


                println!("\nRS-AUDIO: One of your notes is a sine wave and has a volume that is more than 0.20.\n
                This could be dangerous as sine waves are extremely loud.\n Do you wish to proceed?\n
                Continue (not recommended): c\n
                Abort (use defalt): a\n
                Use default (uses 0.20 volume): d OR just press enter\n");

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
                        note.volume = 0.20;
                        volume_warning_given = true;
                    },

                    _ => {
                        eprintln!("RS-AUDIO: Input is invalid\nRS-AUDIO: Exiting...");
                        std::process::exit(1);
                        // no need to change volume_warning_given because we exitted
                    }
                }
            }
            let converted = match note.waveform {
                WaveForm::Sine => SineWave::new(note.frequency as f32),
                _ => unimplemented!("RS-AUDIO: This feature is not implemented! Only sine waves are allowed.\n
                You can still make other waveforms, just with a bit of math.")
            }
                .take_duration(Duration::from_secs_f64(note.duration))
                .amplify(note.volume);
                
            
            sink.append(converted);
        }

        sink.sleep_until_end();
    }
    
}

