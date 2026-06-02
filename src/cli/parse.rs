use crate::error::IntakeError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoRef {
    pub owner: String,
    pub repo: String,
}

pub fn parse_repo_ref(input: &str) -> Result<RepoRef, IntakeError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(IntakeError::EmptyInput);
    }

    if input.len() > 2048 || input.chars().any(|c| c.is_control()) {
        return Err(IntakeError::NotAUrl { input: input.to_string() });
    }

    if input.starts_with("git@") {
        return Err(IntakeError::NotAUrl { input: input.to_string() });
    }

    // Determine scheme and rest of the string
    let rest = if let Some(idx) = input.find("://") {
        let scheme = &input[..idx].to_lowercase();
        if scheme != "http" && scheme != "https" {
            return Err(IntakeError::NotAUrl { input: input.to_string() });
        }
        &input[idx + 3..]
    } else {
        input
    };

    let segments: Vec<&str> = rest.split('/').filter(|s| !s.is_empty()).collect();
    if segments.is_empty() {
        return Err(IntakeError::NotAUrl { input: input.to_string() });
    }

    for seg in &segments {
        if seg.contains("..") || seg.contains('\\') {
            return Err(IntakeError::MalformedRepoPath { input: input.to_string() });
        }
    }

    let first_segment = segments[0];
    let (host, owner, repo) = if first_segment.contains('.') {
        // First segment is host
        if segments.len() < 3 {
            // e.g. "github.com/owner" or "github.com"
            let host_lower = first_segment.to_lowercase();
            if host_lower != "github.com" && host_lower != "www.github.com" {
                return Err(IntakeError::UnsupportedHost { host: first_segment.to_string() });
            }
            return Err(IntakeError::MalformedRepoPath { input: input.to_string() });
        }
        (Some(first_segment), segments[1], segments[2])
    } else {
        // Bare shorthand "owner/repo" or deep link thereof
        if segments.len() < 2 {
            return Err(IntakeError::MalformedRepoPath { input: input.to_string() });
        }
        (None, segments[0], segments[1])
    };

    if let Some(h) = host {
        let h_lower = h.to_lowercase();
        if h_lower != "github.com" && h_lower != "www.github.com" {
            return Err(IntakeError::UnsupportedHost { host: h.to_string() });
        }
    }

    // Strip trailing .git from repo
    let mut repo_str = repo.to_string();
    if repo_str.to_lowercase().ends_with(".git") && repo_str.len() > 4 {
        repo_str.truncate(repo_str.len() - 4);
    }

    // Validate owner and repo segment charsets
    if !is_valid_segment(owner) || !is_valid_segment(&repo_str) {
        return Err(IntakeError::MalformedRepoPath { input: input.to_string() });
    }

    Ok(RepoRef {
        owner: owner.to_string(),
        repo: repo_str,
    })
}

fn is_valid_segment(s: &str) -> bool {
    if s.is_empty() || s.contains("..") {
        return false;
    }
    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_repo_ref_accept() {
        let cases = vec![
            ("https://github.com/tokio-rs/axum", "tokio-rs", "axum"),
            ("https://github.com/tokio-rs/axum.git", "tokio-rs", "axum"),
            ("https://github.com/tokio-rs/axum/", "tokio-rs", "axum"),
            ("github.com/tokio-rs/axum", "tokio-rs", "axum"),
            ("https://github.com/tokio-rs/axum/tree/main/src", "tokio-rs", "axum"),
            ("tokio-rs/axum", "tokio-rs", "axum"),
            ("https://www.github.com/tokio-rs/axum", "tokio-rs", "axum"),
            ("rust-lang/docs.rs", "rust-lang", "docs.rs"),
            ("rust-lang/crates.io", "rust-lang", "crates.io"),
        ];

        for (input, expected_owner, expected_repo) in cases {
            let res = parse_repo_ref(input);
            assert!(
                res.is_ok(),
                "Failed to parse '{}', got {:?}",
                input,
                res
            );
            let repo_ref = res.unwrap();
            assert_eq!(repo_ref.owner, expected_owner, "Owner mismatch for '{}'", input);
            assert_eq!(repo_ref.repo, expected_repo, "Repo mismatch for '{}'", input);
        }
    }

    #[test]
    fn test_parse_repo_ref_reject() {
        let cases = vec![
            ("", IntakeError::EmptyInput),
            ("   ", IntakeError::EmptyInput),
            ("not a url", IntakeError::MalformedRepoPath { input: "not a url".to_string() }),
            ("https://", IntakeError::NotAUrl { input: "https://".to_string() }),
            ("https://gitlab.com/foo/bar", IntakeError::UnsupportedHost { host: "gitlab.com".to_string() }),
            ("gitlab.com/foo/bar", IntakeError::UnsupportedHost { host: "gitlab.com".to_string() }),
            ("https://github.com/onlyowner", IntakeError::MalformedRepoPath { input: "https://github.com/onlyowner".to_string() }),
            ("github.com/onlyowner", IntakeError::MalformedRepoPath { input: "github.com/onlyowner".to_string() }),
            ("onlyowner", IntakeError::MalformedRepoPath { input: "onlyowner".to_string() }),
            ("git@github.com:tokio-rs/axum.git", IntakeError::NotAUrl { input: "git@github.com:tokio-rs/axum.git".to_string() }),
            ("tokio-rs/axum/../path", IntakeError::MalformedRepoPath { input: "tokio-rs/axum/../path".to_string() }),
            ("tokio-rs/ax\\um", IntakeError::MalformedRepoPath { input: "tokio-rs/ax\\um".to_string() }),
        ];

        for (input, expected_err) in cases {
            let res = parse_repo_ref(input);
            assert!(
                res.is_err(),
                "Expected error for '{}', but got {:?}",
                input,
                res
            );
            assert_eq!(res.unwrap_err(), expected_err, "Error mismatch for '{}'", input);
        }
    }
}
