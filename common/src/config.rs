use serde::Deserialize;
use figment::{Figment, providers::{Format, Toml, Env}};
use crate::file::project_root;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub cookie: String,
}

pub fn get_config() -> Result<Config, figment::Error> {
    let config: Config = Figment::new()
        .merge(Toml::file(project_root().join("config.toml")))
        .merge(Env::prefixed("AOC_"))
        .extract()?;

    Ok(config)
}