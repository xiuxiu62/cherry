#![allow(dead_code)]

mod config;
mod editor;
pub mod error;
mod frame_buffer;
mod keymap;
mod status_bar;
mod terminal;
mod util;

pub use config::Config;
pub use editor::Editor;
pub use frame_buffer::FrameBuffer;
pub(crate) use keymap::CHAR_MAP;
pub use status_bar::StatusBar;
pub use terminal::Terminal;

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);
