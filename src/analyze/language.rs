use crate::snapshot::InvestigationSnapshot;

pub struct LanguageEntry {
    pub name: String,
    pub pct: f64,
}

pub struct LanguageSoup {
    pub languages: Vec<LanguageEntry>,
}

pub fn language_soup(snapshot: &InvestigationSnapshot) -> LanguageSoup {
    let languages = snapshot.filesystem.languages.iter()
        .map(|(name, pct)| LanguageEntry {
            name: name.clone(),
            pct: *pct,
        })
        .collect();
    
    LanguageSoup { languages }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::RepoRef;
    use crate::snapshot::{RepoMetaState, HistoryFacts, FilesystemFacts, InfraFootprints};
    use crate::repo::branches::BranchFacts;
    use crate::repo::history::CommitWindow;

    #[test]
    fn test_language_soup_maps_pct() {
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
                languages: vec![("Rust".to_string(), 85.5), ("Markdown".to_string(), 14.5)],
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
        
        let soup = language_soup(&snapshot);
        assert_eq!(soup.languages.len(), 2);
        assert_eq!(soup.languages[0].name, "Rust");
        assert_eq!(soup.languages[0].pct, 85.5);
        assert_eq!(soup.languages[1].name, "Markdown");
        assert_eq!(soup.languages[1].pct, 14.5);
    }
}
