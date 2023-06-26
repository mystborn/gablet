use config::{File, Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Postgres {
    pub username: String,
    pub password: String,
    pub db: String,
    pub host: String,
    pub port: u16
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mail {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub postgres: Postgres,
    pub mail: Mail
}

const CONFIG_FILE_PATH: &str = "./config/credentials.toml";

impl Credentials {
    pub fn new() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .build()?
            .try_deserialize()
    }
}