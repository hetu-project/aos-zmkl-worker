use serde::Deserialize;
use std::path::Path;
use std::path::PathBuf;

use crate::error::{ZKMLResult,ZKMLError};

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
        pub host: String,
        pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
        pub user: String,
        pub password: String,
}


#[derive(Clone, Debug, Deserialize)]
pub struct Config {
        pub server: ServerConfig,
        pub database: DatabaseConfig,
}

impl Config {
    pub fn load_config(path: PathBuf) -> ZKMLResult<Config> {
        let p: &Path = path.as_ref();
        let config_yaml = std::fs::read_to_string(p).map_err(|err| match err {
            e @ std::io::Error { .. } if e.kind() == std::io::ErrorKind::NotFound => {
                ZKMLError::ConfigMissing(path)
            }
            _ => err.into(),
        })?;

        let config: Config =
            serde_yaml::from_str(&config_yaml).map_err(ZKMLError::SerializationError)?;
        Ok(config)
    }
}

