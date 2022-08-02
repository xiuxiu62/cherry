use crate::{error::Result, Span};
use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

mod text_buffer;
use text_buffer::TextBuffer;

// TODO: consider making position (u16, u16) for easier
// arithmetic with terminal internals
pub struct FrameBuffer {
    text_buffer: TextBuffer,
    pub position: (u16, u16),
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

    pub fn get_previous(&self) -> Option<&String> {
        self.text_buffer.get(self.position.1 as usize - 1)
    }

    pub fn get_current(&self) -> Option<&String> {
        self.text_buffer.get(self.position.1 as usize)
    }

    pub fn get_next(&self) -> Option<&String> {
        self.text_buffer.get(self.position.1 as usize + 1)
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
        let view_span = &self.viewable_rows;
        let format_number = |i: usize| match i {
            i if i >= 1000 => i.to_string(),
            i if i >= 100 => format!(" {i}"),
            i if i >= 10 => format!("  {i}"),
            i => format!("   {i}"),
        };

        (view_span.start..=view_span.end)
            .zip(self.text_buffer.format_span(view_span))
            .into_iter()
            .map(|(i, line)| format!("{} {line}", format_number(i)))
            .collect()
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
