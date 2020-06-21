use crate::github::{comment_on_pr, GithubError, PullRequest};
use crate::reporter::{check_files, get_comment_body, CheckFilesError};
use structopt::StructOpt;

#[derive(Debug)]
pub enum SquawkError {
    CheckFilesError(CheckFilesError),
    GithubError(GithubError),
}

impl std::convert::From<GithubError> for SquawkError {
    fn from(e: GithubError) -> Self {
        Self::GithubError(e)
    }
}

impl std::convert::From<CheckFilesError> for SquawkError {
    fn from(e: CheckFilesError) -> Self {
        Self::CheckFilesError(e)
    }
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Comment on a PR with Squawk's results.
    UploadToGithub {
        /// Paths to search
        paths: Vec<String>,
        /// Exclude specific warnings
        ///
        /// For example:
        /// --exclude=require-concurrent-index-creation,ban-drop-database
        #[structopt(short, long, use_delimiter = true)]
        exclude: Option<Vec<String>>,
        #[structopt(long, env = "GITHUB_PRIVATE_KEY")]
        github_private_key: String,
        /// GitHub App Id.
        #[structopt(long, env = "GITHUB_APP_ID")]
        github_app_id: String,
        /// GitHub Install Id. The installation that squawk is acting on.
        #[structopt(long, env = "GITHUB_INSTALL_ID")]
        github_install_id: String,
        /// GitHub Bot Id. The User id of the bot.
        #[structopt(long, env = "GITHUB_BOT_ID")]
        github_bot_id: String,
        /// GitHub Repo Owner
        /// github.com/sbdchd/squawk, sbdchd is the owner
        #[structopt(long, env = "GITHUB_REPO_OWNER")]
        github_repo_owner: String,
        /// GitHub Repo Name
        /// github.com/sbdchd/squawk, squawk is the name
        #[structopt(long, env = "GITHUB_REPO_NAME")]
        github_repo_name: String,
        /// GitHub Pull Request Number
        /// github.com/sbdchd/squawk/pull/10, 10 is the PR number
        #[structopt(long, env = "GITHUB_PR_NUMBER")]
        github_pr_number: i64,
    },
}

pub fn check_and_comment_on_pr(cmd: Command, is_stdin: bool) -> Result<i32, SquawkError> {
    let Command::UploadToGithub {
        paths,
        exclude,
        github_private_key,
        github_app_id,
        github_install_id,
        github_bot_id,
        github_repo_owner,
        github_repo_name,
        github_pr_number,
    } = cmd;
    let violations = check_files(&paths, is_stdin, exclude)?;
    let exit_code = if !violations.is_empty() { 1 } else { 0 };
    let comment_body = get_comment_body(violations);
    let pr = PullRequest {
        issue: github_pr_number,
        owner: github_repo_owner,
        repo: github_repo_name,
    };
    comment_on_pr(
        &github_private_key,
        &github_app_id,
        &github_install_id,
        &github_bot_id,
        pr,
        comment_body,
    )?;
    Ok(exit_code)
}
