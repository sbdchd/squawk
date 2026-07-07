use anyhow::Result;
// see: https://github.com/matklad/cargo-xtask
use clap::{Args, Parser, Subcommand};
use codegen::codegen;
use lsp_eval::lsp_eval;
use new_rule::new_lint;
use sync_builtins::sync_builtins;
use sync_pg::sync_pg;
use update_version::{UpdateVersionArgs, update_version};

mod codegen;
mod keywords;
mod lsp_eval;
mod new_rule;
mod path;
mod sync_builtins;
mod sync_pg;
mod update_version;

#[derive(Subcommand, Debug)]
enum TaskName {
    #[command(long_about = "Generate code for AST, SyntaxKind, and TokenSets")]
    Codegen,
    #[command(
        long_about = "Evaluate goto-definition on a snippet against the live LSP server and render it"
    )]
    LspEval(LspEvalArgs),
    #[command(long_about = "Create a new linter rule")]
    NewRule(NewRuleArgs),
    #[command(long_about = "Generate builtins.sql from PostgreSQL pg_type catalog")]
    SyncBuiltins,
    #[command(long_about = "Fetch the latest kwlist.h and regression suite from Postgres")]
    SyncPg,
    #[command(long_about = "Bump the squawk version across all version-bearing files")]
    UpdateVersion(UpdateVersionArgs),
}

#[derive(Args, Debug)]
struct NewRuleArgs {
    #[arg(required = true)]
    name: String,
}

#[derive(Args, Debug)]
pub(crate) struct LspEvalArgs {
    /// SQL snippet with a `$0` marker. If omitted, the snippet is read from stdin.
    sql: Option<String>,
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
        TaskName::Codegen => codegen(),
        TaskName::LspEval(args) => lsp_eval(args),
        TaskName::NewRule(args) => new_lint(args),
        TaskName::SyncBuiltins => sync_builtins(),
        TaskName::SyncPg => sync_pg(),
        TaskName::UpdateVersion(args) => update_version(args),
    }
}
