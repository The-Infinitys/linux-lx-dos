#[derive(Debug, thiserror::Error)]
pub enum SystemTrayError {
    #[error("Failed to send event")]
    SendError,
}
