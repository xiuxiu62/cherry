use crate::{
    error::{Error, Result},
    util, Span,
};
use std::{cell::RefCell, fmt::Display, fs, iter, path::PathBuf, rc::Rc};

pub const GUTTER_WIDTH: usize = 5;

pub enum Row {
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

    pub fn get(&self, row: Row) -> Option<&String> {
        match row {
            Row::Previous => self.text_buffer.get(self.position.borrow().1 - 1),
            Row::Current => self.text_buffer.get(self.position.borrow().1),
            Row::Next => self.text_buffer.get(self.position.borrow().1 + 1),
            Row::Index(i) => self.text_buffer.get(i),
        }
    }

    pub fn get_mut(&mut self, row: Row) -> Option<&mut String> {
        match row {
            Row::Previous => self.text_buffer.get_mut(self.position.borrow().1 - 1),
            Row::Current => self.text_buffer.get_mut(self.position.borrow().1),
            Row::Next => self.text_buffer.get_mut(self.position.borrow().1 + 1),
            Row::Index(i) => self.text_buffer.get_mut(i),
        }
    }

    pub fn len(&self) -> usize {
        self.text_buffer.len()
    }

    pub fn line_len(&self, row: usize) -> usize {
        match self.get(Row::Index(row)) {
            Some(line) => line.len(),
            None => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.text_buffer.is_empty()
    }

    pub fn line_is_empty(&self, row: usize) -> bool {
        match self.get(Row::Index(row)) {
            Some(line) => line.is_empty(),
            None => true,
        }
    }

    pub fn insert(&mut self, row: usize, data: &str) {
        let buffer_len = self.len();
        if row < buffer_len {
            self.text_buffer.insert(row, data.to_owned());

            return;
        }

        if row > buffer_len {
            (buffer_len..row).for_each(|_| self.text_buffer.push("".to_owned()));
        }

        self.text_buffer.push(data.to_owned());
    }

    pub fn append(&mut self, line: &str) {
        self.text_buffer.push(line.to_owned());
    }

    pub fn remove(&mut self, row: usize) -> Option<String> {
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

    pub fn line_insert(&mut self, (column, row): (usize, usize), character: char) {
        let line_len = self.line_len(row);
        match self.get_mut(Row::Index(row)) {
            Some(line) => {
                if column <= line_len {
                    line.insert(column, character);
                    return;
                }

                let indent: String = iter::repeat(' ').take(column - line_len).collect();
                line.push_str(&format!("{indent}{character}"));
            }
            None => {
                let indent: String = iter::repeat(' ').take(column).collect();
                self.insert(row, &format!("{indent}{character}"));
            }
        }
    }

    // pub fn line_insert_str(&mut self, row: usize, column: usize, segment: &str) {
    //     match self.get_mut(Row::Index(row)) {
    //         Some(line) => {
    //             let len = line.len();
    //             if column < len {
    //                 line.insert_str(column, segment);
    //                 return;
    //             }

    //             let indent: String = (len..column).map(|_| ' ').collect();
    //             line.push_str(&indent);
    //             line.push_str(segment);
    //         }
    //         None => {
    //             let indent: String = (0..=column)
    //                 .into_iter()
    //                 .filter(|i| *i > 0)
    //                 .map(|_| ' ')
    //                 .collect();

    //             self.insert(row, &format!("{indent}{segment}"));
    //         }
    //     }
    // }

    pub fn line_append(&mut self, row: usize, character: char) {
        match self.get_mut(Row::Index(row)) {
            Some(line) => line.push(character),
            None => self.insert(row, character.to_string().as_str()),
        }
    }

    pub fn line_append_str(&mut self, row: usize, segment: &str) {
        match self.get_mut(Row::Index(row)) {
            Some(line) => line.push_str(segment),
            None => self.insert(row, segment),
        }
    }

    pub fn line_remove(&mut self, row: usize, column: usize) -> Option<char> {
        if self.line_len(row) < column {
            return self
                .get_mut(Row::Index(row))
                .map(|line| line.remove(column));
        }

        None
    }

    pub fn line_remove_span(&mut self, row: usize, mut span: Span) -> Option<String> {
        match self.get_mut(Row::Index(row)) {
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

    fn format(&self) -> String {
        let span = Span {
            start: 0,
            end: self.len(),
        };

        self.format_span(&span).collect()
    }

    fn format_span(&self, span: &Span) -> impl Iterator<Item = String> + '_ {
        (span.start..=span.end).map(|i| match self.get(Row::Index(i)) {
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

#[test]
fn works() {
    let mut buffer = FrameBuffer::new(vec![], None, Span { start: 0, end: 5 });
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
