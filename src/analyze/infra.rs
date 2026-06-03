use crate::snapshot::InvestigationSnapshot;

pub struct InfrastructureFootprints {
    pub docker: bool,
    pub terraform: bool,
    pub github_actions: bool,
    pub gitlab_ci: bool,
    pub circleci: bool,
    pub jenkins: bool,
    pub dependabot: bool,
    pub renovate: bool,
}

pub fn infrastructure_footprints(snapshot: &InvestigationSnapshot) -> InfrastructureFootprints {
    let infra = &snapshot.filesystem.infra;
    InfrastructureFootprints {
        docker: infra.docker,
        terraform: infra.terraform,
        github_actions: infra.github_actions,
        gitlab_ci: infra.gitlab_ci,
        circleci: infra.circleci,
        jenkins: infra.jenkins,
        dependabot: infra.dependabot,
        renovate: infra.renovate,
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
    fn test_infra_footprints_mirror() {
        let snapshot = InvestigationSnapshot {
            repo: RepoRef { owner: "owner".to_string(), repo: "repo".to_string() },
            metadata: RepoMetaState::Unavailable,
            history: HistoryFacts {
                total_commits: 0,
                repo_age_days: 0,
                contributor_count: 0,
                bus_factor: 0,
                top_author_share_pct: 0.0,
                window: CommitWindow { scanned: 0, capped: false },
                most_modified_file: None,
                night_pct: 0.0,
                weekend_pct: 0.0,
                business_hours_pct: 0.0,
                commits_this_month: 0,
                top_contributor_name: None,
                oldest_file: None,
                oldest_contributor: None,
            },
            branches: BranchFacts {
                default_branch: "main".to_string(),
                branches: vec![],
                last_activity_secs: 0,
            },
            filesystem: FilesystemFacts {
                languages: vec![],
                infra: InfraFootprints {
                    docker: true,
                    terraform: false,
                    github_actions: true,
                    gitlab_ci: false,
                    circleci: true,
                    jenkins: false,
                    dependabot: true,
                    renovate: false,
                },
            },
        };
        
        let infra = infrastructure_footprints(&snapshot);
        assert!(infra.docker);
        assert!(!infra.terraform);
        assert!(infra.github_actions);
        assert!(!infra.gitlab_ci);
        assert!(infra.circleci);
        assert!(!infra.jenkins);
        assert!(infra.dependabot);
        assert!(!infra.renovate);
    }
}
