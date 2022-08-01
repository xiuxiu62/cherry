#![allow(dead_code)]

mod config;
mod editor;
mod error;
mod frame_buffer;
mod keymap;
mod terminal;

use std::path::PathBuf;

pub use config::Config;
use editor::Editor;
use error::{Error, Result, SerdeError};
use frame_buffer::FrameBuffer;
pub use keymap::CHAR_MAP;
use terminal::Terminal;

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

const DEFAULT_CONFIG: &str = "config.ron";

fn main() -> Result<()> {
    let mut app = App::new()?;

    app.run()
}

struct App(Editor);

impl App {
    pub fn new() -> Result<Self> {
        let config = Self::load_config(DEFAULT_CONFIG)?;
        let terminal = Terminal::new()?;

        let view_span = Span {
            start: 0,
            end: terminal.size()?.1 as usize,
        };
        let buffer = FrameBuffer::try_from_path(PathBuf::from("config.ron"), view_span)?;
        let mut editor = Editor::new(terminal, buffer, config);
        editor.initialize()?;

        Ok(Self(editor))
    }

    pub fn run(&mut self) -> Result<()> {
        self.0.run()
    }

    fn load_config(path: &str) -> Result<Config> {
        let contents = std::fs::read_to_string(path)?;

        match ron::from_str(&contents) {
            Ok(config) => Ok(config),
            Err(err) => Err(Error::Serde(SerdeError::Deserialize(err.to_string()))),
        }
    }
}
