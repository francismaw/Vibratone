

use crate::note::NoteEvent;
use rand::rng;
use crate::note::SAMPLE_RATE;


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

    println!("y[0] = {}, y[N] = {}, y[0]*0.5 = {}", y[0], y[N], y[0] * 0.5);
    y


}

/// Sum each note's pluck into one buffer at its onset, then normalize to [-1, 1].
pub fn render(notes: &[NoteEvent]) -> Vec<f32> { todo!() }
