use git2::{Repository, BranchType};

pub const STALE_BRANCH_DAYS: i64 = 90;

pub struct BranchInfo {
    pub name: String,
    pub tip_time_secs: i64,
}

pub struct BranchFacts {
    pub default_branch: String,
    pub branches: Vec<BranchInfo>,
    pub last_activity_secs: i64,
}

pub fn default_branch(repo: &Repository) -> String {
    if let Ok(head) = repo.head() {
        if let Ok(shorthand) = head.shorthand() {
            return shorthand.to_string();
        }
    }
    "main".to_string()
}

pub fn enumerate_branches(repo: &Repository) -> Result<BranchFacts, git2::Error> {
    let mut branches = Vec::new();
    let mut last_activity_secs = 0;
    
    let default_br = default_branch(repo);
    
    let mut seen = std::collections::HashSet::new();

    for branch_res in repo.branches(Some(BranchType::Remote))? {
        let (branch, _) = branch_res?;
        if let Ok(Some(name)) = branch.name() {
            let stripped = name.strip_prefix("origin/").unwrap_or(name);
            
            if stripped == "HEAD" {
                continue;
            }
            
            if !seen.insert(stripped.to_string()) {
                continue;
            }
            
            let tip_time = if let Ok(commit) = branch.get().peel_to_commit() {
                commit.author().when().seconds()
            } else {
                0
            };
            
            if tip_time > last_activity_secs {
                last_activity_secs = tip_time;
            }
            
            branches.push(BranchInfo {
                name: stripped.to_string(),
                tip_time_secs: tip_time,
            });
        }
    }

    Ok(BranchFacts {
        default_branch: default_br,
        branches,
        last_activity_secs,
    })
}

pub fn release_tag_count(repo: &Repository) -> Result<usize, git2::Error> {
    let tags = repo.tag_names(None)?;
    Ok(tags.iter().flatten().count())
}
