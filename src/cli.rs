use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands;

#[derive(Parser)]
#[command(name = "base64", version, about = "Base64 encode & decode tool")]
pub struct Cli {
    #[arg(short = 'd', long = "decode")]
    pub decode: bool,

    #[arg(short = 'i', long = "ignore-garbage")]
    pub ignore_garbage: bool,

    #[arg(short = 'w', long = "wrap", default_value_t = 76)]
    pub wrap: usize,

    #[arg(value_name = "FILE")]
    pub files: Vec<String>,

    #[arg(long, hide = true)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(commands::encode::EncodeArgs),
    Decode(commands::decode::DecodeArgs),
}

pub fn run(cli: &Cli) -> Result<()> {
    match &cli.command {
        Some(Commands::Encode(args)) => commands::encode::run(args),
        Some(Commands::Decode(args)) => commands::decode::run(args),
        None => commands::run(cli),
    }
}
