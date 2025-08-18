//! * rs-audio is a Rust library for making retro music programmatically.<br>
//! Currently, it has support for: <br>Sine waves,<br>Squares,<br>Sawtooths,<br>and Triangles.<br><br>
//! * Usage:<br>
//! 
//! To create a default song (to make sure everything is working):
//! ```
//! let song = Song::default();
//! song.play();
//! ```
//! <br>To create custom notes:
//! ```
//! let song = Song::new(vec![
//! Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
//! Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Square },
//! Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
//! Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Triangle },
//! ], BPMChoice::Default);
//! ```
//! 
pub mod note;
pub mod player;
pub mod waveform;
pub mod wav;
pub mod assets;

pub use note::{Note};
pub use player::{Song, BPMChoice};
pub use waveform::WaveForm;
