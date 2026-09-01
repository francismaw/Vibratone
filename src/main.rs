mod tab;
mod synth;
mod note;
mod effects;
mod body;
use std::io::{self};
use crate::note::{SAMPLE_RATE};

use hound::{SampleFormat, WavSpec, WavWriter};

fn main(){
    //let note = note::pitch_to_freq(69);
    //println!("Frequency of A4 (pitch 69) is: {} Hz", note);

    //let samples = synth::pluck(220.0, SAMPLE_RATE as usize * 2);

    //println!("Samples at 220 Hz: {:?}", samples);
    // E2; 82.407, A2:  110.0, D3: 146.83, G3: 196.00, B3: 246.94, E4: 329.63
    println!("Enter input file path: ");
    let mut input = String::new();
    io::stdin()
        .read_line(& mut input)
        .expect("Failed to read line");
        println!("Enter file path: ");

    println!("Enter output file path: ");
    let mut  output = String::new();
    io::stdin()
        .read_line(& mut output)
        .expect("Failed to read line");


    wav_test(input.trim(), output.trim());

}


#[allow(dead_code)]
fn tab_test(path: &str){
    let tab = "
e|-----------|--1--1-----0--|--------------|--1-----------|
B|--1--------|--------------|-----1--1-----|--------------|
G|-----------|--------0-----|--4--2--------|-----2--------|
D|--2--------|-----3-----3--|--------------|-----2--------|
A|--3--------|-----2--2-----|-----0--0--3--|--0--------3--|
E|-----------|--------------|--1-----------|--------0--0--|";


    let notes = tab::parser(tab, 120.0);
    println!("Notes: {:?}", notes);
    let mono  = synth::render(&notes);
    let drive = 4.0;
    let amped = effects::tube_amp(&mono, drive);
    let cab = effects::cabinet_sim(&amped);

    let rate = 5.67; // rate of drum spinning rpm / 60 
    let (left, right) = effects::apply_vibratone(&cab, rate);
    //let left = &cab;
    //let right = &cab;
    write_wav(path, &left, &right).expect("Failed to write");
    //write_wav(path, &left, &right).expect("Failed to write");
}


fn wav_test(in_path: &str, out_path: &str){
    let mono = read_wav(in_path).expect("Failed to read wav");
    let drive = 4.0;
    let amped = effects::tube_amp(&mono, drive);
    let cab = effects::cabinet_sim(&amped);

    let rate = 5.67; // rate of drum spinning rpm / 60 
    let (left, right) = effects::apply_vibratone(&cab, rate);
    write_wav(out_path, &left, &right).expect("Failed to write");



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


fn read_wav(path: &str) -> Result<Vec<f32>, hound::Error>{

    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    assert_eq!(spec.sample_rate, SAMPLE_RATE, "Wav sample rate doesn't match constant");

    let samples: Vec<f32> = match spec.sample_format{
        hound::SampleFormat::Float => {reader.samples::<f32>().map(|s| s.unwrap()).collect()}
        hound::SampleFormat::Int =>{
            let max_val = (1i32 << (spec.bits_per_sample -1)) as f32;
            reader.samples::<i32>().map(|s| s.unwrap() as f32 / max_val).collect()
        }
    };


    let mono = match spec.channels{
        1 => samples,
        2 => down_mix_to_mono(&samples),
        n => panic!("Unsupported number of channels: {}", n),
    };
    Ok(mono)




}


fn down_mix_to_mono(stereo: &[f32]) -> Vec<f32>{
    stereo.chunks_exact(2).map(|chunk| (chunk[0] + chunk[1]) / 2.0).collect()
}
