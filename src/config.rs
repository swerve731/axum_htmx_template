use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub database: DatabaseConfig,
    pub app: AppConfig,
    pub mailer: MailerConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub origin: String,
    pub bind_address: String,
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MailerConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub sender_email: String,
    pub sender_name: String,
}

impl ServerConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let config: ServerConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Returns a sender name like "My App - No Reply"
    pub fn full_sender_name(&self) -> String {
        format!("{} - {}", self.app.app_name, self.mailer.sender_name)
    }

}
