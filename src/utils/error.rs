use thiserror::Error;

#[derive(Debug, Error)]
pub enum LxDosError {
    #[error("IO-Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Message(String),
}
