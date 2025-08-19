use std::f64::consts::PI;

/**
This enum is used for declaring waveform types.<br>
*/
#[derive(Clone, PartialEq, Debug)]
pub enum WaveForm {
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

// generates a sample using a waveform
// only intended for use by the library
pub(crate) fn generate_sample(waveform: &WaveForm, phase: f64) -> f64 {
    match waveform {
        // equations
        WaveForm::Sine => (phase * 2.0 * PI).sin(),
        WaveForm::Square => {
            if phase < 0.5 {
                1.0
            } else {
                -1.0
            }
        }
        WaveForm::Sawtooth => 2.0 * phase - 1.0,
        WaveForm::Triangle => {
            if phase < 0.5 {
                -1.0 + 4.0 * phase
            } else {
                1.0 - 4.0 * (phase - 0.5)
            }
        }
    }
}
