use anyhow::Result;
use clanko_zbierac::MediaConfig;
pub mod discord;
pub mod slack;

#[derive(serde::Deserialize)]
pub struct Config {
    pub bot: BotConfig,
    pub media: MediaConfig,
}

#[derive(serde::Deserialize)]
pub struct BotConfig {
    pub discord: discord::Discord,
    pub slack: slack::Slack,
}

pub fn config_from_file() -> Result<Config> {
    let config = std::fs::read_to_string("config.toml").expect("ziaden subor config.toml");
    let config: Config = toml::from_str(&config)?;
    Ok(config)
}
