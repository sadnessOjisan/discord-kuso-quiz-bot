use std::collections::HashMap;

use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::futures::channel::mpsc::Sender;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::model::prelude::Ready;
use tokio::sync::mpsc;

use crate::bot::BotState;
use crate::quiz::State;

pub struct Handler {
    pub channel_sender_pair: HashMap<ChannelId, Sender<String>>,
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
        let channel_id = msg.channel_id;
        let mut rw_lock = ctx.data.write().await;
        let mut bot_state = rw_lock
            .get_mut::<BotState>()
            .expect("Failed to retrieve map!")
            .lock()
            .await;
        let target_sender = bot_state.channel_sender_pair.get_mut(&channel_id);

        if msg.content == "start" {
            if target_sender.is_some() {
                // すでに登録済み
                let sender = target_sender.unwrap();
                let _ = sender.send("hello from sender".to_string()).await;
            } else {
                // まだ登録していない
                let (tx, mut rx) = mpsc::channel(32);
                bot_state.channel_sender_pair.insert(channel_id, tx);
                tokio::spawn(async move {
                    let mut state = State::new();
                    let question = state.get_current_question();
                    let _ = msg.channel_id.say(&ctx.http, &question.content).await;
                    loop {
                        let user_message = rx.recv().await.expect("fail to receive message");
                        let is_correct = state.check_user_answer(&user_message);
                        if is_correct {
                            let _ = msg.channel_id.say(&ctx.http, "正解です。").await;
                        } else {
                            let _ = msg.channel_id.say(&ctx.http, "不正解です。").await;
                        }
                        if state.is_last() {
                            let result_summary = state.summary_result();
                            let result_message =
                                format!("{}問中{}問正解です。", result_summary.0, result_summary.1);
                            let _ = msg.channel_id.say(&ctx.http, result_message).await;
                        } else {
                            state.next_question()
                        }
                    }
                });
            };
        } else {
            // TODO: 初期化前にこっちのブロックに来たら追い返す
            let sender = target_sender.unwrap();
            let _ = sender.send(msg.content).await;
        }
    }
}
