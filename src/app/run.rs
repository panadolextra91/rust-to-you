use crate::app::session::InvestigationSession;

/// Đây là seam tích hợp của Phase 2 (walking skeleton) sẽ thực hiện thu thập thông tin.
/// Trong Phase 1, hàm này chỉ in ra một dòng stub báo mở cuộc điều tra và thoát 0.
pub fn run(session: &InvestigationSession) {
    println!(
        "🦀 Investigation opened: {}/{} — Case {}",
        session.repo.owner, session.repo.repo, session.case_id
    );
}
