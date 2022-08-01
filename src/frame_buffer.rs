use crate::{error::Result, text_buffer::TextBuffer, Span};
use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

pub struct FrameBuffer {
    text_buffer: TextBuffer,
    pub position: (usize, usize),
    pub viewable_rows: Span,
}

impl FrameBuffer {
    pub fn new(text_buffer: TextBuffer, viewable_rows: Span) -> Self {
        Self {
            text_buffer,
            position: (0, 0),
            viewable_rows,
        }
    }

    pub fn try_from_path(path: PathBuf, viewable_rows: Span) -> Result<Self> {
        let text_buffer = TextBuffer::try_from(path)?;

        Ok(Self::new(text_buffer, viewable_rows))
    }

    pub fn insert(&mut self, row: usize, data: &str) {
        self.text_buffer.insert(row, data);
    }

    pub fn append(&mut self, data: &str) {
        self.text_buffer.append(data);
    }

    pub fn remove(&mut self, row: usize) -> Option<String> {
        self.text_buffer.remove(row)
    }

    pub fn line_insert(&mut self, row: usize, column: usize, character: char) {
        self.text_buffer.line_insert(row, column, character)
    }

    pub fn line_insert_str(&mut self, row: usize, column: usize, segment: &str) {
        self.text_buffer.line_insert_str(row, column, segment)
    }

    pub fn line_append(&mut self, row: usize, character: char) {
        self.text_buffer.line_append(row, character)
    }

    pub fn line_append_str(&mut self, row: usize, segment: &str) {
        self.text_buffer.line_append_str(row, segment)
    }

    pub fn line_remove(&mut self, row: usize, column: usize) -> Option<char> {
        self.text_buffer.line_remove(row, column)
    }

    pub fn line_remove_span(&mut self, row: usize, span: Span) -> Option<String> {
        self.text_buffer.line_remove_span(row, span)
    }

    pub fn format_viewable(&self) -> String {
        self.text_buffer.format_span(self.viewable_rows.clone())
    }
}

impl Display for FrameBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text_buffer)
    }
}

impl Debug for FrameBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            self.text_buffer,
        );

        write!(f, "{message}")
    }
}

#[test]
fn works() {
    let mut buffer = FrameBuffer::new(TextBuffer::default(), Span { start: 0, end: 5 });
    buffer.append("hello world");
    buffer.insert(3, "xiu");
    buffer.insert(3, "my name is");
    buffer.insert(6, ":)");

    println!("{}\n", buffer.format_viewable());
    println!("{buffer}");
}

#[test]
fn from_path() -> Result<()> {
    let mut buffer =
        FrameBuffer::try_from_path(PathBuf::from("config.ron"), Span { start: 0, end: 5 })?;
    buffer.append("hello world");

    println!("{}\n", buffer.format_viewable());
    println!("{buffer}");

    Ok(())
}
