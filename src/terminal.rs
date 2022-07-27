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

pub struct Terminal {
    stdout: Stdout,
    pub size: (u16, u16),
}

impl Terminal {
    #[inline]
    pub fn new() -> Result<Self> {
        Ok(Self {
            stdout: io::stdout(),
            size: terminal::size()?,
        })
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
        Cursor::move_(self, direction, n)
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
        self.stdout.execute(command)?;

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
    pub fn move_(terminal: &mut Terminal, direction: Direction, n: u16) -> Result<()> {
        match direction {
            Direction::Left => terminal.execute(cursor::MoveLeft(n)),
            Direction::Right => terminal.execute(cursor::MoveRight(n)),
            Direction::Up => terminal.execute(cursor::MoveUp(n)),
            Direction::Down => terminal.execute(cursor::MoveDown(n)),
        }
    }
}
