pub fn ascii_bar(fraction: f64, width: usize) -> String {
    let clamped = fraction.clamp(0.0, 1.0);
    let filled = (clamped * width as f64).round() as usize;
    let mut s = String::with_capacity(width);
    s.extend(std::iter::repeat_n('█', filled));
    s.extend(std::iter::repeat_n('░', width - filled));
    s
}

pub fn thousands(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let len = s.len();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(c);
    }
    result
}

pub fn dash_or<T>(opt: Option<T>, f: impl FnOnce(T) -> String) -> String {
    match opt {
        Some(val) => f(val),
        None => "—".to_string(),
    }
}

pub fn relative_date(now_secs: i64, then_secs: i64) -> String {
    let d = now_secs - then_secs;
    if d > 730 * 86400 {
        if let Some(dt) = chrono::DateTime::from_timestamp(then_secs, 0) {
            return dt.format("%b %Y").to_string();
        }
    }
    match d {
        x if x < 60 => "just now".to_string(),
        x if x < 3600 => format!("{} min ago", x / 60),
        x if x < 86400 => format!("{} hours ago", x / 3600),
        x if x < 30 * 86400 => format!("{} days ago", x / 86400),
        x if x < 365 * 86400 => format!("{} months ago", x / (30 * 86400)),
        _ => format!("{} years ago", d / (365 * 86400)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_bar() {
        assert_eq!(ascii_bar(0.62, 10), "██████░░░░");
        assert_eq!(ascii_bar(0.0, 10), "░░░░░░░░░░");
        assert_eq!(ascii_bar(1.0, 10), "██████████");
    }

    #[test]
    fn test_thousands() {
        assert_eq!(thousands(12442), "12,442");
        assert_eq!(thousands(0), "0");
        assert_eq!(thousands(999), "999");
    }

    #[test]
    fn test_dash_or() {
        assert_eq!(dash_or::<u64>(None, |n| n.to_string()), "—");
        assert_eq!(dash_or::<u64>(Some(123), |n| n.to_string()), "123");
    }

    #[test]
    fn test_relative_date() {
        let now = 1000000000;
        assert_eq!(relative_date(now, now - 30), "just now");
        assert_eq!(relative_date(now, now - 120), "2 min ago");
        assert_eq!(relative_date(now, now - 7200), "2 hours ago");
        assert_eq!(relative_date(now, now - 3 * 86400), "3 days ago");
        assert_eq!(relative_date(now, now - 45 * 86400), "1 months ago");
        assert_eq!(relative_date(now, now - 400 * 86400), "1 years ago");
        
        // 8 years ago test (1000000000 is Sep 2001, 8 years prior is 1993)
        let eight_years_ago = now - 8 * 365 * 86400;
        let formatted = relative_date(now, eight_years_ago);
        assert!(formatted.contains("1993"), "Should be absolute formatted year, got: {}", formatted);
        assert!(!formatted.contains("years ago"));
    }
}
