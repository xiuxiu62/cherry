use std::fmt::Display;

use crate::{
    error::Result,
    frame_buffer::{FrameBuffer, GUTTER_WIDTH},
    terminal::Terminal,
    Span,
};
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use tracing::info;

mod action;

use action::Action;

use self::action::{HistoryNode, Message};

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
}

#[derive(Debug)]
pub enum Move {
    Left(u16),
    Right(u16),
    Up(u16),
    Down(u16),
    NextLine(u16),
    PreviousLine(u16),
    To(u16, u16),
}

#[derive(Debug)]
pub struct Editor {
    terminal: Terminal,
    pub buffer: FrameBuffer,
    mode: Mode,
    history: Vec<HistoryNode>,
}

impl Editor {
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
            history: vec![],
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let event = event::read()?;
            if let Message::Exit = self.handle_event(&event)? {
                break;
            }
        }

        Ok(())
    }

    // TODO: update start position once frame splitting is implemented
    pub fn initialize(&mut self) -> Result<()> {
        info!("[EDITOR] (initialize) start");
        self.terminal.initialize(0, 0)?;

        let data = self.buffer.format_viewable();
        self.terminal.write(data)?;
        self.terminal.cursor_move_to(GUTTER_WIDTH, 0)?;

        info!("[EDITOR] (initialize) end");
        Ok(())
    }

    pub fn format_history(&mut self) -> String {
        self.history
            .iter()
            .map(|node| format!("{node}\n"))
            .collect()
    }

    #[inline]
    pub fn handle_event(&mut self, event: &Event) -> Result<Message> {
        match *event {
            Event::Key(event) => self.handle_key_event(event),
            Event::Mouse(event) => self.handle_mouse_event(event),
            Event::Resize(width, height) => self.handle_resize_event(width, height),
        }
    }

    #[inline]
    fn handle_key_event(&mut self, event: KeyEvent) -> Result<Message> {
        match self.mode {
            Mode::Insert => self.handle_insert_mode_key_event(event),
            Mode::Normal => self.handle_normal_mode_key_event(event),
            Mode::Visual => Ok(Message::Continue),
        }
    }

    #[inline]
    fn handle_insert_mode_key_event(&mut self, event: KeyEvent) -> Result<Message> {
        match (event.code, event.modifiers) {
            (KeyCode::Esc, KeyModifiers::NONE) => Action::ChangeMode(Mode::Normal),
            (KeyCode::Left, KeyModifiers::NONE) => Action::MoveLeft,
            (KeyCode::Right, KeyModifiers::NONE) => Action::MoveRight,
            (KeyCode::Up, KeyModifiers::NONE) => Action::MoveUp,
            (KeyCode::Down, KeyModifiers::NONE) => Action::MoveDown,
            (KeyCode::Backspace, KeyModifiers::NONE) => Action::DeleteLast,
            (KeyCode::Enter, KeyModifiers::NONE) => Action::Newline,
            (KeyCode::Tab, KeyModifiers::NONE) => Action::Tab,
            (code, KeyModifiers::NONE | KeyModifiers::SHIFT) => Action::Write(code),
            _ => Action::None,
        }
        .execute(self)
    }

    #[inline]
    fn handle_normal_mode_key_event(&mut self, event: KeyEvent) -> Result<Message> {
        match (event.code, event.modifiers) {
            (KeyCode::Char('i'), KeyModifiers::NONE) => Action::ChangeMode(Mode::Insert),
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => Action::Exit,
            (KeyCode::Left | KeyCode::Char('h'), KeyModifiers::NONE) => Action::MoveLeft,
            (KeyCode::Right | KeyCode::Char('l'), KeyModifiers::NONE) => Action::MoveRight,
            (KeyCode::Up | KeyCode::Char('k'), KeyModifiers::NONE) => Action::MoveUp,
            (KeyCode::Down | KeyCode::Char('j'), KeyModifiers::NONE) => Action::MoveDown,
            _ => Action::None,
        }
        .execute(self)
    }

    fn handle_mouse_event(&mut self, event: MouseEvent) -> Result<Message> {
        if let MouseEventKind::Down(MouseButton::Left) = event.kind {
            self.buffer.position = (event.row, event.row);
            Action::MoveTo(event.column, event.row).execute(self)?;
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
}
