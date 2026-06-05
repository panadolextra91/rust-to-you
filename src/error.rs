use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum IntakeError {
    #[error("🦀 Cho Ferris cái URL repo nào: `rust-to-you https://github.com/owner/repo`\nGive Ferris a repo URL: `rust-to-you https://github.com/owner/repo`")]
    EmptyInput,

    #[error("🦀 Cái này không giống URL GitHub. Thử `github.com/owner/repo` nha\nThis doesn't look like a GitHub URL. Try `github.com/owner/repo`")]
    NotAUrl { input: String },

    #[error("🦀 Ferris chỉ hóng repo GitHub thôi — {host} chưa có trong danh sách khách mời\nFerris only visits public GitHub repos — {host} is not on the guest list")]
    UnsupportedHost { host: String },

    #[error("🦀 Ferris cần `owner/repo`, kiểu `github.com/tokio-rs/axum`\nFerris needs `owner/repo`, like `github.com/tokio-rs/axum`")]
    MalformedRepoPath { input: String },

    #[error("🦀 Không với tới {owner}/{repo} — private hoặc không tồn tại. Ferris chỉ làm repo public\nCould not reach {owner}/{repo} — it is private or does not exist. Ferris only does public repos")]
    RepoNotFoundOrPrivate { owner: String, repo: String },

    #[error("🦀 Lỗi kết nối mạng rồi. Thử lại sau nha\nNetwork error. Try again later")]
    Network,

    #[error("🦀 Ferris bị giới hạn lượt gọi API rồi. Thử lại sau nha\nFerris is rate-limited by the API. Try again later")]
    RateLimited,

    #[error("🦀 Lỗi khi đọc lịch sử git: {detail}\nError reading git history: {detail}")]
    CollectionFailed { detail: String },

    #[error("🦀 Repo này to quá ({size_mb} MB, vượt ngưỡng {threshold_mb} MB), Ferris không đào tự động đâu — chạy lại với --deep nếu bạn chấp nhận chờ lâu\nThis repo is too large ({size_mb} MB, over the {threshold_mb} MB limit). Ferris won't auto-dig — re-run with --deep if you accept the longer wait")]
    RepoTooLarge { size_mb: u64, threshold_mb: u64 },

    #[error("🦀 Input này kỳ lạ / không an toàn, Ferris không nhận\nThis input looks unsafe — Ferris won't take it")]
    UnsafeInput { input: String },
}

impl IntakeError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::EmptyInput
            | Self::NotAUrl { .. }
            | Self::UnsupportedHost { .. }
            | Self::MalformedRepoPath { .. }
            | Self::UnsafeInput { .. } => 2,
            Self::RepoNotFoundOrPrivate { .. } => 3,
            Self::Network | Self::RateLimited => 4,
            Self::CollectionFailed { .. } => 5,
            Self::RepoTooLarge { .. } => 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_exit_codes() {
        assert_eq!(IntakeError::EmptyInput.exit_code(), 2);
        assert_eq!(IntakeError::NotAUrl { input: "x".into() }.exit_code(), 2);
        assert_eq!(IntakeError::UnsupportedHost { host: "x".into() }.exit_code(), 2);
        assert_eq!(IntakeError::MalformedRepoPath { input: "x".into() }.exit_code(), 2);
        assert_eq!(IntakeError::UnsafeInput { input: "x".into() }.exit_code(), 2);
        
        assert_eq!(IntakeError::RepoNotFoundOrPrivate { owner: "x".into(), repo: "y".into() }.exit_code(), 3);
        assert_eq!(IntakeError::Network.exit_code(), 4);
        assert_eq!(IntakeError::RateLimited.exit_code(), 4);
        assert_eq!(IntakeError::CollectionFailed { detail: "x".into() }.exit_code(), 5);
        
        assert_eq!(IntakeError::RepoTooLarge { size_mb: 550, threshold_mb: 500 }.exit_code(), 6);
    }

    #[test]
    fn test_repo_too_large_message() {
        let err = IntakeError::RepoTooLarge { size_mb: 550, threshold_mb: 500 };
        let msg = err.to_string();
        assert!(msg.contains("550"));
        assert!(msg.contains("500"));
        assert!(msg.contains("--deep"));
        assert!(msg.contains('\n'));
        assert!(msg.contains("không đào tự động"));
        assert!(msg.contains("too large"));
    }
}
