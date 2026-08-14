use crate::note::{NoteEvent, SAMPLE_RATE};
//standard tuning in MIDI note numbers: EADGBE
//const OPEN_STRINGS: [u8; 6] = [64, 59, 55, 50, 45, 40]; 
const OPEN_STRINGS: [u8; 6] = [62, 57, 53, 48, 43, 38];

const SAMPLES_PER_COLUMN: usize = SAMPLE_RATE as usize / 4; // Assuming 4 columns per second (quarter notes at 60 BPM)







pub fn parser(tab: &str, bpm: f32) -> Vec<NoteEvent>{
    let beat_per_sec = bpm / 60.0;
    let quarter_note_sample = SAMPLE_RATE as f32 / beat_per_sec;
    let eigth_note_sample = (quarter_note_sample / 2.0) as usize;

    let lines: Vec<&str> = tab.lines().filter(|l| !l.trim().is_empty()).collect();
    let stripped_lines = strip_prefixes(&lines);
    let chunks = stripped_lines.chunks(6);
    let mut notes = Vec::new();
    let mut col_offset = 0;
    for chunk in chunks {
        let chunk_notes = parse_block(chunk, col_offset, eigth_note_sample);
        notes.extend(chunk_notes);
        col_offset += chunk[0].len();
    }
    return notes;
}
///path with lots of 12s will run long as its column walked so rythems can be messed up fixable perhaps
fn parse_block(lines: &[&str], col_offset: usize, step_sample: usize) -> Vec<NoteEvent>{
    let mut notes = Vec::new();
    for string in 0..lines.len(){
        let mut col = 0;
        while col < lines[0].len(){
            let bytes = lines[string].as_bytes();
            let c = bytes[col];
            let c_next = bytes.get(col + 1).copied().unwrap_or(b' ');
            if c.is_ascii_digit() && c_next.is_ascii_digit(){
                let pitch = OPEN_STRINGS[string] + (c - b'0') * 10 + (c_next - b'0');
                let onset_samples = (col + col_offset) * SAMPLES_PER_COLUMN;
                let duration_samples = step_sample * 4;
                notes.push(NoteEvent{pitch, onset_samples, duration_samples});
                col += 1; 
            }
            else if c.is_ascii_digit() {
                let pitch = OPEN_STRINGS[string] + (c - b'0');
                let onset_samples = (col + col_offset) * SAMPLES_PER_COLUMN;
                let  duration_samples = step_sample *4 ;
                notes.push(NoteEvent{pitch, onset_samples, duration_samples});
            }
            
            col += 1;
        }
        
    }
    return  notes;

}


fn strip_prefixes<'a>(lines:&[&'a str]) -> Vec<&'a str> {
    lines.iter()
        .map(|line| line.split_once('|').map(|(_, after)| after).unwrap_or(line))
        .collect()

        
}