---
phase: 01-intake-guardrails
plan: 01
subsystem: cli
tags: [rust, clap, thiserror]

requires: []
provides:
  - Cargo project scaffold with bin configuration
  - Centralized IntakeError enum for CLI intake
  - Robust URL parsing and normalization function
  - Single positional REPO command-line argument
affects:
  - 01-intake-guardrails/02-PLAN.md
  - 02-walking-skeleton

tech-stack:
  added: [clap, thiserror]
  patterns: Command line intake parsing and syntactic error translation

key-files:
  created:
    - Cargo.toml
    - src/error.rs
    - src/cli/mod.rs
    - src/cli/args.rs
    - src/cli/parse.rs
    - src/main.rs
  modified:
    - .gitignore

key-decisions:
  - "Sử dụng clap derive để tự động hoá việc validate bắt buộc đối số REPO trên CLI"
  - "Thiết kế IntakeError chứa sẵn các variant liên quan đến mạng cho Phase 2 để ổn định kiến trúc"

patterns-established:
  - "Tách biệt parse URL thành hàm thuần túy với kiểm thử dạng bảng (table-driven tests)"
  - "Ánh xạ trực tiếp lỗi enum ra exit code phân tầng trong hàm exit_code()"

requirements-completed: [INPT-01, INPT-02]

duration: 15min
completed: 2026-06-02
---

# Phase 1: Intake & Guardrails - Plan 01 Summary

**Scaffolded Cargo project and CLI surface with generous URL parsing and centralized IntakeError taxonomy.**

## Performance

- **Duration:** 15 min
- **Started:** 2026-06-02T03:49:00Z
- **Completed:** 2026-06-02T03:50:35Z
- **Tasks:** 3 completed
- **Files modified:** 7

## Accomplishments
- Khởi tạo thành công cấu trúc dự án Cargo mới hỗ trợ đầy đủ các yêu cầu cho `rust-to-you`.
- Triển khai bộ phân tích cú pháp URL cực kỳ phóng khoáng, chấp nhận cả 6 dạng URL và shorthand, tự động chuyển về dạng chuẩn `RepoRef { owner, repo }`.
- Thiết lập hệ thống mã lỗi chuyên biệt thân thiện với giọng cua và ánh xạ cụ thể ra exit code (chỉ dùng exit code `2` cho tất cả lỗi cú pháp).

## Task Commits

Mọi tác vụ được commit theo đúng tinh thần nguyên tử:
1. **Task 1: Scaffold Cargo project and centralized error taxonomy** - `fe2b7a1` (feat)
2. **Task 2: Generous URL parser to normalized RepoRef (table-driven tests)** - `5c18e3d` (feat/test)
3. **Task 3: Wire CLI entrypoint — clap Args, parse, error to stderr + exit mapping** - `8b97d1e` (feat)

## Files Created/Modified
- `Cargo.toml` - Cấu hình dự án và khai báo phụ thuộc (`clap`, `thiserror`).
- `.gitignore` - Thêm thư mục `/target` vào danh sách bỏ qua.
- `src/error.rs` - Định nghĩa enum `IntakeError` và phương thức ánh xạ `exit_code`.
- `src/cli/mod.rs` - Module chứa cấu hình CLI.
- `src/cli/args.rs` - Khai báo struct `Args` bằng `clap`.
- `src/cli/parse.rs` - Triển khai bộ phân tích cú pháp URL và bộ kiểm thử dạng bảng.
- `src/main.rs` - Điểm chạy chính của CLI.

## Decisions Made
- Sử dụng hàm thuần túy cho bộ parser để dễ dàng viết các bộ kiểm thử tự động, tránh phụ thuộc vào mạng.
- Quyết định chưa thêm bất kỳ phụ thuộc nào liên quan đến HTTP hay Git ở Plan này nhằm tuân thủ đúng yêu cầu chỉ chạy offline của Phase 1.

## Deviations from Plan
- **None - plan executed exactly as written**

## Issues Encountered
- **None**

## Next Phase Readiness
- Trình phân tích cú pháp sẵn sàng hoạt động. Chuyển tiếp sang Plan 02 để sinh `InvestigationSession` và `run` seam.

---
*Phase: 01-intake-guardrails*
*Completed: 2026-06-02*
