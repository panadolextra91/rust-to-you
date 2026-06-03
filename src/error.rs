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
}

impl IntakeError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::EmptyInput
            | Self::NotAUrl { .. }
            | Self::UnsupportedHost { .. }
            | Self::MalformedRepoPath { .. } => 2,
            Self::RepoNotFoundOrPrivate { .. } => 3,
            Self::Network | Self::RateLimited => 4,
            Self::CollectionFailed { .. } => 5,
        }
    }
}
