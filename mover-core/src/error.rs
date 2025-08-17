//! Error types for the mover library

use thiserror::Error;

/// Main error type for the mover library
#[derive(Error, Debug)]
pub enum MoverError {
    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),
    
    #[error("Invalid coordinates: ({0}, {1})")]
    InvalidCoordinates(i32, i32),
    
    #[error("Invalid mouse button: {0}")]
    InvalidMouseButton(String),
    
    #[error("Screen capture failed: {0}")]
    ScreenCaptureFailed(String),
    
    #[error("Image not found: {0}")]
    ImageNotFound(String),
    
    #[error("Failsafe triggered: {0}")]
    FailsafeTriggered(String),
    
    #[error("Platform error: {0}")]
    PlatformError(#[from] PlatformError),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Other error: {0}")]
    Other(String),
}

impl From<String> for MoverError {
    fn from(s: String) -> Self {
        MoverError::Other(s)
    }
}

impl From<&str> for MoverError {
    fn from(s: &str) -> Self {
        MoverError::Other(s.to_string())
    }
}

/// Platform-specific errors
#[derive(Error, Debug)]
pub enum PlatformError {
    #[error("Windows API error: {0}")]
    Windows(String),
    
    #[error("macOS Core Graphics error: {0}")]
    MacOS(String),
    
    #[error("Linux X11 error: {0}")]
    Linux(String),
    
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
}

/// Result type for mover operations
pub type MoverResult<T> = Result<T, MoverError>;
