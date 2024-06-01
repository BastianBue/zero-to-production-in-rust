use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::PgConnectOptions;

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_string(&self) -> &'static str {
        match self {
            Environment::Local => "dev",
            Environment::Production => "prod",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "dev" => Ok(Environment::Local),
            "prod" => Ok(Environment::Production),
            _ => Err(format!("{} is not a valid environment", value)),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_settings: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

impl DatabaseSettings {
    pub fn connect_option(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .username(&self.username)
            .password(self.password.expose_secret())
            .host(&self.host)
            .port(self.port)
            .database(&self.db_name)
    }

    pub fn connect_options_root(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .username(&self.username)
            .password(self.password.expose_secret())
            .host(&self.host)
            .port(self.port)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let environment: Environment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "dev".into())
        .try_into()
        .expect("Failed to parse APP_ENV");

    let file_name = format!(
        "configuration/{}-configuration.json",
        environment.as_string()
    );

    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration/base-configuration.json",
            config::FileFormat::Json,
        ))
        .add_source(config::File::new(&file_name, config::FileFormat::Json))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;
    settings.try_deserialize::<Settings>()
}
