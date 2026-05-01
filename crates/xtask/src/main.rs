use anyhow::Result;
// see: https://github.com/matklad/cargo-xtask
use clap::{Args, Parser, Subcommand};
use codegen::codegen;
use new_rule::new_lint;
use sync_builtins::sync_builtins;
use sync_pg::sync_pg;

mod codegen;
mod keywords;
mod new_rule;
mod path;
mod sync_builtins;
mod sync_pg;

#[derive(Subcommand, Debug)]
enum TaskName {
    #[command(long_about = "Generate code for AST, SyntaxKind, and TokenSets")]
    Codegen,
    #[command(long_about = "Create a new linter rule")]
    NewRule(NewRuleArgs),
    #[command(long_about = "Fetch the latest kwlist.h and regression suite from Postgres")]
    SyncPg,
    #[command(long_about = "Generate builtins.sql from PostgreSQL pg_type catalog")]
    SyncBuiltins,
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
        TaskName::SyncPg => sync_pg(),
        TaskName::NewRule(args) => new_lint(args),
        TaskName::Codegen => codegen(),
        TaskName::SyncBuiltins => sync_builtins(),
    }
}
