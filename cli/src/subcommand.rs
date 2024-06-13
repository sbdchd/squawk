#![allow(clippy::too_many_arguments)]
use crate::config::Config;
use crate::{
    file_finding::{find_paths, FindFilesError},
    reporter::{check_files, get_comment_body, CheckFilesError},
};
use log::info;
use squawk_github::{actions, app, comment_on_pr, GitHubApi, GithubError};
use squawk_linter::{versions::Version, violations::RuleViolationKind};
use structopt::StructOpt;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub enum SquawkError {
    CheckFilesError(CheckFilesError),
    FindFilesError(FindFilesError),
    GithubError(GithubError),
    GithubPrivateKeyBase64DecodeError(base64::DecodeError),
    GithubPrivateKeyDecodeError(std::string::FromUtf8Error),
    GithubPrivateKeyMissing,
    GitHubCredentialsMissing,
    RulesViolatedError { violations: usize, files: usize },
}

impl std::fmt::Display for SquawkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::CheckFilesError(ref err) => {
                write!(f, "Failed to dump AST: {err}")
            }
            Self::FindFilesError(ref err) => {
                write!(f, "Failed to find files: {err}")
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
            Self::GitHubCredentialsMissing => write!(
                f,
                "Missing GitHub credentials:

For a GitHub token:
--github-token is required

For a GitHub App:
--github-app-id is required
--github-install-id is required
--github-private-key or --github-private-key-base64 is required
"
            ),
            Self::RulesViolatedError { violations, files } => {
                write!(f, "Found {violations} violation(s) across {files} file(s)")
            }
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
impl std::convert::From<FindFilesError> for SquawkError {
    fn from(e: FindFilesError) -> Self {
        Self::FindFilesError(e)
    }
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Comment on a PR with Squawk's results.
    UploadToGithub {
        /// Paths to search
        paths: Vec<String>,
        /// Exits with an error if violations are found
        #[structopt(long)]
        fail_on_violations: bool,
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

fn create_gh_app(
    github_install_id: Option<i64>,
    github_app_id: Option<i64>,
    github_token: Option<String>,
    github_private_key: Option<String>,
    github_private_key_base64: Option<String>,
) -> Result<Box<dyn GitHubApi>, SquawkError> {
    if let Some(github_install_id) = github_install_id {
        if let Some(github_app_id) = github_app_id {
            info!("using github app client");
            let gh_private_key =
                get_github_private_key(github_private_key, github_private_key_base64)?;
            return Ok(Box::new(app::GitHub::new(
                &gh_private_key,
                github_app_id,
                github_install_id,
            )?));
        }
    }

    if let Some(github_token) = github_token {
        info!("using github actions client");
        return Ok(Box::new(actions::GitHub::new(&github_token)));
    };
    Err(SquawkError::GitHubCredentialsMissing)
}

pub fn check_and_comment_on_pr(
    cmd: Command,
    cfg: &Config,
    is_stdin: bool,
    stdin_path: Option<String>,
    exclude: &[RuleViolationKind],
    exclude_paths: &[String],
    pg_version: Option<Version>,
    assume_in_transaction: bool,
) -> Result<(), SquawkError> {
    let Command::UploadToGithub {
        paths,
        fail_on_violations,
        github_private_key,
        github_token,
        github_app_id,
        github_install_id,
        github_repo_owner,
        github_repo_name,
        github_pr_number,
        github_private_key_base64,
    } = cmd;

    let fail_on_violations =
        if let Some(fail_on_violations_cfg) = cfg.upload_to_github.fail_on_violations {
            fail_on_violations_cfg
        } else {
            fail_on_violations
        };

    let github_app = create_gh_app(
        github_install_id,
        github_app_id,
        github_token,
        github_private_key,
        github_private_key_base64,
    )?;

    let found_paths = find_paths(&paths, exclude_paths)?;

    info!("checking files");
    let file_results = check_files(
        &found_paths,
        is_stdin,
        stdin_path,
        exclude,
        pg_version,
        assume_in_transaction,
    )?;

    // We should only leave a comment when there are files checked.
    if paths.is_empty() {
        info!("no files checked, exiting");
        return Ok(());
    }
    info!("generating github comment body");
    let comment_body = get_comment_body(&file_results, VERSION);

    comment_on_pr(
        github_app.as_ref(),
        &github_repo_owner,
        &github_repo_name,
        github_pr_number,
        &comment_body,
    )?;

    let violations: usize = file_results.iter().map(|f| f.violations.len()).sum();

    if fail_on_violations && violations > 0 {
        return Err(SquawkError::RulesViolatedError {
            violations,
            files: file_results.len(),
        });
    }

    Ok(())
}
