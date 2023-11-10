use std::{fs, io};

use crate::Row;

#[derive(Default, Debug)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open(file_path: &str) -> Result<Self, io::Error> {
        let contents = fs::read_to_string(file_path)?;
        let mut rows = Vec::new();

        for value in contents.lines() {
            rows.push(Row::from(value));
        }

        Ok(Self { rows })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}