use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(str: &str) -> Self {
        let mut row = Self {
            string: str.to_string(),
            len: 0,
        };
        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let start = cmp::min(start, end);
        let end = cmp::min(end, self.string.len());

        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push_str(" ")
            } else {
                result.push_str(grapheme)
            }
        }

        result
    }

    pub fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count();
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }
}
