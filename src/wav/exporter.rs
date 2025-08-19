use std::{
    fs::File,
    io::{BufReader, Error},
};

use hound::{WavSpec, WavWriter};
use rodio::{Decoder, OutputStream, Sink};

use crate::{waveform::generate_sample, Song};



impl Song {
    /**
    Exports a Song struct to a .wav file.<br>
    Usage:
    ```
    use rs_audio::*;
    use rs_audio::{legacyplayer::BasicSong};

    let song = BasicSong::default();
    song.export_to_wav("test.wav".to_string());
    ```

    # Panics
    This function will panic if the file could not be created due to some error.
    */
    pub fn export_to_wav(&self, filename: String) -> Result<(), Box<dyn std::error::Error>> {
        // set up wave file specs
        let spec = WavSpec {
            channels: 1,
            sample_rate: 44100,  // 44.1k Hz
            bits_per_sample: 16, // 16 bit depth
            sample_format: hound::SampleFormat::Int,
        };

        // create writer for writing to files
        let mut writer = match WavWriter::create(filename, spec) {
            Ok(e) => e,
            Err(e) => {
                panic!("RS-AUDIO: Error while creating file: {e}");
            }
        };

        for note in &self.notes {
            let total_samples = (note.dur * 44100.0) as usize;
            for i in 0..total_samples {
                let phase = (i as f64 * note.freq / 44100.0) % 1.0; // generate phase
                let sample = generate_sample(&note.wave, phase) * note.vol as f64; // generate sample from waveform
                writer.write_sample((sample * i16::MAX as f64) as i16)?; // add the sample
            }
        }

        writer.finalize()?;

        Ok(())
    }

    /**
    Plays a .wav file directly.<br>
    Note that `.wav`'s are not converted to Songs in this function due to complexity.
    <br>


    Usage:
    ```
    use rs_audio::*;
    use rs_audio::{player::Song};

    Song::play_wav("test.wav").unwrap();
    ```

    **This function will return an Error if it encounters an error.<br>**
    The recommended way to use it is the following:
    ```
    match Song::play_wav("test.wav") {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
    ```
    */
    pub fn play_wav(file_path: &str) -> Result<(), Error> {
        let (_stream, handle) = match OutputStream::try_default() {
            Ok(e) => e,
            Err(e) => return Err(Error::other(e.to_string())),
        };

        let sink = match Sink::try_new(&handle) {
            Ok(e) => e,

            /* convert PlayError to std::io::Error */
            Err(e) => return Err(Error::other(e.to_string())),
        };

        let file = File::open(file_path)?;

        let source = match Decoder::new(BufReader::new(file)) {
            Ok(e) => e,
            Err(e) => return Err(Error::other(e.to_string()))
        };

        sink.append(source);
        sink.sleep_until_end(); // blocks thread until finished

        Ok(())
    }
}
