use clap::Parser;
use rust_to_you::cli::{Args, parse_repo_ref};
use rust_to_you::app::InvestigationSession;
use rust_to_you::app;

fn main() {
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
