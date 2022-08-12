use crate::{
    error::Result,
    frame_buffer::{FrameBuffer, GUTTER_WIDTH},
    status_bar::StatusBar,
    terminal::Terminal,
    Span,
};
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use std::{cell::RefCell, rc::Rc};

mod action;
use action::{Action, HistoryNode, Message};

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
    pub status_bar: StatusBar,
    mode: Rc<RefCell<Mode>>,
    history: Vec<HistoryNode>,
}

impl Editor {
    pub fn new(terminal: Terminal, mut buffer: FrameBuffer) -> Self {
        buffer.viewable_rows = Span {
            start: 0,
            end: terminal.size.as_ref().borrow().1 as usize - 2,
        };

        let mode = Rc::new(RefCell::new(Mode::Normal));
        let status_bar = StatusBar::new(
            Rc::clone(&terminal.size),
            Rc::clone(&mode),
            Rc::clone(&buffer.entry),
            Rc::clone(&buffer.position),
        );

        Self {
            terminal,
            buffer,
            status_bar,
            mode,
            history: vec![],
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let event = event::read()?;
            if let Message::Exit = self.handle_event(&event)? {
                break;
            }

            self.draw_status_bar()?;
        }

        Ok(())
    }

    // TODO: update start position once frame splitting is implemented
    pub fn initialize(&mut self) -> Result<()> {
        self.terminal.initialize((0, 0))?;

        let data = self.buffer.format_viewable();
        self.terminal.write(data)?;
        self.draw_status_bar()?;
        self.terminal.cursor_move_to((GUTTER_WIDTH, 0))?;

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
        let mode = *self.mode.borrow();
        match mode {
            Mode::Normal => self.handle_normal_mode_key_event(event),
            Mode::Insert => self.handle_insert_mode_key_event(event),
            Mode::Visual => Ok(Message::Continue),
        }
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
            (KeyCode::Char('d'), KeyModifiers::NONE) => Action::DeleteCurrent,
            _ => Action::None,
        }
        .execute(self)
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

    fn handle_mouse_event(&mut self, event: MouseEvent) -> Result<Message> {
        if let MouseEventKind::Down(MouseButton::Left) = event.kind {
            self.buffer
                .position
                .replace((event.row as usize, event.row as usize));
            Action::MoveTo(event.column as usize, event.row as usize).execute(self)?;
        };

        Ok(Message::Continue)
    }

    fn handle_resize_event(&mut self, width: u16, height: u16) -> Result<Message> {
        let dy = height as i16 - self.terminal.size.borrow().1 as i16;
        match dy.is_positive() {
            true => self.buffer.viewable_rows.end += dy as usize,
            false => self.buffer.viewable_rows.end -= dy as usize,
        }
        self.terminal.size.replace((width, height));

        Ok(Message::Continue)
    }

    fn draw_status_bar(&mut self) -> Result<()> {
        let rendered_bar = self.status_bar.to_string();
        let previous_position = self.buffer.position.borrow();
        let size = self.terminal.size.borrow().1;

        self.terminal.cursor_move_to((0, size as usize - 1))?;
        self.terminal.write(&rendered_bar)?;
        self.terminal
            .cursor_move_to((previous_position.0 + GUTTER_WIDTH, previous_position.1))?;

        Ok(())
    }
}
