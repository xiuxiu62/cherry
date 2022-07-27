use std::fmt::Display;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Serde(#[from] SerdeError),
}

#[derive(Debug, Error)]
pub enum SerdeError {
    #[error("{0}")]
    Serialize(String),
    #[error("{0}")]
    Deserialize(String),
}

impl serde::ser::Error for SerdeError {
    fn custom<T: Display>(msg: T) -> Self {
        SerdeError::Serialize(format!("Failed to serialize: {}", msg))
    }
}

impl serde::de::Error for SerdeError {
    fn custom<T: Display>(msg: T) -> Self {
        SerdeError::Deserialize(format!("Failed to deserialize: {}", msg))
    }
}
