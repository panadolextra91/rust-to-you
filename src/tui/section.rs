use ratatui::text::Line;

pub struct Section {
    pub title: Line<'static>,
    pub body: Vec<Line<'static>>,
}

impl Section {
    pub fn into_lines(self) -> Vec<Line<'static>> {
        let mut out = Vec::with_capacity(self.body.len() + 2);
        out.push(self.title);
        out.extend(self.body);
        out.push(Line::default()); // spacer between sections
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_into_lines() {
        let title = Line::raw("Title");
        let body = vec![Line::raw("Body 1"), Line::raw("Body 2")];
        let section = Section { title, body };
        let lines = section.into_lines();
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], Line::raw("Title"));
        assert_eq!(lines[1], Line::raw("Body 1"));
        assert_eq!(lines[2], Line::raw("Body 2"));
        assert_eq!(lines[3], Line::default());
    }
}
