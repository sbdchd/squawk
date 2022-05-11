use crate::app;
use crate::{Comment, GitHubApi, GithubError};

pub struct GitHub {
    github_token: String,
}
impl GitHub {
    #[must_use]
    pub fn new(github_token: &str) -> Self {
        GitHub {
            github_token: github_token.to_string(),
        }
    }
}
impl GitHubApi for GitHub {
    fn app_slug(&self) -> String {
        "github-actions[bot]".to_string()
    }
    fn create_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        issue_id: i64,
        body: &str,
    ) -> Result<(), GithubError> {
        app::create_comment(
            app::CommentArgs {
                owner: owner.to_string(),
                repo: repo.to_string(),
                issue: issue_id,
                body: body.to_string(),
            },
            &self.github_token,
        )
    }
    fn list_issue_comments(
        &self,
        owner: &str,
        repo: &str,
        issue_id: i64,
    ) -> Result<Vec<Comment>, GithubError> {
        app::list_comments(
            &app::PullRequest {
                issue: issue_id,
                owner: owner.to_string(),
                repo: repo.to_string(),
            },
            &self.github_token,
        )
    }
    fn update_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &str,
    ) -> Result<(), GithubError> {
        app::update_comment(
            owner,
            repo,
            comment_id,
            body.to_string(),
            &self.github_token,
        )
    }
}
