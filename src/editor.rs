use crate::{
    error::Result,
    terminal::{Direction, Terminal},
    text_buffer::TextBuffer,
    Config, CHAR_MAP,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use std::{mem, ops::Range};

pub enum Message {
    Continue,
    Stop,
}

type Span<T> = std::ops::Range<T>;
type Spanned<T> = (T, Span<usize>);

pub struct Editor {
    terminal: Terminal,
    position: (u16, u16),
    buffer: TextBuffer,
    top_line: usize,
}

impl Editor {
    #[inline]
    pub fn new(terminal: Terminal, buffer: TextBuffer) -> Self {
        Self {
            terminal,
            buffer,
            position: (0, 0),
            top_line: 0,
        }
    }

    #[inline]
    pub fn initialize(&mut self, config: &Config) -> Result<()> {
        self.terminal.initialize(&config)
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
            (KeyCode::Null, KeyModifiers::NONE) => self.terminal.write("")?,
            (code, KeyModifiers::NONE) => {
                if let Some(value) = CHAR_MAP.get(&code) {
                    self.terminal.write(*value)?;
                }
            }
            _ => {}
        };

        Ok(Message::Continue)
    }

    fn handle_mouse_event(&mut self, _event: MouseEvent) -> Result<Message> {
        Ok(Message::Continue)
    }

    fn handle_resize_event(&mut self, width: u16, height: u16) -> Result<Message> {
        self.terminal.size = (width, height);

        Ok(Message::Continue)
    }

    fn buffer_write(&mut self) -> Result<()> {
        let (start, end) = self.view_range();
        let contents = self.buffer.display_range(start, end);
        self.terminal.write(contents)?;

        Ok(())
    }

    // TODO: Fix write overs on scroll
    fn buffer_rows(&mut self) -> Result<()> {
        let (rows, _columns) = self.terminal.size()?;
        (0..rows).for_each(|i| self.buffer.append(i as usize, "~\n"));

        Ok(())
    }

    fn view_range(&self) -> (usize, usize) {
        (self.top_line, self.terminal.size.1 as usize)
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
