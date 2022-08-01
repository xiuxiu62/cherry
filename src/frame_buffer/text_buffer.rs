use crate::{
    error::{Error, Result},
    Span,
};
use std::{fmt::Display, fs, path::PathBuf};

// A text buffer with integrated range protections
// NOTE: it might be better to have an unsafe TextBuffer
// and handle protections at a higher level, to improve performance
#[derive(Debug)]
pub struct TextBuffer(Vec<Option<String>>);

impl TextBuffer {
    pub fn new(inner: Vec<Option<String>>) -> Self {
        Self(inner)
    }

    pub fn get(&self, i: usize) -> Option<&String> {
        if let Some(line) = self.0.get(i) {
            match line {
                Some(line) => return Some(line),
                None => return None,
            }
        }

        None
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut String> {
        if let Some(line) = self.0.get_mut(i) {
            match line {
                Some(line) => return Some(line),
                None => return None,
            }
        }

        None
    }

    pub fn insert(&mut self, i: usize, line: &str) {
        let len = self.0.len();
        if i < len {
            self.0.insert(i, Some(line.to_owned()));
            return;
        }

        (len..i).for_each(|_| self.0.push(None));
        self.append(line);
    }

    pub fn append(&mut self, line: &str) {
        self.0.push(Some(line.to_owned()));
    }

    pub fn remove(&mut self, i: usize) -> Option<String> {
        self.0.remove(i)
    }

    pub fn line_insert(&mut self, row: usize, column: usize, character: char) {
        match self.get_mut(row) {
            Some(line) => {
                let len = line.len();
                if column < len {
                    line.insert(column, character);
                    return;
                }

                let indent: String = (len..column).map(|_| ' ').collect();
                line.push_str(&indent);
                line.push(character);
            }
            None => {
                let indent: String = (0..column)
                    .into_iter()
                    // .filter(|i| *i > 0)
                    .map(|_| ' ')
                    .collect();

                self.insert(row, &format!("{indent}{character}"));
            }
        }
    }

    pub fn line_insert_str(&mut self, row: usize, column: usize, segment: &str) {
        match self.get_mut(row) {
            Some(line) => {
                let len = line.len();
                if column < len {
                    line.insert_str(column, segment);
                    return;
                }

                let indent: String = (len..column).map(|_| ' ').collect();
                line.push_str(&indent);
                line.push_str(segment);
            }
            None => {
                let indent: String = (0..=column)
                    .into_iter()
                    .filter(|i| *i > 0)
                    .map(|_| ' ')
                    .collect();

                self.insert(row, &format!("{indent}{segment}"));
            }
        }
    }

    pub fn line_append(&mut self, row: usize, character: char) {
        match self.get_mut(row) {
            Some(line) => line.push(character),
            None => self.insert(row, character.to_string().as_str()),
        }
    }

    pub fn line_append_str(&mut self, row: usize, segment: &str) {
        match self.get_mut(row) {
            Some(line) => line.push_str(segment),
            None => self.insert(row, segment),
        }
    }

    pub fn line_remove(&mut self, row: usize, column: usize) -> Option<char> {
        self.get_mut(row).map(|line| line.remove(column))
    }

    pub fn line_remove_span(&mut self, row: usize, mut span: Span) -> Option<String> {
        match self.get_mut(row) {
            Some(line) => {
                let len = line.len();

                if len == 0 || span.start >= len {
                    return None;
                }

                if span.end >= len {
                    span.end = len - 1;
                }

                Some(span.into_iter().map(|i| line.remove(i)).collect())
            }
            None => None,
        }
    }

    pub fn format_span(&self, span: Span) -> String {
        span.into_iter()
            .map(|i| {
                format!(
                    "{}\r\n",
                    match self.get(i) {
                        Some(line) => line,
                        None => "",
                    }
                )
            })
            .collect()
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Display for TextBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message: String = self
            .0
            .iter()
            .enumerate()
            .map(|(i, line)| {
                format!(
                    "{i}: {}\r\n",
                    match line {
                        Some(data) => data,
                        None => "",
                    }
                )
            })
            .collect();

        write!(f, "{message}")
    }
}

impl From<&str> for TextBuffer {
    fn from(data: &str) -> Self {
        let inner: Vec<Option<String>> = data
            .split("\r\n")
            .map(|line| match line.len() {
                0 => None,
                _ => Some(line.to_owned()),
            })
            .collect();

        Self::new(inner)
    }
}

impl From<String> for TextBuffer {
    fn from(data: String) -> Self {
        Self::from(data.as_str())
    }
}

impl TryFrom<PathBuf> for TextBuffer {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self> {
        let data = fs::read_to_string(path)?;

        Ok(Self::from(data))
    }
}
