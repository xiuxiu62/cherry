// use crate::text_buffer::TextBufferError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Crossterm(#[from] crossterm::ErrorKind),
    // #[error(transparent)]
    // TextBuffer(#[from] TextBufferError),
}
