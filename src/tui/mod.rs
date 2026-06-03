use crate::app::session::InvestigationSession;
use crate::report::sections::FactualSections;
use std::io::IsTerminal;

pub mod format;
pub mod section;
pub mod report;
pub mod plain;
pub mod app;

pub fn render(session: &InvestigationSession, sections: &FactualSections, now_secs: i64) -> std::io::Result<()> {
    if std::io::stdout().is_terminal() {
        app::render_tui(session, sections, now_secs)
    } else {
        plain::render(&mut std::io::stdout(), session, sections, now_secs)
    }
}
