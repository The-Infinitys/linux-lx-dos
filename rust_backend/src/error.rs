#[derive(Debug)]
pub enum QemuError {
    QemuBinaryNotFound(String),
    IoError(std::io::Error),
}

impl From<std::io::Error> for QemuError {
    fn from(err: std::io::Error) -> Self {
        QemuError::IoError(err)
    }
}
