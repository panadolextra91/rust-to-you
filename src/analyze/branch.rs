use crate::snapshot::InvestigationSnapshot;
use crate::repo::branches::STALE_BRANCH_DAYS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchActivity {
    Active,
    Stale,
}

pub struct BranchJungle {
    pub total: usize,
    pub active: usize,
    pub stale: usize,
    pub oldest_branch: Option<String>,
}

pub fn branch_jungle(snapshot: &InvestigationSnapshot, now_secs: i64) -> BranchJungle {
    let branches = &snapshot.branches.branches;
    let total = branches.len();
    
    let cutoff = now_secs - STALE_BRANCH_DAYS * 86400;
    
    let mut active = 0;
    let mut stale = 0;
    let mut oldest_info: Option<(&str, i64)> = None;
    
    for branch in branches {
        if branch.tip_time_secs < cutoff {
            stale += 1;
        } else {
            active += 1;
        }
        
        match oldest_info {
            None => {
                oldest_info = Some((&branch.name, branch.tip_time_secs));
            }
            Some((old_name, old_time)) => {
                if branch.tip_time_secs < old_time || (branch.tip_time_secs == old_time && branch.name.as_str() < old_name) {
                    oldest_info = Some((&branch.name, branch.tip_time_secs));
                }
            }
        }
    }
    
    BranchJungle {
        total,
        active,
        stale,
        oldest_branch: oldest_info.map(|(name, _)| name.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::RepoRef;
    use crate::snapshot::{RepoMetaState, HistoryFacts, FilesystemFacts, InfraFootprints};
    use crate::repo::branches::{BranchFacts, BranchInfo};
    use crate::repo::history::CommitWindow;

    fn make_test_snapshot(branches: Vec<BranchInfo>) -> InvestigationSnapshot {
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
                most_modified_file: None,
                night_pct: 0.0,
                weekend_pct: 0.0,
                business_hours_pct: 0.0,
                commits_this_month: 0,
                top_contributor_name: None,
                oldest_file: None,
                oldest_contributor: None,
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
    fn test_branch_jungle_stale_split() {
        let now_secs = 1000000000;
        let stale_time = now_secs - 95 * 86400;
        let active_time = now_secs - 86400;

        let branches = vec![
            BranchInfo {
                name: "stale-branch".to_string(),
                tip_time_secs: stale_time,
            },
            BranchInfo {
                name: "active-branch".to_string(),
                tip_time_secs: active_time,
            },
        ];

        let snapshot = make_test_snapshot(branches);
        let jungle = branch_jungle(&snapshot, now_secs);

        assert_eq!(jungle.total, 2);
        assert_eq!(jungle.active, 1);
        assert_eq!(jungle.stale, 1);
        assert_eq!(jungle.oldest_branch.as_deref(), Some("stale-branch"));
    }

    #[test]
    fn test_branch_jungle_empty() {
        let snapshot = make_test_snapshot(vec![]);
        let jungle = branch_jungle(&snapshot, 1000000000);

        assert_eq!(jungle.total, 0);
        assert_eq!(jungle.active, 0);
        assert_eq!(jungle.stale, 0);
        assert_eq!(jungle.oldest_branch, None);
    }

    #[test]
    fn test_branch_jungle_tie_break() {
        let now_secs = 1000000000;
        let branch_time = now_secs - 10 * 86400;
        
        let branches = vec![
            BranchInfo {
                name: "beta-branch".to_string(),
                tip_time_secs: branch_time,
            },
            BranchInfo {
                name: "alpha-branch".to_string(),
                tip_time_secs: branch_time,
            },
        ];

        let snapshot = make_test_snapshot(branches);
        let jungle = branch_jungle(&snapshot, now_secs);
        assert_eq!(jungle.oldest_branch.as_deref(), Some("alpha-branch"));
    }
}
