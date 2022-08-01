mod config;
mod editor;
mod error;
mod frame_buffer;
mod keymap;
mod terminal;

use std::path::PathBuf;

pub use config::Config;
use crossterm::event;
use editor::Editor;
use error::{Error, Result, SerdeError};
use frame_buffer::FrameBuffer;
pub use keymap::CHAR_MAP;
use terminal::Terminal;

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

const DEFAULT_CONFIG: &'static str = "config.ron";

// fn main_bak() -> Result<()> {
//     let config = load_config(DEFAULT_CONFIG)?;
//     let mut terminal = Terminal::new()?;
//     terminal.initialize(&config)?;

//     terminal.cursor_reset()?;
//     terminal.write("hello world")?;

//     loop {
//         let event = event::read()?;
//         if let crossterm::event::Event::Key(crossterm::event::KeyEvent {
//             // crossterm::event::KeyCode::Char('c'),
//             // crossterm::event::KeyMod
//             code,
//             modifiers,
//         }) = event
//         {
//             if modifiers.contains(crossterm::event::KeyModifiers::CONTROL)
//                 && code == crossterm::event::KeyCode::Char('c')
//             {
//                 break;
//             }
//         }
//     }

//     Ok(())
// }

fn main() -> Result<()> {
    let mut editor = initialize()?;
    editor.run()?;

    Ok(())
}

fn initialize() -> Result<Editor> {
    let config = load_config(DEFAULT_CONFIG)?;
    let terminal = Terminal::new()?;

    let view_span = Span {
        start: 0,
        end: terminal.size()?.1 as usize,
    };
    let buffer = FrameBuffer::try_from_path(PathBuf::from("config.ron"), view_span)?;
    let mut editor = Editor::new(terminal, buffer);
    editor.initialize(&config)?;

    Ok(editor)
}

// fn main_bak() -> Result<()> {
//     let config = load_config(DEFAULT_CONFIG)?;
//     let terminal = Terminal::new()?;

//     let size = terminal.size()?;
//     let span = Span {
//         start: 0,
//         end: size.1 as usize,
//     };
//     let text_buffer = TextBuffer::new(vec![]);
//     let frame_buffer = FrameBuffer::new(text_buffer, span);
//     let mut editor = Editor::new(terminal, frame_buffer);

//     editor.initialize()?;
//     editor.run()?;

//     Ok(())
// }

fn load_config(path: &str) -> Result<Config> {
    let contents = std::fs::read_to_string(path)?;

    match ron::from_str(&contents) {
        Ok(config) => Ok(config),
        Err(err) => Err(Error::Serde(SerdeError::Deserialize(err.to_string()))),
    }
}
