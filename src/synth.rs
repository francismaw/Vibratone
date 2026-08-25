

use crate::note::{NoteEvent, pitch_to_freq};
use crate::note::SAMPLE_RATE;
use crate::body::BodyMode;
use crate::body::apply_body_res;


/// One plucked string: returns `num_samples` of audio at the given frequency.
pub fn pluck(freq: f32, num_samples: usize) -> Vec<f32> { 
    let mut y = vec![0.0f32; num_samples];
    let rho = 0.996;
    //let N = (SAMPLE_RATE as f32 / freq - 0.5).round() as usize;

    let exact_delay = SAMPLE_RATE as f32 / freq;
    let mut N = exact_delay.floor() as usize;
    let mut d = exact_delay - N as f32;

    if d < 0.5 {
        N -= 1;
        d += 1.0;
    }

    let C = (1.0 - d) / (1.0 + d);
    let mut prev_x = 0.0;
    let mut prev_y = 0.0;

    let damp = damp_for_feq(freq);


    for i in 0..N{
        let val = rand::random_range(-1.0 as f32..1.0 as f32);
        y[i] = val;
    }    
    let pickup_pos = 0.20;
    let pickup_delay = (N as f32 * pickup_pos).max(1.0) as usize;


    for _ in 0..2{
        for i in 1..N {
            y[i] = (y[i] + y[i-1]) * 0.5;
        }
    }

    

    for n in N..num_samples{
        let ks_val = if n == N{
            (y[0] * (1.0 - damp) + 0.0 ) * rho
        } else {
          (y[n - N] * (1.0 - damp) + y[n - N - 1 ] * damp )* rho
        };

        let current_x = ks_val;
        let apf_val = C * current_x + prev_x - C * prev_y;
        prev_x = current_x;
        prev_y = apf_val;
        y[n] = apf_val;

    }



    let mut pickup_out = vec![0.0f32; num_samples];
    for i in 0..num_samples{
        let curr_val = y[i];
        let delayed_val = if i >= pickup_delay {y[i - pickup_delay]} else {0.0};
        pickup_out[i] = (curr_val + delayed_val) * 0.5;
    }


    //println!("y[0] = {}, y[N] = {}, y[0]*0.5 = {}", y[0], y[N], y[0] * 0.5);
    pickup_out


}

/// Sum each note's pluck into one buffer at its onset, then normalize to [-1, 1].
pub fn render(notes: &[NoteEvent]) -> Vec<f32> { 

    let master_buffer_length = notes.iter().map(|n| n.onset_samples + n.duration_samples).max().unwrap_or(0);

    let mut master_buffer = vec![0.0; master_buffer_length];

    notes.iter().for_each(|n| {
        let freq = pitch_to_freq(n.pitch);
        let plucked = pluck(freq, n.duration_samples);
        let mut modes = vec![
        BodyMode::new(100.0, 5.0, 0.5, SAMPLE_RATE as f32),
        BodyMode::new(220.0, 8.0, 1.0, SAMPLE_RATE as f32),
        BodyMode::new(700.0, 4.0, 0.3, SAMPLE_RATE as f32),
    ];
    let resonated = apply_body_res(&plucked, &mut modes);
        for i in 0..n.duration_samples{
            master_buffer[n.onset_samples + i] += resonated[i];
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

fn damp_for_feq(freq: f32 ) -> f32 {
    let s_min = 0.35;
    let s_max = 0.65;

    let f_low = 82.0 as f32;
    let f_high = 1300.0 as f32;

    let log_f = freq.log2();
    let log_low = f_low.log2();
    let log_high = f_high.log2();

    let t = ((log_f - log_low) / (log_high - log_low)).clamp(0.0, 1.0);
    s_max + t * (s_min - s_max)
}
