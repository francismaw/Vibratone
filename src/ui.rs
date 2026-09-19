use clap::{Parser, Subcommand};
use std::{path::PathBuf};


#[derive(Parser)]
#[command(name = "tab-synth")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand)]
pub enum Commands{
    /// Synth mode, hear ASCII tabs
    Synth{
        tab_path: PathBuf,
        output: PathBuf,

        #[arg(long, default_value_t = 120.0)]
        tempo: f32
    },
    /// Modify mode. uplaod guitar audio to have it applied
    Process{
        wav_path: PathBuf,
        output: PathBuf, 

        #[arg(long, default_value_t = 4.0)]
        drive: f32,
        #[arg(long, default_value_t = 5.67)]
        rotor_speed: f32


    },

}

