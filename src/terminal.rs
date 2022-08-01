use crate::{error::Result, Config};
use crossterm::{
    cursor, event, style,
    terminal::{self, ClearType},
    Command, ExecutableCommand,
};
use std::{
    fmt::Display,
    io::{self, Stdout},
};

pub enum Move {
    Left(u16),
    Right(u16),
    Up(u16),
    Down(u16),
    NextLine(u16),
    PreviousLine(u16),
    To(u16, u16),
}

pub struct Terminal {
    stdout: Stdout,
    // config: Rc<Config>,
    pub size: (u16, u16),
    alternate_screen: bool,
}

impl Terminal {
    #[inline]
    pub fn new() -> Result<Self> {
        Ok(Self {
            stdout: io::stdout(),
            size: terminal::size()?,
            alternate_screen: false,
        })
    }

    pub fn initialize(&mut self, config: &Config) -> Result<()> {
        self.enable_raw_mode()?;
        self.initialize_terminal(config)?;

        self.initialize_theme(config)
    }

    fn initialize_terminal(&mut self, config: &Config) -> Result<()> {
        if config.alternate_screen {
            self.alternate_screen = true;
            self.execute(terminal::EnterAlternateScreen)?;
        } else {
            self.execute(terminal::Clear(ClearType::All))?;
        }

        if config.line_wrapping {
            self.execute(terminal::EnableLineWrap)?;
        }

        if config.mouse_capture {
            self.execute(event::EnableMouseCapture)?;
        }

        Ok(())
    }

    fn initialize_theme(&mut self, config: &Config) -> Result<()> {
        if let Some(color) = config.theme.foreground_color {
            self.execute(style::SetForegroundColor(color.into()))?;
        }

        if let Some(color) = config.theme.background_color {
            self.execute(style::SetBackgroundColor(color.into()))?;
        }

        if let Some(color) = config.theme.underline_color {
            self.execute(style::SetUnderlineColor(color.into()))?;
        }

        Ok(())
    }

    #[inline]
    pub fn write<T: Display>(&mut self, data: T) -> Result<()> {
        self.execute(style::Print(data))
    }

    pub fn clear(&mut self) -> Result<()> {
        self.execute(terminal::Clear(ClearType::All))
    }

    #[inline]
    pub fn cursor_move(&mut self, move_: Move) -> Result<()> {
        Cursor::move_(self, move_)
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
        if self.alternate_screen {
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
    pub fn move_(terminal: &mut Terminal, move_: Move) -> Result<()> {
        match move_ {
            Move::Left(n) => terminal.execute(cursor::MoveLeft(n)),
            Move::Right(n) => terminal.execute(cursor::MoveRight(n)),
            Move::Up(n) => terminal.execute(cursor::MoveUp(n)),
            Move::Down(n) => terminal.execute(cursor::MoveDown(n)),
            Move::NextLine(n) => terminal.execute(cursor::MoveToNextLine(n)),
            Move::PreviousLine(n) => terminal.execute(cursor::MoveToPreviousLine(n)),
            Move::To(column, row) => terminal.execute(cursor::MoveTo(column, row)),
        }
    }

    #[inline]
    pub fn move_to(terminal: &mut Terminal, column: u16, row: u16) -> Result<()> {
        terminal.execute(cursor::MoveTo(column, row))
    }
}
