use rodio::source::SineWave;

use crate::WaveForm;

/**
This struct represents a note.<br><br>
It is the building block for all songs made with rs-audio.<br>
```
use rs_audio::*;

let note: Note = Note {
    freq: 440.0,
    dur: 3.0,
    vol: 0.20,
    wave: WaveForm::Sine,
};

let default_note = Note::default(); // This outputs the same note as the one above.
```
*/
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Note {
    pub freq: f64, // frequency in hertz
    pub dur: f64,  // duration in seconds

    pub vol: f32, // the volume/amplituide (0.0 to 1.0)

    pub wave: WaveForm, // the wave type (see below for types)
}

impl Default for Note {
    /**
    Generates a default sine wave. <br><br>It has a frequency for 440 Hertz, lasts 3 seconds, and its volume is 0.20.<br><br>
    Usage:
    ```
    use rs_audio::*;

    let default_note: Note = Note::default();
    ```
    */
    fn default() -> Self {
        Note {
            freq: 440.0,
            dur: 3.0,
            vol: 0.20,
            wave: WaveForm::Sine,
        }
    }
}

impl Note {
    /**
    Creates a rest (silence) note.<br><br>
    Usage:
    ```
    use rs_audio::*;
    let rest_note = Note::rest(2.0); // a 2 second rest
    ```
    */
    pub fn rest(dur: f64) -> Self {
        Note {
            freq: 0.0,
            dur,
            vol: 0.0,
            wave: WaveForm::Rest,
        }
    }


    pub(crate) fn to_approx_sine(&self) -> SineWave {
        /*
        this emulates sines, squares, sawtooths and triangles as rodio sine waves
        it's not extremely accurate but it works
        */
        let effective_freq = match self.wave {
            WaveForm::Sine => self.freq,
            WaveForm::Square => self.freq * 1.27, // adds odd harmonics
            WaveForm::Sawtooth => self.freq * 1.5, // rich harmonics
            WaveForm::Triangle => self.freq * 1.16, // soft harmonics
            WaveForm::Rest => 0.0, // silence
        };

        SineWave::new(effective_freq as f32)
    }
}
