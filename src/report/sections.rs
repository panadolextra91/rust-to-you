use crate::snapshot::{InvestigationSnapshot, RepoMetaState};
use crate::analyze::commit::{CommitCrimes, commit_crimes};
use crate::analyze::branch::{BranchJungle, branch_jungle};
use crate::analyze::relics::{AncientRelics, ancient_relics};
use crate::analyze::language::{LanguageSoup, language_soup};
use crate::analyze::infra::{InfrastructureFootprints, infrastructure_footprints};

pub struct FirstImpressions {
    pub repo_age_days: i64,
    pub default_branch: String,
    pub stars: Option<u64>,
    pub forks: Option<u64>,
    pub contributor_count: usize,
    pub last_activity_secs: Option<i64>,
}

pub struct FactualSections {
    pub first_impressions: FirstImpressions,
    pub commit_crimes: CommitCrimes,
    pub branch_jungle: BranchJungle,
    pub ancient_relics: AncientRelics,
    pub language_soup: LanguageSoup,
    pub infrastructure: InfrastructureFootprints,
}

pub fn build_factual_sections(snapshot: &InvestigationSnapshot, now_secs: i64) -> FactualSections {
    let (stars, forks) = match &snapshot.metadata {
        RepoMetaState::Available(meta) => (Some(meta.stargazers_count), Some(meta.forks_count)),
        RepoMetaState::Unavailable => (None, None),
    };
    
    let last_activity_secs = if snapshot.branches.last_activity_secs > 0 {
        Some(snapshot.branches.last_activity_secs)
    } else {
        None
    };

    let first_impressions = FirstImpressions {
        repo_age_days: snapshot.history.repo_age_days,
        default_branch: snapshot.branches.default_branch.clone(),
        stars,
        forks,
        contributor_count: snapshot.history.contributor_count,
        last_activity_secs,
    };

    FactualSections {
        first_impressions,
        commit_crimes: commit_crimes(snapshot),
        branch_jungle: branch_jungle(snapshot, now_secs),
        ancient_relics: ancient_relics(snapshot),
        language_soup: language_soup(snapshot),
        infrastructure: infrastructure_footprints(snapshot),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::RepoRef;
    use crate::snapshot::{HistoryFacts, FilesystemFacts, InfraFootprints};
    use crate::repo::branches::{BranchFacts, BranchInfo};
    use crate::repo::history::CommitWindow;
    use crate::github::RepoMetadata;

    fn make_test_snapshot(
        metadata: RepoMetaState,
        branches: Vec<BranchInfo>,
        last_activity_secs: i64,
    ) -> InvestigationSnapshot {
        InvestigationSnapshot {
            repo: RepoRef { owner: "owner".to_string(), repo: "repo".to_string() },
            metadata,
            history: HistoryFacts {
                total_commits: 100,
                repo_age_days: 365,
                contributor_count: 12,
                bus_factor: 3,
                top_author_share_pct: 45.0,
                window: CommitWindow { scanned: 100, capped: false },
                most_modified_file: Some("main.rs".to_string()),
                night_pct: 10.0,
                weekend_pct: 15.0,
                business_hours_pct: 75.0,
                commits_this_month: 25,
                top_contributor_name: Some("Alice".to_string()),
                oldest_file: Some("init.rs".to_string()),
                oldest_contributor: Some("Bob".to_string()),
            },
            branches: BranchFacts {
                default_branch: "develop".to_string(),
                branches,
                last_activity_secs,
            },
            filesystem: FilesystemFacts {
                languages: vec![("Rust".to_string(), 100.0)],
                infra: InfraFootprints {
                    docker: true,
                    terraform: false,
                    github_actions: true,
                    gitlab_ci: false,
                    circleci: false,
                    jenkins: false,
                    dependabot: false,
                    renovate: false,
                },
            },
        }
    }

    #[test]
    fn test_first_impressions_available() {
        let metadata = RepoMetaState::Available(RepoMetadata {
            stargazers_count: 100,
            forks_count: 50,
            description: None,
            topics: vec![],
            default_branch: "develop".to_string(),
            pushed_at: None,
            created_at: None,
        });

        let snapshot = make_test_snapshot(metadata, vec![], 123456789);
        let sections = build_factual_sections(&snapshot, 123456789);
        let fi = sections.first_impressions;

        assert_eq!(fi.repo_age_days, 365);
        assert_eq!(fi.default_branch, "develop");
        assert_eq!(fi.stars, Some(100));
        assert_eq!(fi.forks, Some(50));
        assert_eq!(fi.contributor_count, 12);
        assert_eq!(fi.last_activity_secs, Some(123456789));
    }

    #[test]
    fn test_first_impressions_unavailable() {
        let snapshot = make_test_snapshot(RepoMetaState::Unavailable, vec![], 0);
        let sections = build_factual_sections(&snapshot, 123456789);
        let fi = sections.first_impressions;

        assert_eq!(fi.stars, None);
        assert_eq!(fi.forks, None);
        assert_eq!(fi.last_activity_secs, None);
    }

    #[test]
    fn test_build_factual_sections_full() {
        let now_secs = 1000000000;
        let branches = vec![
            BranchInfo {
                name: "stale".to_string(),
                tip_time_secs: now_secs - 95 * 86400,
            },
        ];
        
        let snapshot = make_test_snapshot(RepoMetaState::Unavailable, branches, now_secs - 95 * 86400);
        let sections = build_factual_sections(&snapshot, now_secs);

        assert_eq!(sections.branch_jungle.total, 1);
        assert_eq!(sections.branch_jungle.stale, 1);
        assert_eq!(sections.branch_jungle.active, 0);
        
        assert_eq!(sections.commit_crimes.total_commits, 100);
        assert_eq!(sections.ancient_relics.oldest_file.as_deref(), Some("init.rs"));
        assert_eq!(sections.language_soup.languages[0].name, "Rust");
        assert!(sections.infrastructure.docker);
        assert!(!sections.infrastructure.terraform);
    }
}
