use crate::reporter::{check_files, get_comment_body, CheckFilesError};

use log::info;
use squawk_github::{actions, app, comment_on_pr, GithubError};
use squawk_linter::{versions::Version, violations::RuleViolationKind};
use structopt::StructOpt;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub enum SquawkError {
    CheckFilesError(CheckFilesError),
    GithubError(GithubError),
    GithubPrivateKeyBase64DecodeError(base64::DecodeError),
    GithubPrivateKeyDecodeError(std::string::FromUtf8Error),
    GithubPrivateKeyMissing,
    RulesViolatedError,
}

impl std::fmt::Display for SquawkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::CheckFilesError(ref err) => {
                write!(f, "Failed to dump AST: {err}")
            }
            Self::GithubError(ref err) => err.fmt(f),
            Self::GithubPrivateKeyBase64DecodeError(ref err) => write!(
                f,
                "Failed to decode GitHub private key from base64 encoding: {err}"
            ),
            Self::GithubPrivateKeyDecodeError(ref err) => {
                write!(f, "Could not decode GitHub private key to string: {err}")
            }
            Self::GithubPrivateKeyMissing => write!(f, "Missing GitHub private key"),
            Self::RulesViolatedError => write!(f, "Rules were violated"),
        }
    }
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
        exclude: Option<Vec<RuleViolationKind>>,
        /// Exits with an error code when specified
        #[structopt(long)]
        exit_on_error: bool,
        #[structopt(long, env = "SQUAWK_GITHUB_PRIVATE_KEY")]
        github_private_key: Option<String>,
        #[structopt(long, env = "SQUAWK_GITHUB_PRIVATE_KEY_BASE64")]
        github_private_key_base64: Option<String>,
        #[structopt(long, env = "SQUAWK_GITHUB_TOKEN")]
        github_token: Option<String>,
        /// GitHub App Id.
        #[structopt(long, env = "SQUAWK_GITHUB_APP_ID")]
        github_app_id: Option<i64>,
        /// GitHub Install Id. The installation that squawk is acting on.
        #[structopt(long, env = "SQUAWK_GITHUB_INSTALL_ID")]
        github_install_id: Option<i64>,
        /// GitHub Repo Owner
        /// github.com/sbdchd/squawk, sbdchd is the owner
        #[structopt(long, env = "SQUAWK_GITHUB_REPO_OWNER")]
        github_repo_owner: String,
        /// GitHub Repo Name
        /// github.com/sbdchd/squawk, squawk is the name
        #[structopt(long, env = "SQUAWK_GITHUB_REPO_NAME")]
        github_repo_name: String,
        /// GitHub Pull Request Number
        /// github.com/sbdchd/squawk/pull/10, 10 is the PR number
        #[structopt(long, env = "SQUAWK_GITHUB_PR_NUMBER")]
        github_pr_number: i64,
    },
}

fn get_github_private_key(
    github_private_key: Option<String>,
    github_private_key_base64: Option<String>,
) -> Result<String, SquawkError> {
    if let Some(private_key) = github_private_key {
        Ok(private_key)
    } else {
        let key = github_private_key_base64.ok_or(SquawkError::GithubPrivateKeyMissing)?;
        let bytes = base64::decode(key).map_err(SquawkError::GithubPrivateKeyBase64DecodeError)?;
        Ok(String::from_utf8(bytes).map_err(SquawkError::GithubPrivateKeyDecodeError)?)
    }
}

fn concat(a: &[RuleViolationKind], b: &[RuleViolationKind]) -> Vec<RuleViolationKind> {
    // from: https://stackoverflow.com/a/53476705/3720597
    [a, b].concat()
}

pub fn check_and_comment_on_pr(
    cmd: Command,
    is_stdin: bool,
    stdin_path: Option<String>,
    root_cmd_exclude: &[RuleViolationKind],
    pg_version: Option<Version>,
    assume_in_transaction: bool,
) -> Result<(), SquawkError> {
    let Command::UploadToGithub {
        paths,
        exclude,
        exit_on_error,
        github_private_key,
        github_token,
        github_app_id,
        github_install_id,
        github_repo_owner,
        github_repo_name,
        github_pr_number,
        github_private_key_base64,
    } = cmd;
    info!("checking files");
    let file_results = check_files(
        &paths,
        is_stdin,
        stdin_path,
        &concat(&exclude.unwrap_or_default(), root_cmd_exclude),
        pg_version,
        assume_in_transaction,
    )?;
    let is_empty = file_results.is_empty();
    if is_empty {
        info!("no files checked, exiting");
        return Ok(());
    }
    info!("generating github comment body");
    let comment_body = get_comment_body(file_results, VERSION);

    if let Some(github_install_id) = github_install_id {
        if let Some(github_app_id) = github_app_id {
            info!("using github app client");
            let gh_private_key =
                get_github_private_key(github_private_key, github_private_key_base64)?;
            let gh = app::GitHub::new(&gh_private_key, github_app_id, github_install_id)?;

            comment_on_pr(
                &gh,
                &github_repo_owner,
                &github_repo_name,
                github_pr_number,
                &comment_body,
            )?;
        }
    }
    if let Some(github_token) = github_token {
        info!("using github actions client");
        let gh = actions::GitHub::new(&github_token);
        comment_on_pr(
            &gh,
            &github_repo_owner,
            &github_repo_name,
            github_pr_number,
            &comment_body,
        )?;
    }

    if !is_empty && exit_on_error {
        return Err(SquawkError::RulesViolatedError);
    }

    Ok(())
}
