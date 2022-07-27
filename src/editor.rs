use crate::{
    error::Result,
    terminal::{Direction, Terminal},
    Config, CHAR_MAP,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use std::mem;

pub enum Message {
    Continue,
    Stop,
}

pub struct Editor {
    terminal: Terminal,
    position: (u16, u16),
    buffer: String,
}

impl Editor {
    #[inline]
    pub fn new(terminal: Terminal) -> Self {
        Self {
            terminal,
            position: (0, 0),
            buffer: "".to_owned(),
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
            _ => Ok(Message::Continue),
        }
    }

    fn handle_key_event(&mut self, event: KeyEvent) -> Result<Message> {
        match (event.code, event.modifiers) {
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => return Ok(Message::Stop),
            (KeyCode::Left, KeyModifiers::NONE) => self.terminal.cursor_move(Direction::Left, 1)?,
            (KeyCode::Right, KeyModifiers::NONE) => {
                self.terminal.cursor_move(Direction::Right, 1)?
            }
            (KeyCode::Up, KeyModifiers::NONE) => self.terminal.cursor_move(Direction::Up, 1)?,
            (KeyCode::Down, KeyModifiers::NONE) => self.terminal.cursor_move(Direction::Down, 1)?,
            (KeyCode::Backspace, KeyModifiers::NONE) => self.terminal.delete_last()?,
            (code, KeyModifiers::NONE) => {
                if let Some(value) = CHAR_MAP.get(&code) {
                    self.terminal.write(*value)?;
                }
            }
            _ => {}
        };

        Ok(Message::Continue)
    }

    fn handle_mouse_event(&mut self, event: MouseEvent) -> Result<Message> {
        todo!()
    }

    fn handle_resize_event(&mut self, width: u16, height: u16) -> Result<Message> {
        self.terminal.size = (width, height);

        Ok(Message::Continue)
    }

    // fn refresh(&mut self) -> DynResult<()> {
    //     self.terminal.cursor_hide()?;
    //     self.draw_rows()?;
    //     self.terminal.cursor_move(Position::new(0, 0))?;

    //     self.terminal.write(self.append_buffer.to_owned())?;
    //     self.terminal.cursor_show()?;

    //     Ok(())
    // }

    fn buffer_write(&mut self) -> Result<()> {
        let contents = mem::take(&mut self.buffer);
        self.terminal.write(contents)?;

        Ok(())
    }

    #[inline]
    fn buffer_append(&mut self, data: &str) {
        self.buffer.push_str(data);
    }

    fn buffer_rows(&mut self) -> Result<()> {
        let (rows, _columns) = self.terminal.size()?;
        (0..rows).for_each(|_| self.buffer_append("~\r\n"));

        Ok(())
    }
}
