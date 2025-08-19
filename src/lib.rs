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

let mut audio_manager = AudioManager::new(); // This creates an audio thread which handles audio.

audio_manager.play(Song::default()); // Plays a default song.
```
<br>To create custom notes:
```
use rs_audio::*;

let mut audio_manager = AudioManager::new();

let mut song = Song::new(vec![
Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Square },
Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Triangle },
], BPMChoice::Default);

audio_manager.play(song);
```

# NOTE
This priject has recently moved to a new version. Due to drastic changes, I have started rewriting all documentation.<br>
This should do for now.
*/

pub(crate) mod assets;
pub mod note;
pub mod legacyplayer;
pub mod player;
pub mod wav;
pub mod waveform;

pub use note::Note;
pub use legacyplayer::{BPMChoice, BasicSong};
pub use waveform::WaveForm;
