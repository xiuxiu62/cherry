use crate::{error::Result, Config};
use crossterm::{cursor, event, style, terminal, Command, ExecutableCommand};
use std::{
    fmt::Display,
    io::{self, Stdout},
    rc::Rc,
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
    config: Rc<Config>,
}

impl Terminal {
    #[inline]
    pub fn new(config: Rc<Config>) -> Result<Self> {
        Ok(Self {
            stdout: io::stdout(),
            size: terminal::size()?,
            config,
        })
    }

    pub fn initialize(&mut self) -> Result<()> {
        self.enable_raw_mode()?;

        self.execute(event::EnableMouseCapture)?;

        if self.config.alternate_screen {
            self.execute(terminal::EnterAlternateScreen)?;
        }

        if self.config.line_wrapping {
            self.execute(terminal::EnableLineWrap)?;
        }

        if self.config.mouse_capture {
            self.execute(event::EnableMouseCapture)?;
        }

        if let Some(color) = self.config.foreground_color {
            self.execute(style::SetForegroundColor(color.into()))?;
        }

        if let Some(color) = self.config.background_color {
            self.execute(style::SetBackgroundColor(color.into()))?;
        }

        if let Some(color) = self.config.underline_color {
            self.execute(style::SetUnderlineColor(color.into()))?;
        }

        Ok(())
    }

    #[inline]
    pub fn write<T: Display>(&mut self, data: T) -> Result<()> {
        Cursor::hide(self)?;
        self.execute(style::Print(data))?;

        Cursor::show(self)
    }

    pub fn delete_current(&mut self) -> Result<()> {
        self.write(' ')?;

        self.cursor_move(Direction::Left, 1)
    }

    #[inline]
    pub fn cursor_move(&mut self, direction: Direction, n: u16) -> Result<()> {
        Cursor::move_(self, direction, n)
    }

    #[inline]
    pub fn cursor_move_to(&mut self, column: u16, row: u16) -> Result<()> {
        Cursor::move_to(self, column, row)
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
        if self.config.alternate_screen {
            if let Err(err) = self.execute(terminal::LeaveAlternateScreen) {
                eprintln!("failed to leave alternate screen: {err}")
            };
        }

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

    #[inline]
    pub fn move_to(terminal: &mut Terminal, column: u16, row: u16) -> Result<()> {
        terminal.execute(cursor::MoveTo(column, row))
    }
}
