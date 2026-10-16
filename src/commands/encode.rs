use anyhow::{Context, Result};
use clap::Args;

#[derive(Args)]
pub struct EncodeArgs {
    pub raw: String,
}

pub fn run(args: EncodeArgs, _verbose: bool) -> Result<()> {
    println!("Encoding String: {}", args.raw);
    Ok(())
}
