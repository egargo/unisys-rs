use color_eyre::eyre::Result;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs::read_to_string;
use toml::from_str;

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct DBConfig {
    pub dbms: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct EmailConfig {
    pub name: String,
    pub user: String,
    pub pass: String,
    pub smtp: String,
}

#[derive(Debug, Deserialize)]
pub struct VolumesConfig {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api: ApiConfig,
    pub db: DBConfig,
    pub email: EmailConfig,
    pub volumes: VolumesConfig,
}

impl Config {
    fn file(filename: &str) -> Result<Self> {
        Ok(from_str(&read_to_string(filename)?)?)
    }
}

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Config = Config::file("Config.toml").unwrap();
}
