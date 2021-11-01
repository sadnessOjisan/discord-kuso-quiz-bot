use std::collections::HashMap;

use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::futures::channel::mpsc::{self, Sender};
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::model::prelude::Ready;

use crate::bot::BotState;

pub struct Handler {
    channel_sender_pair: HashMap<ChannelId, Sender<String>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot ready with username {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.name == "kuso-quiz" {
            return;
        }
        if msg.content == "start" {
            let channel_id = msg.channel_id;
            let mut rw_lock= ctx.data.write().await;
            let mut bot_state = rw_lock
                .get_mut::<BotState>()
                .expect("Failed to retrieve map!")
                .lock()
                .await;
            let channel_sender_pair = bot_state.channel_sender_pair.get(&channel_id);
            if channel_sender_pair.is_some() {
                // すでに登録済み
                let sender = channel_sender_pair.unwrap();
                // TODO: send message
            } else {
                // まだ登録していない
                let (tx, rx) = mpsc::channel(32);
                bot_state.channel_sender_pair.insert(channel_id, tx);
                tokio::spawn(async move {
                    loop {
                        // TODO: receive message
                    }
                });
            };
        }
    }
}
