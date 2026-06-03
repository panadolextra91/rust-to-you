use git2::{Repository, Sort};
use std::collections::HashMap;

pub fn repo_age_days(repo: &Repository) -> Result<i64, git2::Error> {
    let now = chrono::Utc::now().timestamp();
    repo_age_days_relative_to(repo, now)
}

fn repo_age_days_relative_to(repo: &Repository, now_secs: i64) -> Result<i64, git2::Error> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TIME | Sort::REVERSE)?;

    let first_commit_oid = match revwalk.next() {
        Some(Ok(oid)) => oid,
        Some(Err(e)) => return Err(e),
        None => return Err(git2::Error::from_str("No commits found")),
    };
    
    let commit = repo.find_commit(first_commit_oid)?;
    let time = commit.author().when().seconds();
    
    Ok((now_secs - time) / 86400)
}

pub fn total_commits(repo: &git2::Repository) -> Result<usize, git2::Error> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    Ok(revwalk.count())
}

#[derive(Debug)]
pub struct CommitWindow {
    pub scanned: usize,
    pub capped: bool,
}

fn is_authored_commit(commit: &git2::Commit) -> bool {
    if commit.parent_count() > 1 {
        return false;
    }
    let sig = commit.author();
    let name = sig.name().unwrap_or("");
    let email = sig.email().unwrap_or("");
    
    if name.ends_with("[bot]") {
        return false;
    }
    
    let known_bots = ["dependabot[bot]", "renovate[bot]", "github-actions[bot]"];
    for bot in known_bots {
        if email.contains(bot) || name.contains(bot) {
            return false;
        }
    }
    true
}

fn normalized_identity(repo: &git2::Repository, sig: &git2::Signature) -> String {
    if let Ok(mailmap) = repo.mailmap() {
        if let Ok(resolved) = mailmap.resolve_signature(sig) {
            let email = resolved.email().unwrap_or("").to_lowercase();
            return email;
        }
    }
    sig.email().unwrap_or("").to_lowercase()
}

pub struct ContributorStats {
    pub contributor_count: usize,
    pub bus_factor: usize,
    pub top_author_share_pct: f64,
}

pub fn collect_contributors(repo: &git2::Repository) -> Result<ContributorStats, git2::Error> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    
    let mut counts = HashMap::new();
    let mut total_filtered = 0;
    
    let mut unfiltered_counts = HashMap::new();
    
    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        
        if is_authored_commit(&commit) {
            let id = normalized_identity(repo, &commit.author());
            *counts.entry(id).or_insert(0) += 1;
            total_filtered += 1;
        }
        
        if commit.parent_count() <= 1 {
            let id = normalized_identity(repo, &commit.author());
            *unfiltered_counts.entry(id).or_insert(0) += 1;
        }
    }
    
    let (map, total) = if counts.is_empty() && !unfiltered_counts.is_empty() {
        let sum = unfiltered_counts.values().sum::<u32>();
        (unfiltered_counts, sum)
    } else {
        (counts, total_filtered)
    };
    
    if map.is_empty() {
        return Ok(ContributorStats {
            contributor_count: 0,
            bus_factor: 0,
            top_author_share_pct: 0.0,
        });
    }
    
    let contributor_count = map.len();
    
    let mut entries: Vec<_> = map.into_iter().collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    
    let top_count = entries.first().map(|(_, c)| *c).unwrap_or(0);
    let top_author_share_pct = (top_count as f64 / total as f64) * 100.0;
    
    let mut bus_factor = 0;
    let mut accum = 0;
    let threshold = (total as f64 * 0.5).ceil() as u32;
    
    for (_, count) in entries {
        bus_factor += 1;
        accum += count;
        if accum >= threshold {
            break;
        }
    }
    
    Ok(ContributorStats {
        contributor_count,
        bus_factor,
        top_author_share_pct,
    })
}

pub struct BoundedHistory {
    pub most_modified_file: Option<String>,
    pub night_pct: f64,
    pub weekend_pct: f64,
    pub business_hours_pct: f64,
    pub window: CommitWindow,
}

pub const COMMIT_WINDOW_CAP: usize = 1000;

pub fn collect_bounded(repo: &git2::Repository, cap: usize) -> Result<BoundedHistory, git2::Error> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TIME)?;
    
    let total = total_commits(repo)?;
    let capped = total > cap;
    
    let mut scanned = 0;
    let mut file_counts = HashMap::new();
    let mut night = 0;
    let mut weekend = 0;
    let mut business = 0;
    let mut valid_time_commits = 0;
    
    for oid in revwalk.take(cap) {
        scanned += 1;
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        
        let time = commit.author().when();
        let offset = chrono::FixedOffset::east_opt(time.offset_minutes() * 60).unwrap_or_else(|| chrono::FixedOffset::east_opt(0).unwrap());
        use chrono::TimeZone;
        let dt = offset.timestamp_opt(time.seconds(), 0).unwrap();
        use chrono::Datelike;
        use chrono::Timelike;
        
        let hour = dt.hour();
        let wday = dt.weekday();
        
        valid_time_commits += 1;
        
        if (0..5).contains(&hour) {
            night += 1;
        }
        let is_weekend = wday == chrono::Weekday::Sat || wday == chrono::Weekday::Sun;
        if is_weekend {
            weekend += 1;
        }
        if !is_weekend && (9..18).contains(&hour) {
            business += 1;
        }
        
        if commit.parent_count() <= 1 {
            let tree = commit.tree()?;
            let parent_tree = if commit.parent_count() == 1 {
                Some(commit.parent(0)?.tree()?)
            } else {
                None
            };
            
            let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)?;
            
            for delta in diff.deltas() {
                if let Some(path) = delta.new_file().path() {
                    let p = path.to_string_lossy().into_owned();
                    *file_counts.entry(p).or_insert(0) += 1;
                }
            }
        }
    }
    
    let most_modified_file = file_counts.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(path, _)| path);
        
    let night_pct = if valid_time_commits > 0 { (night as f64 / valid_time_commits as f64) * 100.0 } else { 0.0 };
    let weekend_pct = if valid_time_commits > 0 { (weekend as f64 / valid_time_commits as f64) * 100.0 } else { 0.0 };
    let business_hours_pct = if valid_time_commits > 0 { (business as f64 / valid_time_commits as f64) * 100.0 } else { 0.0 };
    
    Ok(BoundedHistory {
        most_modified_file,
        night_pct,
        weekend_pct,
        business_hours_pct,
        window: CommitWindow { scanned, capped },
    })
}

#[cfg(test)]
pub(crate) fn make_fixture_repo() -> tempfile::TempDir {
    let tmp = tempfile::Builder::new().prefix("fixture-").tempdir().unwrap();
    let repo = Repository::init(tmp.path()).unwrap();
    
    let mailmap_content = "Alice <alice@example.com> <alias@example.com>\n";
    std::fs::write(tmp.path().join(".mailmap"), mailmap_content).unwrap();
    std::fs::write(tmp.path().join("hot.rs"), "hot1").unwrap();
    std::fs::write(tmp.path().join("cold.rs"), "cold").unwrap();
    
    let mut index = repo.index().unwrap();
    index.add_path(std::path::Path::new(".mailmap")).unwrap();
    index.add_path(std::path::Path::new("hot.rs")).unwrap();
    index.add_path(std::path::Path::new("cold.rs")).unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    
    let time1 = 1000000000;
    let sig1 = git2::Signature::new("Alice", "alice@example.com", &git2::Time::new(time1, 0)).unwrap();
    let commit1_oid = repo.commit(Some("HEAD"), &sig1, &sig1, "Initial commit", &tree, &[]).unwrap();
    let commit1 = repo.find_commit(commit1_oid).unwrap();
    
    std::fs::write(tmp.path().join("hot.rs"), "hot2").unwrap();
    let mut index2 = repo.index().unwrap();
    index2.add_path(std::path::Path::new("hot.rs")).unwrap();
    let tree2_id = index2.write_tree().unwrap();
    let tree2 = repo.find_tree(tree2_id).unwrap();
    
    let time2 = time1 + 86400; 
    let sig2 = git2::Signature::new("Bob", "bob@example.com", &git2::Time::new(time2, 0)).unwrap();
    let commit2_oid = repo.commit(Some("HEAD"), &sig2, &sig2, "Bob commit", &tree2, &[&commit1]).unwrap();
    let commit2 = repo.find_commit(commit2_oid).unwrap();
    
    std::fs::write(tmp.path().join("hot.rs"), "hot3").unwrap();
    let mut index3 = repo.index().unwrap();
    index3.add_path(std::path::Path::new("hot.rs")).unwrap();
    let tree3_id = index3.write_tree().unwrap();
    let tree3 = repo.find_tree(tree3_id).unwrap();
    
    let time3 = time1 + 2 * 86400;
    let sig3 = git2::Signature::new("Alice Alias", "alias@example.com", &git2::Time::new(time3, 0)).unwrap();
    let commit3_oid = repo.commit(Some("HEAD"), &sig3, &sig3, "Alias commit", &tree3, &[&commit2]).unwrap();
    let commit3 = repo.find_commit(commit3_oid).unwrap();
    
    std::fs::write(tmp.path().join("hot.rs"), "hot4").unwrap();
    let mut index4 = repo.index().unwrap();
    index4.add_path(std::path::Path::new("hot.rs")).unwrap();
    let tree4_id = index4.write_tree().unwrap();
    let tree4 = repo.find_tree(tree4_id).unwrap();
    
    let time4 = time1 + 3 * 86400;
    let sig4 = git2::Signature::new("dependabot[bot]", "dependabot[bot]@users.noreply.github.com", &git2::Time::new(time4, 0)).unwrap();
    let commit4_oid = repo.commit(Some("HEAD"), &sig4, &sig4, "Bot commit", &tree4, &[&commit3]).unwrap();
    let commit4 = repo.find_commit(commit4_oid).unwrap();
    
    let time5 = time1 + 4 * 86400;
    let sig5 = git2::Signature::new("Alice", "alice@example.com", &git2::Time::new(time5, 0)).unwrap();
    repo.commit(Some("HEAD"), &sig5, &sig5, "Merge commit", &tree4, &[&commit4, &commit1]).unwrap();
    
    tmp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo_age() {
        let tmp = make_fixture_repo();
        let repo = Repository::open(tmp.path()).unwrap();
        
        let time1 = 1000000000;
        let test_now = time1 + 25 * 86400; 
        
        let age_days = repo_age_days_relative_to(&repo, test_now).unwrap();
        
        assert_eq!(age_days, 25);
    }
    
    #[test]
    fn test_total_commits() {
        let tmp = make_fixture_repo();
        let repo = Repository::open(tmp.path()).unwrap();
        assert_eq!(total_commits(&repo).unwrap(), 5);
    }

    #[test]
    fn test_bus_factor() {
        let tmp = make_fixture_repo();
        let repo = Repository::open(tmp.path()).unwrap();
        let stats = collect_contributors(&repo).unwrap();
        assert_eq!(stats.contributor_count, 2);
        assert_eq!(stats.bus_factor, 1);
        assert_eq!(stats.top_author_share_pct, (2.0 / 3.0) * 100.0);
    }
    
    #[test]
    fn test_most_modified() {
        let tmp = make_fixture_repo();
        let repo = Repository::open(tmp.path()).unwrap();
        
        let bounded = collect_bounded(&repo, 10).unwrap();
        assert_eq!(bounded.most_modified_file.as_deref(), Some("hot.rs"));
        assert_eq!(bounded.window.scanned, 5);
        assert_eq!(bounded.window.capped, false);
        
        let bounded2 = collect_bounded(&repo, 2).unwrap();
        assert_eq!(bounded2.most_modified_file.as_deref(), Some("hot.rs")); 
        assert_eq!(bounded2.window.scanned, 2);
        assert_eq!(bounded2.window.capped, true);
    }
    
    #[test]
    fn test_branch_count() {
        let tmp = make_fixture_repo();
        let repo = Repository::open(tmp.path()).unwrap();
        let head_commit = repo.head().unwrap().peel_to_commit().unwrap();
        repo.branch("feature", &head_commit, false).unwrap();
        
        let clone_tmp = tempfile::Builder::new().prefix("clone-").tempdir().unwrap();
        let url = format!("file://{}", tmp.path().display());
        let cloned_repo = Repository::clone(&url, clone_tmp.path()).unwrap();
        
        let facts = crate::repo::branches::enumerate_branches(&cloned_repo).unwrap();
        assert_eq!(facts.branches.len(), 2);
        assert!(facts.default_branch == "master" || facts.default_branch == "main");
    }

    #[test]
    fn test_time_of_day_buckets() {
        let tmp = tempfile::Builder::new().prefix("fixture-tod-").tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        
        let mut index = repo.index().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        
        let time1 = git2::Time::new(1704074400, 0); // 2024-01-01 02:00:00 (Mon) -> Night
        let sig1 = git2::Signature::new("Author", "a@b.com", &time1).unwrap();
        let c1 = repo.commit(Some("HEAD"), &sig1, &sig1, "C1", &tree, &[]).unwrap();
        let c1_obj = repo.find_commit(c1).unwrap();
        
        let time2 = git2::Time::new(1704549600, 0); // 2024-01-06 14:00:00 (Sat) -> Weekend
        let sig2 = git2::Signature::new("Author", "a@b.com", &time2).unwrap();
        let c2 = repo.commit(Some("HEAD"), &sig2, &sig2, "C2", &tree, &[&c1_obj]).unwrap();
        let c2_obj = repo.find_commit(c2).unwrap();
        
        let time3 = git2::Time::new(1704189600, 0); // 2024-01-02 10:00:00 (Tue) -> Business
        let sig3 = git2::Signature::new("Author", "a@b.com", &time3).unwrap();
        let c3 = repo.commit(Some("HEAD"), &sig3, &sig3, "C3", &tree, &[&c2_obj]).unwrap();
        let c3_obj = repo.find_commit(c3).unwrap();
        
        let time4 = git2::Time::new(1704506400, 0); // 2024-01-06 02:00:00 (Sat) -> Night + Weekend
        let sig4 = git2::Signature::new("Author", "a@b.com", &time4).unwrap();
        let _c4 = repo.commit(Some("HEAD"), &sig4, &sig4, "C4", &tree, &[&c3_obj]).unwrap();
        
        let stats = collect_bounded(&repo, 10).unwrap();
        
        assert_eq!(stats.night_pct, 50.0);
        assert_eq!(stats.weekend_pct, 50.0);
        assert_eq!(stats.business_hours_pct, 25.0);
    }
}
