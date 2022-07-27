mod config;
mod editor;
mod error;
mod keymap;
mod terminal;
mod text_buffer;

use std::rc::Rc;

pub use config::Config;
use editor::Editor;
use error::{Error, Result, SerdeError};
pub use keymap::CHAR_MAP;
use terminal::Terminal;
use text_buffer::TextBuffer;

const DEFAULT_CONFIG: &'static str = "config.toml";

fn main() -> Result<()> {
    let config = Rc::new(load_config(DEFAULT_CONFIG)?);
    // let config = Rc::new(Config::new(None, None, None, false, false, true));
    let terminal = Terminal::new(config)?;
    let buffer = TextBuffer::default();
    let mut editor = Editor::new(terminal, buffer);

    editor.initialize()?;
    editor.run()?;

    Ok(())
}

fn load_config(path: &str) -> Result<Config> {
    let contents = std::fs::read_to_string(path)?;

    match toml::from_str(&contents) {
        Ok(config) => Ok(config),
        Err(err) => Err(Error::Serde(SerdeError::Deserialize(err.to_string()))),
    }
}
