use crate::note::{NoteEvent, SAMPLE_RATE};

const OPEN_STRINGS: [u8; 6] = [64, 59, 55, 50, 45, 40]; 

const SAMPLES_PER_COLUMN: usize = SAMPLE_RATE as usize / 4;







pub fn parser(tab: &str) -> Vec<NoteEvent>{
    let lines: Vec<&str> = tab.lines().collect();
    let stripped_lines = strip_prefixes(&lines);
    let notes = parse_block(&stripped_lines);
    return notes;
}

fn parse_block(lines: &[&str]) -> Vec<NoteEvent>{
    let mut notes = Vec::new();
    for string in 0..6{
        let mut col = 0;
        while col < lines[0].len(){
            let bytes = lines[string].as_bytes();
            let c = bytes[col];
            //let c_next = bytes.get(col + 1);
            if c.is_ascii_digit() {
                let pitch = OPEN_STRINGS[string] + (c - b'0');
                let onset_samples = col * SAMPLES_PER_COLUMN;
                let  duration_samples = (SAMPLE_RATE * 2) as usize;
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