use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub app: App,
    pub database: Database,
    pub logging: Logging,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct App {
    pub name: String,
    pub host: String,
    pub port: u16,
    #[serde(default = "default_workers")]
    pub workers: usize,
}

fn default_workers() -> usize {
    num_cpus::get() * 2
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Logging {
    pub level: String,
    pub format: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config.yaml"))
            .build()?;

        config.try_deserialize()
    }
}
