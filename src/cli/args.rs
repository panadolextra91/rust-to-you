use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "rust-to-you",
    about = "🦀 Điều tra các repository public trên GitHub một cách nhanh chóng và đáng yêu!"
)]
pub struct Args {
    #[arg(
        help = "URL repo GitHub hoặc định dạng owner/repo (ví dụ: tokio-rs/axum)",
        required = true
    )]
    pub repo: String,

    #[arg(
        long,
        help = "Đào sâu kể cả repo lớn (bỏ qua giới hạn kích thước) / Dig even large repos (skip the size guard)"
    )]
    pub deep: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parse_deep() {
        let args = Args::parse_from(["bin", "tokio-rs/axum", "--deep"]);
        assert!(args.deep);
        assert_eq!(args.repo, "tokio-rs/axum");

        let args_default = Args::parse_from(["bin", "tokio-rs/axum"]);
        assert!(!args_default.deep);
        assert_eq!(args_default.repo, "tokio-rs/axum");
    }
}
