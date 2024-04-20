use config::{Config, ConfigError, File};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppSettings {
    pub port: u16,
    pub db: DatabaseSettings,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub name: String,
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
}

impl DatabaseSettings {
    pub fn get_connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        ))
    }

    pub fn get_test_connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

pub fn get_config() -> Result<AppSettings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("config"))
        .build()
        .unwrap();

    settings.try_deserialize::<AppSettings>()
}
