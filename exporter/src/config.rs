use anyhow::anyhow;
use config::{ConfigBuilder, ConfigError};
use config::builder::DefaultState;
use serde::{Serialize, Deserialize};
use crate::error::Result;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub log_level : String,
    pub api_port : u16,
    pub api_listen_address : String,
    pub metrics_port : u16,
    pub metrics_listen_address : String,
    pub eloverblik_refresh_token : String,
}

pub fn get_conf_path() -> String {
    std::env::var("ELOVERBLIK_EXPORTER_DATA_DIR").unwrap_or_else(|_| {"./".to_owned()})
}

pub fn load_conf() -> Result<Config> {
    let mut settings = config::Config::builder()
        .add_source(config::Environment::with_prefix("ELOVERBLIK_EXPORTER").separator("__"))
        .add_source(config::File::with_name(format!("{}/{}", get_conf_path(), "config.yaml").as_str()).required(false));
    settings = set_defaults(settings);
    let settings_built = settings
        .build()
        .unwrap();

    let config : Config = settings_built.try_deserialize()?;

    Ok(config)
}

fn set_defaults(builder : ConfigBuilder<DefaultState>) -> ConfigBuilder<DefaultState> {
    let builder = builder
        .set_default("api_port", 8080).unwrap()
        .set_default("api_listen_address", "0.0.0.0").unwrap()
        .set_default("metrics_port", 9000).unwrap()
        .set_default("metrics_listen_address", "0.0.0.0").unwrap()
        .set_default("log_level", "info").unwrap();

    builder
}