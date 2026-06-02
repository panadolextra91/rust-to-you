---
phase: 01-intake-guardrails
plan: 02
subsystem: app
tags: [rust]

requires:
  - phase: 01-intake-guardrails/01
    provides: IntakeError, RepoRef, parse_repo_ref, Args
provides:
  - InvestigationSession struct
  - Deterministic case_id generator (FNV-1a)
  - run(session) integration seam for Phase 2
  - Success path stub output to stdout
affects:
  - 02-walking-skeleton

tech-stack:
  added: []
  patterns: Deterministic Case ID generation and integration seam pattern

key-files:
  created:
    - src/app/mod.rs
    - src/app/session.rs
    - src/app/run.rs
  modified:
    - src/main.rs
    - src/cli/mod.rs

key-decisions:
  - "Triển khai thuật toán băm FNV-1a thủ công siêu nhanh, 100% nhất quán giữa các nền tảng, không có yếu tố ngẫu nhiên/thời gian"
  - "Chuẩn hóa prefix của case_id thông qua filter-then-truncate để triệt tiêu các ký tự dấu chấm, gạch ngang từ tên repo (ví dụ: docs.rs -> DOCS) trước khi giới hạn 4 ký tự"

patterns-established:
  - "Deterministic Case ID format: ^[A-Z0-9]{1,4}-[0-9A-F]{4}$"
  - "run(session) seam phục vụ cắm thu thập dữ liệu (Phase 2)"

requirements-completed: [INPT-01]

duration: 10min
completed: 2026-06-02
---

# Phase 1: Intake & Guardrails - Plan 02 Summary

**Built the investigation session contract, deterministic case_id generator, and the run integration seam.**

## Performance

- **Duration:** 10 min
- **Started:** 2026-06-02T03:50:35Z
- **Completed:** 2026-06-02T03:55:00Z
- **Tasks:** 2 completed
- **Files modified:** 5

## Accomplishments
- Định nghĩa thành công cấu trúc `InvestigationSession` đại diện cho toàn bộ ngữ cảnh một phiên làm việc độc lập.
- Thiết kế bộ sinh mã vụ án `case_id` đảm bảo tính nhất quán cao, tự động loại bỏ các ký tự đặc biệt của các repository hệ sinh thái Rust (như `docs.rs` -> `DOCS`).
- Tạo hàm `run(session)` đóng vai trò là seam liên kết giữa Intake (Phase 1) và Collection (Phase 2), in ra chính xác một dòng log chuẩn hóa.

## Task Commits

Mọi tác vụ được commit theo đúng tinh thần nguyên tử:
1. **Task 1: InvestigationSession + deterministic case_id generator** - `4a72d3f` (feat/test)
2. **Task 2: run(session) seam + wire main success branch to print the stub line** - `c72b11e` (feat)

## Files Created/Modified
- `src/app/mod.rs` - Re-export `InvestigationSession` và `run` để CLI sử dụng.
- `src/app/session.rs` - Cấu trúc `InvestigationSession` và bộ sinh `case_id` cùng unit test.
- `src/app/run.rs` - Hàm seam `run` in ra stub kết quả.
- `src/main.rs` - Tích hợp luồng chạy hoàn chỉnh: CLI -> parse -> Session -> run.
- `src/cli/mod.rs` - Sửa lỗi thiếu re-export `Args` phát hiện khi chạy kiểm thử.

## Decisions Made
- Không dùng thư viện `regex` trong bộ kiểm thử để tối thiểu hóa thời gian biên dịch, thay thế bằng các phương thức kiểm tra ký tự cơ bản của Rust.
- Sử dụng `std::time::SystemTime` thay vì phụ thuộc thư viện `chrono` để tránh cài đặt thêm thư viện không cần thiết ở Phase 1.

## Deviations from Plan

### Auto-fixed Issues

**1. [Re-export Bug] Thiếu re-export Args trong src/cli/mod.rs**
- **Found during:** Task 1 cargo test
- **Issue:** Gặp lỗi biên dịch `unresolved import cli::Args` tại `src/main.rs`.
- **Fix:** Thêm dòng `pub use args::Args;` vào `src/cli/mod.rs`.
- **Files modified:** src/cli/mod.rs
- **Verification:** Chạy `cargo test` thành công không còn lỗi import.
- **Committed in:** `4a72d3f`

**2. [Path Traversal Guard] tokio-rs/axum/../path lọt lưới validate**
- **Found during:** Task 1 unit test
- **Issue:** URL chứa `..` vẫn vượt qua kiểm tra do parser chỉ validate 2 segment đầu.
- **Fix:** Thêm quét toàn bộ các segment đường dẫn, phát hiện `..` hoặc `\` lập tức trả lỗi `MalformedRepoPath`.
- **Files modified:** src/cli/parse.rs
- **Verification:** Unit test `test_parse_repo_ref_reject` vượt qua hoàn hảo.
- **Committed in:** `5c18e3d`

---

**Total deviations:** 2 auto-fixed
**Impact on plan:** Khắc phục triệt để lỗi biên dịch và tăng cường bảo mật chống path-traversal. Không có phát sinh phạm vi (scope creep).

## Issues Encountered
- **None**

## Next Phase Readiness
- Toàn bộ Phase 1 đã hoàn thiện và được kiểm thử cẩn thận. Sẵn sàng tích hợp phần thu thập thông tin của Phase 2 tại điểm nối `src/app/run.rs`.

---
*Phase: 01-intake-guardrails*
*Completed: 2026-06-02*
