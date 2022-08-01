use crate::{
    error::Result,
    frame_buffer::FrameBuffer,
    terminal::{Move, Terminal},
    Config, Span, CHAR_MAP,
};
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

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
    buffer: FrameBuffer,
    config: Config,
    mode: Mode,
}

impl Editor {
    #[inline]
    pub fn new(terminal: Terminal, mut buffer: FrameBuffer, config: Config) -> Self {
        buffer.viewable_rows = Span {
            start: 0,
            end: terminal.size.1 as usize,
        };

        Self {
            terminal,
            buffer,
            config,
            mode: Mode::Normal,
        }
    }

    #[inline]
    pub fn initialize(&mut self) -> Result<()> {
        self.terminal.initialize(&self.config)?;
        self.terminal.clear()?;
        self.terminal.cursor_reset()?;
        self.terminal.write(self.buffer.format_viewable())?;

        self.terminal.cursor_reset()
    }

    pub fn run(&mut self) -> Result<()> {
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
            (KeyCode::Enter, KeyModifiers::NONE) => self.newline()?,
            (KeyCode::Tab, KeyModifiers::NONE) => self.tab()?,
            (code, KeyModifiers::NONE | KeyModifiers::SHIFT) => self.write(code)?,
            _ => {}
        };

        Ok(Message::Continue)
    }

    fn handle_mouse_event(&mut self, event: MouseEvent) -> Result<Message> {
        if let MouseEventKind::Down(MouseButton::Left) = event.kind {
            self.buffer.position = (event.row, event.row);
            self.move_to(event.column, event.row)?;
        };

        Ok(Message::Continue)
    }

    fn handle_resize_event(&mut self, width: u16, height: u16) -> Result<Message> {
        let dy = height as i16 - self.terminal.size.1 as i16;
        match dy.is_positive() {
            true => self.buffer.viewable_rows.end += dy as usize,
            false => self.buffer.viewable_rows.end -= dy as usize,
        }
        self.terminal.size = (width, height);

        Ok(Message::Continue)
    }

    fn move_to(&mut self, column: u16, row: u16) -> Result<()> {
        self.buffer.position = (column, row);

        self.terminal.cursor_move_to(column, row)
    }

    // TODO: Tighten up some of this repetition
    fn move_left(&mut self) -> Result<()> {
        if self.buffer.position.0 > 0 {
            self.buffer.position.0 -= 1;

            return self.terminal.cursor_move(Move::Left(1));
        }

        if self.buffer.position.1 > 0 {
            self.buffer.position.0 = self.terminal.size.0;
            self.buffer.position.1 -= 1;

            return self
                .terminal
                .cursor_move_to(self.buffer.position.0, self.buffer.position.1);
        }

        Ok(())
    }

    fn move_right(&mut self) -> Result<()> {
        if self.buffer.position.0 == self.terminal.size.0 {
            self.buffer.position.0 = 0;
            self.buffer.position.1 += 1;

            return self
                .terminal
                .cursor_move_to(self.buffer.position.0, self.buffer.position.1);
        }

        self.buffer.position.0 += 1;

        self.terminal.cursor_move(Move::Right(1))
    }

    fn move_up(&mut self) -> Result<()> {
        if self.buffer.position.1 > 0 {
            self.buffer.position.1 -= 1;

            return self.terminal.cursor_move(Move::Up(1));
        }

        Ok(())
    }

    fn move_down(&mut self) -> Result<()> {
        self.buffer.position.1 += 1;

        self.terminal.cursor_move(Move::Down(1))
    }

    fn write(&mut self, keycode: KeyCode) -> Result<()> {
        match CHAR_MAP.get(&keycode) {
            Some(value) => {
                self.buffer.position.0 += 1;

                self.terminal.write(value)
            }
            None => Ok(()),
        }
    }

    fn newline(&mut self) -> Result<()> {
        self.buffer.position.0 = 0;
        self.buffer.position.1 += 1;

        self.terminal.write("\r\n")
    }

    fn tab(&mut self) -> Result<()> {
        self.buffer.position.0 += 4;

        self.terminal.write('\t')
    }

    fn delete_last(&mut self) -> Result<()> {
        self.move_left()?;
        self.terminal.write(' ')?;

        self.move_left()
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        println!("{}", self.buffer);
    }
}
