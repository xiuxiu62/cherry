use crate::{
    editor::{Mode, Move},
    error::Result,
    frame_buffer::{Line, GUTTER_WIDTH},
    Editor, Span, CHAR_MAP,
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
    position: (usize, usize),
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
    MoveTo(usize, usize),
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

impl Action {
    pub fn execute(self, editor: &mut Editor) -> Result<Message> {
        match self {
            Action::ChangeMode(mode) => editor.change_mode(mode),
            Action::MoveLeft => editor.move_left()?,
            Action::MoveRight => editor.move_right()?,
            Action::MoveUp => editor.move_up()?,
            Action::MoveDown => editor.move_down()?,
            Action::MoveTo(column, row) => editor.move_to((column, row))?,
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

    fn move_to(&mut self, (column, row): (usize, usize)) -> Result<()> {
        self.buffer.position.replace((column, row));

        self.terminal.cursor_move_to((column + GUTTER_WIDTH, row))
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
        match self.buffer.get(Line::Previous) {
            Some(line) => self.move_to((line.len(), row - 1)),
            None => self.move_to((0, row - 1)),
        }
    }

    fn move_right(&mut self) -> Result<()> {
        if let Some(line) = self.buffer.get(Line::Current) {
            if self.buffer.position.borrow().0 < line.len() {
                self.buffer.position.borrow_mut().0 += 1;
                self.terminal.cursor_move(Move::Right(1))?;

                return Ok(());
            }
        }

        let row = self.buffer.position.borrow().1;
        self.move_to((0, row + 1))
    }

    fn move_up(&mut self) -> Result<()> {
        if self.buffer.position.borrow().1 == 0 {
            return Ok(());
        }

        let row = self.buffer.position.borrow().1 - 1;
        let column = match self.buffer.get(Line::Next) {
            Some(line) => line.len(),
            None => 0,
        };

        self.move_to((column, row))
    }

    fn move_down(&mut self) -> Result<()> {
        let row = self.buffer.position.borrow().1 + 1;
        let column = match self.buffer.get(Line::Next) {
            Some(line) => line.len(),
            None => 0,
        };

        self.move_to((column, row))
    }

    fn write_char(&mut self, keycode: KeyCode) -> Result<()> {
        if let Some(value) = CHAR_MAP.get(&keycode) {
            let column = self.buffer.position.borrow().0;
            self.buffer.line_insert(Line::Current, column, *value);

            // SAFETY: if the line did not exist before, we've just created it
            let current_line = self.buffer.get(Line::Current).unwrap();
            self.terminal.write(&current_line[column..])?;
            self.buffer.position.borrow_mut().0 += 1;
        };

        Ok(())
    }

    fn newline(&mut self) -> Result<()> {
        let (column, row) = *self.buffer.position.borrow();

        // If there is text after our cursor move rest to next line and
        // re-render current and next line,
        // moving cursor to the start of the next line
        // SAFETY: we ensure that there is data present prior to each unwrap
        if let Some(data) = self.buffer.get(Line::Current) {
            let line_len = data.len();
            if column < line_len {
                // remove remainder of the line
                let line_slice = self
                    .buffer
                    .line_remove_span(
                        Line::Current,
                        Span {
                            start: column,
                            end: line_len,
                        },
                    )
                    .unwrap();

                self.buffer.insert(Line::Next, &line_slice);
                let next_row = self.buffer.position.borrow().1 + 1;
                self.buffer.position.replace((0, next_row));

                return self.rerender();

                // return self.rerender();
                // TODO: fix re-render
                // self.terminal.cursor_hide()?;
                // // place remained on the next line
                // self.buffer.insert(Line::Next, &line_slice);

                // // re-render current line
                // self.terminal
                //     .cursor_move(Move::To(GUTTER_WIDTH as u16, row as u16))?;
                // self.terminal.clear_current_line()?;
                // self.terminal
                //     .write(self.buffer.get(Line::Current).unwrap())?;

                // // re-render next line
                // self.terminal
                //     .cursor_move(Move::To(GUTTER_WIDTH as u16, row as u16 + 1))?;
                // self.terminal.clear_current_line()?;
                // self.terminal.write(self.buffer.get(Line::Next).unwrap())?;
                // self.terminal.cursor_show()?;

                // // place cursor at beginning of next line
                // return self.move_to((0, row + 1));
            }
        };

        self.buffer.insert(Line::Next, "");
        self.move_to((0, row + 1))?;

        self.rerender()
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
        let column = self.buffer.position.borrow().0;
        self.buffer.line_remove(Line::Current, column);

        self.terminal.write(' ')
    }

    fn rerender(&mut self) -> Result<()> {
        let current_position = *self.buffer.position.borrow();
        let viewable = self.buffer.format_viewable();
        self.terminal.cursor_hide()?;
        self.terminal.clear()?;
        self.terminal.cursor_reset()?;
        self.terminal.write(viewable)?;
        self.move_to(current_position)?;

        self.terminal.cursor_show()
    }
}
