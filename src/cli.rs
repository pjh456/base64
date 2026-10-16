use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands;

#[derive(Parser)]
#[command(name = "base64", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(commands::encode::EncodeArgs),
    Decode(commands::decode::DecodeArgs),
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Encode(args) => commands::encode::run(args),
        Commands::Decode(args) => commands::decode::run(args),
    }
}
