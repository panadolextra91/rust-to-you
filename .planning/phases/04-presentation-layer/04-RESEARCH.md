# Phase 4: Presentation Layer - Research

**Researched:** 2026-06-03
**Domain:** Rust TUI (ratatui 0.30 + crossterm 0.29) — single scrollable investigation report
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Phase 4 builds a **reusable section-render abstraction** (section header + body lines/widget) and renders the **6 factual sections fully**. Sections 7-9 are **NOT** rendered in Phase 4 — **no placeholders**. Phase 5 adds Vibes/Findings/Verdict via the same abstraction. The abstraction must make adding a section trivial: data + one render call.
- **D-02:** Full-screen **alternate screen** (enter/leave alt-screen + raw mode); **restore the terminal on exit, including on panic**. One long report, vertical line-offset scroll.
- **D-03:** Keymap: `↓`/`↑` (line), `j`/`k` (line), `PgDn`/`PgUp` (page), `g`/`G` (top/bottom), `q`/`Esc` (quit). Mouse-wheel scroll if easy (nice-to-have).
- **D-04:** Formatting lives in the renderer:
  - Language Soup → **ASCII progress bars** (`██████░░░░ 62%`).
  - Dates → **relative** ("3 days ago"); reasonable absolute fallback for very old.
  - Missing data (`Option::None`) → **"—"** (never fabricated 0/empty).
  - Large numbers → **thousands separators** (12,442).
  - Analyzers/view-models stay pure (Phase 3 D-06).
- **D-05:** **Expressive + colorful.** Color (section headers, ☠️/✓/✗ accents) AND generous emoji: 🦀 throughout plus reaction emoji (😭 😋 ☠️ 🔥 👑 🧙 😵) matching the gossip tone. Lean in, keep readable.
- **D-06:** Terminal resilience — **wrap** long lines on narrow terminals (target min ~40 cols), handle resize. **stdout NOT a TTY** (piped/redirected) → print a **plain-text** report instead of launching the TUI (`rust-to-you <repo> | less` works).
- **D-07:** Header renders case-file identity: 🦀 rust-to-you, "Repository Investigation Report", `owner/repo`, investigation date, **Case ID** (`InvestigationSession.case_id`). Sections render in **fixed top-to-bottom order** (no tabs/multi-screen).
- **NO tokio (sync).** Reuse Phase 3 `FactualSections`.

### Claude's Discretion
- Exact per-section colors + emoji choices (keep readable).
- Event-loop tick rate / input-handling details.
- TUI testing approach — prefer unit-testing pure format/layout helpers + a `TestBackend` buffer smoke test; interactive loop verified manually.
- Whether `run.rs`'s current plain printer becomes the non-TTY fallback or a fresh plain renderer.
- New module placement (`src/tui/` per ARCHITECTURE.md).

### Deferred Ideas (OUT OF SCOPE)
- Sections 7-9 rendering (Vibes/Findings/Verdict) → Phase 5 (via this phase's abstraction).
- Mouse-wheel scroll → optional, only if cheap.
- `--json` export → v2 (non-TTY plain path is a stepping stone, not structured JSON).
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PRES-01 | Single scrollable vertical TUI report with header, repository identity, investigation date, case ID, and the MVP sections in order | Terminal setup via `ratatui::run`/`init`+`restore`; `Paragraph::scroll((y,x))` with clamped offset; header built from `InvestigationSession` (`repo.owner`/`repo.repo`/`case_id`/`started_at`); 6 sections rendered in fixed order from `FactualSections`. (PRES-01's "9 sections" completes in Phase 5 per D-01.) |
| PRES-02 | Cute, readable styling in a terminal — crab iconography, no tabs/multi-screen | Styled `Span`/`Line`/`Text` with `Style`/`Color`; emoji in spans; `Paragraph::wrap(Wrap{trim})` for narrow terminals; single `Paragraph` = inherently no tabs. Unicode-width caveats documented for ASCII-bar alignment (Pitfall 4). |
</phase_requirements>

## Summary

This phase converts the already-built `report::sections::FactualSections` (pure Phase 3 data) into a single full-screen scrollable ratatui report and a parallel plain-text fallback for piped output. The data layer is done — every field the renderer needs already exists and is enumerated below ("Render Input Contract"). The work is purely presentational: terminal lifecycle, a scroll/event loop, a section-render abstraction, and a set of pure format helpers (ASCII bar, relative date, thousands separator, missing→"—").

The single most important version-correct finding: **ratatui 0.30 ships `ratatui::init()`, `ratatui::restore()`, and `ratatui::run()` as built-in convenience functions, and `init()` automatically installs a panic hook that restores the terminal before propagating the panic** [CITED: docs.rs/ratatui/0.30.0/ratatui/fn.init.html]. This directly satisfies D-02's "restore on panic, including panic" requirement with **zero hand-rolled panic-hook code** — use `ratatui::run(|terminal| { ... })`, which wraps `init` + `restore` and inherits the panic hook. This is a major simplification over the old `tui-rs`-era manual `enable_raw_mode` + `EnterAlternateScreen` + manual panic hook dance, which downstream agents must NOT copy from stale examples.

For the report itself: render the whole document as one `Text`/`Vec<Line>` inside a single `Paragraph` with `.scroll((offset, 0))` and `.wrap(Wrap { trim: false })`. A single `Paragraph` structurally guarantees "no tabs/multi-screen" (PRES-02). Scroll is a clamped `u16` line offset; max scroll is computed from rendered line count minus viewport height. No async, no tokio — `crossterm::event::read()` blocks the loop, which is correct for a read-only report viewer.

**Primary recommendation:** New `src/tui/` module. Use `ratatui::run` for the TTY path (free panic-safe restore). Branch in `run()` on `std::io::stdout().is_terminal()` — TTY → TUI, non-TTY → plain renderer. Build `FactualSections` once via `build_factual_sections(&snapshot, now_secs)`, then render it into a `Vec<Line>` via a `Section` abstraction (title line + body lines). Keep all formatting in pure, unit-tested helpers; smoke-test the full render with `TestBackend`.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Terminal lifecycle (raw mode, alt screen, panic restore) | TUI (`src/tui/`) | — | OS terminal control belongs in the presentation tier; ratatui owns it. |
| Event loop / keymap (D-03) | TUI | — | Input handling is presentation-only state (`scroll_offset`). |
| Scroll math (clamp, page, max) | TUI | — | Depends on viewport height (runtime terminal state). |
| Format helpers (bar, relative date, thousands, missing→—) | TUI (pure submodule) | — | D-04 explicitly locates formatting in the renderer; analyzers stay pure. |
| Section→`Line` conversion | TUI (section abstraction) | — | View-model→glyphs is rendering; D-01 abstraction lives here. |
| Non-TTY detection + plain output | App orchestrator (`run`) branch → TUI plain renderer | — | The TTY/non-TTY decision is an orchestration branch; both renderers live in `tui`. |
| Section *data* (`FactualSections`) | Analysis (Phase 3, done) | — | Already built; TUI must not recompute (Anti-Pattern: metrics in widgets). |
| Header identity (case_id, repo, date) | TUI | App session (data source) | `InvestigationSession` supplies fields; TUI formats them. |

## Render Input Contract (what the renderer consumes — already built)

> Verified by reading the actual source. The renderer must treat these as read-only inputs and must NOT recompute anything (ARCHITECTURE Anti-Pattern 1).

**Header source — `app::session::InvestigationSession`** [VERIFIED: src/app/session.rs]
- `repo: RepoRef { owner: String, repo: String }`
- `case_id: String` (e.g. `AXUM-7F42`)
- `started_at: std::time::SystemTime` → investigation date (format with chrono — already a dep)

**Body source — `report::sections::FactualSections`** (built via `build_factual_sections(&snapshot, now_secs) -> FactualSections`) [VERIFIED: src/report/sections.rs, src/analyze/*.rs]

| Section (order) | Struct | Fields (exact) | None→"—" candidates |
|---|---|---|---|
| 1. First Impressions | `FirstImpressions` | `repo_age_days: i64`, `default_branch: String`, `stars: Option<u64>`, `forks: Option<u64>`, `contributor_count: usize`, `last_activity_secs: Option<i64>` | `stars`, `forks`, `last_activity_secs` |
| 2. Commit Crimes | `CommitCrimes` | `total_commits: usize`, `commits_this_month: usize`, `top_contributor: Option<String>`, `bus_factor: usize`, `top_author_share_pct: f64` | `top_contributor` |
| 3. Branch Jungle | `BranchJungle` | `total: usize`, `active: usize`, `stale: usize`, `oldest_branch: Option<String>` | `oldest_branch` |
| 4. Ancient Relics | `AncientRelics` | `oldest_file: Option<String>`, `most_modified_file: Option<String>`, `oldest_contributor: Option<String>`, `longest_living_branch: Option<String>` | all four |
| 5. Language Soup | `LanguageSoup { languages: Vec<LanguageEntry> }`, `LanguageEntry { name: String, pct: f64 }` | per-language `name` + `pct` → ASCII bar | empty `languages` vec → "—" |
| 6. Infrastructure | `InfrastructureFootprints` | 8 `bool`s: `docker`, `terraform`, `github_actions`, `gitlab_ci`, `circleci`, `jenkins`, `dependabot`, `renovate` | (bools → ✓/✗, never "—") |

**Notes for the planner:**
- `last_activity_secs: Option<i64>` is a **unix-epoch seconds timestamp** (set from `branches.last_activity_secs` when `> 0`). The relative-date helper takes `now_secs` and this value. Inject `now` for deterministic tests (the codebase already passes `now_secs` into `build_factual_sections` — mirror that pattern).
- `top_author_share_pct` / `night_pct` etc. are `f64` percentages already computed — the renderer only formats (e.g. ASCII bar for share, or plain `%`). Do not recompute.
- `repo_age_days: i64` is days, not a timestamp — format as "X days" or "~Y years" directly.
- Section structs are **public but have no `Debug`/`Clone` derive** on most — the renderer only borrows them (`&FactualSections`), so no derive is needed. If a snapshot test wants to print, add nothing; assert on the rendered `Buffer` instead.

## Standard Stack

### Core (new deps to add)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `ratatui` | 0.30.x | Terminal report rendering, `Paragraph` scroll, `init`/`restore`/`run` | Current maintained Rust TUI standard; 0.30 adds the built-in panic-safe lifecycle helpers this phase needs [CITED: docs.rs/ratatui/0.30.0]. |
| `crossterm` | 0.29.x | Backend events (keys, resize, mouse), used by ratatui's `CrosstermBackend` | The default ratatui backend; provides `event::read`/`poll` for the sync loop [CITED: docs.rs/crossterm/0.29.0/crossterm/event]. |

### Supporting (already in Cargo.toml — reuse, do NOT re-add)
| Library | Version (in repo) | Purpose | When to Use |
|---------|---------|---------|-------------|
| `chrono` | 0.4 | Format `started_at` as the investigation date; relative-date math | Header date + relative "X days ago" formatting. Already a dep. |
| `thiserror` | 1.0 | `IntakeError` already covers app errors | TUI render errors can map into `IntakeError::CollectionFailed`-style or a new variant if needed. |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `ratatui::run` (closure wrapper) | Manual `init()` + `restore()` | `run` auto-restores even on early `?` return; manual requires explicit `restore()` in every exit path. Use `run` unless you need the terminal handle to outlive the closure (you don't). |
| `Paragraph::scroll` on one big `Text` | `List` widget / custom stateful widget / `Scrollbar` | For one long *static* vertical document, `Paragraph::scroll` is the simplest correct approach. `List` adds selection semantics you don't want; a `Scrollbar` is a *visual* add-on (nice-to-have), not the scroll mechanism. |
| Hand-rolled thousands separator | `num-format` / `thousands` crate | Hand-roll: ~10 lines, zero new dep, trivially testable. New crate not justified (D: prefer hand-rolling). |
| Hand-rolled relative date | `chrono-humanize` / `timeago` crate | Hand-roll from chrono (already a dep): a handful of branches (secs/mins/hours/days/years). Avoids a new dep and gives full control over the gossip tone. |

**Installation:**
```bash
cargo add ratatui@0.30
cargo add crossterm@0.29
```

**Version verification (run before locking the plan):**
```bash
cargo search ratatui          # confirm 0.30.x is latest
cargo search crossterm        # confirm 0.29.x pairs with ratatui 0.30
cargo add ratatui@0.30 --dry-run
cargo add crossterm@0.29 --dry-run
```
ratatui 0.30 depends on crossterm 0.29 internally — adding both at these majors avoids a duplicate-crossterm-version mismatch. [CITED: STACK.md version compatibility table; confirm with `cargo tree -i crossterm` after add.]

## Package Legitimacy Audit

| Package | Registry | Age | Downloads | Source Repo | slopcheck | Disposition |
|---------|----------|-----|-----------|-------------|-----------|-------------|
| `ratatui` | crates.io | ~3 yrs (fork of tui-rs, 2023+) | very high (de-facto std) | github.com/ratatui/ratatui | n/a (slopcheck unavailable — Rust) | Approved — canonical, named in STACK.md, official docs at docs.rs/ratatui |
| `crossterm` | crates.io | ~7 yrs | very high | github.com/crossterm-rs/crossterm | n/a | Approved — canonical, ratatui's default backend |

**Packages removed due to slopcheck [SLOP] verdict:** none
**Packages flagged as suspicious [SUS]:** none

*slopcheck targets npm/PyPI hallucination vectors and does not cover crates.io. Both packages are corroborated by official docs.rs documentation (HIGH-confidence source) and were named in the project's own STACK.md, so they are tagged `[VERIFIED: crates.io via docs.rs official documentation]` rather than `[ASSUMED]`. Still confirm exact patch versions with `cargo add --dry-run` at plan time.*

## Architecture Patterns

### System Architecture Diagram

```text
main.rs
  └─ app::run(&session)
        │  build snapshot (collect) ──► build_factual_sections(&snapshot, now_secs)
        │                                          │
        │                                          ▼
        │                              FactualSections (6 structs, pure)
        │                                          │
        ▼                                          │
   stdout().is_terminal()? ───── NO ──────────────┤
        │                                          ▼
       YES                            tui::render_plain(&session, &sections) ─► println!/write! to stdout
        │                                  (evolved from current run.rs printer; pipe-friendly)
        ▼
   tui::render_tui(&session, &sections)
        │
        ├─ ratatui::run(|terminal| {                 ◄── init() installs panic-restore hook
        │     let lines = build_report_lines(&session, &sections);   // Vec<Line<'static>>
        │     let mut state = TuiState { offset: 0 };
        │     loop {
        │        terminal.draw(|f| {
        │            let area = f.area();
        │            let max = max_scroll(lines.len(), area.height);
        │            state.offset = state.offset.min(max);
        │            let p = Paragraph::new(lines.clone())
        │                       .wrap(Wrap { trim: false })
        │                       .scroll((state.offset, 0));
        │            f.render_widget(p, area);
        │        })?;
        │        match event::read()? {              // blocking, sync
        │            Event::Key(k) if k.kind == Press => handle_key(k.code, &mut state, max),
        │            Event::Resize(_, _) => {}        // next draw recomputes
        │            _ => {}
        │        }
        │        if state.quit { break; }
        │     }
        │  })
        ▼
   restore() runs automatically on closure exit AND on panic
```

The diagram traces the primary use case (repo URL → report) end to end. File mapping is in the structure block below, not the diagram.

### Recommended Project Structure
```text
src/tui/
├── mod.rs        # pub fn render(&session, &sections) -> entry; TTY vs non-TTY branch lives in app::run or here
├── app.rs        # event loop, TuiState { offset, quit }, key handling, scroll math
├── report.rs     # build_report_lines(): header + Section abstraction → Vec<Line>
├── section.rs    # the reusable Section abstraction (D-01): title + body lines + one render call
├── format.rs     # PURE helpers: ascii_bar, relative_date(now,then), thousands, dash_or
└── plain.rs      # non-TTY plain-text renderer (D-06; evolves run.rs printer)
```
Register `pub mod tui;` in `src/lib.rs`. Have `app::run` call `tui::render(...)` instead of the inline `println!`s.

### Pattern 1: Panic-safe terminal lifecycle (D-02) — use `ratatui::run`
**What:** ratatui 0.30 owns raw-mode + alt-screen + panic-restore. Do not hand-roll.
**When to use:** Always, for the TTY path.
**Example:**
```rust
// Source: docs.rs/ratatui/0.30.0/ratatui/fn.run.html
// run() = init() (enables raw mode + alt screen + installs panic-restore hook) then restore() after closure.
ratatui::run(|terminal| -> std::io::Result<()> {
    let lines = build_report_lines(session, sections);
    let mut state = TuiState::default();
    loop {
        terminal.draw(|frame| draw(frame, &lines, &mut state))?;
        if !handle_event(&mut state)? { break; }   // false = quit
    }
    Ok(())
})?;
```
> Critical: `init()` doc says the panic hook "restores the terminal before panicking" and "Ensure this is called *after* any other panic hooks." Since this project installs no other panic hook, `ratatui::run` is sufficient with no extra code. [CITED: docs.rs/ratatui/0.30.0/ratatui/fn.init.html]

### Pattern 2: One-document scroll via `Paragraph::scroll`
**What:** Whole report = one `Vec<Line>` → one `Paragraph`; scroll is a clamped `u16` vertical offset.
**Example:**
```rust
// Source: docs.rs/ratatui/0.30.0/ratatui/widgets/struct.Paragraph.html
// scroll() tuple order is (y, x) — y first (documented as different from convention).
let para = Paragraph::new(lines.clone())          // Vec<Line> -> Text
    .wrap(Wrap { trim: false })                    // wrap on narrow terminals (D-06)
    .scroll((state.offset, 0));                     // (vertical, horizontal)
frame.render_widget(para, frame.area());
```
**Scroll math (no `Block` borders; full-area paragraph):**
```rust
// When wrap is ON, the true rendered height depends on width. Two options:
// (A) Approximate with logical line count (simple, slightly over-scrolls when lines wrap):
fn max_scroll(line_count: usize, viewport_h: u16) -> u16 {
    line_count.saturating_sub(viewport_h as usize) as u16
}
// (B) Exact with Paragraph::line_count(width) — but it is UNSTABLE (feature "unstable-rendered-line-info").
//     Recommendation: use (A). Always re-clamp offset = offset.min(max_scroll(..)) inside draw so a
//     resize that grows the viewport snaps the offset back into range. Over-scroll by a few wrapped
//     lines is harmless; the clamp prevents scrolling past the end.
```

### Pattern 3: Reusable Section abstraction (D-01)
**What:** A `Section` = a title line (styled, with emoji) + body `Vec<Line>`. Adding a section = build data lines + push. Sections 7-9 (Phase 5) reuse this verbatim.
**Example:**
```rust
pub struct Section { pub title: Line<'static>, pub body: Vec<Line<'static>> }

impl Section {
    pub fn into_lines(self) -> Vec<Line<'static>> {
        let mut out = Vec::with_capacity(self.body.len() + 2);
        out.push(self.title);
        out.extend(self.body);
        out.push(Line::default());          // spacer between sections
        out
    }
}

// build_report_lines collects header + each section.into_lines()
fn build_report_lines(s: &InvestigationSession, f: &FactualSections) -> Vec<Line<'static>> {
    let mut lines = header_lines(s);
    lines.extend(first_impressions_section(&f.first_impressions).into_lines());
    lines.extend(commit_crimes_section(&f.commit_crimes).into_lines());
    // ... branch_jungle, ancient_relics, language_soup, infrastructure
    lines
}
```

### Pattern 4: Styled content with emoji (D-05, PRES-02)
```rust
// Source: docs.rs/ratatui/0.30.0 Span/Line/Style/Color
use ratatui::style::{Style, Color, Modifier};
use ratatui::text::{Line, Span};

let title = Line::from(vec![
    Span::styled("🦀 COMMIT CRIMES ☠️", Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
]);
let row = Line::from(vec![
    Span::raw("  Total commits: "),
    Span::styled(thousands(total), Style::new().fg(Color::Cyan)),
]);
```
> Keep styling separate from content (Pitfall 4): build the *string* in a pure `format::` helper, then wrap it in a `Span::styled` at the `Line`-building site. Helpers return `String`; the section builder applies color. This keeps helpers terminal-free and unit-testable.

### Anti-Patterns to Avoid
- **Computing metrics in widgets** (ARCHITECTURE Anti-Pattern 1): the renderer borrows `FactualSections`; never recompute stars/bus-factor/etc. in `src/tui/`.
- **Manual `enable_raw_mode` + `EnterAlternateScreen` + hand-rolled panic hook** copied from `tui-rs`/old ratatui examples: superseded by `ratatui::run`/`init`. STACK.md explicitly warns "tui-rs era examples copied blindly … are stale."
- **Right-padding ASCII bars with spaces for alignment** while using double-width emoji in the same column: misaligns (see Width/Emoji caveats).
- **Tabs / multiple screens / multiple paragraphs stacked as "pages":** violates PRES-02. One `Paragraph`, one scroll.
- **Forgetting `KeyEventKind::Press` filter:** on Windows, `read()` yields both Press and Release; without the filter every key fires twice. [CITED: docs.rs/crossterm/0.29.0/crossterm/event]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Raw mode / alt screen / panic restore | Manual crossterm `execute!` + `std::panic::set_hook` | `ratatui::run` / `ratatui::init`+`restore` | 0.30 does it correctly incl. panic-before-restore ordering. |
| Text wrapping on narrow terminals | Custom word-wrap | `Paragraph::wrap(Wrap{ trim: false })` | Unicode-aware, integrates with scroll. |
| Terminal event reading | Custom stdin parsing | `crossterm::event::read()` / `poll()` | Handles key/resize/mouse decoding cross-platform. |
| Buffer diff / partial redraw | Manual cursor moves | `terminal.draw(\|f\| ...)` | ratatui double-buffers and diffs. |
| Rendered-buffer test harness | Capturing real terminal output | `ratatui::backend::TestBackend` + `assert_buffer_lines` | Deterministic, no TTY needed. |

**Key insight:** ratatui 0.30 absorbed the entire "terminal boilerplate" surface (init/restore/run + panic hook). The only things worth hand-rolling here are the tiny pure formatters (bar/date/thousands), because a dep for each would be heavier than the code and harder to tune to the gossip tone.

## Common Pitfalls

### Pitfall 1: TUI pretty but not readable (PITFALLS.md Pitfall 4)
**What goes wrong:** Looks cute in a screenshot, annoying in a real terminal — wrapped labels, clipped sections, scroll loses context.
**Why:** Optimizing decoration over scroll/spacing/small-window behavior.
**How to avoid:** One scrollable layout; verify ~40-col width early (resize the terminal during manual test); separate styling from content (helpers return `String`, color applied at `Span` site).
**Warning signs:** ASCII bars wrap to a second line; section titles truncate.

### Pitfall 2: Double-width emoji misaligns ASCII bars
**What goes wrong:** `🦀` and most reaction emoji render as **2 cells** wide; ratatui uses `unicode-width` to advance the cursor. If a bar/label column is positioned by counting `char`s rather than display width, emoji rows shift right and ASCII bars stop lining up.
**Why:** `String::len()` (bytes) and `.chars().count()` both differ from terminal display width for emoji.
**How to avoid:**
- Don't put emoji *inside* a column that must align with a bar. Put emoji in the title/label area, keep the `██████░░░░ 62%` bar on its own segment with leading spaces, and right-pad the *label* using display width (ratatui re-exports/uses `unicode-width`; or add a tiny `unicode-width` dep only if you need `UnicodeWidthStr::width`).
- For the language bars, build each row as: fixed-width left-padded language name (pad by display width) + space + bar + ` {pct}%`. Compute the bar from `pct` so widths are uniform regardless of name.
- Test bar alignment with `TestBackend` by asserting on specific `Buffer` rows.
**Warning signs:** Bars look ragged when a language name contains no emoji vs a header row that does.

### Pitfall 3: Keys fire twice (Windows) / quit feels sticky
**What goes wrong:** `event::read()` returns both `KeyEventKind::Press` and `Release`; unfiltered, `j` scrolls two lines per press on Windows.
**How to avoid:** `if let Event::Key(k) = ev { if k.kind == KeyEventKind::Press { ... } }`. [CITED: docs.rs/crossterm/0.29.0]

### Pitfall 4: Scroll offset out of range after resize / at bottom
**What goes wrong:** Growing the terminal leaves `offset` past the new max → blank screen.
**How to avoid:** Re-clamp `offset = offset.min(max_scroll(line_count, area.height))` **inside `draw`** every frame, where `area.height` is current. `g` sets offset 0; `G` sets offset = max_scroll.

### Pitfall 5: Non-TTY path tries to launch TUI under a pipe
**What goes wrong:** `rust-to-you owner/repo | less` garbles because alt-screen escape codes go into the pipe.
**How to avoid:** Branch on `std::io::stdout().is_terminal()` BEFORE any ratatui call. Non-TTY → `tui::plain::render(&session, &sections)` writing plain text. (PROJECT/D-06.)

## Code Examples

### Non-TTY detection + branch (D-06) — confirmed std API
```rust
// std::io::IsTerminal — stable since Rust 1.70. No external crate (atty is deprecated/unmaintained).
use std::io::IsTerminal;

pub fn render(session: &InvestigationSession, sections: &FactualSections) -> std::io::Result<()> {
    if std::io::stdout().is_terminal() {
        render_tui(session, sections)        // ratatui::run path
    } else {
        plain::render(session, sections)     // println!/write! plain text (pipe-friendly)
    }
}
```
> Branch point: inside `app::run` after `build_factual_sections`, replacing the current inline `println!` block in `src/app/run.rs`. The existing printer's field access pattern is the seed for `plain::render`. [VERIFIED: src/app/run.rs reads exactly these snapshot fields.]

### Keymap handler (D-03)
```rust
// Source: docs.rs/crossterm/0.29.0/crossterm/event KeyCode variants
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

const PAGE: u16 = 10; // or derive from viewport height in draw

// returns false when the user wants to quit
fn handle_event(state: &mut TuiState, max: u16) -> std::io::Result<bool> {
    if let Event::Key(k) = event::read()? {
        if k.kind != KeyEventKind::Press { return Ok(true); }
        match k.code {
            KeyCode::Char('q') | KeyCode::Esc                 => return Ok(false),
            KeyCode::Down | KeyCode::Char('j')               => state.offset = (state.offset + 1).min(max),
            KeyCode::Up   | KeyCode::Char('k')               => state.offset = state.offset.saturating_sub(1),
            KeyCode::PageDown                                => state.offset = (state.offset + PAGE).min(max),
            KeyCode::PageUp                                  => state.offset = state.offset.saturating_sub(PAGE),
            KeyCode::Char('g')                               => state.offset = 0,
            KeyCode::Char('G')                               => state.offset = max,
            _ => {}
        }
    }
    Ok(true)
}
// Mouse-wheel (nice-to-have, D-03): enable with EnableMouseCapture, then
// Event::Mouse(MouseEvent{ kind: MouseEventKind::ScrollDown, ..}) => offset+1. Skip if not cheap.
```

### Pure format helpers (unit-testable, no terminal)
```rust
// ascii_bar(0.62, 10) -> "██████░░░░"
pub fn ascii_bar(fraction: f64, width: usize) -> String {
    let filled = ((fraction.clamp(0.0, 1.0)) * width as f64).round() as usize;
    let mut s = String::new();
    s.extend(std::iter::repeat('█').take(filled));
    s.extend(std::iter::repeat('░').take(width - filled));
    s
}

// thousands(12442) -> "12,442"
pub fn thousands(n: u64) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut out = String::new();
    for (i, c) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i) % 3 == 0 { out.push(','); }
        out.push(*c as char);
    }
    out
}

// dash_or: Option -> "—" or formatted value (D-04 missing data rule)
pub fn dash_or<T>(opt: Option<T>, f: impl FnOnce(T) -> String) -> String {
    opt.map(f).unwrap_or_else(|| "—".to_string())
}

// relative_date(now_secs, then_secs) -> "3 days ago" / absolute fallback for very old
pub fn relative_date(now_secs: i64, then_secs: i64) -> String {
    let d = now_secs - then_secs;
    match d {
        x if x < 60        => "just now".into(),
        x if x < 3600      => format!("{} min ago", x / 60),
        x if x < 86_400    => format!("{} hours ago", x / 3600),
        x if x < 30*86_400 => format!("{} days ago", x / 86_400),
        x if x < 365*86_400=> format!("{} months ago", x / (30*86_400)),
        _                  => format!("{} years ago", d / (365*86_400)), // or chrono absolute date
    }
}
```
> `now` is injected → deterministic tests (mirror `build_factual_sections(.., now_secs)`).

### TestBackend smoke test
```rust
// Source: docs.rs/ratatui/0.30.0/ratatui/backend/struct.TestBackend.html
use ratatui::{Terminal, backend::TestBackend, widgets::{Paragraph, Wrap}};

#[test]
fn report_renders_header() {
    let backend = TestBackend::new(60, 20);
    let mut terminal = Terminal::new(backend).unwrap();
    let lines = build_report_lines(&test_session(), &test_sections());
    terminal.draw(|f| {
        let p = Paragraph::new(lines.clone()).wrap(Wrap { trim: false }).scroll((0, 0));
        f.render_widget(p, f.area());
    }).unwrap();
    let buf = terminal.backend().buffer();
    // assert a known cell/row, or use assert_buffer_lines with expected Lines
    assert!(buffer_contains(buf, "Repository Investigation Report"));
}
```
> `assert_buffer_lines(expected)` exists for full-row snapshot assertions; for a smoke test, a substring scan of the buffer's cells is sufficient and resilient to color attributes.

## Runtime State Inventory

> Phase 4 is greenfield rendering (new `src/tui/` module + a branch in `run`). No rename/refactor/migration. **Section omitted** — no stored data, live-service config, OS-registered state, secrets, or build artifacts carry forward-incompatible state. The only mutation to existing files is replacing the `println!` block in `src/app/run.rs` with a call into `tui::render` (pure code edit; the plain renderer preserves equivalent output for the non-TTY path).

## State of the Art

| Old Approach | Current Approach (0.30) | When Changed | Impact |
|--------------|-------------------------|--------------|--------|
| Manual `enable_raw_mode()` + `execute!(EnterAlternateScreen)` + custom `set_hook` panic restore (tui-rs / early ratatui) | `ratatui::init()` / `restore()` / `run()` with built-in panic-restore hook | ratatui 0.28→0.30 stabilized the convenience API | D-02 satisfied with ~0 boilerplate; ignore old examples. |
| `atty` crate for TTY detection | `std::io::IsTerminal` (`stdout().is_terminal()`) | stable since Rust 1.70 (2023) | No dep for D-06; `atty` is deprecated. |
| `Terminal::new(CrosstermBackend::new(stdout))` everywhere | `DefaultTerminal` via `init()` | 0.28+ | Less ceremony; `DefaultTerminal` = `Terminal<CrosstermBackend<Stdout>>`. |

**Deprecated/outdated:**
- `tui-rs` (unmaintained predecessor) — use `ratatui`.
- `atty` — use `std::io::IsTerminal`.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `last_activity_secs` is unix-epoch seconds (treated as such for relative_date). | Render Input Contract | If it were a different epoch/unit, relative dates would be wrong. Mitigation: verify against `repo/branches.rs` `last_activity_secs` derivation at plan/impl time. |
| A2 | `Paragraph::line_count` requires `unstable-rendered-line-info` feature; recommend avoiding it. | Pattern 2 | If stabilized in the patch picked, exact scroll math becomes available — strictly better, low risk. Confirm on docs.rs for the chosen patch. |
| A3 | Adding `ratatui@0.30` pulls `crossterm@0.29` as its backend (no version conflict). | Standard Stack | A mismatch would cause a duplicate-crossterm compile issue. Mitigation: `cargo tree -i crossterm` after add. |

**Note:** ratatui/crossterm versions, `init` panic-hook behavior, `Paragraph::scroll`/`wrap`, `run` signature, and `TestBackend` API are all `[CITED: docs.rs/{ratatui,crossterm}/{0.30.0,0.29.0}]` (HIGH). `IsTerminal` is stable-std `[VERIFIED: training + std stabilization 1.70]`.

## Open Questions

1. **Exact patch versions (ratatui 0.30.x / crossterm 0.29.x).**
   - What we know: 0.30 / 0.29 majors are correct and compatible.
   - What's unclear: latest patch at execution time.
   - Recommendation: `cargo add ratatui@0.30 crossterm@0.29` (caret) and run `cargo build` + `cargo tree -i crossterm` in the first task; let Cargo resolve patch.
2. **Should `plain::render` reuse `run.rs`'s exact strings (incl. Vietnamese labels) or be a fresh cute plain layout?**
   - What we know: D-06 says the existing printer "can evolve into this fallback"; Claude's discretion (CONTEXT) on reuse-vs-fresh.
   - Recommendation: build a fresh plain renderer that mirrors the TUI section order/labels (consistency between piped and TUI output), reusing `run.rs`'s field-access pattern. Decide in planning.
3. **Page size for PgUp/PgDn** — fixed (e.g. 10) vs viewport-relative (height-2).
   - Recommendation: viewport-relative computed in `draw`; simpler UX. Implementer's discretion (D-03 details are Claude's discretion).

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust toolchain / cargo | build+test | ✓ (project builds Phases 1-3) | edition 2021 | — |
| A TTY (for manual interactive verification) | manual scroll/key check | depends on tester's terminal | — | TestBackend covers automated render; interactive is manual-only by design |
| Network/GitHub (to produce real `FactualSections`) | end-to-end manual run | runtime-only | — | Unit/TestBackend tests use synthetic `FactualSections` (no network) — see test fixtures already in analyze/*.rs |

**Missing dependencies with no fallback:** none.
**Missing dependencies with fallback:** interactive TTY verification is manual; all deterministic checks run headless via `TestBackend` + pure-helper unit tests.

## Validation Architecture

> nyquist_validation = true (config.json) → section included.

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Built-in Rust test harness (`#[cfg(test)] mod tests` + `#[test]`) — same pattern as Phases 1-3 |
| Config file | none (Cargo built-in) |
| Quick run command | `cargo test --lib tui::` |
| Full suite command | `cargo test` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PRES-01 | Header contains repo identity, date, Case ID | unit (TestBackend buffer substring) | `cargo test --lib tui::report` | ❌ Wave 0 (`src/tui/report.rs`) |
| PRES-01 | All 6 sections appear in fixed order in the rendered buffer | unit (TestBackend, assert row order) | `cargo test --lib tui::report` | ❌ Wave 0 |
| PRES-01 | Scroll offset clamps to `max_scroll`; `G`→max, `g`→0 | unit (pure `max_scroll` + key handler over `TuiState`) | `cargo test --lib tui::app` | ❌ Wave 0 (`src/tui/app.rs`) |
| PRES-02 | ASCII bar correct fill for fraction/width | unit (pure) | `cargo test --lib tui::format` | ❌ Wave 0 (`src/tui/format.rs`) |
| PRES-02 | thousands separator (12442→"12,442"), dash_or(None→"—") | unit (pure) | `cargo test --lib tui::format` | ❌ Wave 0 |
| PRES-02 | relative_date with injected `now` (boundaries: 3 days, months, years) | unit (pure) | `cargo test --lib tui::format` | ❌ Wave 0 |
| D-06 | Non-TTY plain renderer emits expected section labels | unit (capture `Write` into a `Vec<u8>`) | `cargo test --lib tui::plain` | ❌ Wave 0 (`src/tui/plain.rs`) |
| PRES-02 | Bar alignment under emoji-bearing rows | unit (TestBackend, assert specific cells) | `cargo test --lib tui::report` | ❌ Wave 0 |
| PRES-01/02 | Interactive scroll feel, narrow-terminal resize, color in real terminal | **manual** | n/a (run `rust-to-you <repo>` and resize) | manual-only (justified: interactive input + real terminal rendering can't be asserted headlessly) |

### Sampling Rate
- **Per task commit:** `cargo test --lib tui::` (fast, headless — pure helpers + TestBackend)
- **Per wave merge:** `cargo test` (full suite, includes Phases 1-3 regressions)
- **Phase gate:** `cargo test` green + `cargo clippy` clean + one manual TTY run (scroll all keys, resize to ~40 cols, pipe to `less`) before `/gsd-verify-work`.

### Wave 0 Gaps
- [ ] `src/tui/format.rs` — pure helpers + tests (PRES-02 formatting)
- [ ] `src/tui/section.rs` — Section abstraction (D-01)
- [ ] `src/tui/report.rs` + tests — `build_report_lines` + TestBackend smoke tests (PRES-01)
- [ ] `src/tui/app.rs` + tests — `TuiState`, `max_scroll`, key handler (PRES-01 scroll)
- [ ] `src/tui/plain.rs` + tests — non-TTY renderer (D-06)
- [ ] `src/tui/mod.rs` — `render()` TTY/non-TTY branch; register `pub mod tui;` in `lib.rs`
- [ ] Dep add: `cargo add ratatui@0.30 crossterm@0.29`
- [ ] Wire `app::run` to call `tui::render` (replace inline `println!` block)

*Test fixtures: construct synthetic `FactualSections` directly (the analyze/*.rs test modules already show how to build the underlying structs) — no network, no real repo needed.*

## Security Domain

> security_enforcement = true, ASVS level 1. This phase is a read-only local terminal renderer of already-collected public data. No auth, no network calls, no persistence, no user-supplied data beyond the repo identity already validated in Phase 1.

### Applicable ASVS Categories
| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | No auth in V1 (public repos only). |
| V3 Session Management | no | No sessions (`InvestigationSession` is an in-process struct, not a security session). |
| V4 Access Control | no | No protected resources; read-only local render. |
| V5 Input Validation | partial | Repo identity already validated in Phase 1 (`parse_repo_ref`). Renderer treats all section data as display text; it must not interpret terminal control sequences embedded in data. |
| V6 Cryptography | no | No crypto. |
| V7 Error Handling/Logging | yes | Map render/IO errors into the existing `IntakeError` taxonomy; never panic past the restore hook (handled by `ratatui::init`). Do not log secrets — there are none. |

### Known Threat Patterns for ratatui TUI rendering
| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Terminal escape-sequence injection via repo-derived strings (branch/file/contributor names, repo description) rendered into the report | Tampering / spoofing of terminal | ratatui renders through its `Buffer`/`Cell` model and crossterm, which write cell content rather than passing raw bytes through — control chars in a `Span` are rendered as cells, not executed. **Do not bypass ratatui** by `print!`-ing raw repo strings to the alt screen. For the **plain (non-TTY) path**, repo strings go to a pipe — acceptable for `\| less`, but avoid emitting raw unsanitized escape bytes; the data fields here are names/paths/booleans (low risk). |
| Panic leaving terminal in raw/alt-screen (DoS-ish UX) | Denial/availability | `ratatui::init`/`run` panic-restore hook (D-02) — already covered. |
| Resource exhaustion from gigantic `Vec<Line>` on a huge repo | DoS | Section data is bounded (6 sections, language list is short, branch counts are integers — not per-branch lines). No unbounded growth in the render input. |

**security_block_on = high:** no high-severity items identified for this phase.

## Sources

### Primary (HIGH confidence)
- docs.rs/ratatui/0.30.0/ratatui/fn.init.html — `init()` enables raw mode + alt screen + **installs panic-restore hook**; ordering note.
- docs.rs/ratatui/0.30.0/ratatui/fn.run.html — `run<F,R>(f: FnOnce(&mut DefaultTerminal)->R)`; wraps init+restore.
- docs.rs/ratatui/0.30.0/ratatui/widgets/struct.Paragraph.html — `scroll((y,x))` (y-first), `wrap(Wrap{trim})`, `line_count(width)` (unstable feature).
- docs.rs/ratatui/0.30.0/ratatui/backend/struct.TestBackend.html — `new(w,h)`, `buffer()`, `assert_buffer_lines`.
- docs.rs/crossterm/0.29.0/crossterm/event — `read()`/`poll()`, `Event::{Key,Resize,Mouse}`, `KeyCode`, `KeyEventKind::Press` (Windows double-fire note), mouse capture.
- Codebase (read this session): `src/report/sections.rs`, `src/analyze/{commit,branch,relics,language,infra}.rs`, `src/app/{run,session,mod}.rs`, `src/error.rs`, `Cargo.toml`, `src/lib.rs`, `src/main.rs`.
- Project docs: `04-CONTEXT.md`, `REQUIREMENTS.md`, `STACK.md`, `PITFALLS.md`, `ARCHITECTURE.md`, `.planning/config.json`.

### Secondary (MEDIUM confidence)
- `std::io::IsTerminal` stable since Rust 1.70 (training knowledge, widely established).

### Tertiary (LOW confidence)
- None.

## Metadata

**Confidence breakdown:**
- Standard stack (ratatui 0.30/crossterm 0.29 APIs): HIGH — confirmed on docs.rs for the exact versions.
- Architecture / render input contract: HIGH — read directly from source; field names/types are exact.
- Pitfalls (emoji width, Windows key double-fire, scroll clamp): HIGH — documented behavior + well-known TUI issues.
- Patch-version pinning: MEDIUM — confirm with `cargo add --dry-run` at plan time.

**Research date:** 2026-06-03
**Valid until:** 2026-07-03 (ratatui moves; re-verify if a new minor lands before execution)
