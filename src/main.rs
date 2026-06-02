mod cli;
mod error;
mod app;

use clap::Parser;
use cli::{Args, parse_repo_ref};
use app::InvestigationSession;

fn main() {
    let args = Args::parse();

    match parse_repo_ref(&args.repo) {
        Ok(repo_ref) => {
            let session = InvestigationSession::new(repo_ref);
            app::run(&session);
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(e.exit_code());
        }
    }
}
