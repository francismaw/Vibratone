use crate::note::SAMPLE_RATE;



pub fn apply_vibratone(samples: &[f32], rate_hz: f32)-> Vec<f32>{
    return tremolo(samples, rate_hz);

}

fn tremolo(samples: &[f32], rate_hz: f32) -> Vec<f32>{
    let mut output = Vec::new();
    for (n, &sample) in samples.iter().enumerate() {
        let t = n as f32 / SAMPLE_RATE as f32;
        let theta = 2.0 * std::f32::consts::PI * rate_hz * t;
        let gain = 1.0 + 0.5 * theta.cos();
        output.push(sample * gain);
    }
    output
}

fn dopplar(samples: &[f32], rate_hz: f32) -> Vec<f32>{
    todo!()
}