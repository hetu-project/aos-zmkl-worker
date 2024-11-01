use std::path::PathBuf;
use thiserror::Error;

pub type ZKMLResult<T> = Result<T, ZKMLError>;

#[repr(u16)]
#[derive(Debug)]
pub enum ErrorCodes {
    ConfigMissing = 1001,
    SerializationError = 1002,
    IoError = 1003,
    OtherError = 1004,
}

#[derive(Error, Debug)]
pub enum ZKMLError {
    #[error("No operator config found at this path: {0}")]
    ConfigMissing(PathBuf),

    #[error("Config deserialization error: {0}")]
    SerializationError(#[from] serde_yaml::Error),

    #[error("Error while performing IO for the Operator: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Other Error for the Operator: {0}")]
    OtherError(String),
}

impl ZKMLError {
    pub fn error_code(&self) -> u16 {
        match self {
            ZKMLError::ConfigMissing(_) => ErrorCodes::ConfigMissing as u16,
            ZKMLError::SerializationError(_) => ErrorCodes::SerializationError as u16,
            ZKMLError::IoError(_) => ErrorCodes::IoError as u16,
            ZKMLError::OtherError(_) => ErrorCodes::OtherError as u16,
        }
    }

    pub fn error_message(&self) -> String {
        self.to_string()
    }
}
