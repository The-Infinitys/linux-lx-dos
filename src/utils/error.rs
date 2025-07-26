use thiserror::Error;

#[derive(Debug, Error)]
pub enum LxDosError {
    #[error("IO-Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Message(String),
    #[error("{0}")]
    Qt6(#[from] qt6::Qt6Error),
    #[error("{0}")]
    Crossbeam(#[from] crossbeam_channel::RecvError),
}
