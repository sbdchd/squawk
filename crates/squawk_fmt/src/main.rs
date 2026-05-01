use std::io::{self, Read};
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "squawk-fmt")]
struct Cli {
    /// File to format; reads from stdin if omitted
    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let input = match cli.file {
        Some(path) => std::fs::read_to_string(&path)?,
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    print!("{}", squawk_fmt::fmt(&input));
    Ok(())
}
