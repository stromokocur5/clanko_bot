use anyhow::Result;
use clanko_zbierac::MediaConfig;
pub mod discord;
pub mod slack;

#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub bot: BotConfig,
    pub media: Option<MediaConfig>,
}

#[derive(serde::Deserialize, Clone)]
pub struct BotConfig {
    pub discord: Option<discord::Discord>,
    pub slack: Option<slack::Slack>,
}

pub fn config_from_file() -> Result<Config> {
    let config = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config)?;
    Ok(config)
}
