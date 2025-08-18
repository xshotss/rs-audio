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
  let mut song = Song::default(); // Creates a default song with a single sine wave. It is useful for debugging.

  song.play().unwrap(); // Plays the song. This pauses the main thread until the song is finished.

  song.export_to_wav("test.wav".to_string()).unwrap(); // Creates a .wav file containing your song.
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

song.play().unwrap();
```

BPMChoice is an enum for picking beats per minute. Try adjusting it by using BPMChoice::Custom(number).
Adjust the frequencies, volumes, waves, add more notes, etc... as you like!

You can export a song to a .wav file using the "export_to_wav" function
```Rust
song.export_to_wav("helloworld.wav".to_string()).unwrap();
```

You can also play a .wav file directly:
```Rust
play_wav("test.wav").unwrap();
```


### Multithreading
Multithreading allows you to run multiple tasks at once. Rs-audio has native multithreading support using the following command:
```Rust
song.play_from_thread().unwrap();
```

This allows you to do stuff on the main/other thread like rendering, physics, calculations, etc... while still playing audio!
**Note that this feature is quite new and has not recieved much testing yet.**


## License
This engine is [MIT licensed](LICENSE). Read LICENSE.md for more details.
