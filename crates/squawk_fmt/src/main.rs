use std::io::{self, Read};
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "squawk-fmt")]
struct Cli {
    /// File to format; reads from stdin if omitted
    file: Option<PathBuf>,

    /// Print the pretty printing IR instead of the formatted SQL
    #[arg(long)]
    ir: bool,
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

    if cli.ir {
        print!("{}", squawk_fmt::fmt_ir(&input)?);
    } else {
        print!("{}", squawk_fmt::fmt(&input)?);
    }
    Ok(())
}
