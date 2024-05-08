use config::{Config, ConfigError, Environment, File};
use dotenv::dotenv;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::domain::SubscriberEmail;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub app: AppSettings,
    pub db: DatabaseSettings,
    pub email_client: EmailClientSettings,
}

#[derive(Deserialize, Debug)]
pub struct AppSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct EmailClientSettings {
    pub base_url: String,
    pub sender_email: String,
}

impl EmailClientSettings {
    pub fn get_sender(&self) -> Result<SubscriberEmail, String> {
        SubscriberEmail::parse(self.sender_email.clone())
    }
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub name: String,
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
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

pub enum Env {
    Dev,
    Prod,
}

pub fn get_config() -> Result<Settings, ConfigError> {
    dotenv().ok();
    let base_path = std::env::current_dir().expect("unable to get current directory");
    let config_dir = base_path.join("config");

    let default_env = Env::Dev.try_into().unwrap();
    let env: Env = std::env::var("APP_ENV")
        .unwrap_or_else(|_| default_env)
        .try_into()
        .expect("Failed to parse APP_ENV");

    let settings = Config::builder()
        .add_source(File::from(config_dir.join("base")))
        .add_source(File::from(config_dir.join(env.as_str())).required(true))
        .add_source(Environment::default().separator("_"))
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl TryFrom<String> for Env {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            unknown => Err(format!(
                "{} is not a supported environment. Use either `dev` or `prod`.",
                unknown
            )),
        }
    }
}

impl TryInto<String> for Env {
    type Error = String;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Env::Dev => Ok("dev".to_string()),
            Env::Prod => Ok("prod".to_string()),
        }
    }
}

impl Env {
    pub fn as_str(self) -> &'static str {
        match self {
            Env::Dev => "dev",
            Env::Prod => "prod",
        }
    }
}
