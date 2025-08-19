/*!
# RS-AUDIO
rs-audio is a Rust library for making retro music programmatically.<br>
  Currently, it has support for: 
  * Sine waves,
  * Squares,
  * Sawtooths,
  * and Triangles.<br>

This library is MIT licensed. <br>Learn more in our repository: <https://github.com/xshotss/rs-audio/blob/main/LICENSE><br><br>
## Usage:<br>

To create a default song (to make sure everything is working):
```
use rs_audio::*;

let mut song = Song::default();
song.play().unwrap();
```
<br>To create custom notes:
```
use rs_audio::*;

let mut song = Song::new(vec![
Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Square },
Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Triangle },
], BPMChoice::Default);

song.play().unwrap();
```

# Multithreading

To use a separate thread for playing songs, you need to use the following function.<br>
Multithreading means that you can perform other tasks while playing music.<br><br>

## Usage
```
use rs_audio::*;

let mut song = Song::default();
song.play_from_thread().unwrap();
```


*/

pub(crate) mod assets;
pub mod note;
pub mod player;
pub mod wav;
pub mod waveform;

pub use note::Note;
pub use player::{BPMChoice, Song};
pub use waveform::WaveForm;
