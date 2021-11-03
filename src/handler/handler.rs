use std::collections::HashMap;
use std::fmt::Debug;

use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::model::prelude::Ready;
use tokio::sync::mpsc;

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
        use serenity::futures::SinkExt;
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
            let target_sender = bot_state.channel_sender_pair.get_mut(&channel_id);
            if target_sender.is_some() {
                // すでに登録済み
                let sender = target_sender.unwrap();
             let _ =  sender.send("hello from sender".to_string()).await;
                // TODO: send message
            } else {
                // まだ登録していない
                let (tx, rx) = mpsc::channel(32);
                bot_state.channel_sender_pair.insert(channel_id, tx);
                tokio::spawn(async move {
                    loop {
                        // TODO: receive message
                        let msg = rx.recv().await;
                        println!("received: {:?}", msg);
                    }
                });
            };
        }
    }
}
