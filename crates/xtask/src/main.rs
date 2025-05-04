use anyhow::Result;
// see: https://github.com/matklad/cargo-xtask
use clap::{arg, Args, Parser, Subcommand};
use generate_keywords::generate_keywords;
use new_lint::new_lint;
use sync_kwlist::sync_kwlist;

mod generate_keywords;
mod new_lint;
mod path_util;
mod sync_kwlist;

#[derive(Subcommand, Debug)]
enum TaskName {
    #[command(long_about = "Generate code for keywords using the Postgres kwlist.h")]
    GenerateKeywords,
    #[command(long_about = "Fetch the latest version of kwlist.h from Postgres")]
    SyncKwlist,
    #[command(long_about = "Create a new linter rule")]
    NewLint(NewLintArgs),
}

#[derive(Args, Debug)]
struct NewLintArgs {
    #[arg(required = true)]
    name: String,
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Arguments {
    /// The task to perform
    #[command(subcommand)]
    task: TaskName,
}

fn main() -> Result<()> {
    let args = Arguments::parse();
    match args.task {
        TaskName::GenerateKeywords => generate_keywords(),
        TaskName::SyncKwlist => sync_kwlist(),
        TaskName::NewLint(args) => new_lint(args),
    }
}
