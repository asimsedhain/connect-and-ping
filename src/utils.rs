use anyhow::{Context, Result};
use serde::Deserialize;

fn default_port() -> u16 {
    3000
}

fn default_log_level() -> String {
    "axum_dryrun=debug,tower_http=info".to_string()
}
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_log_level")]
    pub log_level: String,
}

pub fn load_env() -> Result<Config> {
    let _ = dotenvy::dotenv();
    envy::from_env::<Config>().context("Could not load env vars")
}
