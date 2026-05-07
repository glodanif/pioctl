#[derive(Debug, thiserror::Error, PartialEq)]
pub enum DesktopError {
    #[error("Failed to execute command {0}")]
    CommandExecutionError(String),
}
