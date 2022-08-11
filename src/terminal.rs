use crate::{editor::Move, error::Result, Config};
use crossterm::{
    cursor, event, style,
    terminal::{self, ClearType},
    Command, ExecutableCommand,
};
use std::{
    cell::RefCell,
    fmt::Display,
    io::{self, Stdout},
    rc::Rc,
};

#[derive(Debug)]
pub struct Terminal {
    stdout: Stdout,
    config: Config,
    pub size: Rc<RefCell<(u16, u16)>>,
}

impl Terminal {
    #[inline]
    pub fn new(config: Config) -> Result<Self> {
        let terminal = Self {
            stdout: io::stdout(),
            config,
            size: Rc::new(RefCell::new(terminal::size()?)),
        };

        Ok(terminal)
    }

    pub fn initialize(&mut self, start_position: (usize, usize)) -> Result<()> {
        self.enable_raw_mode()?;
        self.initialize_terminal()?;
        self.initialize_theme()?;
        self.cursor_move_to(start_position)?;

        Ok(())
    }

    fn initialize_terminal(&mut self) -> Result<()> {
        if self.config.alternate_screen {
            self.execute(terminal::EnterAlternateScreen)?;
        } else {
            self.clear()?;
        }

        if self.config.line_wrapping {
            self.execute(terminal::EnableLineWrap)?;
        }

        if self.config.mouse_capture {
            self.execute(event::EnableMouseCapture)?;
        }

        Ok(())
    }

    fn initialize_theme(&mut self) -> Result<()> {
        if let Some(color) = self.config.theme.foreground_color {
            self.execute(style::SetForegroundColor(color.into()))?;
        }

        if let Some(color) = self.config.theme.background_color {
            self.execute(style::SetBackgroundColor(color.into()))?;
        }

        if let Some(color) = self.config.theme.underline_color {
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
    pub fn cursor_move_to(&mut self, (column, row): (usize, usize)) -> Result<()> {
        Cursor::move_to(self, column as u16, row as u16)
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

        if self.config.line_wrapping {
            if let Err(err) = self.execute(terminal::DisableLineWrap) {
                eprintln!("failed to disable line wrapping: {err}")
            }
        }

        if self.config.mouse_capture {
            if let Err(err) = self.execute(event::DisableMouseCapture) {
                eprintln!("failed to disable mouse capture: {err}")
            }
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
