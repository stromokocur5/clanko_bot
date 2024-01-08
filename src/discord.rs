use crate::{Config, Result};
use anyhow::anyhow;
use clanko_zbierac::medium::MediumClient;
use serenity::async_trait;
use serenity::builder;
use serenity::builder::CreateAttachment;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::sync::Arc;

#[derive(serde::Deserialize, Clone)]
pub struct Discord {
    pub token: String,
}

struct Data;

impl TypeMapKey for Data {
    type Value = Arc<RwLock<MediumClient>>;
}

impl Discord {
    pub async fn start(&self, medium_client: MediumClient) -> Result<()> {
        let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
        let mut client = Client::builder(self.token.to_string(), intents)
            .event_handler(Handler)
            .await
            .map_err(|_| anyhow!("Err creating client"))?;

        {
            let mut data = client.data.write().await;

            data.insert::<Data>(Arc::new(RwLock::new(medium_client)));
        }

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

struct Handler;
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
            let data = ctx.data.write().await;
            let data = data.get::<Data>();
            if let None = data {
                println!("Something went wrong",);
                return;
            }
            let mut article = data.unwrap().write().await;
            let article = { article.get_article(&url).await };
            if let None = data {
                println!("Something went wrong",);
                return;
            }
            let (article, title) = article.unwrap();
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
