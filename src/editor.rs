use crate::{
    error::Result,
    frame_buffer::FrameBuffer,
    terminal::{Move, Terminal},
    Config, Span, CHAR_MAP,
};
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use tracing::info;

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
    pub buffer: FrameBuffer,
    mode: Mode,
}

impl Editor {
    #[inline]
    pub fn new(terminal: Terminal, mut buffer: FrameBuffer) -> Self {
        info!("[EDITOR] (new) start");
        buffer.viewable_rows = Span {
            start: 0,
            end: terminal.size.1 as usize,
        };

        info!("[EDITOR] (new) end");
        Self {
            terminal,
            buffer,
            mode: Mode::Normal,
        }
    }

    #[inline]
    pub fn initialize(&mut self) -> Result<()> {
        info!("[EDITOR] (initialize) start");
        self.terminal.initialize()?;
        self.terminal.write(self.buffer.format_viewable())?;
        self.terminal.cursor_reset()?;

        info!("[EDITOR] (initialize) end");
        Ok(())
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
        match self.mode {
            Mode::Insert => self.handle_insert_mode_key_event(event),
            Mode::Normal => self.handle_normal_mode_key_event(event),
            Mode::Visual => Ok(Message::Continue),
        }
    }

    fn handle_insert_mode_key_event(&mut self, event: KeyEvent) -> Result<Message> {
        match (event.code, event.modifiers) {
            (KeyCode::Esc, KeyModifiers::NONE) => self.mode = Mode::Normal,
            (KeyCode::Left, KeyModifiers::NONE) => self.move_left()?,
            (KeyCode::Right, KeyModifiers::NONE) => self.move_right()?,
            (KeyCode::Up, KeyModifiers::NONE) => self.move_up()?,
            (KeyCode::Down, KeyModifiers::NONE) => self.move_down()?,
            (KeyCode::Backspace, KeyModifiers::NONE) => self.delete_last()?,
            (KeyCode::Enter, KeyModifiers::NONE) => self.newline()?,
            (KeyCode::Tab, KeyModifiers::NONE) => self.tab()?,
            (code, KeyModifiers::NONE | KeyModifiers::SHIFT) => self.write_char(code)?,
            _ => {}
        };

        Ok(Message::Continue)
    }

    fn handle_normal_mode_key_event(&mut self, event: KeyEvent) -> Result<Message> {
        match (event.code, event.modifiers) {
            (KeyCode::Char('i'), KeyModifiers::NONE) => self.mode = Mode::Insert,
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => return Ok(Message::Stop),
            (KeyCode::Left | KeyCode::Char('h'), KeyModifiers::NONE) => self.move_left()?,
            (KeyCode::Right | KeyCode::Char('l'), KeyModifiers::NONE) => self.move_right()?,
            (KeyCode::Up | KeyCode::Char('k'), KeyModifiers::NONE) => self.move_up()?,
            (KeyCode::Down | KeyCode::Char('j'), KeyModifiers::NONE) => self.move_down()?,
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
            let column = match self.buffer.get_previous() {
                Some(line) => line.len() as u16 + 1,
                None => 0,
            };
            self.buffer.position.0 = column;
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

    fn write_char(&mut self, keycode: KeyCode) -> Result<()> {
        match CHAR_MAP.get(&keycode) {
            Some(value) => {
                let (row, column) = self.buffer.position;
                self.buffer
                    .line_insert(row as usize, column as usize, *value);

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
