use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, prelude::Ready},
};
use std::borrow::Cow;
use tokio::sync::mpsc;

use crate::bot::BotState;
use crate::quiz::Quiz;

pub struct Handler;

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
                            let mut quiz = Quiz::new();
                            let mut quiz_iter = quiz.iter_mut();
                            let _ = msg
                                .channel_id
                                .say(&ctx.http, &quiz_iter.next().unwrap().content)
                                .await;
                            loop {
                                if let Some(user_message) = rx.recv().await {
                                    let response = quiz_iter.check(&user_message);
                                    let _ = msg.channel_id.say(&ctx.http, response).await;
                                    if let Some(next) = quiz_iter.next() {
                                        let _ = msg.channel_id.say(&ctx.http, &next.content).await;
                                    } else {
                                        let (num, ans) = quiz_iter.summary_result();
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
