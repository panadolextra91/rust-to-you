use crate::app::session::InvestigationSession;
use crate::snapshot::InvestigationSnapshot;
use crate::report::sections::FactualSections;
use crate::i18n::{bi, two_line, inline_label};
use crate::tui::section::Section;
use crate::tui::format::{ascii_bar, thousands, dash_or, relative_date};
use ratatui::text::{Line, Span};
use ratatui::style::{Style, Color, Modifier};

pub fn build_report_lines(
    session: &InvestigationSession,
    snapshot: &InvestigationSnapshot,
    sections: &FactualSections,
    now_secs: i64,
) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    // --- Header ---
    lines.push(Line::from(vec![
        Span::styled("🦀 rust-to-you", Style::new().fg(Color::LightRed).add_modifier(Modifier::BOLD))
    ]));
    
    let sub = two_line(&bi("Báo cáo Điều tra Repository", "Repository Investigation Report"));
    lines.push(Line::from(vec![
        Span::styled(format!("   {}", sub[0]), Style::new().add_modifier(Modifier::BOLD))
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("   {}", sub[1]), Style::new().add_modifier(Modifier::BOLD))
    ]));

    lines.push(Line::from(vec![
        Span::raw("Repository: "),
        Span::styled(format!("{}/{}", session.repo.owner, session.repo.repo), Style::new().fg(Color::LightBlue).add_modifier(Modifier::BOLD))
    ]));

    let datetime: chrono::DateTime<chrono::Utc> = session.started_at.into();
    let formatted_date = datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    lines.push(Line::from(vec![
        Span::raw("Điều tra ngày / Date: "),
        Span::styled(formatted_date, Style::new().fg(Color::LightGreen))
    ]));

    lines.push(Line::from(vec![
        Span::raw("Case ID: "),
        Span::styled(session.case_id.clone(), Style::new().fg(Color::LightYellow))
    ]));
    lines.push(Line::default());

    // --- Section 1: First Impressions ---
    let s1_title = "🌱 BÁO CÁO BAN ĐẦU";
    let s1_en = "🌱 FIRST IMPRESSIONS";
    let s1_narrative = bi(
        "Ferris ghé mắt nhìn qua dự án này lần đầu...",
        "Ferris took a quick glance at this project..."
    );
    let s1_narrative_lines = two_line(&s1_narrative);
    
    let fi = &sections.first_impressions;
    let s1_body = vec![
        Line::from(vec![Span::styled(format!("   {}", s1_en), Style::new().fg(Color::Green).add_modifier(Modifier::BOLD))]),
        Line::from(vec![Span::styled(format!("   {}", s1_narrative_lines[0]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::from(vec![Span::styled(format!("   {}", s1_narrative_lines[1]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::default(),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Tuổi repo", "Repo age")))),
            Span::styled(format!("{} ngày / days", fi.repo_age_days), Style::new().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Nhánh mặc định", "Default branch")))),
            Span::styled(fi.default_branch.clone(), Style::new().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Số sao", "Stars")))),
            Span::styled(dash_or(fi.stars, thousands), Style::new().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Số fork", "Forks")))),
            Span::styled(dash_or(fi.forks, thousands), Style::new().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Người đóng góp", "Contributors")))),
            Span::styled(thousands(fi.contributor_count as u64), Style::new().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Hoạt động gần nhất", "Last activity")))),
            Span::styled(dash_or(fi.last_activity_secs, |t| relative_date(now_secs, t)), Style::new().fg(Color::Cyan))
        ]),
    ];
    lines.extend(Section {
        title: Line::from(vec![Span::styled(s1_title, Style::new().fg(Color::Green).add_modifier(Modifier::BOLD))]),
        body: s1_body,
    }.into_lines());

    // --- Section 2: Commit Crimes ---
    let s2_title = "☠️ TỘI ÁC COMMIT";
    let s2_en = "☠️ COMMIT CRIMES";
    let s2_narrative = bi(
        "Ferris lục lại đống lịch sử commit của dự án và phát hiện vài điều...",
        "Ferris dug through the commit history and uncovered some secrets..."
    );
    let s2_narrative_lines = two_line(&s2_narrative);
    
    let cc = &sections.commit_crimes;
    let s2_body = vec![
        Line::from(vec![Span::styled(format!("   {}", s2_en), Style::new().fg(Color::Red).add_modifier(Modifier::BOLD))]),
        Line::from(vec![Span::styled(format!("   {}", s2_narrative_lines[0]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::from(vec![Span::styled(format!("   {}", s2_narrative_lines[1]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::default(),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Tổng commit", "Total commits")))),
            Span::styled(thousands(cc.total_commits as u64), Style::new().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Commit tháng này", "Commits this month")))),
            Span::styled(thousands(cc.commits_this_month as u64), Style::new().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Trùm commit", "Top contributor")))),
            Span::styled(dash_or(cc.top_contributor.clone(), |n| n), Style::new().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Hệ số xe buýt", "Bus factor")))),
            Span::styled(thousands(cc.bus_factor as u64), Style::new().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Tỷ lệ đóng góp lớn nhất", "Top author share")))),
            Span::styled(format!("{:.1}%", cc.top_author_share_pct), Style::new().fg(Color::Cyan))
        ]),
    ];
    lines.extend(Section {
        title: Line::from(vec![Span::styled(s2_title, Style::new().fg(Color::Red).add_modifier(Modifier::BOLD))]),
        body: s2_body,
    }.into_lines());

    // --- Section 3: Branch Jungle ---
    let s3_title = "🔥 RỪNG RẬM UM TÙM";
    let s3_en = "🔥 BRANCH JUNGLE";
    let s3_narrative = bi(
        "Ferris lạc vào mê cung các nhánh của repo này...",
        "Ferris got lost in the labyrinth of branches here..."
    );
    let s3_narrative_lines = two_line(&s3_narrative);
    
    let bj = &sections.branch_jungle;
    let s3_body = vec![
        Line::from(vec![Span::styled(format!("   {}", s3_en), Style::new().fg(Color::LightRed).add_modifier(Modifier::BOLD))]),
        Line::from(vec![Span::styled(format!("   {}", s3_narrative_lines[0]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::from(vec![Span::styled(format!("   {}", s3_narrative_lines[1]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::default(),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Tổng số nhánh", "Total branches")))),
            Span::styled(thousands(bj.total as u64), Style::new().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Nhánh hoạt động", "Active branches")))),
            Span::styled(thousands(bj.active as u64), Style::new().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Nhánh bỏ hoang", "Stale branches")))),
            Span::styled(thousands(bj.stale as u64), Style::new().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Nhánh cổ nhất", "Oldest branch")))),
            Span::styled(dash_or(bj.oldest_branch.clone(), |n| n), Style::new().fg(Color::Cyan))
        ]),
    ];
    lines.extend(Section {
        title: Line::from(vec![Span::styled(s3_title, Style::new().fg(Color::LightRed).add_modifier(Modifier::BOLD))]),
        body: s3_body,
    }.into_lines());

    // --- Section 4: Ancient Relics ---
    let s4_title = "🏺 CỔ VẬT VÔ GIÁ";
    let s4_en = "🏺 ANCIENT RELICS";
    let s4_narrative = bi(
        "Ferris đi tìm những mảnh khảo cổ cổ xưa nhất còn sót lại...",
        "Ferris went looking for the oldest archaeological remains left..."
    );
    let s4_narrative_lines = two_line(&s4_narrative);
    
    let ar = &sections.ancient_relics;
    let s4_body = vec![
        Line::from(vec![Span::styled(format!("   {}", s4_en), Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
        Line::from(vec![Span::styled(format!("   {}", s4_narrative_lines[0]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::from(vec![Span::styled(format!("   {}", s4_narrative_lines[1]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::default(),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("File cổ nhất", "Oldest file")))),
            Span::styled(dash_or(ar.oldest_file.clone(), |n| n), Style::new().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("File bị sửa nhiều nhất", "Most modified file")))),
            Span::styled(dash_or(ar.most_modified_file.clone(), |n| n), Style::new().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Người cũ nhất", "Oldest contributor")))),
            Span::styled(dash_or(ar.oldest_contributor.clone(), |n| n), Style::new().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Nhánh lâu đời nhất", "Longest living branch")))),
            Span::styled(dash_or(ar.longest_living_branch.clone(), |n| n), Style::new().fg(Color::Cyan))
        ]),
    ];
    lines.extend(Section {
        title: Line::from(vec![Span::styled(s4_title, Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
        body: s4_body,
    }.into_lines());

    // --- Section 5: Language Soup ---
    let s5_title = "🌿 SÚP NGÔN NGỮ";
    let s5_en = "🌿 LANGUAGE SOUP";
    let s5_narrative = bi(
        "Ferris phân tích các thành phần ngôn ngữ cấu tạo nên repo...",
        "Ferris analyzed the linguistic ingredients of this repo..."
    );
    let s5_narrative_lines = two_line(&s5_narrative);
    
    let ls = &sections.language_soup;
    let mut s5_body = vec![
        Line::from(vec![Span::styled(format!("   {}", s5_en), Style::new().fg(Color::LightBlue).add_modifier(Modifier::BOLD))]),
        Line::from(vec![Span::styled(format!("   {}", s5_narrative_lines[0]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::from(vec![Span::styled(format!("   {}", s5_narrative_lines[1]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::default(),
    ];
    if ls.languages.is_empty() {
        s5_body.push(Line::raw("  —"));
    } else {
        for lang in &ls.languages {
            let bar_str = ascii_bar(lang.pct / 100.0, 10);
            s5_body.push(Line::from(vec![
                Span::raw(format!("  {:>15} ", lang.name)),
                Span::styled(bar_str, Style::new().fg(Color::LightBlue)),
                Span::styled(format!(" {:.1}%", lang.pct), Style::new().fg(Color::Cyan))
            ]));
        }
    }
    lines.extend(Section {
        title: Line::from(vec![Span::styled(s5_title, Style::new().fg(Color::LightBlue).add_modifier(Modifier::BOLD))]),
        body: s5_body,
    }.into_lines());

    // --- Section 6: Infrastructure Footprints ---
    let s6_title = "⚙️ DẤU VẾT HẠ TẦNG";
    let s6_en = "⚙️ INFRASTRUCTURE FOOTPRINTS";
    let s6_narrative = bi(
        "Ferris quét qua các file cấu hình và phát hiện dấu vết hạ tầng...",
        "Ferris scanned config files and detected infrastructure footprints..."
    );
    let s6_narrative_lines = two_line(&s6_narrative);
    
    let infra = &sections.infrastructure;
    let tick = |val: bool| {
        if val {
            Span::styled(" ✓", Style::new().fg(Color::Green).add_modifier(Modifier::BOLD))
        } else {
            Span::styled(" ✗", Style::new().fg(Color::Red).add_modifier(Modifier::BOLD))
        }
    };
    let s6_body = vec![
        Line::from(vec![Span::styled(format!("   {}", s6_en), Style::new().fg(Color::Gray).add_modifier(Modifier::BOLD))]),
        Line::from(vec![Span::styled(format!("   {}", s6_narrative_lines[0]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::from(vec![Span::styled(format!("   {}", s6_narrative_lines[1]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::default(),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Docker", "Docker")))),
            tick(infra.docker)
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Terraform", "Terraform")))),
            tick(infra.terraform)
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("GitHub Actions", "GitHub Actions")))),
            tick(infra.github_actions)
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("GitLab CI", "GitLab CI")))),
            tick(infra.gitlab_ci)
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("CircleCI", "CircleCI")))),
            tick(infra.circleci)
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Jenkins", "Jenkins")))),
            tick(infra.jenkins)
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Dependabot", "Dependabot")))),
            tick(infra.dependabot)
        ]),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Renovate", "Renovate")))),
            tick(infra.renovate)
        ]),
    ];
    lines.extend(Section {
        title: Line::from(vec![Span::styled(s6_title, Style::new().fg(Color::Gray).add_modifier(Modifier::BOLD))]),
        body: s6_body,
    }.into_lines());

    // --- Section 7: Repository Vibes ---
    let vibe_res = crate::analyze::vibes::classify_vibes(snapshot, sections, now_secs);
    let s7_title = "🔮 KHÍ CHẤT REPOSITORY";
    let s7_en = "🔮 REPOSITORY VIBES";
    let s7_narrative = bi(
        "Ferris cảm nhận được năng lượng từ repo này...",
        "Ferris senses the vibes of this repo..."
    );
    let s7_narrative_lines = two_line(&s7_narrative);
    
    let mut s7_body = vec![
        Line::from(vec![Span::styled(format!("   {}", s7_en), Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD))]),
        Line::from(vec![Span::styled(format!("   {}", s7_narrative_lines[0]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::from(vec![Span::styled(format!("   {}", s7_narrative_lines[1]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::default(),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Khí chất chính", "Primary vibe")))),
            Span::styled(vibe_res.primary.display().vi, Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        ]),
        Line::default(),
        Line::from(vec![Span::styled(format!("  {}", inline_label(&bi("Bằng chứng", "Evidence"))), Style::new().add_modifier(Modifier::BOLD))]),
    ];
    
    for bullet in &vibe_res.evidence {
        s7_body.push(Line::from(vec![
            Span::raw("   • "),
            Span::raw(inline_label(bullet))
        ]));
    }
    
    lines.extend(Section {
        title: Line::from(vec![Span::styled(s7_title, Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD))]),
        body: s7_body,
    }.into_lines());

    // --- Section 8: Interesting Findings ---
    let findings_res = crate::analyze::findings::interesting_findings(snapshot, sections, &vibe_res);
    let s8_title = "🔎 PHÁT HIỆN THÚ VỊ";
    let s8_en = "🔎 INTERESTING FINDINGS";
    let s8_narrative = bi(
        "Ferris ghi chép lại các chi tiết đáng chú ý...",
        "Ferris notes down interesting observations..."
    );
    let s8_narrative_lines = two_line(&s8_narrative);

    let mut s8_body = vec![
        Line::from(vec![Span::styled(format!("   {}", s8_en), Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from(vec![Span::styled(format!("   {}", s8_narrative_lines[0]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::from(vec![Span::styled(format!("   {}", s8_narrative_lines[1]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::default(),
    ];

    for finding in &findings_res {
        s8_body.push(Line::from(vec![
            Span::raw("  • "),
            Span::raw(inline_label(&finding.text))
        ]));
    }

    lines.extend(Section {
        title: Line::from(vec![Span::styled(s8_title, Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        body: s8_body,
    }.into_lines());

    // --- Section 9: Crab Verdict ---
    let verdict_res = crate::analyze::verdict::crab_verdict(snapshot, sections, now_secs);
    let s9_title = "🦀 PHÁN QUYẾT CỦA FERRIS";
    let s9_en = "🦀 CRAB VERDICT";
    let s9_narrative = bi(
        "Ferris tổng hợp và đưa ra đánh giá cuối cùng...",
        "Ferris compiles and delivers the final verdict..."
    );
    let s9_narrative_lines = two_line(&s9_narrative);

    let mut s9_body = vec![
        Line::from(vec![Span::styled(format!("   {}", s9_en), Style::new().fg(Color::Red).add_modifier(Modifier::BOLD))]),
        Line::from(vec![Span::styled(format!("   {}", s9_narrative_lines[0]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::from(vec![Span::styled(format!("   {}", s9_narrative_lines[1]), Style::new().add_modifier(Modifier::ITALIC).fg(Color::DarkGray))]),
        Line::default(),
        Line::from(vec![
            Span::raw(format!("  {}: ", inline_label(&bi("Đánh giá chung", "Overall rating")))),
            Span::styled(verdict_res.overall_label.vi.clone(), Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::default(),
    ];

    if !verdict_res.strengths.is_empty() {
        s9_body.push(Line::from(vec![Span::styled(format!("  {}", inline_label(&bi("Điểm mạnh", "Strengths"))), Style::new().add_modifier(Modifier::BOLD).fg(Color::Green))]));
        for s in &verdict_res.strengths {
            s9_body.push(Line::from(vec![
                Span::styled("   ✓ ", Style::new().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(inline_label(s))
            ]));
        }
        s9_body.push(Line::default());
    }

    if !verdict_res.risks.is_empty() {
        s9_body.push(Line::from(vec![Span::styled(format!("  {}", inline_label(&bi("Rủi ro", "Risks"))), Style::new().add_modifier(Modifier::BOLD).fg(Color::Red))]));
        for r in &verdict_res.risks {
            s9_body.push(Line::from(vec![
                Span::styled("   ⚠ ", Style::new().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(inline_label(r))
            ]));
        }
    }

    lines.extend(Section {
        title: Line::from(vec![Span::styled(s9_title, Style::new().fg(Color::Red).add_modifier(Modifier::BOLD))]),
        body: s9_body,
    }.into_lines());

    lines
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
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;
    use ratatui::widgets::{Paragraph, Wrap};

    fn make_test_session() -> InvestigationSession {
        InvestigationSession {
            repo: RepoRef { owner: "owner".to_string(), repo: "repo".to_string() },
            case_id: "OWNER-1234".to_string(),
            started_at: std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1700000000),
            deep: false,
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

    #[test]
    fn test_report_renders_header_and_sections() {
        let backend = TestBackend::new(80, 100);
        let mut terminal = Terminal::new(backend).unwrap();
        let session = make_test_session();
        let snapshot = make_test_snapshot();
        let sections = make_test_sections();
        
        terminal.draw(|f| {
            let lines = build_report_lines(&session, &snapshot, &sections, 1700000000);
            let p = Paragraph::new(lines).wrap(Wrap { trim: false });
            f.render_widget(p, f.area());
        }).unwrap();
        
        let buffer = terminal.backend().buffer();
        
        // Assert header contents
        let mut found_subtitle = false;
        let mut found_case_id = false;
        let mut found_mình = false;

        for y in 0..buffer.area().height {
            let mut line_str = String::new();
            for x in 0..buffer.area().width {
                line_str.push_str(buffer[(x, y)].symbol());
            }
            println!("ROW {}: {:?}", y, line_str);
            if line_str.contains("Repository Investigation Report") {
                found_subtitle = true;
            }
            if line_str.contains("OWNER-1234") {
                found_case_id = true;
            }
            if line_str.contains("mình") {
                found_mình = true;
            }
        }
        
        assert!(found_subtitle, "Should find English subtitle");
        assert!(found_case_id, "Should find Case ID OWNER-1234");
        assert!(!found_mình, "Should not contain 'mình'");

        // Verify section titles are monotonically increasing in row index
        let titles = vec![
            "🌱 BÁO CÁO BAN ĐẦU",
            "☠️ TỘI ÁC COMMIT",
            "🔥 RỪNG RẬM UM TÙM",
            "🏺 CỔ VẬT VÔ GIÁ",
            "🌿 SÚP NGÔN NGỮ",
            "⚙️ DẤU VẾT HẠ TẦNG",
            "🔮 KHÍ CHẤT REPOSITORY",
            "🔎 PHÁT HIỆN THÚ VỊ",
            "🦀 PHÁN QUYẾT CỦA FERRIS",
        ];

        let mut current_idx = 0;
        let mut found_titles = 0;
        for title in titles {
            let mut title_row = None;
            for y in current_idx..buffer.area().height {
                let mut line_str = String::new();
                for x in 0..buffer.area().width {
                    line_str.push_str(buffer[(x, y)].symbol());
                }
                let normalized: String = line_str.split_whitespace().collect::<Vec<&str>>().join(" ");
                if normalized.contains(title) {
                    title_row = Some(y);
                    break;
                }
            }
            assert!(title_row.is_some(), "Title '{}' not found in expected order", title);
            current_idx = title_row.unwrap() + 1;
            found_titles += 1;
        }
        assert_eq!(found_titles, 9);

        // Confirm Section 7-9 English titles are present in the buffer
        let mut found_vibe = false;
        let mut found_findings = false;
        let mut found_verdict = false;
        for y in 0..buffer.area().height {
            let mut line_str = String::new();
            for x in 0..buffer.area().width {
                line_str.push_str(buffer[(x, y)].symbol());
            }
            if line_str.contains("REPOSITORY VIBES") {
                found_vibe = true;
            }
            if line_str.contains("INTERESTING FINDINGS") {
                found_findings = true;
            }
            if line_str.contains("CRAB VERDICT") {
                found_verdict = true;
            }
        }
        assert!(found_vibe, "Vibes title should be present");
        assert!(found_findings, "Findings title should be present");
        assert!(found_verdict, "Verdict title should be present");
    }
}
