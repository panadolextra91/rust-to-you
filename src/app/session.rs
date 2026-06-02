use crate::cli::RepoRef;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct InvestigationSession {
    pub repo: RepoRef,
    pub case_id: String,
    pub started_at: SystemTime,
}

impl InvestigationSession {
    pub fn new(repo: RepoRef) -> Self {
        let case_id = generate_case_id(&repo.owner, &repo.repo);
        let started_at = SystemTime::now();
        Self {
            repo,
            case_id,
            started_at,
        }
    }
}

pub fn generate_case_id(owner: &str, repo: &str) -> String {
    let repo_upper = repo.to_uppercase();
    let filtered: String = repo_upper
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();

    let prefix = if filtered.is_empty() {
        "REPO".to_string()
    } else if filtered.len() > 4 {
        filtered[..4].to_string()
    } else {
        filtered
    };

    // Deterministic FNV-1a hash of the literal "owner/repo"
    let hash_input = format!("{}/{}", owner, repo);
    let mut hash = 2166136261u32;
    for byte in hash_input.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(16777619);
    }
    let suffix_val = (hash & 0xFFFF) as u16;
    let suffix = format!("{:04X}", suffix_val);

    format!("{}-{}", prefix, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_id_determinism() {
        let id1 = generate_case_id("tokio-rs", "axum");
        let id2 = generate_case_id("tokio-rs", "axum");
        assert_eq!(id1, id2, "case_id must be deterministic and identical across calls");
    }

    #[test]
    fn test_case_id_format() {
        let cases = vec![
            ("tokio-rs", "axum"),
            ("rust-lang", "docs.rs"),
            ("rust-lang", "crates.io"),
            ("x", "y"),
            ("empty-filter", ".."),
        ];

        for (owner, repo) in cases {
            let id = generate_case_id(owner, repo);
            let parts: Vec<&str> = id.split('-').collect();
            assert_eq!(parts.len(), 2, "case_id '{}' must have exactly one dash", id);
            
            let prefix = parts[0];
            let suffix = parts[1];

            assert!(
                prefix.len() >= 1 && prefix.len() <= 4,
                "prefix '{}' in case_id '{}' must be 1 to 4 characters",
                prefix,
                id
            );
            assert!(
                prefix.chars().all(|c| c.is_ascii_alphanumeric() && c.is_ascii_uppercase()),
                "prefix '{}' must be uppercase alphanumeric",
                prefix
            );

            assert_eq!(
                suffix.len(),
                4,
                "suffix '{}' in case_id '{}' must be exactly 4 characters",
                suffix,
                id
            );
            assert!(
                suffix.chars().all(|c| c.is_ascii_digit() || (c.is_ascii_alphabetic() && c.is_ascii_uppercase())),
                "suffix '{}' must be uppercase hex digits",
                suffix
            );
        }
    }

    #[test]
    fn test_case_id_prefix_rule() {
        // repo "axum" -> "AXUM"
        assert_eq!(&generate_case_id("tokio-rs", "axum")[..4], "AXUM");

        // repo "x" -> "X"
        assert_eq!(&generate_case_id("foo", "x")[..1], "X");

        // repo "docs.rs" -> "DOCS" (filter-then-truncate)
        assert_eq!(&generate_case_id("rust-lang", "docs.rs")[..4], "DOCS");

        // repo "crates.io" -> "CRAT" (filter-then-truncate)
        assert_eq!(&generate_case_id("rust-lang", "crates.io")[..4], "CRAT");

        // repo ".." -> empty fallback "REPO"
        assert_eq!(&generate_case_id("foo", "..")[..4], "REPO");
    }

    #[test]
    fn test_case_id_snapshot() {
        let id = generate_case_id("tokio-rs", "axum");
        assert!(id.starts_with("AXUM-"), "Expected AXUM- prefix, got {}", id);
        let parts: Vec<&str> = id.split('-').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "AXUM");
        assert_eq!(parts[1].len(), 4);
    }
}
