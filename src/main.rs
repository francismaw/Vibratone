mod tab;
mod synth;
mod note;
use std::io::{self, Read};
use crate::note::{NoteEvent, SAMPLE_RATE};

use hound::{SampleFormat, WavIntoSamples, WavSpec, WavWriter};

fn main(){
    let note = note::pitch_to_freq(69);
    println!("Frequency of A4 (pitch 69) is: {} Hz", note);

    //let samples = synth::pluck(220.0, SAMPLE_RATE as usize * 2);

    //println!("Samples at 220 Hz: {:?}", samples);
    // E2; 82.407, A2:  110.0, D3: 146.83, G3: 196.00, B3: 246.94, E4: 329.63



    




    println!("Enter file path: ");
    let mut input = String::new();
    io::stdin()
        .read_line(& mut input)
        .expect("Failed to read line");
    //write_wav(&input.trim(), &samples).expect("Failed to write");
    note_test(&input.trim());

}

fn note_test(path: &str){
    let notes = vec![NoteEvent{pitch: 60, onset_samples: 0, duration_samples: SAMPLE_RATE as usize * 2},
    NoteEvent{pitch: 64, onset_samples: SAMPLE_RATE as usize *2, duration_samples: SAMPLE_RATE as usize * 2},
    NoteEvent{pitch: 67, onset_samples: SAMPLE_RATE as usize *4, duration_samples: SAMPLE_RATE as usize * 2}];

    let rendered = synth::render(&notes);
    write_wav(path, &rendered).expect("Failed to write");
    }
















fn write_wav(path: &str, samples: &[f32]) -> Result<(), hound::Error>{
    let spec = WavSpec{
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    let mut writer = WavWriter::create(path, spec)?;
    let extra_samples = SAMPLE_RATE * 2;
    
    for data in samples{
        let scaled_data = *data;
        writer.write_sample(scaled_data)?;
    }
    for _ in 0..extra_samples {
        writer.write_sample(0.0f32).unwrap();
    }

    writer.finalize()
}
