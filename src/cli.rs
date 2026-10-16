use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands;

#[derive(Parser)]
#[command(name = "base64", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(commands::encode::EncodeArgs),
    Decode(commands::decode::DecodeArgs),
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Encode(args) => commands::encode::run(args, cli.verbose),
        Commands::Decode(args) => commands::decode::run(args, cli.verbose),
    }
}
