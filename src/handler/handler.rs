use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::prelude::Ready;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot ready with username {}", ready.user.name);
    }
}
