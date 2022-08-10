use crate::editor::Mode;
use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug)]
pub struct StatusBar {
    terminal_size: Rc<RefCell<(u16, u16)>>,
    mode: Rc<RefCell<Mode>>,
    entry_name: Rc<RefCell<String>>,
    position: Rc<RefCell<(u16, u16)>>,
}

impl StatusBar {
    pub fn new(
        terminal_size: Rc<RefCell<(u16, u16)>>,
        mode: Rc<RefCell<Mode>>,
        entry_name: Rc<RefCell<String>>,
        position: Rc<RefCell<(u16, u16)>>,
    ) -> Self {
        Self {
            terminal_size,
            mode,
            entry_name,
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
        let position = format!("{}:{}", self.position.borrow().0, self.position.borrow().1);

        let mut message = String::new();
        message.push(' ');
        message.push_str(mode);
        message.push_str("    ");
        message.push_str(&*self.entry_name.borrow());
        (message.len() + 1..self.terminal_size.borrow().1 as usize - position.len())
            .for_each(|_| message.push(' '));
        message.push_str(&position);

        write!(f, "{message}")
    }
}
