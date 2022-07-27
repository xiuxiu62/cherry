use crate::{error::Result, Config};
use crossterm::{cursor, event, style, terminal, Command, ExecutableCommand};
use std::{
    fmt::Display,
    io::{self, Stdout},
};

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Terminal(Stdout);

impl Terminal {
    #[inline]
    pub fn new() -> Self {
        Self(io::stdout())
    }

    pub fn initialize(&mut self, config: &Config) -> Result<()> {
        self.enable_raw_mode()?;

        self.execute(event::EnableMouseCapture)?;
        self.execute(terminal::EnterAlternateScreen)?;

        if config.line_wrapping {
            self.execute(terminal::EnableLineWrap)?;
        }

        if config.mouse_capture {
            self.execute(event::EnableMouseCapture)?;
        }

        if let Some(color) = config.foreground_color {
            self.execute(style::SetForegroundColor(color))?;
        }

        if let Some(color) = config.background_color {
            self.execute(style::SetBackgroundColor(color))?;
        }

        if let Some(color) = config.underline_color {
            self.execute(style::SetUnderlineColor(color))?;
        }

        Ok(())
    }

    #[inline]
    pub fn write<T: Display>(&mut self, data: T) -> Result<()> {
        self.execute(style::Print(data))
    }

    pub fn delete_last(&mut self) -> Result<()> {
        self.cursor_move(Direction::Left, 1)?;
        self.write(" ")?;

        self.cursor_move(Direction::Left, 1)
    }

    #[inline]
    pub fn cursor_move(&mut self, direction: Direction, n: u16) -> Result<()> {
        match direction {
            Direction::Left => Cursor::left(self, n),
            Direction::Right => Cursor::right(self, n),
            Direction::Up => Cursor::up(self, n),
            Direction::Down => Cursor::down(self, n),
        }
    }

    #[inline]
    pub fn cursor_reset(&mut self) -> Result<()> {
        Cursor::reset(self)
    }

    #[inline]
    pub fn size(&self) -> Result<(u16, u16)> {
        Ok(terminal::size()?)
    }

    #[inline]
    fn enable_raw_mode(&self) -> Result<()> {
        terminal::enable_raw_mode()?;

        Ok(())
    }

    #[inline]
    fn disable_raw_mode(&self) -> Result<()> {
        terminal::disable_raw_mode()?;

        Ok(())
    }

    #[inline]
    fn execute(&mut self, command: impl Command) -> Result<()> {
        self.0.execute(command)?;

        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        if let Err(err) = self.execute(terminal::LeaveAlternateScreen) {
            eprintln!("failed to leave alternate screen: {err}")
        };

        if let Err(err) = self.disable_raw_mode() {
            eprintln!("failed to disable raw mode: {err}")
        };
    }
}

pub struct Cursor;

impl Cursor {
    #[inline]
    pub fn show(terminal: &mut Terminal) -> Result<()> {
        terminal.execute(cursor::Show)
    }

    #[inline]
    pub fn hide(terminal: &mut Terminal) -> Result<()> {
        terminal.execute(cursor::Hide)
    }

    #[inline]
    pub fn reset(terminal: &mut Terminal) -> Result<()> {
        terminal.execute(cursor::MoveTo(0, 0))
    }

    #[inline]
    pub fn left(terminal: &mut Terminal, n: u16) -> Result<()> {
        terminal.execute(cursor::MoveLeft(n))
    }

    #[inline]
    pub fn right(terminal: &mut Terminal, n: u16) -> Result<()> {
        terminal.execute(cursor::MoveRight(n))
    }

    #[inline]
    pub fn up(terminal: &mut Terminal, n: u16) -> Result<()> {
        terminal.execute(cursor::MoveUp(n))
    }

    #[inline]
    pub fn down(terminal: &mut Terminal, n: u16) -> Result<()> {
        terminal.execute(cursor::MoveDown(n))
    }
}
