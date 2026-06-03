use crate::snapshot::InvestigationSnapshot;
use crate::report::sections::FactualSections;
use crate::analyze::vibes::VibeResult;
use crate::i18n::{bi, Bilingual};

#[derive(Debug, Clone)]
pub struct Finding {
    pub text: Bilingual,
    pub weight: u32,
}

pub fn interesting_findings(
    snapshot: &InvestigationSnapshot,
    sections: &FactualSections,
    vibe: &VibeResult,
) -> Vec<Finding> {
    let mut list = Vec::new();
    
    // Rule 1: Bus factor == 1
    if snapshot.history.bus_factor == 1 {
        let author = snapshot.history.top_contributor_name.as_deref().unwrap_or("Unknown");
        list.push(Finding {
            text: bi(
                format!("Hệ số xe buýt của dự án bằng 1. Nếu {} biến mất, dự án sẽ gặp nguy cơ lớn!", author),
                format!("The project's bus factor is 1. If {} goes missing, the project is in high danger!", author)
            ),
            weight: 100,
        });
    }

    // Rule 2: Single developer dominates (top_author_share_pct >= 80%)
    if snapshot.history.top_author_share_pct >= 80.0 {
        let author = snapshot.history.top_contributor_name.as_deref().unwrap_or("Unknown");
        list.push(Finding {
            text: bi(
                format!("{} đóng góp áp đảo với {:.1}% tổng số commit.", author, snapshot.history.top_author_share_pct),
                format!("{} dominates contribution with {:.1}% of all commits.", author, snapshot.history.top_author_share_pct)
            ),
            weight: 95,
        });
    }

    // Rule 3: No CI/CD safety net
    let has_ci = sections.infrastructure.github_actions
        || sections.infrastructure.gitlab_ci
        || sections.infrastructure.circleci
        || sections.infrastructure.jenkins;
    if !has_ci {
        list.push(Finding {
            text: bi(
                "Dự án hoạt động không có lưới bảo vệ CI/CD tự động.",
                "The project operates without any automated CI/CD safety net."
            ),
            weight: 80,
        });
    }

    // Rule 4: High stale branch count (stale >= 15)
    let stale_branches = sections.branch_jungle.stale;
    if stale_branches >= 15 {
        list.push(Finding {
            text: bi(
                format!("Có {} nhánh bỏ hoang chưa được dọn dẹp trong repo.", stale_branches),
                format!("There are {} stale branches left abandoned in the repo.", stale_branches)
            ),
            weight: 70,
        });
    }

    // Rule 5: Night owl commits (night_pct >= 30%)
    if snapshot.history.night_pct >= 30.0 {
        list.push(Finding {
            text: bi(
                format!("Các nhà phát triển làm việc nhiều về đêm ({:.1}% commit thực hiện từ nửa đêm đến 5 giờ sáng).", snapshot.history.night_pct),
                format!("Developers work hard through the night ({:.1}% commits between midnight and 5am).", snapshot.history.night_pct)
            ),
            weight: 60,
        });
    }

    // Rule 6: Longevity (age >= 5 years)
    let age_years = snapshot.history.repo_age_days / 365;
    if age_years >= 5 {
        list.push(Finding {
            text: bi(
                format!("Dự án đã hoạt động bền bỉ trong {} năm qua.", age_years),
                format!("The project has been active for {} years.", age_years)
            ),
            weight: 50,
        });
    }

    // Sort by weight descending
    list.sort_by(|a, b| b.weight.cmp(&a.weight));

    // Capped at 6 elements
    list.truncate(6);

    // If there is room and runner_up exists, add runner-up vibe finding line
    if let Some(runner) = vibe.runner_up {
        if list.len() < 6 {
            let runner_name = runner.display();
            list.push(Finding {
                text: bi(
                    format!("Cũng phảng phất khí chất {}", runner_name.vi),
                    format!("Also gives off {} energy", runner_name.en)
                ),
                weight: 5,
            });
        }
    }

    list
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::RepoRef;
    use crate::snapshot::{RepoMetaState, HistoryFacts, FilesystemFacts, InfraFootprints, BranchFacts};
    use crate::repo::history::CommitWindow;
    use crate::analyze::vibes::VibeLabel;

    fn make_test_snapshot(history: HistoryFacts, infra: InfraFootprints, _stale_branches: usize) -> InvestigationSnapshot {
        InvestigationSnapshot {
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
                infra,
            },
        }
    }

    fn default_history() -> HistoryFacts {
        HistoryFacts {
            total_commits: 10,
            repo_age_days: 10,
            contributor_count: 2,
            bus_factor: 2,
            top_author_share_pct: 40.0,
            window: CommitWindow { scanned: 10, capped: false },
            most_modified_file: None,
            night_pct: 0.0,
            weekend_pct: 0.0,
            business_hours_pct: 0.0,
            commits_this_month: 0,
            top_contributor_name: None,
            oldest_file: None,
            oldest_contributor: None,
            release_tag_count: 0,
        }
    }

    #[test]
    fn test_findings_bus_factor_and_no_ci() {
        let mut history = default_history();
        history.bus_factor = 1;
        history.top_contributor_name = Some("Alice".to_string());

        let infra = InfraFootprints {
            docker: false,
            terraform: false,
            github_actions: false,
            gitlab_ci: false,
            circleci: false,
            jenkins: false,
            dependabot: false,
            renovate: false,
        };

        let snapshot = make_test_snapshot(history, infra, 0);
        let sections = FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 10,
                default_branch: "main".to_string(),
                stars: None,
                forks: None,
                contributor_count: 2,
                last_activity_secs: None,
            },
            commit_crimes: crate::analyze::commit::CommitCrimes { total_commits: 10, commits_this_month: 0, top_contributor: Some("Alice".to_string()), bus_factor: 1, top_author_share_pct: 40.0 },
            branch_jungle: crate::analyze::branch::BranchJungle { total: 0, active: 0, stale: 0, oldest_branch: None },
            ancient_relics: crate::analyze::relics::AncientRelics { oldest_file: None, most_modified_file: None, oldest_contributor: None, longest_living_branch: None },
            language_soup: crate::analyze::language::LanguageSoup { languages: vec![] },
            infrastructure: crate::analyze::infra::InfrastructureFootprints { docker: false, terraform: false, github_actions: false, gitlab_ci: false, circleci: false, jenkins: false, dependabot: false, renovate: false },
        };

        let vibe = VibeResult {
            primary: VibeLabel::ChaoticGood,
            primary_score: 5,
            evidence: vec![],
            runner_up: Some(VibeLabel::SoloWizard),
        };

        let res = interesting_findings(&snapshot, &sections, &vibe);
        
        // Assert we got bus factor finding (weight 100), no CI finding (weight 80), and runner-up wizard finding (weight 5)
        assert_eq!(res.len(), 3);
        assert_eq!(res[0].weight, 100);
        assert_eq!(res[1].weight, 80);
        assert_eq!(res[2].weight, 5);
        assert!(res[0].text.vi.contains("Alice"));
        assert!(res[2].text.vi.contains("Solo Wizard"));
    }
}
