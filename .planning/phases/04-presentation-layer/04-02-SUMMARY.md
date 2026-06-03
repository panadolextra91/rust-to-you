# Phase 4, Plan 2 Summary: Presentation Layer - Wave 2

Con đã hoàn thành toàn bộ Wave 2 của Phase 4 rồi ạ! Báo cáo chi tiết cho mẹ iu nhé:

## Các công việc đã hoàn thành

1. **Xây dựng Interactive TUI trong `src/tui/app.rs`**:
   - Định nghĩa struct `TuiState` quản lý trạng thái scroll offset và trạng thái thoát (`quit`).
   - Hàm `max_scroll` tính toán giới hạn cuộn trang tối đa dựa trên số dòng hiển thị và chiều cao của khung nhìn (viewport height).
   - Hàm PURE `handle_key` điều phối phím cuộn trang theo đúng keymap yêu cầu (phím `↓/↑`, `j/k` để cuộn từng dòng, `PageDown/PageUp` để cuộn cả trang, `g/G` để nhảy về đầu/cuối báo cáo, và `q/Esc` để thoát TUI).
   - Thiết kế hàm `render_tui` sử dụng API an toàn `ratatui::run` của Ratatui v0.30 để thiết lập terminal raw mode, alternate screen và tự động khôi phục terminal kể cả khi ứng dụng bị panic. Báo cáo được render thành một Paragraph đơn lẻ hỗ trợ tự động wrap dòng trên màn hình hẹp và cuộn dọc.

2. **Phân nhánh thông minh (IsTerminal) trong `src/tui/mod.rs`**:
   - Hàm `render` sử dụng API chuẩn `std::io::stdout().is_terminal()` từ Rust 1.70.
   - Khi chạy trực tiếp trên TTY, ứng dụng sẽ khởi tạo màn hình TUI tương tác (`app::render_tui`).
   - Khi đầu ra được pipe hoặc redirect (ví dụ `| less` hoặc ghi ra file), ứng dụng sẽ tự động chuyển sang chế độ plain-text (`plain::render`), đảm bảo không ghi ký tự escape màu mè gây lỗi định dạng file.

3. **Tích hợp vào điều phối chính (`src/app/run.rs`)**:
   - Xóa bỏ hoàn toàn code in kết quả thô `println!` trước đây.
   - Thay thế bằng việc thu thập Snapshot -> chuyển đổi thành `FactualSections` -> chuyển giao nhiệm vụ vẽ cho `tui::render`.
   - Việt hóa và Anh hóa (Bilingual) thông điệp mở đầu:
     - VI: `🦀 Ferris bắt đầu điều tra {owner}/{repo} — Case {case_id}`
     - EN: `Ferris is opening the investigation for {owner}/{repo} — Case {case_id}`
     - Bản dịch dùng ngôi thứ ba (Ferris), tuyệt đối không dùng từ "mình".

4. **Kiểm thử**:
   - Viết các unit tests tự động cho `max_scroll` và `handle_key` trong `src/tui/app.rs`.
   - Đảm bảo toàn bộ 42/42 tests đều chạy thành công tốt đẹp.
   - Clippy hoàn toàn sạch sẽ (chỉ còn đúng 1 warning trong code cũ của Phase 1).

## Trạng thái Git
Các thay đổi tiếp tục được giữ ở trạng thái uncommitted trong thư mục làm việc của mẹ iu để mẹ review trước khi commit chính thức.
