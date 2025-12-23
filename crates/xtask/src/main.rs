use anyhow::Result;
// see: https://github.com/matklad/cargo-xtask
use clap::{Args, Parser, Subcommand, arg};
use codegen::codegen;
use new_rule::new_lint;
use sync_kwlist::sync_kwlist;
use sync_regression_suite::sync_regression_suite;

mod codegen;
mod keywords;
mod new_rule;
mod path;
mod sync_kwlist;
mod sync_regression_suite;

#[derive(Subcommand, Debug)]
enum TaskName {
    #[command(long_about = "Generate code for AST, SyntaxKind, and TokenSets")]
    Codegen,
    #[command(long_about = "Fetch the latest version of kwlist.h from Postgres")]
    SyncKwlist,
    #[command(long_about = "Create a new linter rule")]
    NewRule(NewRuleArgs),
    #[command(long_about = "Fetch the latest regression suite from Postgres")]
    SyncRegressionSuite,
}

#[derive(Args, Debug)]
struct NewRuleArgs {
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
        TaskName::SyncKwlist => sync_kwlist(),
        TaskName::SyncRegressionSuite => sync_regression_suite(),
        TaskName::NewRule(args) => new_lint(args),
        TaskName::Codegen => codegen(),
    }
}
