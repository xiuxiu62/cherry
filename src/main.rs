mod config;
mod editor;
mod error;
mod keymap;
mod terminal;

pub use config::Config;
use editor::Editor;
use error::Result;
pub use keymap::CHAR_MAP;
use terminal::Terminal;

fn main() -> Result<()> {
    let config = Config::new(None, None, None, true, true);
    let terminal = Terminal::new();
    let mut editor = Editor::new(terminal);

    editor.initialize(&config)?;
    editor.run()?;

    Ok(())
}
