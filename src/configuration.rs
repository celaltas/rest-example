use config::{Config, File, FileFormat};

#[derive(serde::Deserialize, Debug)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}
#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub namespace: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut builder = Config::builder();
    builder = builder.add_source(File::new("configuration", FileFormat::Yaml));

    match builder.build() {
        Ok(config) => config.try_deserialize::<Settings>(),
        Err(e) => Err(e),
    }
}
