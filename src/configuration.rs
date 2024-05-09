use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub application_port: u16,
    pub database: DatabaseSettings,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        return format!(
            "postgres://{}:{}@{}:{}/{}",
            self.host, self.port, self.username, self.password, self.db_name
        );
    }
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()?;
    return settings.try_deserialize();
}
