use crate::{
    error::{Error, Result},
    util, Span,
};
use std::{cell::RefCell, fmt::Display, fs, iter, path::PathBuf, rc::Rc};

pub const GUTTER_WIDTH: usize = 5;

#[derive(Debug, Clone, Copy)]
pub enum Line {
    Previous,
    Current,
    Next,
    Index(usize),
}

#[derive(Debug, Clone)]
pub struct FrameBuffer {
    text_buffer: Vec<String>,
    pub entry: Rc<RefCell<Option<PathBuf>>>, // Directory entry being edited
    pub position: Rc<RefCell<(/*column*/ usize, /*row*/ usize)>>,
    pub viewable_rows: Span,
}

impl FrameBuffer {
    pub fn new(text_buffer: Vec<String>, entry: Option<PathBuf>, viewable_rows: Span) -> Self {
        let entry = Rc::new(RefCell::new(entry));
        let position = Rc::new(RefCell::new((0, 0)));

        Self {
            text_buffer,
            entry,
            position,
            viewable_rows,
        }
    }

    pub fn try_from_path(path: PathBuf, viewable_rows: Span) -> Result<Self> {
        let data = fs::read_to_string(path.clone())?;
        let text_buffer = Self::text_buffer_from_str(&data);

        Ok(Self::new(text_buffer, Some(path), viewable_rows))
    }

    fn text_buffer_from_str(data: &str) -> Vec<String> {
        data.split(util::newline())
            .map(|line| match line.len() {
                0 => "".to_owned(),
                _ => line.to_owned(),
            })
            .collect()
    }

    pub fn save(&self, path: PathBuf) -> Result<()> {
        fs::write(path, self.format()).map_err(Error::from)
    }

    pub fn get(&self, line: Line) -> Option<&String> {
        match line {
            Line::Previous => self.text_buffer.get(self.position.borrow().1 - 1),
            Line::Current => self.text_buffer.get(self.position.borrow().1),
            Line::Next => self.text_buffer.get(self.position.borrow().1 + 1),
            Line::Index(i) => self.text_buffer.get(i),
        }
    }

    pub fn get_mut(&mut self, line: Line) -> Option<&mut String> {
        match line {
            Line::Previous => self.text_buffer.get_mut(self.position.borrow().1 - 1),
            Line::Current => self.text_buffer.get_mut(self.position.borrow().1),
            Line::Next => self.text_buffer.get_mut(self.position.borrow().1 + 1),
            Line::Index(i) => self.text_buffer.get_mut(i),
        }
    }

    pub fn get_row(&self, line: Line) -> usize {
        match line {
            Line::Current => self.position.borrow().1,
            Line::Next => self.position.borrow().1 + 1,
            Line::Previous => self.position.borrow().1 - 1,
            Line::Index(i) => i,
        }
    }

    pub fn len(&self) -> usize {
        self.text_buffer.len()
    }

    pub fn line_len(&self, line: Line) -> usize {
        match self.get(line) {
            Some(line) => line.len(),
            None => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.text_buffer.is_empty()
    }

    pub fn line_is_empty(&self, line: Line) -> bool {
        match self.get(line) {
            Some(line) => line.is_empty(),
            None => true,
        }
    }

    pub fn insert(&mut self, line: Line, data: &str) {
        let buffer_len = self.len();
        let row = self.get_row(line);
        if row < buffer_len {
            self.text_buffer.insert(row, data.to_owned());

            return;
        }

        if row > buffer_len {
            (buffer_len..row).for_each(|_| self.text_buffer.push("".to_owned()));
        }

        self.text_buffer.push(data.to_owned());
    }

    pub fn append(&mut self, data: &str) {
        self.text_buffer.push(data.to_owned());
    }

    pub fn remove(&mut self, line: Line) -> Option<String> {
        let row = self.get_row(line);
        if row < self.len() {
            return Some(self.text_buffer.remove(row));
        }

        None
    }

    pub fn remove_span(&mut self, span: Span) -> Vec<String> {
        if span.start <= span.end && span.end < self.len() {
            return self.text_buffer.drain(span).collect();
        }

        vec![]
    }

    pub fn line_insert(&mut self, line: Line, column: usize, character: char) {
        match self.get_mut(line) {
            Some(data) => {
                let line_len = data.len();
                if column <= line_len {
                    data.insert(column, character);
                    return;
                }

                let indent: String = iter::repeat(' ').take(column - line_len).collect();
                data.push_str(&format!("{indent}{character}"));
            }
            None => {
                let indent: String = iter::repeat(' ').take(column).collect();
                self.insert(line, &format!("{indent}{character}"));
            }
        }
    }

    pub fn line_insert_str(&mut self, line: Line, column: usize, segment: &str) {
        match self.get_mut(line) {
            Some(data) => {
                let line_len = data.len();
                if column <= line_len {
                    data.insert_str(column, segment);
                    return;
                }

                let indent: String = iter::repeat(' ').take(column - line_len).collect();
                data.push_str(&format!("{indent}{segment}"));
            }
            None => {
                let indent: String = iter::repeat(' ').take(column).collect();
                self.insert(line, &format!("{indent}{segment}"));
            }
        }
    }

    pub fn line_append(&mut self, line: Line, character: char) {
        match self.get_mut(line) {
            Some(data) => data.push(character),
            None => self.insert(line, character.to_string().as_str()),
        }
    }

    pub fn line_append_str(&mut self, line: Line, segment: &str) {
        match self.get_mut(line) {
            Some(data) => data.push_str(segment),
            None => self.insert(line, segment),
        }
    }

    pub fn line_remove(&mut self, line: Line, column: usize) -> Option<char> {
        if column < self.line_len(line) {
            return self.get_mut(line).map(|data| data.remove(column));
        }

        None
    }

    pub fn line_remove_span(&mut self, line: Line, mut span: Span) -> Option<String> {
        match self.get_mut(line) {
            Some(data) => {
                let len = data.len();
                if len == 0 || span.start >= len {
                    return None;
                }

                if span.end >= len {
                    span.end = len;
                }

                Some(data.drain(span).collect())
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

    fn format(&self) -> String {
        let span = Span {
            start: 0,
            end: self.len(),
        };

        self.format_span(&span).collect()
    }

    fn format_span(&self, span: &Span) -> impl Iterator<Item = String> + '_ {
        (span.start..=span.end).map(|i| match self.get(Line::Index(i)) {
            Some(line) => format!("{line}{}", util::newline()),
            None => util::newline().to_owned(),
        })
    }
}

impl Display for FrameBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = format!(
            "FrameBuffer: {{
  entry: {:#?},
  position:  ({}, {}),
  view_span: ({}, {}),
{}
}}",
            self.entry.borrow(),
            self.position.borrow().0,
            self.position.borrow().1,
            self.viewable_rows.start,
            self.viewable_rows.end,
            self.format(),
        );

        write!(f, "{message}")
    }
}

#[cfg(test)]
mod test {
    use super::{FrameBuffer, Line};
    use crate::{error::Result, Span};
    use std::path::PathBuf;

    #[test]
    fn insert() {
        let mut buffer = FrameBuffer::new(vec![], None, Span { start: 0, end: 5 });
        buffer.append("hello world");
        buffer.insert(Line::Index(1), "xiu");
        buffer.insert(Line::Index(1), "my name is");
        buffer.insert(Line::Index(3), ":)");

        assert_eq!(buffer.text_buffer[0], "hello world");
        assert_eq!(buffer.text_buffer[1], "my name is");
        assert_eq!(buffer.text_buffer[2], "xiu");
        assert_eq!(buffer.text_buffer[3], ":)");
    }

    #[test]
    fn from_path() -> Result<()> {
        let mut buffer =
            FrameBuffer::try_from_path(PathBuf::from("config.ron"), Span { start: 0, end: 5 })?;
        buffer.append("hello world");

        Ok(())
    }

    #[test]
    fn line_remove_span() -> Result<()> {
        let mut buffer = FrameBuffer::new(
            vec!["Hello world".to_owned()],
            None,
            Span { start: 0, end: 5 },
        );

        let segment = buffer.line_remove_span(Line::Current, Span { start: 0, end: 5 });
        assert_eq!(segment, Some("Hello".to_owned()));

        Ok(())
    }

    #[test]
    fn line_remove_span_full() -> Result<()> {
        let mut buffer = FrameBuffer::new(
            vec!["Hello world".to_owned()],
            None,
            Span { start: 0, end: 5 },
        );

        let line_len = buffer.line_len(Line::Current);
        let segment = buffer.line_remove_span(
            Line::Current,
            Span {
                start: 0,
                end: line_len,
            },
        );
        assert_eq!(segment, Some("Hello world".to_owned()));

        Ok(())
    }
}
