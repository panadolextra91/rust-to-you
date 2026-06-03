use crate::snapshot::InvestigationSnapshot;

pub struct CommitCrimes {
    pub total_commits: usize,
    pub commits_this_month: usize,
    pub top_contributor: Option<String>,
    pub bus_factor: usize,
    pub top_author_share_pct: f64,
}

pub fn commit_crimes(snapshot: &InvestigationSnapshot) -> CommitCrimes {
    CommitCrimes {
        total_commits: snapshot.history.total_commits,
        commits_this_month: snapshot.history.commits_this_month,
        top_contributor: snapshot.history.top_contributor_name.clone(),
        bus_factor: snapshot.history.bus_factor,
        top_author_share_pct: snapshot.history.top_author_share_pct,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::RepoRef;
    use crate::snapshot::{RepoMetaState, HistoryFacts, FilesystemFacts, InfraFootprints};
    use crate::repo::branches::BranchFacts;
    use crate::repo::history::CommitWindow;

    #[test]
    fn test_commit_crimes_maps_fields() {
        let history = HistoryFacts {
            total_commits: 150,
            repo_age_days: 10,
            contributor_count: 5,
            bus_factor: 2,
            top_author_share_pct: 60.0,
            window: CommitWindow { scanned: 100, capped: false },
            most_modified_file: Some("src/main.rs".to_string()),
            night_pct: 10.0,
            weekend_pct: 20.0,
            business_hours_pct: 70.0,
            commits_this_month: 40,
            top_contributor_name: Some("Alice".to_string()),
            oldest_file: None,
            oldest_contributor: None,
        };
        
        let snapshot = InvestigationSnapshot {
            repo: RepoRef { owner: "owner".to_string(), repo: "repo".to_string() },
            metadata: RepoMetaState::Unavailable,
            history,
            branches: BranchFacts {
                default_branch: "main".to_string(),
                branches: vec![],
                last_activity_secs: 0,
            },
            filesystem: FilesystemFacts {
                languages: vec![],
                infra: InfraFootprints {
                    docker: false,
                    terraform: false,
                    github_actions: false,
                    gitlab_ci: false,
                    circleci: false,
                    jenkins: false,
                    dependabot: false,
                    renovate: false,
                },
            },
        };
        
        let crimes = commit_crimes(&snapshot);
        assert_eq!(crimes.total_commits, 150);
        assert_eq!(crimes.commits_this_month, 40);
        assert_eq!(crimes.top_contributor.as_deref(), Some("Alice"));
        assert_eq!(crimes.bus_factor, 2);
        assert_eq!(crimes.top_author_share_pct, 60.0);
    }
}
