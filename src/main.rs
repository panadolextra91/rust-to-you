use clap::Parser;
use rust_to_you::cli::{Args, parse_repo_ref};
use rust_to_you::app::InvestigationSession;
use rust_to_you::app;

fn main() {
    rust_to_you::repo::hygiene::install_panic_hook();
    rust_to_you::repo::hygiene::install_signal_handler();

    let removed = rust_to_you::repo::hygiene::sweep_orphans(
        &std::env::temp_dir(),
        std::time::SystemTime::now(),
        rust_to_you::repo::hygiene::ORPHAN_MAX_AGE,
    );
    if removed > 0 {
        let w = rust_to_you::i18n::two_line(&rust_to_you::i18n::bi(
            format!("🦀 Ferris dọn {} temp cũ từ lần trước nha", removed),
            format!("Ferris swept {} stale temp dir(s) from a previous run", removed),
        ));
        eprintln!("{}", w[0]);
        eprintln!("{}", w[1]);
    }

    let args = Args::parse();

    match parse_repo_ref(&args.repo) {
        Ok(repo_ref) => {
            let session = InvestigationSession::new(repo_ref, args.deep);
            if let Err(e) = app::run(&session) {
                eprintln!("{}", e);
                std::process::exit(e.exit_code());
            }
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(e.exit_code());
        }
    }
}
