mod tab;
mod synth;
mod note;

use note::{NoteEvent, SAMPLE_RATE};

fn main(){
    let note = note::pitch_to_freq(69);
    println!("Frequency of A4 (pitch 69) is: {} Hz", note);

}