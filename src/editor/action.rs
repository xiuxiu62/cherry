use crate::{
    editor::{Mode, Move},
    error::Result,
    frame_buffer::{Row, GUTTER_WIDTH},
    Editor, CHAR_MAP,
};
use crossterm::event::KeyCode;
use std::fmt::Display;

pub enum Message {
    Continue,
    Exit,
}

#[derive(Debug)]
pub struct HistoryNode {
    action: Action,
    position: (u16, u16),
}

impl Display for HistoryNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Action: {}\nPosition: ({}, {})",
            self.action, self.position.0, self.position.1
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    ChangeMode(Mode),
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    MoveTo(u16, u16),
    Write(KeyCode),
    Newline,
    Tab,
    DeleteLast,
    DeleteCurrent,
    Exit,
    None,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::ChangeMode(Mode::Insert) => "Insert Mode",
            Self::ChangeMode(Mode::Normal) => "Normal Mode",
            Self::ChangeMode(Mode::Visual) => "Visual Mode",
            Self::MoveRight => "Move Right",
            Self::MoveLeft => "Move Left",
            Self::MoveUp => "Move Up",
            Self::MoveDown => "Move Down",
            Self::MoveTo(column, row) => return write!(f, "Move `({column}, {row})`"),
            Self::Write(char) => return write!(f, "Write `{char:?}`"),
            Self::Newline => "Newline",
            Self::Tab => "Tab",
            Self::DeleteLast => "Delete Last",
            Self::DeleteCurrent => "Delete Current",
            Self::Exit => "Exit",
            Self::None => "None",
        };

        write!(f, "{message}")
    }
}

// TODO: spin all conversions into base methods and refactor position data to be uniform
impl Action {
    pub fn execute(self, editor: &mut Editor) -> Result<Message> {
        match self {
            Action::ChangeMode(mode) => editor.change_mode(mode),
            Action::MoveLeft => editor.move_left()?,
            Action::MoveRight => editor.move_right()?,
            Action::MoveUp => editor.move_up()?,
            Action::MoveDown => editor.move_down()?,
            Action::MoveTo(column, row) => editor.move_to(column, row)?,
            Action::Write(code) => editor.write_char(code)?,
            Action::Newline => editor.newline()?,
            Action::Tab => editor.tab()?,
            Action::DeleteLast => editor.delete_last()?,
            Action::DeleteCurrent => editor.delete_current()?,
            Action::Exit => return Ok(Message::Exit),
            Action::None => return Ok(Message::Continue),
        };

        editor.history.push(HistoryNode {
            action: self,
            position: *editor.buffer.position.borrow(),
        });

        Ok(Message::Continue)
    }
}

impl Editor {
    fn change_mode(&mut self, mode: Mode) {
        self.mode.replace(mode);
    }

    fn move_to(&mut self, column: u16, row: u16) -> Result<()> {
        self.buffer.position.replace((column, row));

        self.terminal.cursor_move_to(column + GUTTER_WIDTH, row)
    }

    fn move_left(&mut self) -> Result<()> {
        if self.buffer.position.borrow().0 > 0 {
            self.buffer.position.borrow_mut().0 -= 1;

            return self.terminal.cursor_move(Move::Left(1));
        }

        if self.buffer.position.borrow().1 == 0 {
            return Ok(());
        }

        let row = self.buffer.position.borrow().1;
        match self.buffer.get(Row::Previous) {
            Some(line) => self.move_to(line.len() as u16, row - 1),
            None => self.move_to(0, row - 1),
        }
    }

    fn move_right(&mut self) -> Result<()> {
        if let Some(line) = self.buffer.get(Row::Current) {
            if self.buffer.position.borrow().0 < line.len() as u16 {
                self.buffer.position.borrow_mut().0 += 1;
                self.terminal.cursor_move(Move::Right(1))?;

                return Ok(());
            }
        }

        let row = self.buffer.position.borrow().1;
        self.move_to(0, row + 1)
    }

    fn move_up(&mut self) -> Result<()> {
        if self.buffer.position.borrow().1 == 0 {
            return Ok(());
        }

        let row = self.buffer.position.borrow().1 - 1;
        let column = match self.buffer.get(Row::Next) {
            Some(line) => line.len() as u16,
            None => 0,
        };

        self.move_to(column, row)
    }

    fn move_down(&mut self) -> Result<()> {
        let row = self.buffer.position.borrow().1 + 1;
        let column = match self.buffer.get(Row::Next) {
            Some(line) => line.len() as u16,
            None => 0,
        };

        self.move_to(column, row)
    }

    fn write_char(&mut self, keycode: KeyCode) -> Result<()> {
        if let Some(value) = CHAR_MAP.get(&keycode) {
            let (row, column) = *self.buffer.position.borrow();
            self.buffer
                .line_insert(row as usize, column as usize, *value);

            self.terminal.write(value)?;
            self.buffer.position.borrow_mut().0 += 1;
        };

        Ok(())
    }

    fn newline(&mut self) -> Result<()> {
        let next_row = self.buffer.position.borrow().1 + 1;
        self.buffer.insert(next_row as usize, "");

        self.move_to(0, next_row)
    }

    fn tab(&mut self) -> Result<()> {
        self.buffer.position.borrow_mut().0 += 4;

        self.terminal.write('\t')
    }

    fn delete_last(&mut self) -> Result<()> {
        self.move_left()?;

        self.delete_current()
    }

    fn delete_current(&mut self) -> Result<()> {
        self.terminal.write(' ')?;

        self.move_left()
    }
}
