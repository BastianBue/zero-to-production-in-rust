use secrecy::{ExposeSecret, Secret};

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
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.db_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
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
        .build()?;
    settings.try_deserialize::<Settings>()
}
