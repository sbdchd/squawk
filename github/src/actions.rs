use crate::{Comment, GitHubApi, GithubError};

struct GitHubActions {}
impl GitHubApi for GitHubActions {
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
        unimplemented!()
    }
    fn list_issue_comments(
        &self,
        owner: &str,
        repo: &str,
        issue_id: i64,
    ) -> Result<Vec<Comment>, GithubError> {
        unimplemented!()
    }
    fn update_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &str,
    ) -> Result<(), GithubError> {
        unimplemented!()
    }
}
