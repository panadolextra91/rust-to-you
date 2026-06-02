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
}
