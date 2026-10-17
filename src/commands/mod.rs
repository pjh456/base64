use anyhow::Result;
use std::io::{Read, Write};

use crate::cli::Cli;

pub mod decode;
pub mod encode;

pub fn run(cli: &Cli) -> Result<()> {
    if cli.files.len() > 1 {
        eprintln!("base64: extra operand '{}'", cli.files[1]);
        eprintln!("Try 'base64 --help' for more information.'");
        std::process::exit(1);
    }

    let wrap: usize = match cli.wrap.parse() {
        Ok(w) => w,
        Err(_) => {
            eprintln!("base64: invalid number of columns: '{}'", cli.wrap);
            std::process::exit(1);
        }
    };

    let input = read_input(&cli.files)?;
    let out = if cli.decode {
        decode::decode_impl(&input, cli.ignore_garbage)?
    } else {
        encode::encode_impl(&input, wrap)?
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
            let mut b =
                std::fs::read(f).map_err(|e| std::io::Error::new(e.kind(), format!("{f}: {e}")))?;
            buf.append(&mut b);
        }
    }
    Ok(buf)
}
