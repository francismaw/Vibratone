use crate::note::{NoteEvent, SAMPLE_RATE};

const OPEN_STRINGS: [u8; 6] = [64, 59, 55, 50, 45, 40]; 

const SAMPLES_PER_COLUMN: usize = SAMPLE_RATE as usize / 4;







pub fn parser(tab: &str) -> Vec<NoteEvent>{

}

fn parse_block(lines: &[&str]) -> Vec<NoteEvent>{
    let col = 0;
    //lines = strip_prefixes(lines);
    
    while col < lines.len(){
        for string in OPEN_STRINGS.iter(){
            val = lines.chars().nth(col);
            val_next = lines.chars().nth(col + 1);
            if val == '-' || '|' {
                continue;
            } else if val.is_digit() && val_next.is_digit(){
                let pitch = string + val.to_digit(10).unwrap() as u8;
                let onset_samples = col * SAMPLES_PER_COLUMN;
                let mut duration_samples = 0;
                while lines.chars().nth(col + duration_samples) == '-' || '|' {
                    duration_samples += 1;
                }
                duration_samples *= SAMPLES_PER_COLUMN;
                let note_event = NoteEvent{pitch, onset_samples, duration_samples};
                notes.push(note_event);
                
            } else {
                
            }
        }
    }

} 


fn strip_prefixes(lines:&[&str]) -> &str {
    let trimmed_str: &str = lines.as_slice().trim_chars([' ',','].as_slice());
    trimmed_str
}