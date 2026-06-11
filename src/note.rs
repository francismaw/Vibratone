pub const SAMPLE_RATE: u32 = 44_100;

pub struct NoteEvent {
    pub pitch: u8,            
    pub onset_samples: usize,
    pub duration_samples: usize,
}

pub fn pitch_to_freq(pitch: u8) -> f32 { 


   let mut freq = 440.0 * 2.0_f32.powf((pitch - 69) as f32/ 12.0);

   freq
}

