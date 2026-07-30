

use crate::note::{self, NoteEvent, pitch_to_freq};
use rand::rng;
use crate::note::SAMPLE_RATE;
use crate::effects::apply_vibratone;


/// One plucked string: returns `num_samples` of audio at the given frequency.
pub fn pluck(freq: f32, num_samples: usize) -> Vec<f32> { 
    let mut y = vec![0.0f32; num_samples];

    let N = (SAMPLE_RATE as f32 / freq - 0.5).round() as usize;
    //let mut rng = rand::rng(); 
    for i in 0..N{
        let val = rand::random_range(-1.0 as f32..1.0 as f32);
        y[i] = val;
    }

    for n in N..num_samples{
        if n == N{
            y[N] = y[0] * 0.5;
        } else {
          y[n] = (y[n - N] + y[n - N - 1 ]) * 0.5;
        }
    }

    //println!("y[0] = {}, y[N] = {}, y[0]*0.5 = {}", y[0], y[N], y[0] * 0.5);
    y


}

/// Sum each note's pluck into one buffer at its onset, then normalize to [-1, 1].
pub fn render(notes: &[NoteEvent]) -> Vec<f32> { 

    let master_buffer_length = notes.iter().map(|n| n.onset_samples + n.duration_samples).max().unwrap_or(0);

    let mut master_buffer = vec![0.0; master_buffer_length];

    notes.iter().for_each(|n| {
        let freq = pitch_to_freq(n.pitch);
        let plucked = pluck(freq, n.duration_samples);
        for i in 0..n.duration_samples{
            master_buffer[n.onset_samples + i] += plucked[i];
        }
    });
    let mut max = 0.0;
    for &item in master_buffer.iter() {
        if item.abs() > max {
            max = item.abs();
        }
    }

    if max > 1.0 {
        for i in 0..master_buffer.len(){
            master_buffer[i] = master_buffer[i] / max;
        }
    }

    master_buffer


}
