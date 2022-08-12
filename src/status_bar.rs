use crate::editor::Mode;
use std::{cell::RefCell, fmt::Display, iter, path::PathBuf, rc::Rc};

#[derive(Debug)]
pub struct StatusBar {
    terminal_size: Rc<RefCell<(u16, u16)>>,
    mode: Rc<RefCell<Mode>>,
    entry: Rc<RefCell<Option<PathBuf>>>,
    position: Rc<RefCell<(usize, usize)>>,
}

impl StatusBar {
    pub fn new(
        terminal_size: Rc<RefCell<(u16, u16)>>,
        mode: Rc<RefCell<Mode>>,
        entry: Rc<RefCell<Option<PathBuf>>>,
        position: Rc<RefCell<(usize, usize)>>,
    ) -> Self {
        Self {
            terminal_size,
            mode,
            entry,
            position,
        }
    }
}

impl Display for StatusBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match *self.mode.borrow() {
            Mode::Insert => "Insert",
            Mode::Normal => "Normal",
            Mode::Visual => "Visual",
        };
        let entry = match self.entry.borrow().as_ref() {
            Some(entry) => format!("{}", entry.display()),
            None => "[scratch]".to_owned(),
        };
        let position = format!("{}:{}", self.position.borrow().0, self.position.borrow().1);
        let center_indent: String = {
            let lhs_length = 1 + mode.len() + 4 + entry.len();
            let width = self.terminal_size.borrow().0 as usize;

            iter::repeat(' ')
                .take(width - lhs_length - position.len() - 1)
                .collect()
        };

        write!(f, " {mode}    {entry}{center_indent}{position} ")
    }
}
