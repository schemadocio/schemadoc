// use serde_json::Error as JsonError;
use std::io::Error as IoError;
use thiserror::Error;

/// errors that openapi functions may return
#[derive(Error, Debug)]
pub enum Error {
    #[error("Source schema JSON serialization error")]
    InvalidSourceSchema,
    #[error("Target schema JSON serialization error")]
    InvalidTargetSchema,

    #[error("I/O error")]
    Io(#[from] IoError),
}
