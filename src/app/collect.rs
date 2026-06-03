use crate::app::session::InvestigationSession;
use crate::error::IntakeError;
use crate::github::{client::fetch_metadata, GithubError};
use crate::repo::clone::clone_repo;
use crate::repo::history::{total_commits, repo_age_days, collect_contributors, collect_bounded, COMMIT_WINDOW_CAP};
use crate::repo::branches::enumerate_branches;
use crate::scan::lang::language_breakdown;
use crate::scan::infra::detect_infra;
use crate::snapshot::{InvestigationSnapshot, RepoMetaState, HistoryFacts, FilesystemFacts};

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
            eprintln!("🦀 Lỗi khi gọi metadata: {:?}. Continuing git-only...", e);
            RepoMetaState::Unavailable
        }
    };

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
