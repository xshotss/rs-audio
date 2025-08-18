//! * rs-audio is a Rust library for making retro music programmatically.<br>
//!   Currently, it has support for: <br>Sine waves,<br>Squares,<br>Sawtooths,<br>and Triangles.<br><br>
//! * Usage:<br>
//!
//! To create a default song (to make sure everything is working):
//! ```
//! use rs_audio::*;
//!
//! let mut song = Song::default();
//! song.play();
//! ```
//! <br>To create custom notes:
//! ```
//! use rs_audio::*;
//!
//! let mut song = Song::new(vec![
//! Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
//! Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Square },
//! Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
//! Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Triangle },
//! ], BPMChoice::Default);
//!
//! song.play();
//! ```
//!
//!

pub mod assets;
pub mod note;
pub mod player;
pub mod wav;
pub mod waveform;

pub use note::Note;
pub use player::{BPMChoice, Song};
pub use waveform::WaveForm;
