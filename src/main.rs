mod tab;
mod synth;
mod note;
mod effects;
use std::io::{self, Read};
use crate::note::{NoteEvent, SAMPLE_RATE};

use hound::{SampleFormat, WavIntoSamples, WavSpec, WavWriter};

fn main(){
    //let note = note::pitch_to_freq(69);
    //println!("Frequency of A4 (pitch 69) is: {} Hz", note);

    //let samples = synth::pluck(220.0, SAMPLE_RATE as usize * 2);

    //println!("Samples at 220 Hz: {:?}", samples);
    // E2; 82.407, A2:  110.0, D3: 146.83, G3: 196.00, B3: 246.94, E4: 329.63



    




    println!("Enter file path: ");
    let mut input = String::new();
    io::stdin()
        .read_line(& mut input)
        .expect("Failed to read line");
    //write_wav(&input.trim(), &samples).expect("Failed to write");
    //note_test(&input.trim());
    tab_test(&input.trim());

}

fn note_test(path: &str){
    let notes = vec![NoteEvent{pitch: 60, onset_samples: 0, duration_samples: SAMPLE_RATE as usize * 2},
    NoteEvent{pitch: 64, onset_samples: SAMPLE_RATE as usize *2, duration_samples: SAMPLE_RATE as usize * 2},
    NoteEvent{pitch: 67, onset_samples: SAMPLE_RATE as usize *4, duration_samples: SAMPLE_RATE as usize * 2}];

    let rendered = synth::render(&notes);
    //write_wav(path, &rendered).expect("Failed to write");
}

fn tab_test(path: &str){
    let tab = "
D|-------|-----------------|-----------------|
A|-------|-----------------|-----------------|
F|-------|-----------------|-----------------|
C|-------|-----------------|-----------------|
G|-------|-----0---0-------|---2-------2-----|
D|-0-0-1-|-2-----2---2-2-1-|-0---0-0-----0-1-|
 
D|-----------------|-----------------|
A|-----------------|-----------------|
F|-----------------|-----------------|
C|-----------------|-----------------|
G|-----0---0-------|---2-------2-----|
D|-2-----2---2-2-1-|-0---0-0-----0-1-|";


    let notes = tab::parser(tab);
    println!("Notes: {:?}", notes);
    let mono  = synth::render(&notes);
    let rate = 5.67; // rate of drum spinning rpm / 60 
    let (left, right) = effects::apply_vibratone(&mono, rate);
    write_wav(path, &left, &right).expect("Failed to write");
    //write_wav(path, &left, &right).expect("Failed to write");
}









fn write_wav(path: &str, left: &[f32], right: &[f32]) -> Result<(), hound::Error>{
    let spec = WavSpec{
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    let mut writer = WavWriter::create(path, spec)?;

    let frames = left.len().min(right.len());
    
    for i in 0..frames{
        writer.write_sample(left[i])?;
        writer.write_sample(right[i])?;

    }

    writer.finalize()
}
