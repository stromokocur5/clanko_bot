use anyhow::{anyhow, Result};
use clanko_bot::config_from_file;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config_from_file()?;
    Ok(())
}
