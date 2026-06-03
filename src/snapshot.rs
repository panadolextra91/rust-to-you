use crate::cli::RepoRef;
use crate::github::RepoMetadata;
use crate::repo::branches::BranchFacts;
use crate::repo::history::CommitWindow;

pub enum RepoMetaState {
    Available(RepoMetadata),
    Unavailable,
}

pub struct HistoryFacts {
    pub total_commits: usize,
    pub repo_age_days: i64,
    pub contributor_count: usize,
    pub bus_factor: usize,
    pub top_author_share_pct: f64,
    pub window: CommitWindow,
    pub most_modified_file: Option<String>,
    pub night_pct: f64,
    pub weekend_pct: f64,
    pub business_hours_pct: f64,
}

pub struct InfraFootprints {
    pub docker: bool,
    pub terraform: bool,
    pub github_actions: bool,
    pub gitlab_ci: bool,
    pub circleci: bool,
    pub jenkins: bool,
    pub dependabot: bool,
    pub renovate: bool,
}

pub struct FilesystemFacts {
    pub languages: Vec<(String, f64)>,
    pub infra: InfraFootprints,
}

pub struct InvestigationSnapshot {
    pub repo: RepoRef,
    pub metadata: RepoMetaState,
    pub history: HistoryFacts,
    pub branches: BranchFacts,
    pub filesystem: FilesystemFacts,
}
