use crate::file::project_root;
use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub cookie: String,
}

pub fn get_config() -> Result<Config, Box<figment::Error>> {
    let config: Config = Figment::new()
        .merge(Toml::file(project_root().join("config.toml")))
        .merge(Env::prefixed("AOC_"))
        .extract()?;

    Ok(config)
}
