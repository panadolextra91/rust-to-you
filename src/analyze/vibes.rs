use crate::snapshot::InvestigationSnapshot;
use crate::report::sections::FactualSections;
use crate::i18n::{bi, Bilingual};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VibeLabel {
    SoloWizard,
    AncientTemple,
    CorporateFortress,
    OpenSourceKingdom,
    StartupGoblin,
    SleepDeprivedStartup,
    ChaoticGood,
}

impl VibeLabel {
    pub fn display(&self) -> Bilingual {
        match self {
            Self::SoloWizard => bi("🧙 Solo Wizard / Phù thủy đơn độc", "🧙 Solo Wizard / Phù thủy đơn độc"),
            Self::AncientTemple => bi("🏛️ Ancient Temple / Đền cổ", "🏛️ Ancient Temple / Đền cổ"),
            Self::CorporateFortress => bi("🏢 Corporate Fortress / Pháo đài công sở", "🏢 Corporate Fortress / Pháo đài công sở"),
            Self::OpenSourceKingdom => bi("👑 Open Source Kingdom / Vương quốc mã nguồn mở", "👑 Open Source Kingdom / Vương quốc mã nguồn mở"),
            Self::StartupGoblin => bi("🧌 Startup Goblin / Yêu tinh khởi nghiệp", "🧌 Startup Goblin / Yêu tinh khởi nghiệp"),
            Self::SleepDeprivedStartup => bi("😵 Sleep-Deprived Startup / Khởi nghiệp thiếu ngủ", "😵 Sleep-Deprived Startup / Khởi nghiệp thiếu ngủ"),
            Self::ChaoticGood => bi("🔥 Chaotic Good / Hỗn loạn nhưng tốt bụng", "🔥 Chaotic Good / Hỗn loạn nhưng tốt bụng"),
        }
    }
}

pub const MIN_SCORE: u32 = 4;

#[derive(Debug, Clone)]
pub struct VibeResult {
    pub primary: VibeLabel,
    pub primary_score: u32,
    pub evidence: Vec<Bilingual>,
    pub runner_up: Option<VibeLabel>,
}

pub fn classify_vibes(snapshot: &InvestigationSnapshot, sections: &FactualSections, now_secs: i64) -> VibeResult {
    let contributor_count = snapshot.history.contributor_count;
    let top_author_share = snapshot.history.top_author_share_pct;
    let bus_factor = snapshot.history.bus_factor;
    
    let age_years = snapshot.history.repo_age_days / 365;
    let days_since_last_commit = sections.first_impressions.last_activity_secs.map(|t| {
        (now_secs - t).max(0) / 86400
    });
    
    let release_tag_count = snapshot.history.release_tag_count;
    
    let has_ci = sections.infrastructure.github_actions
        || sections.infrastructure.gitlab_ci
        || sections.infrastructure.circleci
        || sections.infrastructure.jenkins;
        
    let mut infra_count = 0;
    if sections.infrastructure.docker { infra_count += 1; }
    if sections.infrastructure.terraform { infra_count += 1; }
    if sections.infrastructure.github_actions { infra_count += 1; }
    if sections.infrastructure.gitlab_ci { infra_count += 1; }
    if sections.infrastructure.circleci { infra_count += 1; }
    if sections.infrastructure.jenkins { infra_count += 1; }
    if sections.infrastructure.dependabot { infra_count += 1; }
    if sections.infrastructure.renovate { infra_count += 1; }
    
    let has_bot = sections.infrastructure.dependabot || sections.infrastructure.renovate;
    let business_hours_pct = snapshot.history.business_hours_pct;
    
    let branch_count = snapshot.branches.branches.len();
    let stale_branch_count = sections.branch_jungle.stale;
    let commits_last_30d = snapshot.history.commits_this_month;
    let night_pct = snapshot.history.night_pct;

    // 1. Solo Wizard
    let mut solo_wizard_score = 0;
    let mut solo_wizard_evidence = Vec::new();
    let top_contributor = snapshot.history.top_contributor_name.as_deref().unwrap_or("Unknown");
    
    if top_author_share >= 65.0 {
        solo_wizard_score += 3;
        solo_wizard_evidence.push(bi(
            format!("Tác giả chính {} đóng góp {:.1}% tổng số commit", top_contributor, top_author_share),
            format!("Top author {} contributed {:.1}% of all commits", top_contributor, top_author_share)
        ));
    }
    if contributor_count <= 5 {
        solo_wizard_score += 2;
        solo_wizard_evidence.push(bi(
            format!("Chỉ có {} người đóng góp từ trước đến nay", contributor_count),
            format!("Only {} contributors ever", contributor_count)
        ));
    }
    if bus_factor == 1 {
        solo_wizard_score += 2;
        solo_wizard_evidence.push(bi(
            format!("Hệ số xe buýt là 1 — {} nắm giữ {:.1}% ☠️", top_contributor, top_author_share),
            format!("Bus factor 1 — {} owns {:.1}% ☠️", top_contributor, top_author_share)
        ));
    }

    // 2. Ancient Temple
    let mut ancient_temple_score = 0;
    let mut ancient_temple_evidence = Vec::new();
    if age_years >= 5 {
        ancient_temple_score += 3;
        ancient_temple_evidence.push(bi(
            format!("Dự án đã {} tuổi", age_years),
            format!("{} years old", age_years)
        ));
    }
    if let Some(days) = days_since_last_commit {
        if days > 120 {
            ancient_temple_score += 2;
            ancient_temple_evidence.push(bi(
                format!("Commit cuối cùng từ {} ngày trước", days),
                format!("Last commit {} days ago", days)
            ));
        }
    }
    if release_tag_count >= 5 {
        ancient_temple_score += 1;
        ancient_temple_evidence.push(bi(
            format!("Có {} thẻ phát hành (release tags)", release_tag_count),
            format!("{} release tags", release_tag_count)
        ));
    }

    // 3. Corporate Fortress
    let mut corporate_fortress_score = 0;
    let mut corporate_fortress_evidence = Vec::new();
    if has_ci {
        corporate_fortress_score += 2;
        corporate_fortress_evidence.push(bi(
            "Có quy trình CI/CD",
            "CI/CD pipeline present"
        ));
    }
    if infra_count >= 3 {
        corporate_fortress_score += 2;
        corporate_fortress_evidence.push(bi(
            format!("Phát hiện {} dấu vết hạ tầng", infra_count),
            format!("{} infrastructure signals detected", infra_count)
        ));
    }
    if has_bot {
        corporate_fortress_score += 1;
        corporate_fortress_evidence.push(bi(
            "Đã cấu hình bot cập nhật thư viện",
            "Dependency bot configured"
        ));
    }
    if business_hours_pct >= 80.0 {
        corporate_fortress_score += 2;
        corporate_fortress_evidence.push(bi(
            format!("{:.1}% commit được thực hiện trong giờ làm việc (Thứ 2–Thứ 6)", business_hours_pct),
            format!("{:.1}% of commits during Mon–Fri business hours", business_hours_pct)
        ));
    }
    if contributor_count >= 10 {
        corporate_fortress_score += 1;
        corporate_fortress_evidence.push(bi(
            format!("Có {} người đóng góp", contributor_count),
            format!("{} contributors", contributor_count)
        ));
    }

    // 4. Open Source Kingdom
    let mut open_source_kingdom_score = 0;
    let mut open_source_kingdom_evidence = Vec::new();
    if contributor_count >= 30 {
        open_source_kingdom_score += 3;
        open_source_kingdom_evidence.push(bi(
            format!("Có {} người đóng góp", contributor_count),
            format!("{} contributors", contributor_count)
        ));
    }
    if top_author_share < 40.0 {
        open_source_kingdom_score += 2;
        open_source_kingdom_evidence.push(bi(
            format!("Không có nhà phát triển nào nắm giữ quá {:.1}% commit", top_author_share),
            format!("No single dev owns more than {:.1}% of commits", top_author_share)
        ));
    }
    if let Some(days) = days_since_last_commit {
        if days <= 30 {
            open_source_kingdom_score += 1;
            open_source_kingdom_evidence.push(bi(
                format!("Hoạt động tích cực trong vòng {} ngày qua", days),
                format!("Active within the last {} days", days)
            ));
        }
    }
    if has_ci {
        open_source_kingdom_score += 1;
        open_source_kingdom_evidence.push(bi(
            "Có quy trình CI/CD",
            "CI/CD pipeline present"
        ));
    }

    // 5. Startup Goblin
    let mut startup_goblin_score = 0;
    let mut startup_goblin_evidence = Vec::new();
    if age_years < 3 {
        startup_goblin_score += 2;
        startup_goblin_evidence.push(bi(
            format!("Dự án mới chỉ {} tuổi", age_years),
            format!("Only {} years old", age_years)
        ));
    }
    if commits_last_30d >= 50 {
        startup_goblin_score += 2;
        startup_goblin_evidence.push(bi(
            format!("Có {} commit trong 30 ngày qua", commits_last_30d),
            format!("{} commits in the last 30 days", commits_last_30d)
        ));
    }
    if release_tag_count == 0 {
        startup_goblin_score += 1;
        startup_goblin_evidence.push(bi(
            "Chưa có thẻ phát hành (release tag) nào",
            "0 release tags"
        ));
    }
    if (2..=8).contains(&contributor_count) {
        startup_goblin_score += 1;
        startup_goblin_evidence.push(bi(
            format!("Có {} người đóng góp", contributor_count),
            format!("{} contributors", contributor_count)
        ));
    }

    // 6. Sleep-Deprived Startup
    let mut sleep_deprived_startup_score = startup_goblin_score;
    let mut sleep_deprived_startup_evidence = startup_goblin_evidence.clone();
    let mut crunch_fired = false;
    if branch_count >= 25 {
        sleep_deprived_startup_score += 1;
        sleep_deprived_startup_evidence.push(bi(
            format!("Có {} nhánh đang hoạt động song song", branch_count),
            format!("{} branches in flight", branch_count)
        ));
        crunch_fired = true;
    }
    if night_pct >= 25.0 {
        sleep_deprived_startup_score += 3;
        sleep_deprived_startup_evidence.push(bi(
            format!("{:.1}% commit được thực hiện vào ban đêm (nửa đêm đến 5 giờ sáng)", night_pct),
            format!("{:.1}% of commits between midnight and 5am", night_pct)
        ));
        crunch_fired = true;
    }
    if !crunch_fired {
        sleep_deprived_startup_score = 0;
    }

    // 7. Chaotic Good
    let mut chaotic_good_score = 0;
    let mut chaotic_good_evidence = Vec::new();
    if let Some(days) = days_since_last_commit {
        if days <= 30 {
            chaotic_good_score += 1;
            chaotic_good_evidence.push(bi(
                "Vẫn đang hoạt động",
                "Still active"
            ));
        }
    }
    if stale_branch_count >= 15 {
        chaotic_good_score += 2;
        chaotic_good_evidence.push(bi(
            format!("Có {} nhánh bỏ hoang chưa được dọn dẹp", stale_branch_count),
            format!("{} stale branches abandoned in the wild", stale_branch_count)
        ));
    }
    if release_tag_count == 0 {
        chaotic_good_score += 1;
        chaotic_good_evidence.push(bi(
            "Phát hành với 0 thẻ tag",
            "Ships with 0 release tags"
        ));
    }
    if !has_ci {
        chaotic_good_score += 1;
        chaotic_good_evidence.push(bi(
            "Không có lưới an toàn CI/CD",
            "No CI/CD safety net"
        ));
    }

    let candidates = vec![
        (VibeLabel::SleepDeprivedStartup, sleep_deprived_startup_score, sleep_deprived_startup_evidence),
        (VibeLabel::SoloWizard, solo_wizard_score, solo_wizard_evidence),
        (VibeLabel::StartupGoblin, startup_goblin_score, startup_goblin_evidence),
        (VibeLabel::CorporateFortress, corporate_fortress_score, corporate_fortress_evidence),
        (VibeLabel::OpenSourceKingdom, open_source_kingdom_score, open_source_kingdom_evidence),
        (VibeLabel::AncientTemple, ancient_temple_score, ancient_temple_evidence),
        (VibeLabel::ChaoticGood, chaotic_good_score, chaotic_good_evidence),
    ];

    let mut winner_idx = 0;
    let mut max_score = candidates[0].1;
    for (i, candidate) in candidates.iter().enumerate().skip(1) {
        if candidate.1 > max_score {
            max_score = candidate.1;
            winner_idx = i;
        }
    }

    let (mut primary, mut primary_score, mut evidence) = candidates[winner_idx].clone();

    if primary_score < MIN_SCORE {
        primary = VibeLabel::ChaoticGood;
        primary_score = candidates[6].1;
        evidence = candidates[6].2.clone();
    }

    // Runner-up: 2nd highest score >= 2 excluding the primary label
    let mut runner_up = None;
    let mut max_runner_score = 0;
    for (label, score, _) in &candidates {
        if *label == primary {
            continue;
        }
        if *score >= 2 && *score > max_runner_score {
            max_runner_score = *score;
            runner_up = Some(*label);
        }
    }

    VibeResult {
        primary,
        primary_score,
        evidence,
        runner_up,
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
            bus_factor: 1,
            top_author_share_pct: 50.0,
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

    fn default_infra() -> InfraFootprints {
        InfraFootprints {
            docker: false,
            terraform: false,
            github_actions: false,
            gitlab_ci: false,
            circleci: false,
            jenkins: false,
            dependabot: false,
            renovate: false,
        }
    }

    #[test]
    fn test_vibe_solo_wizard() {
        let mut history = default_history();
        history.top_author_share_pct = 75.0;
        history.contributor_count = 3;
        history.bus_factor = 1;
        history.top_contributor_name = Some("Gandalf".to_string());

        let snapshot = make_test_snapshot(history, default_infra(), 1700000000);
        let sections = FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 10,
                default_branch: "main".to_string(),
                stars: None,
                forks: None,
                contributor_count: 3,
                last_activity_secs: Some(1700000000),
            },
            commit_crimes: crate::analyze::commit::CommitCrimes {
                total_commits: 10,
                commits_this_month: 0,
                top_contributor: Some("Gandalf".to_string()),
                bus_factor: 1,
                top_author_share_pct: 75.0,
            },
            branch_jungle: crate::analyze::branch::BranchJungle { total: 0, active: 0, stale: 0, oldest_branch: None },
            ancient_relics: crate::analyze::relics::AncientRelics { oldest_file: None, most_modified_file: None, oldest_contributor: None, longest_living_branch: None },
            language_soup: crate::analyze::language::LanguageSoup { languages: vec![] },
            infrastructure: crate::analyze::infra::InfrastructureFootprints { docker: false, terraform: false, github_actions: false, gitlab_ci: false, circleci: false, jenkins: false, dependabot: false, renovate: false },
        };

        let result = classify_vibes(&snapshot, &sections, 1700000000);
        assert_eq!(result.primary, VibeLabel::SoloWizard);
        assert_eq!(result.primary_score, 7);
        assert_eq!(result.evidence.len(), 3);
        
        // Assert bilingual formatting
        assert!(result.evidence[0].vi.contains("Gandalf"));
        assert!(result.evidence[0].en.contains("Gandalf"));
    }

    #[test]
    fn test_vibe_chaotic_good_fallback() {
        let mut history = default_history();
        history.repo_age_days = 1200;
        history.contributor_count = 9;
        history.bus_factor = 3;
        history.release_tag_count = 1;
        
        let snapshot = make_test_snapshot(history, default_infra(), 1700000000);
        let sections = FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 1200,
                default_branch: "main".to_string(),
                stars: None,
                forks: None,
                contributor_count: 9,
                last_activity_secs: Some(1700000000),
            },
            commit_crimes: crate::analyze::commit::CommitCrimes { total_commits: 10, commits_this_month: 0, top_contributor: None, bus_factor: 3, top_author_share_pct: 50.0 },
            branch_jungle: crate::analyze::branch::BranchJungle { total: 0, active: 0, stale: 0, oldest_branch: None },
            ancient_relics: crate::analyze::relics::AncientRelics { oldest_file: None, most_modified_file: None, oldest_contributor: None, longest_living_branch: None },
            language_soup: crate::analyze::language::LanguageSoup { languages: vec![] },
            infrastructure: crate::analyze::infra::InfrastructureFootprints { docker: false, terraform: false, github_actions: false, gitlab_ci: false, circleci: false, jenkins: false, dependabot: false, renovate: false },
        };

        let result = classify_vibes(&snapshot, &sections, 1700000000);
        assert_eq!(result.primary, VibeLabel::ChaoticGood);
    }

    #[test]
    fn test_vibe_ancient_temple() {
        let mut history = default_history();
        history.repo_age_days = 2000;
        history.release_tag_count = 5;
        history.contributor_count = 9;
        history.bus_factor = 3;
        
        let snapshot = make_test_snapshot(history, default_infra(), 1700000000);
        let sections = FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 2000,
                default_branch: "main".to_string(),
                stars: None,
                forks: None,
                contributor_count: 9,
                last_activity_secs: Some(1700000000),
            },
            commit_crimes: crate::analyze::commit::CommitCrimes { total_commits: 100, commits_this_month: 0, top_contributor: None, bus_factor: 3, top_author_share_pct: 30.0 },
            branch_jungle: crate::analyze::branch::BranchJungle { total: 0, active: 0, stale: 0, oldest_branch: None },
            ancient_relics: crate::analyze::relics::AncientRelics { oldest_file: None, most_modified_file: None, oldest_contributor: None, longest_living_branch: None },
            language_soup: crate::analyze::language::LanguageSoup { languages: vec![] },
            infrastructure: crate::analyze::infra::InfrastructureFootprints { docker: false, terraform: false, github_actions: false, gitlab_ci: false, circleci: false, jenkins: false, dependabot: false, renovate: false },
        };
        let result = classify_vibes(&snapshot, &sections, 1700000000);
        assert_eq!(result.primary, VibeLabel::AncientTemple);
        assert_eq!(result.primary_score, 4);
    }

    #[test]
    fn test_vibe_corporate_fortress() {
        let mut history = default_history();
        history.repo_age_days = 2000;
        history.contributor_count = 12;
        history.bus_factor = 4;
        history.business_hours_pct = 85.0;
        
        let mut infra = default_infra();
        infra.github_actions = true;
        infra.docker = true;
        infra.terraform = true;
        
        let snapshot = make_test_snapshot(history, infra, 1700000000);
        let sections = FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 2000,
                default_branch: "main".to_string(),
                stars: None,
                forks: None,
                contributor_count: 12,
                last_activity_secs: Some(1700000000),
            },
            commit_crimes: crate::analyze::commit::CommitCrimes { total_commits: 100, commits_this_month: 0, top_contributor: None, bus_factor: 4, top_author_share_pct: 20.0 },
            branch_jungle: crate::analyze::branch::BranchJungle { total: 0, active: 0, stale: 0, oldest_branch: None },
            ancient_relics: crate::analyze::relics::AncientRelics { oldest_file: None, most_modified_file: None, oldest_contributor: None, longest_living_branch: None },
            language_soup: crate::analyze::language::LanguageSoup { languages: vec![] },
            infrastructure: crate::analyze::infra::InfrastructureFootprints { docker: true, terraform: true, github_actions: true, gitlab_ci: false, circleci: false, jenkins: false, dependabot: false, renovate: false },
        };
        let result = classify_vibes(&snapshot, &sections, 1700000000);
        assert_eq!(result.primary, VibeLabel::CorporateFortress);
        assert_eq!(result.primary_score, 7);
    }

    #[test]
    fn test_vibe_open_source_kingdom() {
        let mut history = default_history();
        history.repo_age_days = 2000;
        history.contributor_count = 35;
        history.top_author_share_pct = 25.0;
        history.bus_factor = 10;
        
        let snapshot = make_test_snapshot(history, default_infra(), 1700000000);
        let sections = FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 2000,
                default_branch: "main".to_string(),
                stars: None,
                forks: None,
                contributor_count: 35,
                last_activity_secs: Some(1700000000),
            },
            commit_crimes: crate::analyze::commit::CommitCrimes { total_commits: 1000, commits_this_month: 10, top_contributor: None, bus_factor: 10, top_author_share_pct: 25.0 },
            branch_jungle: crate::analyze::branch::BranchJungle { total: 0, active: 0, stale: 0, oldest_branch: None },
            ancient_relics: crate::analyze::relics::AncientRelics { oldest_file: None, most_modified_file: None, oldest_contributor: None, longest_living_branch: None },
            language_soup: crate::analyze::language::LanguageSoup { languages: vec![] },
            infrastructure: crate::analyze::infra::InfrastructureFootprints { docker: false, terraform: false, github_actions: false, gitlab_ci: false, circleci: false, jenkins: false, dependabot: false, renovate: false },
        };
        let result = classify_vibes(&snapshot, &sections, 1700000000);
        assert_eq!(result.primary, VibeLabel::OpenSourceKingdom);
        assert_eq!(result.primary_score, 6);
    }

    #[test]
    fn test_vibe_startup_goblin() {
        let mut history = default_history();
        history.repo_age_days = 500;
        history.commits_this_month = 60;
        history.contributor_count = 6;
        history.bus_factor = 3;
        
        let snapshot = make_test_snapshot(history, default_infra(), 1700000000);
        let sections = FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 500,
                default_branch: "main".to_string(),
                stars: None,
                forks: None,
                contributor_count: 6,
                last_activity_secs: Some(1700000000),
            },
            commit_crimes: crate::analyze::commit::CommitCrimes { total_commits: 100, commits_this_month: 60, top_contributor: None, bus_factor: 3, top_author_share_pct: 30.0 },
            branch_jungle: crate::analyze::branch::BranchJungle { total: 0, active: 0, stale: 0, oldest_branch: None },
            ancient_relics: crate::analyze::relics::AncientRelics { oldest_file: None, most_modified_file: None, oldest_contributor: None, longest_living_branch: None },
            language_soup: crate::analyze::language::LanguageSoup { languages: vec![] },
            infrastructure: crate::analyze::infra::InfrastructureFootprints { docker: false, terraform: false, github_actions: false, gitlab_ci: false, circleci: false, jenkins: false, dependabot: false, renovate: false },
        };
        let result = classify_vibes(&snapshot, &sections, 1700000000);
        assert_eq!(result.primary, VibeLabel::StartupGoblin);
        assert_eq!(result.primary_score, 6);
    }

    #[test]
    fn test_vibe_sleep_deprived_startup() {
        let mut history = default_history();
        history.repo_age_days = 500;
        history.commits_this_month = 60;
        history.contributor_count = 6;
        history.bus_factor = 3;
        history.night_pct = 30.0;
        
        let snapshot = make_test_snapshot(history, default_infra(), 1700000000);
        let sections = FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 500,
                default_branch: "main".to_string(),
                stars: None,
                forks: None,
                contributor_count: 6,
                last_activity_secs: Some(1700000000),
            },
            commit_crimes: crate::analyze::commit::CommitCrimes { total_commits: 100, commits_this_month: 60, top_contributor: None, bus_factor: 3, top_author_share_pct: 30.0 },
            branch_jungle: crate::analyze::branch::BranchJungle { total: 0, active: 0, stale: 0, oldest_branch: None },
            ancient_relics: crate::analyze::relics::AncientRelics { oldest_file: None, most_modified_file: None, oldest_contributor: None, longest_living_branch: None },
            language_soup: crate::analyze::language::LanguageSoup { languages: vec![] },
            infrastructure: crate::analyze::infra::InfrastructureFootprints { docker: false, terraform: false, github_actions: false, gitlab_ci: false, circleci: false, jenkins: false, dependabot: false, renovate: false },
        };
        let result = classify_vibes(&snapshot, &sections, 1700000000);
        assert_eq!(result.primary, VibeLabel::SleepDeprivedStartup);
        assert_eq!(result.primary_score, 9);
    }

    #[test]
    fn test_vibe_chaotic_good_positive() {
        let mut history = default_history();
        history.repo_age_days = 2000;
        history.contributor_count = 9;
        history.bus_factor = 3;
        history.release_tag_count = 0;
        
        let snapshot = make_test_snapshot(history, default_infra(), 1700000000);
        let sections = FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 2000,
                default_branch: "main".to_string(),
                stars: None,
                forks: None,
                contributor_count: 9,
                last_activity_secs: Some(1700000000),
            },
            commit_crimes: crate::analyze::commit::CommitCrimes { total_commits: 100, commits_this_month: 0, top_contributor: None, bus_factor: 3, top_author_share_pct: 30.0 },
            branch_jungle: crate::analyze::branch::BranchJungle { total: 20, active: 0, stale: 20, oldest_branch: None },
            ancient_relics: crate::analyze::relics::AncientRelics { oldest_file: None, most_modified_file: None, oldest_contributor: None, longest_living_branch: None },
            language_soup: crate::analyze::language::LanguageSoup { languages: vec![] },
            infrastructure: crate::analyze::infra::InfrastructureFootprints { docker: false, terraform: false, github_actions: false, gitlab_ci: false, circleci: false, jenkins: false, dependabot: false, renovate: false },
        };
        let result = classify_vibes(&snapshot, &sections, 1700000000);
        assert_eq!(result.primary, VibeLabel::ChaoticGood);
        assert_eq!(result.primary_score, 5);
    }

    #[test]
    fn test_vibe_tie_break() {
        let mut history = default_history();
        history.repo_age_days = 1500; // age_years = 4 (not < 3)
        history.commits_this_month = 0;
        history.contributor_count = 1; // <= 5 (+2 SoloWizard)
        history.bus_factor = 1; // == 1 (+2 SoloWizard)
        history.release_tag_count = 0; // +1 StartupGoblin
        history.night_pct = 30.0; // +3 SleepDeprivedStartup
        
        let snapshot = make_test_snapshot(history, default_infra(), 1700000000);
        let sections = FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 1500,
                default_branch: "main".to_string(),
                stars: None,
                forks: None,
                contributor_count: 1,
                last_activity_secs: Some(1700000000),
            },
            commit_crimes: crate::analyze::commit::CommitCrimes { total_commits: 10, commits_this_month: 0, top_contributor: None, bus_factor: 1, top_author_share_pct: 50.0 },
            branch_jungle: crate::analyze::branch::BranchJungle { total: 0, active: 0, stale: 0, oldest_branch: None },
            ancient_relics: crate::analyze::relics::AncientRelics { oldest_file: None, most_modified_file: None, oldest_contributor: None, longest_living_branch: None },
            language_soup: crate::analyze::language::LanguageSoup { languages: vec![] },
            infrastructure: crate::analyze::infra::InfrastructureFootprints { docker: false, terraform: false, github_actions: false, gitlab_ci: false, circleci: false, jenkins: false, dependabot: false, renovate: false },
        };
        let result = classify_vibes(&snapshot, &sections, 1700000000);
        assert_eq!(result.primary, VibeLabel::SleepDeprivedStartup);
        assert_eq!(result.primary_score, 4);
    }

    #[test]
    fn test_vibe_runner_up() {
        let mut history = default_history();
        history.repo_age_days = 2000;
        history.contributor_count = 35;
        history.top_author_share_pct = 25.0;
        history.bus_factor = 10;
        history.release_tag_count = 5;
        
        let snapshot = make_test_snapshot(history, default_infra(), 1700000000);
        let sections = FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 2000,
                default_branch: "main".to_string(),
                stars: None,
                forks: None,
                contributor_count: 35,
                last_activity_secs: Some(1700000000),
            },
            commit_crimes: crate::analyze::commit::CommitCrimes { total_commits: 1000, commits_this_month: 10, top_contributor: None, bus_factor: 10, top_author_share_pct: 25.0 },
            branch_jungle: crate::analyze::branch::BranchJungle { total: 0, active: 0, stale: 0, oldest_branch: None },
            ancient_relics: crate::analyze::relics::AncientRelics { oldest_file: None, most_modified_file: None, oldest_contributor: None, longest_living_branch: None },
            language_soup: crate::analyze::language::LanguageSoup { languages: vec![] },
            infrastructure: crate::analyze::infra::InfrastructureFootprints { docker: false, terraform: false, github_actions: false, gitlab_ci: false, circleci: false, jenkins: false, dependabot: false, renovate: false },
        };
        let result = classify_vibes(&snapshot, &sections, 1700000000);
        assert_eq!(result.primary, VibeLabel::OpenSourceKingdom);
        assert_eq!(result.runner_up, Some(VibeLabel::AncientTemple));
    }

    #[test]
    fn test_vibe_empty_no_panic() {
        let snapshot = InvestigationSnapshot {
            repo: RepoRef { owner: "".to_string(), repo: "".to_string() },
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
                default_branch: "".to_string(),
                branches: vec![],
                last_activity_secs: 0,
            },
            filesystem: FilesystemFacts {
                languages: vec![],
                infra: default_infra(),
            },
        };
        let sections = FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 0,
                default_branch: "".to_string(),
                stars: None,
                forks: None,
                contributor_count: 0,
                last_activity_secs: None,
            },
            commit_crimes: crate::analyze::commit::CommitCrimes { total_commits: 0, commits_this_month: 0, top_contributor: None, bus_factor: 0, top_author_share_pct: 0.0 },
            branch_jungle: crate::analyze::branch::BranchJungle { total: 0, active: 0, stale: 0, oldest_branch: None },
            ancient_relics: crate::analyze::relics::AncientRelics { oldest_file: None, most_modified_file: None, oldest_contributor: None, longest_living_branch: None },
            language_soup: crate::analyze::language::LanguageSoup { languages: vec![] },
            infrastructure: crate::analyze::infra::InfrastructureFootprints { docker: false, terraform: false, github_actions: false, gitlab_ci: false, circleci: false, jenkins: false, dependabot: false, renovate: false },
        };
        let result = classify_vibes(&snapshot, &sections, 0);
        assert_eq!(result.primary, VibeLabel::ChaoticGood);
    }
}
