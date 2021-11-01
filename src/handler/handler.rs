use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::prelude::Ready;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot ready with username {}", ready.user.name);
    }
    
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.name == "kuso-quiz" {
            return;
        }
        tokio::spawn(||{
            
        });
        if msg.content == "start" {}
    }
}
