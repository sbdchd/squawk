#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::single_match_else)]
pub mod actions;
pub mod app;
use std::error::Error;

use log::info;
use serde::{Deserialize, Serialize};

pub(crate) const DEFAULT_GITHUB_API_URL: &'static str = "https://api.github.com";

#[derive(Debug)]
pub enum GithubError {
    JsonWebTokenCreation(jsonwebtoken::errors::Error),
    HttpError(reqwest::Error),
    CommentTooLarge(String),
}

impl Error for GithubError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GithubError::JsonWebTokenCreation(err) => Some(err),
            GithubError::HttpError(err) => Some(err),
            GithubError::CommentTooLarge(_) => None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Comment {
    pub id: i64,
    pub url: String,
    pub html_url: String,
    pub body: String,
    pub user: User,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub login: String,
    pub r#type: String,
}

pub trait GitHubApi {
    fn app_slug(&self) -> String;
    fn create_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        issue_id: i64,
        body: &str,
    ) -> Result<(), GithubError>;
    fn list_issue_comments(
        &self,
        owner: &str,
        repo: &str,
        issue_id: i64,
    ) -> Result<Vec<Comment>, GithubError>;
    fn update_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &str,
    ) -> Result<(), GithubError>;
}

pub fn comment_on_pr(
    gh: &dyn GitHubApi,
    owner: &str,
    repo: &str,
    issue: i64,
    body: &str,
    existing_comment_text_includes: &str,
) -> Result<(), GithubError> {
    let comments = gh.list_issue_comments(owner, repo, issue)?;

    let bot_name = gh.app_slug();

    info!("checking for existing comment");
    match comments.iter().find(|x| {
        x.user.r#type == "Bot"
            && x.user.login == bot_name
            // NOTE: We filter comments by their contents so we don't accidentally
            // overwrite a comment made by some other tool. This happens often in
            // GitHub repos that reuse the default GHA bot for all linters.
            //
            // This only works if `existing_comment_text_includes` is a "stable"
            // piece of text included in all comments made by squawk!
            && x.body.contains(existing_comment_text_includes)
    }) {
        Some(prev_comment) => {
            info!("updating comment");
            gh.update_issue_comment(owner, repo, prev_comment.id, body)
        }
        None => {
            info!("creating comment");
            gh.create_issue_comment(owner, repo, issue, body)
        }
    }
}
