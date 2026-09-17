use clap::{Parser, Subcommand};
use std::{path::PathBuf, process::Command};


#[derive(Parser)]
#[command(name = "tab-synth")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands{
    /// Synth mode, hear ASCII tabs
    Synth{
        tab_path: PatBuf,
        output: Pathbuf,

        #[arg(long, default_value_t = 120 BPM)]
        tempo: f32
    },
    /// Modify mode. uplaod guitar audio to have it applied
    Process{
        wav_path: PathBuf,
        output: Pathbuf, 

        #[arg(long, default_value_t = 4.0)]
        drive: f32,
        #[arg(long, default_value_t = 5.67)]
        rotor_speed: f32


    },

}

