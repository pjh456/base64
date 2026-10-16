use anyhow::{Context, Result};
use clap::Args;

#[derive(Args)]
pub struct DecodeArgs {
    pub raw: String,
}

pub fn run(args: DecodeArgs, _verbose: bool) -> Result<()> {
    println!("Decoding string: {}", args.raw);
    Ok(())
}
