# RS-AUDIO

Rs-audio is a Rust library and audio "engine" for making retro songs. It is made to simplify the process of making music programmatically while still being feature-rich.

**NOTE:** rs-audio is under development. Please report any issues or bugs.

## HOW TO USE
To start using rs-audio, make a new project and navigate to its folder:
```bash
cargo new my_project && cd my_project
```

Add the rs-audio crate to your project:
```bash
cargo add rs-audio
```

Open your project, click on the src folder and click on the `main.rs` file. This is the entry point of your program.

To use all of rs-audio's features, add this at the top of your script's file:
```Rust
use rs_audio::*;
```


To make sure that everything is working, add this to your main() function:
```Rust
fn main() {
  let mut amngr = AudioManager::new(); // Creates a thread for audio.

  let _ = amngr.play(Song::default()); // This plays the default song for debugging.
  // We used "let _ =" to discard the value of play() as it returns the track_id for our track.
  // You can use the track id to control what the Audio Manager does to the song.
}
```

Run your project:
```bash
cargo run
```
If you hear a beep for 3 seconds, and if you see a `test.wav` file in your src folder, everything is working!

To make a custom song, you can try the following.
```Rust
let mut song = Song::new(vec![
Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Square },
Note { freq: 880.0, dur: 1.0, vol: 0.20, wave: WaveForm::Sine },
Note { freq: 220.0, dur: 1.0, vol: 0.20, wave: WaveForm::Triangle },
], BPMChoice::Default);

amngr.play(song); // Make sure you have an audio manager set up.
```

BPMChoice is an enum for picking beats per minute. Try adjusting it by using BPMChoice::Custom(number).
Adjust the frequencies, volumes, waves, add more notes, etc... as you like!

# NOTE (VERY IMPORTANT)
This project has undergone a massive version change. A lot of changes have happened, including a new project structure, a much more robust system, and much more. The documentation for this library is incomplete, as it is currently being worked on.


## License
This engine is [MIT licensed](LICENSE). Read LICENSE.md for more details.
