use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, id::ChannelId, prelude::Ready},
};
use std::{borrow::Cow, collections::HashMap};
use tokio::sync::mpsc;

use crate::bot::BotState;
use crate::quiz::State;

#[derive(Default)]
pub struct Handler {
    pub channel_sender_pair: HashMap<ChannelId, mpsc::Sender<String>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        let channel_id = msg.channel_id;
        let mut rw_lock = ctx.data.write().await;
        let mut bot_state = rw_lock
            .get_mut::<BotState>()
            .expect("Failed to retrieve map!")
            .lock()
            .await;

        match &*msg.content {
            "start" => {
                bot_state
                    .channel_sender_pair
                    .entry(channel_id)
                    .or_insert_with(|| {
                        let (tx, mut rx) = mpsc::channel::<Cow<'static, str>>(32);

                        // global 環境に sender を登録し、外側の環境から msg 待ち受けloopにユーザーの回答を送る
                        tokio::spawn(async move {
                            let mut state = State::new();
                            let question = state.next_question().unwrap();
                            let _ = msg.channel_id.say(&ctx.http, &question.content).await;
                            loop {
                                if let Some(user_message) = rx.recv().await {
                                    let response = state.check_user_answer(&user_message);
                                    let _ = msg.channel_id.say(&ctx.http, response).await;
                                    if let Some(next) = state.next_question() {
                                        let _ = msg.channel_id.say(&ctx.http, &next.content).await;
                                    } else {
                                        let (num, ans) = state.summary_result();
                                        let response = format!("{}問中{}問正解です。", num, ans);
                                        let _ = msg.channel_id.say(&ctx.http, response).await;
                                    }
                                }
                            }
                        });

                        tx
                    });
            }
            _ => {
                if let Some(sender) = bot_state.channel_sender_pair.get_mut(&channel_id) {
                    let _ = sender.send(msg.content.into()).await;
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot ready with username {}", ready.user.name);
    }
}
