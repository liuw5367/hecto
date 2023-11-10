use std::cmp;

#[derive(Debug)]
pub struct Row {
    string: String,
}

impl From<&str> for Row {
    fn from(str: &str) -> Self {
        Self {
            string: str.to_string(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let start = cmp::min(start, end);
        let end = cmp::min(end, self.string.len());

        self.string.get(start..end).unwrap_or_default().to_string()
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }

    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }
}