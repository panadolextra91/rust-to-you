use crate::snapshot::InvestigationSnapshot;
use crate::report::sections::FactualSections;
use crate::i18n::{bi, Bilingual};

#[derive(Debug, Clone)]
pub struct CrabVerdict {
    pub strengths: Vec<Bilingual>,
    pub risks: Vec<Bilingual>,
    pub overall_label: Bilingual,
}

pub fn crab_verdict(
    snapshot: &InvestigationSnapshot,
    sections: &FactualSections,
    now_secs: i64,
) -> CrabVerdict {
    let mut strengths = Vec::new();
    let mut risks = Vec::new();

    let days_since_last_commit = sections.first_impressions.last_activity_secs.map(|t| {
        (now_secs - t).max(0) / 86400
    });

    let has_ci = sections.infrastructure.github_actions
        || sections.infrastructure.gitlab_ci
        || sections.infrastructure.circleci
        || sections.infrastructure.jenkins;

    let release_tag_count = snapshot.history.release_tag_count;

    // --- Strengths ---
    if let Some(days) = days_since_last_commit {
        if days <= 30 {
            strengths.push(bi(
                "Dự án có hoạt động gần đây khá tích cực.",
                "The project has recent activity and is actively maintained."
            ));
        }
    }
    if has_ci {
        strengths.push(bi(
            "Hạ tầng có trang bị lưới bảo vệ CI/CD tự động.",
            "CI/CD pipeline provides automatic integration checks."
        ));
    }
    if snapshot.history.bus_factor >= 3 {
        strengths.push(bi(
            format!("Tri thức dự án được phân bổ tốt (Hệ số xe buýt {}).", snapshot.history.bus_factor),
            format!("Project knowledge is distributed (Bus factor {}).", snapshot.history.bus_factor)
        ));
    }
    if snapshot.history.top_author_share_pct < 45.0 {
        strengths.push(bi(
            "Đóng góp từ cộng đồng cân bằng, không phụ thuộc quá mức vào một cá nhân.",
            "Healthy contributor distribution without single-dev dominance."
        ));
    }
    if release_tag_count > 0 {
        strengths.push(bi(
            format!("Dự án có phát hành phiên bản rõ ràng ({} thẻ phát hành).", release_tag_count),
            format!("The project tracks releases with tags ({} release tags).", release_tag_count)
        ));
    }

    // --- Risks ---
    if snapshot.history.bus_factor == 1 {
        risks.push(bi(
            "Nguy cơ nghẽn tri thức cao khi hệ số xe buýt bằng 1.",
            "High risk of knowledge loss with bus factor 1."
        ));
    }
    if !has_ci {
        risks.push(bi(
            "Không có hệ thống kiểm thử tự động, dễ gặp lỗi khi có người đóng góp mới.",
            "No automated CI/CD safety net increases regression risks."
        ));
    }
    if sections.branch_jungle.stale >= 10 {
        risks.push(bi(
            format!("Repo chứa nhiều nhánh bị bỏ hoang ({} nhánh stale).", sections.branch_jungle.stale),
            format!("Repo holds many abandoned branches ({} stale branches).", sections.branch_jungle.stale)
        ));
    }
    if let Some(days) = days_since_last_commit {
        if days > 120 {
            risks.push(bi(
                format!("Dự án đang trong trạng thái ngủ đông (không hoạt động {} ngày).", days),
                format!("The project seems dormant (no activity for {} days).", days)
            ));
        }
    }
    if snapshot.history.top_author_share_pct >= 70.0 {
        risks.push(bi(
            format!("Đóng góp phụ thuộc quá nhiều vào tác giả chính ({:.1}%).", snapshot.history.top_author_share_pct),
            format!("Contributions are highly dependent on the main author ({:.1}%).", snapshot.history.top_author_share_pct)
        ));
    }

    // --- Overall Label Heuristics ---
    let overall_label = if risks.is_empty() && strengths.len() >= 3 {
        bi("Khỏe mạnh / Healthy", "Khỏe mạnh / Healthy")
    } else if risks.len() >= 3 {
        bi("Bị ám / Haunted", "Bị ám / Haunted")
    } else {
        bi("Cần chăm sóc / Needs care", "Cần chăm sóc / Needs care")
    };

    CrabVerdict {
        strengths,
        risks,
        overall_label,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::RepoRef;
    use crate::snapshot::{RepoMetaState, HistoryFacts, FilesystemFacts, InfraFootprints, BranchFacts};
    use crate::repo::history::CommitWindow;

    fn make_test_snapshot(history: HistoryFacts, infra: InfraFootprints, last_activity_secs: i64) -> InvestigationSnapshot {
        InvestigationSnapshot {
            repo: RepoRef { owner: "owner".to_string(), repo: "repo".to_string() },
            metadata: RepoMetaState::Unavailable,
            history,
            branches: BranchFacts {
                default_branch: "main".to_string(),
                branches: vec![],
                last_activity_secs,
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
    fn test_verdict_healthy() {
        let mut history = default_history();
        history.bus_factor = 3;
        history.top_author_share_pct = 30.0;
        history.release_tag_count = 5;

        let infra = InfraFootprints {
            docker: false,
            terraform: false,
            github_actions: true,
            gitlab_ci: false,
            circleci: false,
            jenkins: false,
            dependabot: false,
            renovate: false,
        };

        let snapshot = make_test_snapshot(history, infra, 1700000000);
        let sections = FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 10,
                default_branch: "main".to_string(),
                stars: None,
                forks: None,
                contributor_count: 5,
                last_activity_secs: Some(1700000000),
            },
            commit_crimes: crate::analyze::commit::CommitCrimes { total_commits: 10, commits_this_month: 0, top_contributor: None, bus_factor: 3, top_author_share_pct: 30.0 },
            branch_jungle: crate::analyze::branch::BranchJungle { total: 0, active: 0, stale: 0, oldest_branch: None },
            ancient_relics: crate::analyze::relics::AncientRelics { oldest_file: None, most_modified_file: None, oldest_contributor: None, longest_living_branch: None },
            language_soup: crate::analyze::language::LanguageSoup { languages: vec![] },
            infrastructure: crate::analyze::infra::InfrastructureFootprints { docker: false, terraform: false, github_actions: true, gitlab_ci: false, circleci: false, jenkins: false, dependabot: false, renovate: false },
        };

        let verdict = crab_verdict(&snapshot, &sections, 1700000000);
        assert!(verdict.overall_label.vi.contains("Healthy"));
        assert!(verdict.risks.is_empty());
        assert!(!verdict.strengths.is_empty());
    }

    #[test]
    fn test_verdict_haunted() {
        let mut history = default_history();
        history.bus_factor = 1;
        history.top_author_share_pct = 85.0;

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

        let snapshot = make_test_snapshot(history, infra, 1600000000);
        let sections = FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 10,
                default_branch: "main".to_string(),
                stars: None,
                forks: None,
                contributor_count: 2,
                last_activity_secs: Some(1600000000),
            },
            commit_crimes: crate::analyze::commit::CommitCrimes { total_commits: 10, commits_this_month: 0, top_contributor: None, bus_factor: 1, top_author_share_pct: 85.0 },
            branch_jungle: crate::analyze::branch::BranchJungle { total: 0, active: 0, stale: 15, oldest_branch: None },
            ancient_relics: crate::analyze::relics::AncientRelics { oldest_file: None, most_modified_file: None, oldest_contributor: None, longest_living_branch: None },
            language_soup: crate::analyze::language::LanguageSoup { languages: vec![] },
            infrastructure: crate::analyze::infra::InfrastructureFootprints { docker: false, terraform: false, github_actions: false, gitlab_ci: false, circleci: false, jenkins: false, dependabot: false, renovate: false },
        };

        let verdict = crab_verdict(&snapshot, &sections, 1700000000); // 100,000,000 secs difference, which is > 120 days
        assert!(verdict.overall_label.vi.contains("Haunted"));
        assert!(!verdict.risks.is_empty());
    }
}
