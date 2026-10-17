use anyhow::Result;
use std::io::{Read, Write};

use crate::cli::Cli;

pub mod decode;
pub mod encode;

pub fn run(cli: &Cli) -> Result<()> {
    let input = read_input(&cli.files)?;
    let out = if cli.decode {
        decode::decode_impl(&input, cli.ignore_garbage)?
    } else {
        encode::encode_impl(&input, cli.wrap)?
    };
    std::io::stdout().write_all(&out)?;
    Ok(())
}

pub fn read_input(files: &[String]) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    if files.is_empty() || files == ["-"] {
        std::io::stdin().read_to_end(&mut buf)?;
    } else {
        for f in files {
            let mut b = std::fs::read(f)?;
            buf.append(&mut b);
        }
    }
    Ok(buf)
}
