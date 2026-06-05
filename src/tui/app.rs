use crate::app::session::InvestigationSession;
use crate::report::sections::FactualSections;
use crate::snapshot::InvestigationSnapshot;
use crate::tui::report::build_report_lines;
use ratatui::widgets::{Paragraph, Wrap};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use std::io;

#[derive(Debug, Default)]
pub struct TuiState {
    pub offset: u16,
    pub quit: bool,
}

pub fn max_scroll(line_count: usize, viewport_h: u16) -> u16 {
    line_count.saturating_sub(viewport_h as usize) as u16
}

pub fn handle_key(code: KeyCode, state: &mut TuiState, max: u16, page: u16) {
    match code {
        KeyCode::Down | KeyCode::Char('j') => {
            state.offset = (state.offset + 1).min(max);
        }
        KeyCode::Up | KeyCode::Char('k') => {
            state.offset = state.offset.saturating_sub(1);
        }
        KeyCode::PageDown => {
            state.offset = (state.offset + page).min(max);
        }
        KeyCode::PageUp => {
            state.offset = state.offset.saturating_sub(page);
        }
        KeyCode::Char('g') => {
            state.offset = 0;
        }
        KeyCode::Char('G') => {
            state.offset = max;
        }
        KeyCode::Char('q') | KeyCode::Esc => {
            state.quit = true;
        }
        _ => {}
    }
}

/// Lines scrolled per mouse-wheel / trackpad notch.
pub const SCROLL_STEP: u16 = 3;

pub fn handle_mouse(kind: MouseEventKind, state: &mut TuiState, max: u16) {
    match kind {
        MouseEventKind::ScrollDown => {
            state.offset = (state.offset + SCROLL_STEP).min(max);
        }
        MouseEventKind::ScrollUp => {
            state.offset = state.offset.saturating_sub(SCROLL_STEP);
        }
        _ => {}
    }
}

pub fn render_tui(session: &InvestigationSession, snapshot: &InvestigationSnapshot, sections: &FactualSections, now_secs: i64) -> io::Result<()> {
    let lines = build_report_lines(session, snapshot, sections, now_secs);
    let mut state = TuiState::default();

    ratatui::run(|terminal| -> io::Result<()> {
        // Enable mouse capture so trackpad / wheel scroll reaches us.
        crossterm::execute!(io::stdout(), crossterm::event::EnableMouseCapture)?;
        loop {
            terminal.draw(|frame| {
                let area = frame.area();
                let page = area.height;
                let max = max_scroll(lines.len(), page);
                state.offset = state.offset.min(max);

                let paragraph = Paragraph::new(lines.clone())
                    .wrap(Wrap { trim: false })
                    .scroll((state.offset, 0));
                frame.render_widget(paragraph, area);
            })?;

            let page = terminal.size()?.height;
            let max = max_scroll(lines.len(), page);
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        handle_key(key.code, &mut state, max, page);
                    }
                }
                Event::Mouse(m) => {
                    handle_mouse(m.kind, &mut state, max);
                }
                Event::Resize(..) => {}
                _ => {}
            }
            if state.quit {
                break;
            }
        }
        // Best-effort: turn mouse capture back off before ratatui restores the screen.
        let _ = crossterm::execute!(io::stdout(), crossterm::event::DisableMouseCapture);
        Ok(())
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_scroll() {
        assert_eq!(max_scroll(100, 20), 80);
        assert_eq!(max_scroll(10, 20), 0);
    }

    #[test]
    fn test_handle_key() {
        let mut state = TuiState::default();
        let max = 50;
        let page = 10;

        // Down / j
        handle_key(KeyCode::Down, &mut state, max, page);
        assert_eq!(state.offset, 1);
        handle_key(KeyCode::Char('j'), &mut state, max, page);
        assert_eq!(state.offset, 2);

        // Up / k
        handle_key(KeyCode::Up, &mut state, max, page);
        assert_eq!(state.offset, 1);
        handle_key(KeyCode::Char('k'), &mut state, max, page);
        assert_eq!(state.offset, 0);
        handle_key(KeyCode::Up, &mut state, max, page);
        assert_eq!(state.offset, 0); // saturating

        // G / g
        handle_key(KeyCode::Char('G'), &mut state, max, page);
        assert_eq!(state.offset, 50);
        handle_key(KeyCode::Char('g'), &mut state, max, page);
        assert_eq!(state.offset, 0);

        // PageDown / PageUp
        handle_key(KeyCode::PageDown, &mut state, max, page);
        assert_eq!(state.offset, 10);
        handle_key(KeyCode::PageDown, &mut state, max, page);
        assert_eq!(state.offset, 20);
        handle_key(KeyCode::PageUp, &mut state, max, page);
        assert_eq!(state.offset, 10);

        // Clamping to max
        state.offset = 48;
        handle_key(KeyCode::PageDown, &mut state, max, page);
        assert_eq!(state.offset, 50);

        // Quit keys
        assert!(!state.quit);
        handle_key(KeyCode::Char('q'), &mut state, max, page);
        assert!(state.quit);

        state.quit = false;
        handle_key(KeyCode::Esc, &mut state, max, page);
        assert!(state.quit);
    }

    #[test]
    fn test_handle_mouse() {
        let mut state = TuiState::default();
        let max = 50;

        // Scroll down by SCROLL_STEP per notch
        handle_mouse(MouseEventKind::ScrollDown, &mut state, max);
        assert_eq!(state.offset, SCROLL_STEP);
        handle_mouse(MouseEventKind::ScrollDown, &mut state, max);
        assert_eq!(state.offset, SCROLL_STEP * 2);

        // Scroll up (saturating at 0)
        handle_mouse(MouseEventKind::ScrollUp, &mut state, max);
        assert_eq!(state.offset, SCROLL_STEP);
        handle_mouse(MouseEventKind::ScrollUp, &mut state, max);
        handle_mouse(MouseEventKind::ScrollUp, &mut state, max);
        assert_eq!(state.offset, 0);

        // Clamp to max
        state.offset = max - 1;
        handle_mouse(MouseEventKind::ScrollDown, &mut state, max);
        assert_eq!(state.offset, max);
    }

    #[test]
    fn test_headless_scroll_contract() {
        use ratatui::backend::TestBackend;
        use ratatui::Terminal;
        use crate::cli::RepoRef;
        
        let session = InvestigationSession {
            repo: RepoRef { owner: "owner".to_string(), repo: "repo".to_string() },
            case_id: "OWNER-1234".to_string(),
            started_at: std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1700000000),
            deep: false,
        };
        let snapshot = InvestigationSnapshot {
            repo: RepoRef { owner: "owner".to_string(), repo: "repo".to_string() },
            metadata: crate::snapshot::RepoMetaState::Unavailable,
            history: crate::snapshot::HistoryFacts {
                total_commits: 500,
                repo_age_days: 100,
                contributor_count: 5,
                bus_factor: 2,
                top_author_share_pct: 60.0,
                window: crate::snapshot::CommitWindow { scanned: 100, capped: false },
                most_modified_file: Some("main.rs".to_string()),
                night_pct: 10.0,
                weekend_pct: 15.0,
                business_hours_pct: 75.0,
                commits_this_month: 50,
                top_contributor_name: Some("Alice".to_string()),
                oldest_file: Some("lib.rs".to_string()),
                oldest_contributor: Some("Bob".to_string()),
                release_tag_count: 5,
            },
            branches: crate::snapshot::BranchFacts {
                default_branch: "main".to_string(),
                branches: vec![],
                last_activity_secs: 1699990000,
            },
            filesystem: crate::snapshot::FilesystemFacts {
                languages: vec![("Rust".to_string(), 90.0), ("Markdown".to_string(), 10.0)],
                infra: crate::snapshot::InfraFootprints {
                    docker: true,
                    terraform: false,
                    github_actions: true,
                    gitlab_ci: false,
                    circleci: false,
                    jenkins: false,
                    dependabot: true,
                    renovate: false,
                },
            },
        };
        let sections = crate::report::sections::FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 100,
                default_branch: "main".to_string(),
                stars: Some(1500),
                forks: Some(250),
                contributor_count: 5,
                last_activity_secs: Some(1699990000),
            },
            commit_crimes: crate::analyze::commit::CommitCrimes {
                total_commits: 500,
                commits_this_month: 50,
                top_contributor: Some("Alice".to_string()),
                bus_factor: 2,
                top_author_share_pct: 60.0,
            },
            branch_jungle: crate::analyze::branch::BranchJungle {
                total: 4,
                active: 2,
                stale: 2,
                oldest_branch: Some("old-feature".to_string()),
            },
            ancient_relics: crate::analyze::relics::AncientRelics {
                oldest_file: Some("lib.rs".to_string()),
                most_modified_file: Some("main.rs".to_string()),
                oldest_contributor: Some("Bob".to_string()),
                longest_living_branch: Some("old-feature".to_string()),
            },
            language_soup: crate::analyze::language::LanguageSoup {
                languages: vec![
                    crate::analyze::language::LanguageEntry { name: "Rust".to_string(), pct: 100.0 }
                ],
            },
            infrastructure: crate::analyze::infra::InfrastructureFootprints {
                docker: true,
                terraform: false,
                github_actions: true,
                gitlab_ci: false,
                circleci: false,
                jenkins: false,
                dependabot: true,
                renovate: false,
            },
        };

        let lines = build_report_lines(&session, &snapshot, &sections, 1700000000);

        // Render 1 with scroll (0, 0)
        let backend1 = TestBackend::new(80, 10);
        let mut terminal1 = Terminal::new(backend1).unwrap();
        terminal1.draw(|f| {
            let p = Paragraph::new(lines.clone()).wrap(Wrap { trim: false }).scroll((0, 0));
            f.render_widget(p, f.area());
        }).unwrap();
        let buffer1 = terminal1.backend().buffer();

        // Render 2 with scroll (1, 0)
        let backend2 = TestBackend::new(80, 10);
        let mut terminal2 = Terminal::new(backend2).unwrap();
        terminal2.draw(|f| {
            let p = Paragraph::new(lines.clone()).wrap(Wrap { trim: false }).scroll((1, 0));
            f.render_widget(p, f.area());
        }).unwrap();
        let buffer2 = terminal2.backend().buffer();

        // Capture first row string of each render
        let mut row1 = String::new();
        let mut row2 = String::new();
        for x in 0..80 {
            row1.push_str(buffer1[(x, 0)].symbol());
            row2.push_str(buffer2[(x, 0)].symbol());
        }

        assert_ne!(row1, row2, "Scroll of 1 line should make the first visible row differ");
    }
}
