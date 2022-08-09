use crate::{error::Result, util, Span};
use std::{fmt::Display, fs, path::PathBuf};

pub const GUTTER_WIDTH: u16 = 5;

pub enum Row {
    Previous,
    Current,
    Next,
    Index(usize),
}

#[derive(Debug)]
pub struct FrameBuffer {
    text_buffer: Vec<Option<String>>,
    pub position: (u16, u16),
    pub viewable_rows: Span,
}

impl FrameBuffer {
    pub fn new(text_buffer: Vec<Option<String>>, viewable_rows: Span) -> Self {
        Self {
            text_buffer,
            position: (0, 0),
            viewable_rows,
        }
    }

    pub fn try_from_path(path: PathBuf, viewable_rows: Span) -> Result<Self> {
        let data = fs::read_to_string(path)?;
        let text_buffer = Self::text_buffer_from_str(&data);

        Ok(Self::new(text_buffer, viewable_rows))
    }

    fn text_buffer_from_str(data: &str) -> Vec<Option<String>> {
        data.split(util::newline())
            .map(|line| match line.len() {
                0 => None,
                _ => Some(line.to_owned()),
            })
            .collect()
    }

    pub fn get(&self, row: Row) -> Option<&String> {
        match row {
            Row::Previous => self._get(self.position.1 as usize - 1),
            Row::Current => self._get(self.position.1 as usize),
            Row::Next => self._get(self.position.1 as usize + 1),
            Row::Index(i) => self._get(i as usize),
        }
    }

    fn _get(&self, i: usize) -> Option<&String> {
        if let Some(line) = self.text_buffer.get(i) {
            match line {
                Some(line) => return Some(line),
                None => return None,
            }
        }

        None
    }
    pub fn get_mut(&mut self, row: Row) -> Option<&mut String> {
        match row {
            Row::Previous => self._get_mut(self.position.1 as usize - 1),
            Row::Current => self._get_mut(self.position.1 as usize),
            Row::Next => self._get_mut(self.position.1 as usize + 1),
            Row::Index(i) => self._get_mut(i as usize),
        }
    }

    fn _get_mut(&mut self, i: usize) -> Option<&mut String> {
        if let Some(line) = self.text_buffer.get_mut(i) {
            match line {
                Some(line) => return Some(line),
                None => return None,
            }
        }

        None
    }

    pub fn len(&self) -> usize {
        self.text_buffer.len()
    }

    pub fn line_len(&self, row: usize) -> usize {
        match self._get(row) {
            Some(line) => line.len(),
            None => 0,
        }
    }

    pub fn insert(&mut self, row: usize, data: &str) {
        let len = self.len();
        if row < len {
            self.text_buffer.insert(row, Some(data.to_owned()));
            return;
        }

        (len..row).for_each(|_| self.text_buffer.push(None));
        self.append(data);
    }

    pub fn append(&mut self, data: &str) {
        self.text_buffer.push(Some(data.to_owned()));
    }

    pub fn remove(&mut self, row: usize) -> Option<String> {
        self.text_buffer.remove(row)
    }

    pub fn remove_span(&mut self, span: Span) -> Vec<Option<String>> {
        if span.start <= span.end && span.end < self.len() {
            return self.text_buffer.drain(span).collect();
        }

        vec![]
    }

    pub fn line_insert(&mut self, row: usize, column: usize, character: char) {
        match self._get_mut(row) {
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
                let indent: String = (0..column).into_iter().map(|_| ' ').collect();

                self.insert(row, &format!("{indent}{character}"));
            }
        }
    }

    pub fn line_insert_str(&mut self, row: usize, column: usize, segment: &str) {
        match self._get_mut(row) {
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
        match self._get_mut(row) {
            Some(line) => line.push(character),
            None => self.insert(row, character.to_string().as_str()),
        }
    }

    pub fn line_append_str(&mut self, row: usize, segment: &str) {
        match self._get_mut(row) {
            Some(line) => line.push_str(segment),
            None => self.insert(row, segment),
        }
    }

    pub fn line_remove(&mut self, row: usize, column: usize) -> Option<char> {
        self._get_mut(row).map(|line| line.remove(column))
    }

    pub fn line_remove_span(&mut self, row: usize, mut span: Span) -> Option<String> {
        match self._get_mut(row) {
            Some(line) => {
                let len = line.len();

                if len == 0 || span.start >= len {
                    return None;
                }

                if span.end >= len {
                    span.end = len - 1;
                }

                Some(line.drain(span).collect())
            }
            None => None,
        }
    }

    pub fn format_viewable(&self) -> String {
        let view_span = &self.viewable_rows;
        let format_number = |i: usize| match i {
            i if i >= 1000 => i.to_string(),
            i if i >= 100 => format!(" {i}"),
            i if i >= 10 => format!("  {i}"),
            i => format!("   {i}"),
        };

        (view_span.start..=view_span.end)
            .zip(self.format_span(view_span))
            .into_iter()
            .map(|(i, line)| format!("{} {line}", format_number(i)))
            .collect()
    }

    fn format_span(&self, span: &Span) -> Vec<String> {
        (span.start..=span.end)
            .map(|i| match self._get(i) {
                Some(line) => format!("{line}{}", util::newline()),
                None => util::newline().to_owned(),
            })
            .collect()
    }
}

impl Display for FrameBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self
            .format_span(&Span {
                start: 0,
                end: self.len(),
            })
            .join(util::newline());

        let message = format!(
            r#"FrameBuffer: {{
  position:  ({}, {}),
  view_span: ({}, {}),
{}
}}"#,
            self.position.0,
            self.position.1,
            self.viewable_rows.start,
            self.viewable_rows.end,
            data,
        );

        write!(f, "{message}")
    }
}

#[test]
fn works() {
    let mut buffer = FrameBuffer::new(vec![], Span { start: 0, end: 5 });
    buffer.append("hello world");
    buffer.insert(3, "xiu");
    buffer.insert(3, "my name is");
    buffer.insert(6, ":)");

    println!("{}{}", buffer.format_viewable(), util::newline());
    println!("{buffer}");
}

#[test]
fn from_path() -> Result<()> {
    let mut buffer =
        FrameBuffer::try_from_path(PathBuf::from("config.ron"), Span { start: 0, end: 5 })?;
    buffer.append("hello world");

    println!("{}{}", buffer.format_viewable(), util::newline());
    println!("{buffer}");

    Ok(())
}
