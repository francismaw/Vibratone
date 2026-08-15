use core::f32;

use rand::seq::index::sample;

use crate::note::SAMPLE_RATE;



pub fn apply_vibratone(samples: &[f32], rate_hz: f32)-> (Vec<f32>, Vec<f32>){

    let pi = std::f32::consts::PI;

    let mut left = lowpass(&tremolo(&delay_doppler(samples, rate_hz, 0.0), rate_hz, 0.0), rate_hz, 0.0);
    let mut right  = lowpass(&tremolo(&delay_doppler(samples, rate_hz, pi), rate_hz, pi), rate_hz, pi);

    let n = left.len().min(right.len());
    left.truncate(n);
    right.truncate(n);

    let peak = left.iter().chain(right.iter())
        .map(|s| s.abs()) 
        .fold(0.0 , f32::max);
    if peak > 1.0{
        for s in left.iter_mut().chain(right.iter_mut()){
        *s /= peak;
        }
    }
    let output = (left, right);

    return output

}

fn tremolo(samples: &[f32], rate_hz: f32, phase: f32) -> Vec<f32>{
    let mut output = Vec::new();
    for (n, &sample) in samples.iter().enumerate() {
        let t = n as f32 / SAMPLE_RATE as f32;
        let theta = 2.0 * std::f32::consts::PI * rate_hz * t;
        let gain = 1.0 + 0.5 * (theta + phase).sin();
        output.push(sample * gain);
    }
    output
}

fn doppler(samples: &[f32], rate_hz: f32, phase: f32) -> Vec<f32>{
    let mut output: Vec<f32> = Vec::with_capacity(samples.len());
    let mut n = 0;
    let mut read_pos = 0.0;
    let v = 343.0;
    let v_s = 0.254 as f32 * rate_hz * std::f32::consts::PI;


    while read_pos < (samples.len() - 1) as f32{
        let t = n as f32 / SAMPLE_RATE as f32;
        let theta = 2.0 * std::f32::consts::PI * rate_hz * t;
        let step = v / (v - v_s * (theta + phase).cos() );
        output.push(sample_at(samples, read_pos));
        read_pos += step;
        n += 1;
    }
    output
}


fn delay_doppler(samples: &[f32], rate_hz: f32, phase: f32) -> Vec<f32>{
    let radius = 0.381 / 2.0;
    let mut output: Vec<f32> = Vec::with_capacity(samples.len());
    let d_center = 400.0;
    let d_depth = radius / 343.0 * SAMPLE_RATE as f32; 
    let mut read_pos = 0.0;

    for n in 0..samples.len(){
        let t = n as f32 / SAMPLE_RATE as f32;
        let theta = 2.0 * std::f32::consts::PI * rate_hz * (t + phase);

        let d = d_center + d_depth * theta.cos();
        read_pos = n as f32 - d;
        if read_pos < 0.0 {
            output.push(0.0);
        }
        else {
            output.push(sample_at(samples, read_pos));
        }
    }

    output
}

fn sample_at(input: &[f32], read_pos: f32) -> f32{
    let i = read_pos.floor() as usize;
    let frac = read_pos - i as f32;
    let out = input[i] * (1 as f32 - frac) + input[i + 1] * frac;
    out
    
}


fn lowpass(samples: &[f32], rate_hz: f32, phase: f32) -> Vec<f32>{
    let mut prev = 0.0;
    let a_center = 0.5;
    let a_depth = 0.3;
    let mut output: Vec<f32> = Vec::with_capacity(samples.len());
    for n in 0..samples.len(){
        let t = n as f32 / SAMPLE_RATE as f32;
        let theta = 2.0 * std::f32::consts::PI * rate_hz * (t + phase);
        let alpha = a_center + a_depth * theta.cos();
        let y = prev + alpha * (samples[n] - prev);
        output.push(y);
        prev = y;
    }
    output

}

pub fn tube_amp (samples: &[f32], drive: f32) -> Vec<f32>{
    let mut output: Vec<f32> = Vec::with_capacity(samples.len());

    let gain = 1.0 / drive.tanh();

    for &sample in samples{
        let shape = (sample * drive).tanh() * gain;
        output.push(shape);
    }
    output

}


pub fn cabinet_sim(samples: &[f32]) -> Vec<f32>{
    let mut output: Vec<f32> = Vec::with_capacity(samples.len());
    let mut prev = 0.0;
    let alpha = 0.25;

    for &sample in samples{
        let y = prev + alpha * (sample - prev);
        output.push(y);
        prev = y;
    }
    output

}

