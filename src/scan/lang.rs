use std::path::Path;
use tokei::{Config, Languages};

pub fn language_breakdown(root: &Path) -> Vec<(String, f64)> {
    let mut languages = Languages::new();
    let config = Config::default();
    
    languages.get_statistics(&[root], &[".git", "target"], &config);
    
    let total_code: usize = languages.values().map(|l| l.code).sum();
    
    if total_code == 0 {
        return Vec::new();
    }
    
    let mut breakdown = Vec::new();
    for (lang_type, lang_stats) in languages.iter() {
        if lang_stats.code > 0 {
            let pct = 100.0 * (lang_stats.code as f64) / (total_code as f64);
            breakdown.push((lang_type.to_string(), pct));
        }
    }
    
    breakdown.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    breakdown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentages() {
        let tmp = tempfile::Builder::new().prefix("lang-").tempdir().unwrap();
        
        std::fs::write(tmp.path().join("main.rs"), "fn main() {\n\n}\n").unwrap();
        std::fs::write(tmp.path().join("script.py"), "print('hello')\n").unwrap();
        
        let breakdown = language_breakdown(tmp.path());
        assert!(!breakdown.is_empty());
        let sum: f64 = breakdown.iter().map(|(_, p)| p).sum();
        assert!((sum - 100.0).abs() < 0.1);
        
        assert!(breakdown[0].1 > breakdown[1].1);
        
        let empty_tmp = tempfile::Builder::new().prefix("empty-").tempdir().unwrap();
        let empty_breakdown = language_breakdown(empty_tmp.path());
        assert!(empty_breakdown.is_empty());
    }
}
