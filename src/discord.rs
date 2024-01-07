use crate::{Config, Result};
use anyhow::anyhow;
use clanko_zbierac::medium::MediumClient;
use serenity::async_trait;
use serenity::builder;
use serenity::builder::CreateAttachment;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

#[derive(serde::Deserialize, Clone)]
pub struct Discord {
    pub token: String,
}

impl Discord {
    pub async fn start(&self, medium_client: MediumClient) -> Result<()> {
        let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
        let mut client = Client::builder(self.token.to_string(), intents)
            .event_handler(Handler {
                client: medium_client,
            })
            .await
            .map_err(|_| anyhow!("Err creating client"))?;

        if let Err(why) = client.start().await {
            println!("Client error: {why:?}");
        }
        Ok(())
    }
}

impl From<Config> for Discord {
    fn from(config: Config) -> Self {
        let discord = config.bot.discord.unwrap_or_else(|| Self {
            token: "".to_string(),
        });
        discord
    }
}

struct Handler {
    client: MediumClient,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let message = msg.content.split(" ").collect::<Vec<&str>>();
        if message[0] == "!clanok" {
            let url = reqwest::Url::parse(message[1]);
            if let Err(why) = url {
                println!("Something went wrong: {}", why);
                return;
            }
            let url = url.unwrap();
            let temp = self.client.get_article(&url).await;
            if let Err(why) = temp {
                println!("Something went wrong: {}", why);
                return;
            }
            let (article, title) = temp.unwrap();
            if let Err(why) = clanko_zbierac::markdown_to_pdf(&article, &title) {
                println!("Something went wrong: {}", why);
                return;
            }
            let attachment = CreateAttachment::path(format!("{title}.pdf")).await;
            if let Err(why) = attachment {
                println!("Something went wrong: {}", why);
                return;
            }
            let attachment = attachment.unwrap();
            let msg_builder = builder::CreateMessage::new().add_file(attachment);
            let channel = msg.channel(ctx.http()).await;

            if let Err(why) = channel {
                println!("Something went wrong: {}", why);
                return;
            }
            let channel = channel
                .unwrap()
                .id()
                .send_message(ctx.http(), msg_builder)
                .await;
            if let Err(why) = channel {
                println!("Something went wrong: {}", why);
                return;
            }
            if let Err(why) = std::fs::remove_file(format!("{title}.pdf")) {
                println!("Something went wrong: {}", why);
                return;
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
