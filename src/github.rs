use jsonwebtoken::{Algorithm, EncodingKey, Header};
use reqwest::header::{ACCEPT, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
struct CommentBody {
    pub body: String,
}

#[derive(Debug)]
pub struct CommentArgs {
    pub owner: String,
    pub repo: String,
    pub issue: i64,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubAccessToken {
    pub expires_at: String,
    pub permissions: Value,
    pub repository_selection: String,
    pub token: String,
}

/// https://developer.github.com/v3/apps/#create-an-installation-access-token-for-an-app
fn create_access_token(jwt: &str, install_id: &str) -> Result<GithubAccessToken, GithubError> {
    reqwest::Client::new()
        .post(&format!(
            "https://api.github.com/app/installations/{install_id}/access_tokens",
            install_id = install_id
        ))
        .header(AUTHORIZATION, format!("Bearer {}", jwt))
        .header(ACCEPT, "application/vnd.github.machine-man-preview+json")
        .send()
        .map_err(|_| GithubError::Unknown)?
        .json::<GithubAccessToken>()
        .map_err(|_| GithubError::Unknown)
}

/// https://developer.github.com/v3/issues/comments/#create-an-issue-comment
fn create_comment(comment: CommentArgs, secret: &str) -> Result<Value, GithubError> {
    let comment_body = CommentBody { body: comment.body };
    reqwest::Client::new()
        .post(&format!(
            "https://api.github.com/repos/{owner}/{repo}/issues/{issue_number}/comments",
            owner = comment.owner,
            repo = comment.repo,
            issue_number = comment.issue
        ))
        .header(AUTHORIZATION, format!("Bearer {}", secret))
        .json(&comment_body)
        .send()
        .map_err(|_| GithubError::Unknown)?
        .json::<Value>()
        .map_err(|_| GithubError::Unknown)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub login: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Comment {
    pub id: i64,
    pub url: String,
    pub html_url: String,
    pub body: String,
    pub user: User,
}

#[derive(Debug)]
pub enum GithubError {
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claim {
    /// Issued at
    iat: u64,
    /// Expiration time
    exp: u64,
    /// Issuer
    iss: String,
}

/// Create an authentication token to make application requests.
/// https://developer.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app
/// This is different from authenticating as an installation
fn generate_jwt(
    private_key: &str,
    app_identifier: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now_unix_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("problem getting current time");
    let claim = Claim {
        iat: now_unix_time.as_secs(),
        exp: (now_unix_time + Duration::from_secs(10 * 60)).as_secs(),
        iss: app_identifier.into(),
    };

    jsonwebtoken::encode(
        &Header::new(Algorithm::RS256),
        &claim,
        &EncodingKey::from_rsa_pem(private_key.as_ref())?,
    )
}

/// https://developer.github.com/v3/issues/comments/#list-issue-comments
fn list_comments(pr: &PullRequest, secret: &str) -> Result<Vec<Comment>, GithubError> {
    // TODO(sbdchd): use the next links to get _all_ the comments
    // see: https://developer.github.com/v3/guides/traversing-with-pagination/
    reqwest::Client::new()
        .get(&format!(
            "https://api.github.com/repos/{owner}/{repo}/issues/{issue_number}/comments",
            owner = pr.owner,
            repo = pr.repo,
            issue_number = pr.issue
        ))
        .query(&[("per_page", 100)])
        .header(AUTHORIZATION, format!("Bearer {}", secret))
        .send()
        .map_err(|_| GithubError::Unknown)?
        .json::<Vec<Comment>>()
        .map_err(|_| GithubError::Unknown)
}

/// https://developer.github.com/v3/issues/comments/#update-an-issue-comment
fn update_comment(
    owner: &str,
    repo: &str,
    comment_id: i64,
    body: String,
    secret: &str,
) -> Result<Value, GithubError> {
    reqwest::Client::new()
        .patch(&format!(
            "https://api.github.com/repos/{owner}/{repo}/issues/comments/{comment_id}",
            owner = owner,
            repo = repo,
            comment_id = comment_id
        ))
        .header(AUTHORIZATION, format!("Bearer {}", secret))
        .json(&CommentBody { body })
        .send()
        .map_err(|_| GithubError::Unknown)?
        .json::<Value>()
        .map_err(|_| GithubError::Unknown)
}

pub struct PullRequest {
    pub owner: String,
    pub repo: String,
    pub issue: i64,
}

pub fn comment_on_pr(
    private_key: &str,
    app_id: &str,
    install_id: &str,
    bot_id: &str,
    pr: PullRequest,
    comment_body: String,
) -> Result<Value, GithubError> {
    let jwt = generate_jwt(private_key, app_id).expect("successfully generated jwt");
    let access_token = create_access_token(&jwt, install_id)?;
    let comments = list_comments(&pr, &access_token.token)?;

    match comments.iter().find(|x| x.user.id.to_string() == bot_id) {
        Some(prev_comment) => update_comment(
            &pr.owner,
            &pr.repo,
            prev_comment.id,
            comment_body,
            &access_token.token,
        ),
        None => create_comment(
            CommentArgs {
                owner: pr.owner,
                repo: pr.repo,
                issue: pr.issue,
                body: comment_body,
            },
            &access_token.token,
        ),
    }
}
