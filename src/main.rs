mod cli;
mod commands;

use clap::Parser;

use crate::cli::Cli;

fn main() {
    let cli = Cli::parse();
    if let Err(err) = cli::run(&cli) {
        if cli.debug {
            eprintln!("base64: {err}");
        } else if let Some(io) = err.downcast_ref::<std::io::Error>() {
            if io.kind() == std::io::ErrorKind::BrokenPipe {
                std::process::exit(141);
            }
            eprintln!("base64: {io}");
        } else {
            eprintln!("base64: invalid input");
        }
        std::process::exit(1);
    }
}
