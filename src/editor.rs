use crate::{
    error::Result,
    terminal::{Direction, Terminal},
    text_buffer::TextBuffer,
    CHAR_MAP,
};
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

pub enum Message {
    Continue,
    Stop,
}

pub enum Mode {
    Normal,
    Insert,
    Visual,
}
pub struct Editor {
    terminal: Terminal,
    position: (u16, u16),
    mode: Mode,
    buffer: TextBuffer,
    span: Span,
}

impl Editor {
    #[inline]
    pub fn new(terminal: Terminal, buffer: TextBuffer) -> Self {
        let span = Span {
            start: 0,
            end: terminal.size.1 as usize,
        };

        Self {
            terminal,
            buffer,
            mode: Mode::Normal,
            position: (0, 0),
            span,
        }
    }

    #[inline]
    pub fn initialize(&mut self) -> Result<()> {
        self.terminal.initialize()
    }

    pub fn run(&mut self) -> Result<()> {
        self.buffer_rows()?;
        self.buffer_write()?;

        self.terminal.cursor_reset()?;

        loop {
            let event = event::read()?;
            if let Message::Stop = self.handle_event(&event)? {
                break;
            }
        }

        Ok(())
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<Message> {
        match *event {
            Event::Key(event) => self.handle_key_event(event),
            Event::Mouse(event) => self.handle_mouse_event(event),
            Event::Resize(width, height) => self.handle_resize_event(width, height),
        }
    }

    fn handle_key_event(&mut self, event: KeyEvent) -> Result<Message> {
        match (event.code, event.modifiers) {
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => return Ok(Message::Stop),

            (KeyCode::Left, KeyModifiers::NONE) => self.move_left()?,
            (KeyCode::Right, KeyModifiers::NONE) => self.move_right()?,
            (KeyCode::Up, KeyModifiers::NONE) => self.move_up()?,
            (KeyCode::Down, KeyModifiers::NONE) => self.move_down()?,
            (KeyCode::Backspace, KeyModifiers::NONE) => self.delete_last()?,
            (code, KeyModifiers::NONE) | (code, KeyModifiers::SHIFT) => match code {
                KeyCode::Enter => {
                    self.terminal.write("\r\n")?;
                    self.position.0 = 0;
                    self.position.1 += 1;
                }
                KeyCode::Tab => {
                    self.terminal.write(CHAR_MAP.get(&code).unwrap())?;

                    let dx = self.position.0 + 4;
                    if dx > self.terminal.size.0 {
                        self.position.0 = dx - self.terminal.size.0;
                        self.position.1 += 1;
                    }
                }
                code => {
                    if let Some(value) = CHAR_MAP.get(&code) {
                        self.terminal.write(*value)?;
                    }
                }
            },
            _ => {}
        };

        Ok(Message::Continue)
    }

    fn handle_mouse_event(&mut self, event: MouseEvent) -> Result<Message> {
        if let MouseEventKind::Down(MouseButton::Left) = event.kind {
            self.move_to(event.column, event.row)?;
        };

        Ok(Message::Continue)
    }

    fn handle_resize_event(&mut self, width: u16, height: u16) -> Result<Message> {
        let dy = height as i16 - self.terminal.size.1 as i16;
        self.span.end += dy as usize;
        self.terminal.size = (width, height);

        Ok(Message::Continue)
    }

    fn buffer_write(&mut self) -> Result<()> {
        let contents = self.buffer.display_range(&self.span);
        self.terminal.write(contents)?;

        Ok(())
    }

    // TODO: Fix write overs on scroll
    fn buffer_rows(&mut self) -> Result<()> {
        let (rows, _columns) = self.terminal.size()?;
        (0..rows).for_each(|i| self.buffer.append(i as usize, "~\r\n"));

        Ok(())
    }

    fn move_to(&mut self, column: u16, row: u16) -> Result<()> {
        self.position.0 = column;
        self.position.1 = row;

        self.terminal.cursor_move_to(column, row)
    }

    fn move_left(&mut self) -> Result<()> {
        if self.position.0 > 0 {
            self.position.0 -= 1;

            return self.terminal.cursor_move(Direction::Left, 1);
        }

        if self.position.1 > 0 {
            self.position.0 = self.terminal.size.0;
            self.position.1 -= 1;

            return self
                .terminal
                .cursor_move_to(self.position.0, self.position.1);
        }

        return Ok(());
    }

    fn move_right(&mut self) -> Result<()> {
        if self.position.0 == self.terminal.size.0 {
            self.position.0 = 0;
            self.position.1 += 1;

            return self
                .terminal
                .cursor_move_to(self.position.0, self.position.1);
        }

        self.position.0 += 1;

        self.terminal.cursor_move(Direction::Right, 1)
    }

    fn move_up(&mut self) -> Result<()> {
        if self.position.1 > 0 {
            self.position.1 -= 1;

            return self.terminal.cursor_move(Direction::Up, 1);
        }

        Ok(())
    }

    fn move_down(&mut self) -> Result<()> {
        self.position.1 += 1;

        self.terminal.cursor_move(Direction::Down, 1)
    }

    fn delete_last(&mut self) -> Result<()> {
        self.move_left()?;

        self.terminal.delete_current()
    }
}
