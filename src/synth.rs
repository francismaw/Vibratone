
/// One plucked string: returns `num_samples` of audio at the given frequency.
pub fn pluck(freq: f32, num_samples: usize) -> Vec<f32> { 
    
    todo!() 
}

/// Sum each note's pluck into one buffer at its onset, then normalize to [-1, 1].
pub fn render(notes: &[NoteEvent]) -> Vec<f32> { todo!() }
