use crate::app::session::InvestigationSession;
use crate::snapshot::InvestigationSnapshot;
use crate::report::sections::FactualSections;
use crate::i18n::{bi, two_line, inline_label};
use crate::tui::format::{ascii_bar, thousands, dash_or, relative_date};
use std::io::Write;

pub fn render<W: Write>(
    w: &mut W,
    session: &InvestigationSession,
    snapshot: &InvestigationSnapshot,
    sections: &FactualSections,
    now_secs: i64,
) -> std::io::Result<()> {
    // Header
    writeln!(w, "🦀 rust-to-you")?;
    let sub = two_line(&bi("Báo cáo Điều tra Repository", "Repository Investigation Report"));
    writeln!(w, "   {}", sub[0])?;
    writeln!(w, "   {}", sub[1])?;
    writeln!(w, "Repository: {}/{}", session.repo.owner, session.repo.repo)?;
    
    let datetime: chrono::DateTime<chrono::Utc> = session.started_at.into();
    let formatted_date = datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    writeln!(w, "Điều tra ngày / Date: {}", formatted_date)?;
    writeln!(w, "Case ID: {}", session.case_id)?;
    writeln!(w)?;

    // Section 1: First Impressions
    let s1_narrative = bi(
        "Ferris ghé mắt nhìn qua dự án này lần đầu...",
        "Ferris took a quick glance at this project..."
    );
    let s1_narrative_lines = two_line(&s1_narrative);
    let fi = &sections.first_impressions;
    writeln!(w, "🌱 BÁO CÁO BAN ĐẦU")?;
    writeln!(w, "   🌱 FIRST IMPRESSIONS")?;
    writeln!(w, "   {}", s1_narrative_lines[0])?;
    writeln!(w, "   {}", s1_narrative_lines[1])?;
    writeln!(w)?;
    writeln!(w, "  {}: {} ngày / days", inline_label(&bi("Tuổi repo", "Repo age")), fi.repo_age_days)?;
    writeln!(w, "  {}: {}", inline_label(&bi("Nhánh mặc định", "Default branch")), fi.default_branch)?;
    writeln!(w, "  {}: {}", inline_label(&bi("Số sao", "Stars")), dash_or(fi.stars, thousands))?;
    writeln!(w, "  {}: {}", inline_label(&bi("Số fork", "Forks")), dash_or(fi.forks, thousands))?;
    writeln!(w, "  {}: {}", inline_label(&bi("Người đóng góp", "Contributors")), thousands(fi.contributor_count as u64))?;
    writeln!(w, "  {}: {}", inline_label(&bi("Hoạt động gần nhất", "Last activity")), dash_or(fi.last_activity_secs, |t| relative_date(now_secs, t)))?;
    writeln!(w)?;

    // Section 2: Commit Crimes
    let s2_narrative = bi(
        "Ferris lục lại đống lịch sử commit của dự án và phát hiện vài điều...",
        "Ferris dug through the commit history and uncovered some secrets..."
    );
    let s2_narrative_lines = two_line(&s2_narrative);
    let cc = &sections.commit_crimes;
    writeln!(w, "☠️ TỘI ÁC COMMIT")?;
    writeln!(w, "   ☠️ COMMIT CRIMES")?;
    writeln!(w, "   {}", s2_narrative_lines[0])?;
    writeln!(w, "   {}", s2_narrative_lines[1])?;
    writeln!(w)?;
    writeln!(w, "  {}: {}", inline_label(&bi("Tổng commit", "Total commits")), thousands(cc.total_commits as u64))?;
    writeln!(w, "  {}: {}", inline_label(&bi("Commit tháng này", "Commits this month")), thousands(cc.commits_this_month as u64))?;
    writeln!(w, "  {}: {}", inline_label(&bi("Trùm commit", "Top contributor")), dash_or(cc.top_contributor.clone(), |n| n))?;
    writeln!(w, "  {}: {}", inline_label(&bi("Hệ số xe buýt", "Bus factor")), thousands(cc.bus_factor as u64))?;
    writeln!(w, "  {}: {:.1}%", inline_label(&bi("Tỷ lệ đóng góp lớn nhất", "Top author share")), cc.top_author_share_pct)?;
    writeln!(w)?;

    // Section 3: Branch Jungle
    let s3_narrative = bi(
        "Ferris lạc vào mê cung các nhánh của repo này...",
        "Ferris got lost in the labyrinth of branches here..."
    );
    let s3_narrative_lines = two_line(&s3_narrative);
    let bj = &sections.branch_jungle;
    writeln!(w, "🔥 RỪNG RẬM UM TÙM")?;
    writeln!(w, "   🔥 BRANCH JUNGLE")?;
    writeln!(w, "   {}", s3_narrative_lines[0])?;
    writeln!(w, "   {}", s3_narrative_lines[1])?;
    writeln!(w)?;
    writeln!(w, "  {}: {}", inline_label(&bi("Tổng số nhánh", "Total branches")), thousands(bj.total as u64))?;
    writeln!(w, "  {}: {}", inline_label(&bi("Nhánh hoạt động", "Active branches")), thousands(bj.active as u64))?;
    writeln!(w, "  {}: {}", inline_label(&bi("Nhánh bỏ hoang", "Stale branches")), thousands(bj.stale as u64))?;
    writeln!(w, "  {}: {}", inline_label(&bi("Nhánh cổ nhất", "Oldest branch")), dash_or(bj.oldest_branch.clone(), |n| n))?;
    writeln!(w)?;

    // Section 4: Ancient Relics
    let s4_narrative = bi(
        "Ferris đi tìm những mảnh khảo cổ cổ xưa nhất còn sót lại...",
        "Ferris went looking for the oldest archaeological remains left..."
    );
    let s4_narrative_lines = two_line(&s4_narrative);
    let ar = &sections.ancient_relics;
    writeln!(w, "🏺 CỔ VẬT VÔ GIÁ")?;
    writeln!(w, "   🏺 ANCIENT RELICS")?;
    writeln!(w, "   {}", s4_narrative_lines[0])?;
    writeln!(w, "   {}", s4_narrative_lines[1])?;
    writeln!(w)?;
    writeln!(w, "  {}: {}", inline_label(&bi("File cổ nhất", "Oldest file")), dash_or(ar.oldest_file.clone(), |n| n))?;
    writeln!(w, "  {}: {}", inline_label(&bi("File bị sửa nhiều nhất", "Most modified file")), dash_or(ar.most_modified_file.clone(), |n| n))?;
    writeln!(w, "  {}: {}", inline_label(&bi("Người cũ nhất", "Oldest contributor")), dash_or(ar.oldest_contributor.clone(), |n| n))?;
    writeln!(w, "  {}: {}", inline_label(&bi("Nhánh lâu đời nhất", "Longest living branch")), dash_or(ar.longest_living_branch.clone(), |n| n))?;
    writeln!(w)?;

    // Section 5: Language Soup
    let s5_narrative = bi(
        "Ferris phân tích các thành phần ngôn ngữ cấu tạo nên repo...",
        "Ferris analyzed the linguistic ingredients of this repo..."
    );
    let s5_narrative_lines = two_line(&s5_narrative);
    let ls = &sections.language_soup;
    writeln!(w, "🌿 SÚP NGÔN NGỮ")?;
    writeln!(w, "   🌿 LANGUAGE SOUP")?;
    writeln!(w, "   {}", s5_narrative_lines[0])?;
    writeln!(w, "   {}", s5_narrative_lines[1])?;
    writeln!(w)?;
    if ls.languages.is_empty() {
        writeln!(w, "  —")?;
    } else {
        for lang in &ls.languages {
            writeln!(w, "  {:>15} {} {:.1}%", lang.name, ascii_bar(lang.pct / 100.0, 10), lang.pct)?;
        }
    }
    writeln!(w)?;

    // Section 6: Infrastructure Footprints
    let s6_narrative = bi(
        "Ferris quét qua các file cấu hình và phát hiện dấu vết hạ tầng...",
        "Ferris scanned config files and detected infrastructure footprints..."
    );
    let s6_narrative_lines = two_line(&s6_narrative);
    let infra = &sections.infrastructure;
    let format_bool = |val: bool| if val { "✓" } else { "✗" };
    writeln!(w, "⚙️ DẤU VẾT HẠ TẦNG")?;
    writeln!(w, "   ⚙️ INFRASTRUCTURE FOOTPRINTS")?;
    writeln!(w, "   {}", s6_narrative_lines[0])?;
    writeln!(w, "   {}", s6_narrative_lines[1])?;
    writeln!(w)?;
    writeln!(w, "  {}: {}", inline_label(&bi("Docker", "Docker")), format_bool(infra.docker))?;
    writeln!(w, "  {}: {}", inline_label(&bi("Terraform", "Terraform")), format_bool(infra.terraform))?;
    writeln!(w, "  {}: {}", inline_label(&bi("GitHub Actions", "GitHub Actions")), format_bool(infra.github_actions))?;
    writeln!(w, "  {}: {}", inline_label(&bi("GitLab CI", "GitLab CI")), format_bool(infra.gitlab_ci))?;
    writeln!(w, "  {}: {}", inline_label(&bi("CircleCI", "CircleCI")), format_bool(infra.circleci))?;
    writeln!(w, "  {}: {}", inline_label(&bi("Jenkins", "Jenkins")), format_bool(infra.jenkins))?;
    writeln!(w, "  {}: {}", inline_label(&bi("Dependabot", "Dependabot")), format_bool(infra.dependabot))?;
    writeln!(w, "  {}: {}", inline_label(&bi("Renovate", "Renovate")), format_bool(infra.renovate))?;
    writeln!(w)?;

    // Section 7: Repository Vibes
    let vibe_res = crate::analyze::vibes::classify_vibes(snapshot, sections, now_secs);
    let s7_narrative = bi(
        "Ferris cảm nhận được năng lượng từ repo này...",
        "Ferris senses the vibes of this repo..."
    );
    let s7_narrative_lines = two_line(&s7_narrative);
    writeln!(w, "🔮 KHÍ CHẤT REPOSITORY")?;
    writeln!(w, "   🔮 REPOSITORY VIBES")?;
    writeln!(w, "   {}", s7_narrative_lines[0])?;
    writeln!(w, "   {}", s7_narrative_lines[1])?;
    writeln!(w)?;
    writeln!(w, "  {}: {}", inline_label(&bi("Khí chất chính", "Primary vibe")), vibe_res.primary.display().vi)?;
    writeln!(w)?;
    writeln!(w, "  {}", inline_label(&bi("Bằng chứng", "Evidence")))?;
    for bullet in &vibe_res.evidence {
        writeln!(w, "   • {}", inline_label(bullet))?;
    }
    writeln!(w)?;

    // Section 8: Interesting Findings
    let findings_res = crate::analyze::findings::interesting_findings(snapshot, sections, &vibe_res);
    let s8_narrative = bi(
        "Ferris ghi chép lại các chi tiết đáng chú ý...",
        "Ferris notes down interesting observations..."
    );
    let s8_narrative_lines = two_line(&s8_narrative);
    writeln!(w, "🔎 PHÁT HIỆN THÚ VỊ")?;
    writeln!(w, "   🔎 INTERESTING FINDINGS")?;
    writeln!(w, "   {}", s8_narrative_lines[0])?;
    writeln!(w, "   {}", s8_narrative_lines[1])?;
    writeln!(w)?;
    for finding in &findings_res {
        writeln!(w, "  • {}", inline_label(&finding.text))?;
    }
    writeln!(w)?;

    // Section 9: Crab Verdict
    let verdict_res = crate::analyze::verdict::crab_verdict(snapshot, sections, now_secs);
    let s9_narrative = bi(
        "Ferris tổng hợp và đưa ra đánh giá cuối cùng...",
        "Ferris compiles and delivers the final verdict..."
    );
    let s9_narrative_lines = two_line(&s9_narrative);
    writeln!(w, "🦀 PHÁN QUYẾT CỦA FERRIS")?;
    writeln!(w, "   🦀 CRAB VERDICT")?;
    writeln!(w, "   {}", s9_narrative_lines[0])?;
    writeln!(w, "   {}", s9_narrative_lines[1])?;
    writeln!(w)?;
    writeln!(w, "  {}: {}", inline_label(&bi("Đánh giá chung", "Overall rating")), verdict_res.overall_label.vi.clone())?;
    writeln!(w)?;

    if !verdict_res.strengths.is_empty() {
        writeln!(w, "  {}", inline_label(&bi("Điểm mạnh", "Strengths")))?;
        for s in &verdict_res.strengths {
            writeln!(w, "   ✓ {}", inline_label(s))?;
        }
        writeln!(w)?;
    }

    if !verdict_res.risks.is_empty() {
        writeln!(w, "  {}", inline_label(&bi("Rủi ro", "Risks")))?;
        for r in &verdict_res.risks {
            writeln!(w, "   ⚠ {}", inline_label(r))?;
        }
        writeln!(w)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::RepoRef;
    use crate::analyze::commit::CommitCrimes;
    use crate::analyze::branch::BranchJungle;
    use crate::analyze::relics::AncientRelics;
    use crate::analyze::language::{LanguageSoup, LanguageEntry};
    use crate::analyze::infra::InfrastructureFootprints;

    fn make_test_session() -> InvestigationSession {
        InvestigationSession {
            repo: RepoRef { owner: "owner".to_string(), repo: "repo".to_string() },
            case_id: "OWNER-1234".to_string(),
            started_at: std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1700000000),
            deep: false,
        }
    }

    fn make_test_snapshot() -> InvestigationSnapshot {
        use crate::snapshot::{HistoryFacts, FilesystemFacts, InfraFootprints, BranchFacts, RepoMetaState};
        use crate::repo::history::CommitWindow;
        
        InvestigationSnapshot {
            repo: RepoRef { owner: "owner".to_string(), repo: "repo".to_string() },
            metadata: RepoMetaState::Unavailable,
            history: HistoryFacts {
                total_commits: 500,
                repo_age_days: 100,
                contributor_count: 5,
                bus_factor: 2,
                top_author_share_pct: 60.0,
                window: CommitWindow { scanned: 100, capped: false },
                most_modified_file: Some("main.rs".to_string()),
                night_pct: 10.0,
                weekend_pct: 15.0,
                business_hours_pct: 75.0,
                commits_this_month: 50,
                top_contributor_name: Some("Alice".to_string()),
                oldest_file: Some("lib.rs".to_string()),
                oldest_contributor: Some("Bob".to_string()),
                release_tag_count: 5,
            },
            branches: BranchFacts {
                default_branch: "main".to_string(),
                branches: vec![],
                last_activity_secs: 1699990000,
            },
            filesystem: FilesystemFacts {
                languages: vec![("Rust".to_string(), 90.0), ("Markdown".to_string(), 10.0)],
                infra: InfraFootprints {
                    docker: true,
                    terraform: false,
                    github_actions: true,
                    gitlab_ci: false,
                    circleci: false,
                    jenkins: false,
                    dependabot: true,
                    renovate: false,
                },
            },
        }
    }

    fn make_test_sections() -> FactualSections {
        FactualSections {
            first_impressions: crate::report::sections::FirstImpressions {
                repo_age_days: 100,
                default_branch: "main".to_string(),
                stars: Some(1500),
                forks: Some(250),
                contributor_count: 5,
                last_activity_secs: Some(1699990000),
            },
            commit_crimes: CommitCrimes {
                total_commits: 500,
                commits_this_month: 50,
                top_contributor: Some("Alice".to_string()),
                bus_factor: 2,
                top_author_share_pct: 60.0,
            },
            branch_jungle: BranchJungle {
                total: 4,
                active: 2,
                stale: 2,
                oldest_branch: Some("old-feature".to_string()),
            },
            ancient_relics: AncientRelics {
                oldest_file: Some("lib.rs".to_string()),
                most_modified_file: Some("main.rs".to_string()),
                oldest_contributor: Some("Bob".to_string()),
                longest_living_branch: Some("old-feature".to_string()),
            },
            language_soup: LanguageSoup {
                languages: vec![
                    LanguageEntry { name: "Rust".to_string(), pct: 90.0 },
                    LanguageEntry { name: "Markdown".to_string(), pct: 10.0 },
                ],
            },
            infrastructure: InfrastructureFootprints {
                docker: true,
                terraform: false,
                github_actions: true,
                gitlab_ci: false,
                circleci: false,
                jenkins: false,
                dependabot: true,
                renovate: false,
            },
        }
    }

    #[test]
    fn test_plain_render() {
        let session = make_test_session();
        let snapshot = make_test_snapshot();
        let sections = make_test_sections();
        let mut buf = Vec::new();
        
        render(&mut buf, &session, &snapshot, &sections, 1700000000).unwrap();
        
        let output = String::from_utf8(buf).unwrap();
        
        assert!(output.contains("Repository Investigation Report"));
        assert!(output.contains("OWNER-1234"));
        assert!(output.contains("Tuổi repo / Repo age"));
        assert!(output.contains("Total commits"));
        assert!(output.contains("BRANCH JUNGLE"));
        assert!(output.contains("ANCIENT RELICS"));
        assert!(output.contains("LANGUAGE SOUP"));
        assert!(output.contains("INFRASTRUCTURE FOOTPRINTS"));
        
        // Assert Section 7-9 titles
        assert!(output.contains("REPOSITORY VIBES"));
        assert!(output.contains("INTERESTING FINDINGS"));
        assert!(output.contains("CRAB VERDICT"));
        
        // Assert no ANSI escapes
        assert!(!output.contains("\x1b["));
        // Assert no "mình"
        assert!(!output.contains("mình"));
    }
}
