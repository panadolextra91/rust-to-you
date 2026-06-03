use crate::snapshot::InvestigationSnapshot;

pub struct AncientRelics {
    pub oldest_file: Option<String>,
    pub most_modified_file: Option<String>,
    pub oldest_contributor: Option<String>,
    pub longest_living_branch: Option<String>,
}

pub fn ancient_relics(snapshot: &InvestigationSnapshot) -> AncientRelics {
    let branches = &snapshot.branches.branches;
    let mut longest_living: Option<(&str, i64)> = None;
    
    for branch in branches {
        match longest_living {
            None => {
                longest_living = Some((&branch.name, branch.tip_time_secs));
            }
            Some((old_name, old_time)) => {
                if branch.tip_time_secs < old_time || (branch.tip_time_secs == old_time && branch.name.as_str() < old_name) {
                    longest_living = Some((&branch.name, branch.tip_time_secs));
                }
            }
        }
    }

    AncientRelics {
        oldest_file: snapshot.history.oldest_file.clone(),
        most_modified_file: snapshot.history.most_modified_file.clone(),
        oldest_contributor: snapshot.history.oldest_contributor.clone(),
        longest_living_branch: longest_living.map(|(name, _)| name.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::RepoRef;
    use crate::snapshot::{RepoMetaState, HistoryFacts, FilesystemFacts, InfraFootprints};
    use crate::repo::branches::{BranchFacts, BranchInfo};
    use crate::repo::history::CommitWindow;

    fn make_test_snapshot(branches: Vec<BranchInfo>, oldest_file: Option<String>, oldest_contributor: Option<String>, most_modified_file: Option<String>) -> InvestigationSnapshot {
        InvestigationSnapshot {
            repo: RepoRef { owner: "owner".to_string(), repo: "repo".to_string() },
            metadata: RepoMetaState::Unavailable,
            history: HistoryFacts {
                total_commits: 0,
                repo_age_days: 0,
                contributor_count: 0,
                bus_factor: 0,
                top_author_share_pct: 0.0,
                window: CommitWindow { scanned: 0, capped: false },
                most_modified_file,
                night_pct: 0.0,
                weekend_pct: 0.0,
                business_hours_pct: 0.0,
                commits_this_month: 0,
                top_contributor_name: None,
                oldest_file,
                oldest_contributor,
                release_tag_count: 0,
            },
            branches: BranchFacts {
                default_branch: "main".to_string(),
                branches,
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
        }
    }

    #[test]
    fn test_longest_living_branch() {
        let branches = vec![
            BranchInfo {
                name: "active-branch".to_string(),
                tip_time_secs: 2000,
            },
            BranchInfo {
                name: "old-branch".to_string(),
                tip_time_secs: 1000,
            },
        ];
        
        let snapshot = make_test_snapshot(
            branches,
            Some("oldest.txt".to_string()),
            Some("Alice".to_string()),
            Some("most_modified.txt".to_string())
        );
        
        let relics = ancient_relics(&snapshot);
        assert_eq!(relics.oldest_file.as_deref(), Some("oldest.txt"));
        assert_eq!(relics.most_modified_file.as_deref(), Some("most_modified.txt"));
        assert_eq!(relics.oldest_contributor.as_deref(), Some("Alice"));
        assert_eq!(relics.longest_living_branch.as_deref(), Some("old-branch"));
    }

    #[test]
    fn test_relics_empty_branches() {
        let snapshot = make_test_snapshot(
            vec![],
            None,
            None,
            None
        );
        
        let relics = ancient_relics(&snapshot);
        assert_eq!(relics.oldest_file, None);
        assert_eq!(relics.most_modified_file, None);
        assert_eq!(relics.oldest_contributor, None);
        assert_eq!(relics.longest_living_branch, None);
    }
}
