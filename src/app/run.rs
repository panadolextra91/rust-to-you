use crate::app::session::InvestigationSession;
use crate::app::collect::collect;
use crate::error::IntakeError;

pub fn run(session: &InvestigationSession) -> Result<(), IntakeError> {
    let status_lines = crate::i18n::two_line(&crate::i18n::bi(
        format!("🦀 Ferris bắt đầu điều tra {}/{} — Case {}", session.repo.owner, session.repo.repo, session.case_id),
        format!("Ferris is opening the investigation for {}/{} — Case {}", session.repo.owner, session.repo.repo, session.case_id)
    ));
    println!("{}", status_lines[0]);
    println!("{}", status_lines[1]);

    match collect(session) {
        Ok(snapshot) => {
            let now_secs = chrono::Utc::now().timestamp();
            let sections = crate::report::sections::build_factual_sections(&snapshot, now_secs);
            crate::tui::render(session, &sections, now_secs).map_err(|e| {
                IntakeError::CollectionFailed { detail: format!("Failed to render report: {:?}", e) }
            })
        }
        Err(e) => Err(e),
    }
}
