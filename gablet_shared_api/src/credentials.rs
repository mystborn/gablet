use serde::Deserialize;
use config::{File, Config, ConfigError};

#[derive(Debug, Clone, Deserialize)]
pub struct App {
    pub host: String,
    pub port: u16
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logging {
    file: Option<String>,
    host: Option<String>
}

#[derive(Debug, Clone, Deserialize)]
pub struct Postgres {
    pub username: String,
    pub password: String,
    pub db: String,
    pub host: String,
    pub port: u16
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthCredentials {
    pub access_secret: String,
    pub refresh_secret: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mail {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16
}

#[derive(Debug, Clone, Deserialize)]
pub struct Kafka {
    pub hosts: Vec<String>,
    pub topics: Vec<String>,
    pub group: String
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Credentials {
    pub postgres: Option<Postgres>,
    pub auth: Option<AuthCredentials>,
    pub mail: Option<Mail>,
    pub kafka: Option<Kafka>,
    pub logs: Option<Logging>
}

impl Credentials {
    pub fn new(fname: &str) -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name(fname))
            .build()?
            .try_deserialize()
    }
}