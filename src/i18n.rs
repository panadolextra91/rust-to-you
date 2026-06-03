pub struct Bilingual {
    pub vi: String,
    pub en: String,
}

impl Bilingual {
    pub fn new(vi: impl Into<String>, en: impl Into<String>) -> Self {
        Self {
            vi: vi.into(),
            en: en.into(),
        }
    }
}

pub fn bi(vi: impl Into<String>, en: impl Into<String>) -> Bilingual {
    Bilingual::new(vi, en)
}

pub fn two_line(b: &Bilingual) -> [String; 2] {
    [b.vi.clone(), b.en.clone()]
}

pub fn inline_label(b: &Bilingual) -> String {
    format!("{} / {}", b.vi, b.en)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i18n() {
        let b = bi("vi", "en");
        let lines = two_line(&b);
        assert_eq!(lines[0], "vi");
        assert_eq!(lines[1], "en");
        
        let label = inline_label(&b);
        assert_eq!(label, "vi / en");
    }
}
