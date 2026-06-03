use crate::cli::RepoRef;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RepoMetadata {
    pub stargazers_count: u64,
    pub forks_count: u64,
    pub description: Option<String>,
    #[serde(default)]
    pub topics: Vec<String>,
    pub default_branch: String,
    pub pushed_at: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GithubError {
    NotFound,
    RateLimited,
    Network,
}

pub fn classify(status: StatusCode) -> Result<(), GithubError> {
    match status {
        StatusCode::OK => Ok(()),
        StatusCode::NOT_FOUND => Err(GithubError::NotFound),
        StatusCode::FORBIDDEN | StatusCode::TOO_MANY_REQUESTS => Err(GithubError::RateLimited),
        _ => Err(GithubError::Network),
    }
}

pub fn fetch_metadata(repo_ref: &RepoRef) -> Result<RepoMetadata, GithubError> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("rust-to-you/0.1")
        .build()
        .map_err(|_| GithubError::Network)?;

    let url = format!("https://api.github.com/repos/{}/{}", repo_ref.owner, repo_ref.repo);

    let mut request = client
        .get(&url)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28");

    if let Ok(tok) = std::env::var("GITHUB_TOKEN") {
        if !tok.trim().is_empty() {
            request = request.bearer_auth(tok.trim());
        }
    }

    let resp = request.send().map_err(|_| GithubError::Network)?;

    classify(resp.status())?;

    resp.json::<RepoMetadata>().map_err(|_| GithubError::Network)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_mapping() {
        assert_eq!(classify(StatusCode::OK), Ok(()));
        assert_eq!(classify(StatusCode::NOT_FOUND), Err(GithubError::NotFound));
        assert_eq!(classify(StatusCode::FORBIDDEN), Err(GithubError::RateLimited));
        assert_eq!(classify(StatusCode::TOO_MANY_REQUESTS), Err(GithubError::RateLimited));
        assert_eq!(classify(StatusCode::INTERNAL_SERVER_ERROR), Err(GithubError::Network));
        assert_eq!(classify(StatusCode::BAD_REQUEST), Err(GithubError::Network));
    }

    #[test]
    fn decode() {
        let json_str = r#"{
            "stargazers_count": 42,
            "forks_count": 10,
            "description": null,
            "topics": ["rust", "cli"],
            "default_branch": "main",
            "pushed_at": "2023-01-01T00:00:00Z",
            "created_at": "2022-01-01T00:00:00Z"
        }"#;

        let meta: RepoMetadata = serde_json::from_str(json_str).unwrap();
        assert_eq!(meta.stargazers_count, 42);
        assert_eq!(meta.forks_count, 10);
        assert_eq!(meta.description, None);
        assert_eq!(meta.topics, vec!["rust", "cli"]);
        assert_eq!(meta.default_branch, "main");
        assert_eq!(meta.pushed_at.as_deref(), Some("2023-01-01T00:00:00Z"));
        assert_eq!(meta.created_at.as_deref(), Some("2022-01-01T00:00:00Z"));

        let json_no_topics = r#"{
            "stargazers_count": 1,
            "forks_count": 0,
            "description": "test",
            "default_branch": "master",
            "pushed_at": null,
            "created_at": null
        }"#;

        let meta2: RepoMetadata = serde_json::from_str(json_no_topics).unwrap();
        assert!(meta2.topics.is_empty());
        assert_eq!(meta2.description.as_deref(), Some("test"));
    }
}
