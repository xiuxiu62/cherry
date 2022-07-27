use crate::editor::Span;

#[repr(transparent)]
pub struct TextBuffer(pub Vec<String>);

impl TextBuffer {
    pub fn new(inner: Vec<String>) -> Self {
        Self(inner)
    }

    pub fn insert(&mut self, i: usize, line: String) {
        self.0.insert(i, line);
    }

    pub fn append(&mut self, i: usize, data: &str) {
        match self.0.get_mut(i) {
            Some(line) => line.push_str(data),
            None => self.insert(i, data.to_owned()),
        }
    }

    pub fn line_len(&self, i: usize) -> usize {
        match self.0.get(i) {
            Some(line) => line.len(),
            None => 0,
        }
    }

    pub fn display_range(&self, span: &Span) -> String {
        (span.start..span.end)
            .map(|i| match self.0.get(i) {
                Some(line) => line,
                None => "\n",
            })
            .fold("".to_owned(), |acc, line| format!("{acc}{line}"))
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self(vec![])
    }
}

impl From<String> for TextBuffer {
    fn from(data: String) -> Self {
        Self(data.split('\n').map(|s| s.to_owned()).collect())
    }
}

impl Into<String> for TextBuffer {
    fn into(self) -> String {
        self.0.join("")
    }
}
