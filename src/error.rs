use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum IntakeError {
    #[error("🦀 Cho mình cái URL repo nào: `rust-to-you https://github.com/owner/repo`")]
    EmptyInput,

    #[error("🦀 Cái này không giống URL GitHub. Thử `github.com/owner/repo` nha")]
    NotAUrl { input: String },

    #[error("🦀 V1 chỉ hóng repo GitHub thôi — {host} chưa có trong danh sách khách mời")]
    UnsupportedHost { host: String },

    #[error("🦀 Mình cần `owner/repo`, kiểu `github.com/tokio-rs/axum`")]
    MalformedRepoPath { input: String },

    #[error("🦀 Không với tới {owner}/{repo} — private hoặc không tồn tại. Mình chỉ làm repo public")]
    RepoNotFoundOrPrivate { owner: String, repo: String },

    #[error("🦀 Lỗi kết nối mạng rồi. Thử lại sau nha")]
    Network,

    #[error("🦀 Bạn bị giới hạn lượt gọi API rồi. Thử lại sau nha")]
    RateLimited,

    #[error("🦀 Lỗi khi đọc lịch sử git: {detail}")]
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
