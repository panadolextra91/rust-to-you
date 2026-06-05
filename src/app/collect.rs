use crate::app::session::InvestigationSession;
use crate::error::IntakeError;
use crate::github::{client::fetch_metadata, GithubError};
use crate::repo::clone::clone_repo;
use crate::repo::history::{total_commits, repo_age_days, collect_contributors, collect_bounded, COMMIT_WINDOW_CAP, commits_this_month, oldest_file};
use crate::repo::branches::{enumerate_branches, release_tag_count};
use crate::scan::lang::language_breakdown;
use crate::scan::infra::detect_infra;
use crate::snapshot::{InvestigationSnapshot, RepoMetaState, HistoryFacts, FilesystemFacts};

const MAX_REPO_KB: u64 = 500 * 1024;

#[derive(Debug, PartialEq, Eq)]
pub enum SizeDecision {
    Proceed,
    WarnDeep { size_mb: u64 },
    WarnUnknown,
    TooLarge { size_mb: u64 },
}

pub fn size_decision(meta_state: &RepoMetaState, deep: bool) -> SizeDecision {
    match meta_state {
        RepoMetaState::Available(m) if m.size > MAX_REPO_KB => {
            // Round up so a repo just over the limit never displays AS the limit.
            let size_mb = m.size.div_ceil(1024);
            if deep {
                SizeDecision::WarnDeep { size_mb }
            } else {
                SizeDecision::TooLarge { size_mb }
            }
        }
        RepoMetaState::Unavailable => SizeDecision::WarnUnknown,
        _ => SizeDecision::Proceed,
    }
}

pub fn collect(session: &InvestigationSession) -> Result<InvestigationSnapshot, IntakeError> {
    // 1. Metadata API-first, abort-on-404, degrade-on-transient
    let meta_state = match fetch_metadata(&session.repo) {
        Ok(meta) => RepoMetaState::Available(meta),
        Err(GithubError::NotFound) => {
            return Err(IntakeError::RepoNotFoundOrPrivate {
                owner: session.repo.owner.clone(),
                repo: session.repo.repo.clone(),
            });
        }
        Err(e) => {
            eprintln!("🦀 Ferris không gọi được metadata ({:?}) — tiếp tục với git thôi\nFerris could not reach repo metadata, continuing git-only", e);
            RepoMetaState::Unavailable
        }
    };

    // Validate size guard (D-04, D-05)
    match size_decision(&meta_state, session.deep) {
        SizeDecision::TooLarge { size_mb } => {
            return Err(IntakeError::RepoTooLarge { size_mb, threshold_mb: MAX_REPO_KB / 1024 });
        }
        SizeDecision::WarnDeep { size_mb } => {
            let w = crate::i18n::two_line(&crate::i18n::bi(
                format!("🦀 Repo lớn ({} MB), Ferris vẫn đào vì --deep — sẽ lâu đó", size_mb),
                format!("Large repo ({} MB) — Ferris digs anyway (--deep); this will take a while", size_mb),
            ));
            eprintln!("{}", w[0]);
            eprintln!("{}", w[1]);
        }
        SizeDecision::WarnUnknown => {
            let w = crate::i18n::two_line(&crate::i18n::bi(
                "🦀 Không biết kích thước repo, Ferris cứ đào nha",
                "Unknown repo size — Ferris digs anyway"
            ));
            eprintln!("{}", w[0]);
            eprintln!("{}", w[1]);
        }
        SizeDecision::Proceed => {}
    }

    // 2. Clone repo (hard-fail)
    let ws = clone_repo(&session.repo).map_err(|_| IntakeError::Network)?;

    // 3. History + branches
    let total = total_commits(&ws.repo)
        .map_err(|e| IntakeError::CollectionFailed { detail: e.to_string() })?;
    let age = repo_age_days(&ws.repo)
        .map_err(|e| IntakeError::CollectionFailed { detail: e.to_string() })?;
    let contributors = collect_contributors(&ws.repo)
        .map_err(|e| IntakeError::CollectionFailed { detail: e.to_string() })?;
    let bounded = collect_bounded(&ws.repo, COMMIT_WINDOW_CAP)
        .map_err(|e| IntakeError::CollectionFailed { detail: e.to_string() })?;
    let branches = enumerate_branches(&ws.repo)
        .map_err(|e| IntakeError::CollectionFailed { detail: e.to_string() })?;
    let commits_this_month = commits_this_month(&ws.repo)
        .map_err(|e| IntakeError::CollectionFailed { detail: e.to_string() })?;
    let oldest_file_val = oldest_file(&ws.repo)
        .map_err(|e| IntakeError::CollectionFailed { detail: e.to_string() })?;
    let tag_count = release_tag_count(&ws.repo)
        .map_err(|e| IntakeError::CollectionFailed { detail: e.to_string() })?;

    let history = HistoryFacts {
        total_commits: total,
        repo_age_days: age,
        contributor_count: contributors.contributor_count,
        bus_factor: contributors.bus_factor,
        top_author_share_pct: contributors.top_author_share_pct,
        window: bounded.window,
        most_modified_file: bounded.most_modified_file,
        night_pct: bounded.night_pct,
        weekend_pct: bounded.weekend_pct,
        business_hours_pct: bounded.business_hours_pct,
        commits_this_month,
        top_contributor_name: contributors.top_contributor_name,
        oldest_file: oldest_file_val,
        oldest_contributor: contributors.oldest_contributor,
        release_tag_count: tag_count,
    };

    // 4. Scan
    let filesystem = FilesystemFacts {
        languages: language_breakdown(ws.repo.workdir().unwrap()),
        infra: detect_infra(ws.repo.workdir().unwrap()),
    };

    // 5. Assemble snapshot
    Ok(InvestigationSnapshot {
        repo: session.repo.clone(),
        metadata: meta_state,
        history,
        branches,
        filesystem,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::github::client::RepoMetadata;

    fn mock_metadata(size_kb: u64) -> RepoMetaState {
        RepoMetaState::Available(RepoMetadata {
            stargazers_count: 0,
            forks_count: 0,
            description: None,
            topics: vec![],
            default_branch: "main".to_string(),
            pushed_at: None,
            created_at: None,
            size: size_kb,
        })
    }

    #[test]
    fn test_size_decision_proceed() {
        let meta = mock_metadata(100 * 1024);
        assert_eq!(size_decision(&meta, false), SizeDecision::Proceed);
        assert_eq!(size_decision(&meta, true), SizeDecision::Proceed);
    }

    #[test]
    fn test_size_decision_boundary() {
        // exactly 500 MB is safe
        let meta_exact = mock_metadata(500 * 1024);
        assert_eq!(size_decision(&meta_exact, false), SizeDecision::Proceed);

        // strictly greater than 500 MB triggers TooLarge / WarnDeep;
        // the displayed size rounds up so it never reads as exactly the limit.
        let meta_large = mock_metadata(500 * 1024 + 1);
        assert_eq!(size_decision(&meta_large, false), SizeDecision::TooLarge { size_mb: 501 });
        assert_eq!(size_decision(&meta_large, true), SizeDecision::WarnDeep { size_mb: 501 });
    }

    #[test]
    fn test_size_decision_large() {
        let meta = mock_metadata(600 * 1024);
        assert_eq!(size_decision(&meta, false), SizeDecision::TooLarge { size_mb: 600 });
        assert_eq!(size_decision(&meta, true), SizeDecision::WarnDeep { size_mb: 600 });
    }

    #[test]
    fn test_size_decision_unavailable() {
        assert_eq!(size_decision(&RepoMetaState::Unavailable, false), SizeDecision::WarnUnknown);
        assert_eq!(size_decision(&RepoMetaState::Unavailable, true), SizeDecision::WarnUnknown);
    }
}
