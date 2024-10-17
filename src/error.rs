use std::path::PathBuf;
use thiserror::Error;

pub type ZKMLResult<T> = Result<T, ZKMLError>;

#[derive(Error, Debug)]
pub enum ZKMLError {
    #[error( "No operator config found at this path: {0}")]
    ConfigMissing(PathBuf),

    #[error("Config deserialization error: {0}")]
    SerializationError(#[from] serde_yaml::Error),

    #[error("Error while performing IO for the Operator: {0}")]
    IoError(#[from] std::io::Error),

}


