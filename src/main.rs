mod config;
mod editor;
mod error;
mod keymap;
mod terminal;
mod text_buffer;

pub use config::Config;
use editor::Editor;
use error::Result;
pub use keymap::CHAR_MAP;
use terminal::Terminal;
use text_buffer::TextBuffer;

fn main() -> Result<()> {
    let config = Config::new(None, None, None, true, true);
    let terminal = Terminal::new()?;
    let buffer = TextBuffer::default();
    let mut editor = Editor::new(terminal, buffer);

    editor.initialize(&config)?;
    editor.run()?;

    Ok(())
}
