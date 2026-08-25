
pub struct BodyMode {
    b0: f32, b2: f32, a1: f32, a2: f32,
    x1: f32, x2: f32, y1: f32, y2: f32,
    gain: f32,
}

impl BodyMode {
    pub fn new(f0: f32, q: f32, gain: f32, sample_rate: f32) -> Self {
        // Implementation for creating a new BodyMode instance
       let w_0 = 2.0 * std::f32::consts::PI * f0 / sample_rate;
        let alpha = w_0.sin() / (2.0 * q);

        let b0 = alpha / (1.0 + alpha);
        let b2 = -alpha / (1.0 + alpha);
        let a1 = -2.0 * w_0.cos() / (1.0 + alpha);
        let a2 = (1.0 - alpha) / (1.0 + alpha);

        Self { b0, b2, a1, a2, x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0, gain }

    }


    pub fn process_sample(&mut self, x:f32) -> f32{
       let y = self.b0 * x + self.b2 * self.x2 - self.a1 *self.y1 - self.a2 *self.y2;
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;
        y * self.gain
    }
}



pub fn apply_body_res(samples: &[f32], modes: &mut [BodyMode]) -> Vec<f32> {
    let mut output = Vec::new();
    for sample in samples.iter(){
        let mut result = 0.0;
        for mode in modes.iter_mut(){
            result += BodyMode::process_sample(mode, *sample);
        }
        output.push(result);
    }
    output
}