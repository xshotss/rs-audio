use rodio::source::SineWave;

use crate::WaveForm;

/**
This struct represents a note.<br><br>
It is the building block for all songs made with rs-audio.<br>
```
let note: Note = Note {
    frequency: 440.0,
    duration: 3.0,
    volume: 0.20,
    waveform: WaveForm::Sine,
}

let default_note = Note::default(); // This outputs the same note as the one above.
```
*/
#[derive(Clone)]
pub struct Note {
    pub frequency: f64, // frequency in hertz
    pub duration: f64, // duration in seconds

    pub volume: f32, // the volume/amplituide (0.0 to 1.0)

    pub waveform: WaveForm, // the wave type (see below for types)
}

impl Default for Note {
    /**
    Generates a default sine wave. <br><br>It has a frequency for 440 Hertz, lasts 3 seconds, and its volume is 0.20.<br><br>
    Usage:
    ```
    let default_note: Note = Note::default();
    ```
    */
    fn default() -> Self {
        Note {
            frequency: 440.0,
            duration: 3.0,
            volume: 0.20,
            waveform: WaveForm::Sine,
        }
    }
}


impl Note {
    pub fn to_approx_sine(&self) -> SineWave {
        /*
        this emulates sines, squares, sawtooths and triangles as rodio sine waves
        it's not extremely accurate but it works
        */
        let effective_freq = match self.waveform {
            WaveForm::Sine => self.frequency,
            WaveForm::Square => self.frequency * 1.27, // adds odd harmonics
            WaveForm::Sawtooth => self.frequency * 1.5, // rich harmonics
            WaveForm::Triangle => self.frequency * 1.16, // soft harmonics
        };
        
        SineWave::new(effective_freq as f32)
    }
}