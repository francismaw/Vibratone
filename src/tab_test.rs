use anyhow::Ok;

use crate::note::{NoteEvent, SAMPLE_RATE};
//standard tuning in MIDI note numbers: EADGBE
const OPEN_STRINGS: [u8; 6] = [64, 59, 55, 50, 45, 40]; 
//const OPEN_STRINGS: [u8; 6] = [62, 57, 53, 48, 43, 38];


struct RawEvent{ string: usize, fret: usize, col: u8}

struct Grid{cols_per_slot: usize, phase: usize}


fn split_measure<'a>(lines: &[&'a str]) ->anyhow::Result<Vec<Vec<&'a str>>> {
    let per_string: Vec<Vec<&'a str>> = lines.iter()
        .map(|line| line.trim().split('|').skip(1).filter(|seg| !seg.is_empty()).collect::<Vec<&'a str>>())
        .collect();
    anyhow::ensure!(!per_string.is_empty(), "no tab lines");
    let count = per_string[0].len();
    anyhow::ensure!(per_string.iter().all(|s| s.len() == count), "inconsistent number of columns per string");

    Ok((0..count) .map(|m| per_string.iter().map(|s| s[m]).collect()).collect())

} 

fn extract_events(measure:&[&str]) -> Vec<RawEvent>{
    todo!()
}


fn infer_grid(measures: &[Vec<RawEvent>]) -> Grid {
    todo!()
}

fn slot_of(col: usize, grid: &Grid ) -> usize {
    todo!()
}


fn build_notes(measures: Vec<Vec<RawEvent>, grid: Grid, slot_samples: usize) -> Vec<NoteEvent>{
    todo!()
}