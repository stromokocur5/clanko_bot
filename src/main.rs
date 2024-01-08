use anyhow::Result;
use clanko_bot::{config_from_file, discord};
use clanko_zbierac::medium::MediumClient;
use clanko_zbierac::MediaConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config_from_file()?;
    let medium_client = MediumClient::new(config.media.clone().unwrap_or_default()).await;
    discord::Discord::from(config.clone())
        .start(medium_client)
        .await?;
    Ok(())
}
