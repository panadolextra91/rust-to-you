# Phase 4: Presentation Layer - Context

**Gathered:** 2026-06-03
**Status:** Ready for planning

<domain>
## Phase Boundary

Render the investigation as a **single scrollable vertical TUI report** (ratatui + crossterm):
header + case-file metadata + the report sections, read top-to-bottom like a case file. No tabs,
no multi-screen. Consumes Phase 3's `FactualSections` view models + the `InvestigationSession`
(case_id).

**In scope:** a reusable section-render abstraction, rendering the **6 factual sections**, the
header/case-file identity, vertical scrolling + keyboard handling, formatting (ASCII bars,
relative dates, missing→"—", thousands separators), expressive crab styling, terminal
resilience (narrow widths, non-TTY fallback).

**Explicitly NOT in this phase:** Sections 7-9 (Vibes/Findings/Verdict) logic AND rendering →
Phase 5 (they use the same section abstraction built here). `--json` export → v2.
</domain>

<decisions>
## Implementation Decisions

### Sections 7-9 Handling (resolves the 9-vs-6 tension)
- **D-01:** Phase 4 builds a **reusable section-render abstraction** (section header + body
  lines/widget) and renders the **6 factual sections fully**. Sections 7-9 are **NOT** rendered
  in Phase 4 — **no placeholders**. Phase 5 adds Vibes/Findings/Verdict (logic + render) via the
  **same abstraction**. PRES-01's "9 sections" completes across Phase 4 (framework + 6) and
  Phase 5 (7-9). The abstraction must make adding a section trivial: data + one render call.

### Scroll, Keys, Screen Mode
- **D-02:** Full-screen **alternate screen** (crossterm enter/leave alt-screen + raw mode);
  **restore the terminal on exit, including on panic**. One long report, vertical line-offset scroll.
- **D-03:** Keymap: `↓`/`↑` (line), `j`/`k` (line), `PgDn`/`PgUp` (page), `g`/`G` (top/bottom),
  `q`/`Esc` (quit). Mouse-wheel scroll if easy (nice-to-have).

### Formatting Conventions (deferred from Phase 3 D-06 → rendered HERE)
- **D-04:**
  - Language Soup → **ASCII progress bars** (e.g., `██████░░░░ 62%`).
  - Dates → **relative** ("3 days ago"); reasonable absolute fallback for very old.
  - Missing data (`Option::None`) → **"—"** (never fabricated 0/empty).
  - Large numbers → **thousands separators** (12,442).
  - All formatting lives in the renderer (analyzers/view-models stay pure per Phase 3 D-06).

### Visual Style & Resilience
- **D-05:** **Expressive + colorful.** Use color (section headers, ☠️/✓/✗ accents) AND
  **generous emoji** for personality: 🦀 throughout plus reaction/character emoji
  (😭, 😋, ☠️, 🔥, 👑, 🧙, 😵, …) matching the gossip tone. (User explicitly wants the
  🦀😭😋 expressiveness — lean into it, keep it readable.)
- **D-06:** Terminal resilience — **wrap** long lines on narrow terminals (target min ~40
  cols), handle resize. When **stdout is NOT a TTY** (piped/redirected) → print a **plain-text**
  version of the report instead of launching the TUI, so `rust-to-you <repo> | less` works. (The
  existing `src/app/run.rs` plain printer can evolve into this fallback.)

### Header / Case File
- **D-07:** Header renders the case-file identity: 🦀 rust-to-you, "Repository Investigation
  Report", repository `owner/repo`, investigation date, **Case ID** (`InvestigationSession.case_id`).
  Sections render in **fixed top-to-bottom order** (no tabs/multi-screen).

### Bilingual + Ferris Voice (cross-cutting — NEW)
- **D-08:** All user-facing text is narrated by **Ferris** (Rust crab mascot 🦀) and is
  **bilingual Vietnamese + English**. Specifics:
  - **Voice:** Ferris NEVER says "tôi/mình" — always third-person "Ferris" (both VI and EN).
    Example error: `🦀 Ferris chỉ hóng repo GitHub thôi — {host} chưa có trong danh sách khách mời`
    then `Ferris can only visit public GitHub repos`.
  - **Rendering (balanced):** errors, status messages, **section titles**, and narrative render
    as **two lines** (Vietnamese line, then English line). Dense **data-row labels** render
    bilingually **inline** (e.g. `Tổng commit / Total commits: 12,442`) to keep the report compact.
  - **Centralize:** build a small i18n/message helper (e.g. a `Bilingual { vi, en }` type +
    render helpers) in `src/tui/` (or a small `src/i18n.rs`) that both the TUI and the plain
    renderer use. Pure + unit-testable.
  - **Retrofit Phase 1-3:** `IntakeError` Display (src/error.rs) is ALREADY Ferris-voiced but
    **VI-only** → add the **English line** to each variant (bilingual, VI then EN). The degrade
    message in `src/app/collect.rs` likewise → Ferris + bilingual. Errors print VI line then EN
    line to stderr (`main.rs` already does `eprintln!("{}", e)`, so the Display impl should embed
    both lines). In-scope for Phase 4 (it owns presentation + the i18n helper).
  - **Strengthen the guard:** assert each `IntakeError` variant's Display contains BOTH a VI and
    an EN line (not just "no 'mình'", which is already trivially true).

### Claude's Discretion
- Exact per-section colors + emoji choices (within the expressive style) — implementer's taste, keep readable.
- Exact English copy for each string (keep Ferris's playful gossip tone in both languages).
- Whether the i18n helper lives in `src/tui/` or a top-level `src/i18n.rs`.
- Event-loop tick rate / input-handling details.
- TUI testing approach — prefer unit-testing the **pure format/layout helpers** (bars, relative
  dates, number formatting) + a `ratatui::TestBackend` buffer smoke test; the interactive loop is
  verified manually.
- Whether `run.rs`'s current plain printer becomes the non-TTY fallback or a fresh plain renderer.
- New module placement (`src/tui/` per ARCHITECTURE.md).
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope & requirements
- `.planning/ROADMAP.md` § "Phase 4: Presentation Layer" — goal, success criteria, plans 04-01..04-02
- `.planning/REQUIREMENTS.md` — PRES-01 (single scrollable report, header/identity/date/case-id + 9 sections in order), PRES-02 (cute readable, crab iconography, no tabs)

### Constraints & stack
- `.planning/PROJECT.md` — single-scroll vertical report, no tabs/multi-screen (locked constraint)
- `.planning/research/STACK.md` — `ratatui` 0.30 + `crossterm` 0.29 (current pairing)
- `.planning/research/PITFALLS.md` — Pitfall 4 (pretty-but-not-readable TUI): verify narrow widths early, separate styling from content
- `.planning/research/ARCHITECTURE.md` — `tui/` widgets + report view model; TUI stays dumb (renders prepared view models)

### Data to render (Phase 3 output)
- `src/report/sections.rs` — `FactualSections` (+ `FirstImpressions` and the 6 section structs) — the render input
- `src/app/session.rs` — `InvestigationSession.case_id` for the header
- `src/app/run.rs` — current plain printer (becomes/inspires the non-TTY fallback)
- The vision's Section 1-9 layout (PROJECT brief) — order, labels, crab section titles
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets (Phase 3, on `main`)
- `report::sections::FactualSections` bundles the 6 factual section structs (pure data) — the
  exact render input. `FirstImpressions` carries Option fields (stars/forks/last_activity) → render "—" on None.
- `InvestigationSession.case_id` (deterministic, e.g. `AXUM-7F42`) → header.
- `src/app/run.rs` currently prints raw snapshot fields to stdout — this is the seed for the
  non-TTY plain fallback; the TTY path replaces it with the ratatui TUI.

### Established Patterns
- Pure helpers + `#[cfg(test)] mod tests` (Phases 1-3) — apply to format helpers (bars, relative
  dates, number formatting) for deterministic unit tests; inject `now` for relative-date tests.

### Integration Points
- New `src/tui/` module renders `FactualSections` + session header.
- `run(&session)` chooses TUI (TTY) vs plain (non-TTY) and must build the `FactualSections`
  (call the Phase 3 analyzers/`build_factual_sections`) before rendering.
</code_context>

<specifics>
## Specific Ideas

- User wants the report to FEEL alive: 🦀 everywhere + reaction emoji (😭 😋 ☠️ 🔥 👑 🧙 😵)
  used to punctuate sections/findings — playful "gossip" energy, not a sterile dashboard.
- Case-file header aesthetic (matches the vision mock): title, repo, date, Case ID.
- Relative dates and ASCII bars are the two highest-visibility formatting touches — get them nice.
- Keep styling separate from content (Pitfall 4) so narrow-terminal wrapping doesn't break layout.
</specifics>

<deferred>
## Deferred Ideas

- **Sections 7-9 rendering** (Vibes/Findings/Verdict) → Phase 5 (with their logic, via this phase's section abstraction).
- **Mouse-wheel scroll** → optional; include only if cheap.
- **`--json` export** → v2 (the non-TTY plain-text path is a stepping stone, but structured JSON is separate).

None of the above is scope creep into Phase 4 — they are intentional boundaries.
</deferred>

---

*Phase: 4-Presentation Layer*
*Context gathered: 2026-06-03*
