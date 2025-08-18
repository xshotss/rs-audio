use std::{fs::File, io::BufReader};

use hound::{WavSpec, WavWriter};
use rodio::{Decoder, OutputStream, Sink};

use crate::{waveform::generate_sample, Song};


impl Song {
/**
     Exports a Song struct to a .wav file.<br>It creates a .wav file in the current directory.<br>
     Usage:
     ```
     let song = Song::default();
     
     song.export_to_wav("test.wav");
     ```
     */
    pub fn export_to_wav(&self, filename: String) -> Result<(), Box<dyn std::error::Error>> {
        // set up wave file specs
        let spec = WavSpec {
            channels: 1,
            sample_rate: 44100, // 44.1k Hz
            bits_per_sample: 16, // 16 bit depth
            sample_format: hound::SampleFormat::Int,
        };

        // create writer for writing to files
        let mut writer = match WavWriter::create(filename, spec) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("RS-AUDIO: Error while creating file: {e}");
                std::process::exit(1);
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


    pub fn play_wav(file_path: &str) {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
    
        let file = File::open(file_path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
    
        sink.append(source);
        sink.sleep_until_end(); // Blocks until finished
    }
}