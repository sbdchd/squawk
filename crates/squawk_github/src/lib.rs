#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::single_match_else)]
pub mod actions;
pub mod app;
use std::error::Error;

use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum GithubError {
    JsonWebTokenCreation(jsonwebtoken::errors::Error),
    HttpError(reqwest::Error),
}

impl Error for GithubError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GithubError::JsonWebTokenCreation(err) => Some(err),
            GithubError::HttpError(err) => Some(err),
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
) -> Result<(), GithubError> {
    let comments = gh.list_issue_comments(owner, repo, issue)?;

    let bot_name = gh.app_slug();

    info!("checking for existing comment");
    match comments
        .iter()
        .find(|x| x.user.r#type == "Bot" && x.user.login == bot_name)
    {
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
