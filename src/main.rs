use discord_kuso_quiz_bot::quiz::quiz::{AllBotState, BotState, Mode};
use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult, StandardFramework,
};
use serenity::futures::lock::Mutex;
use serenity::model::{channel::Message, gateway::Ready};
use serenity::prelude::*;
use std::sync::Arc;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot ready with username {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.name == "kuso-quiz" {
            return;
        }

        let channelId = msg.channel_id;
        let mut data = ctx.data.write().await;
        let mut state = data
            .get_mut::<AllBotState>()
            .expect("Failed to retrieve map!")
            .lock()
            .await;

        let current_state = state.states.get_mut(&channelId).unwrap();
        match &current_state.mode {
            Mode::WaitingUserAnswer(_) => {
                let user_answer = msg.content;
                let is_correct = current_state.user_answer(user_answer);
                // Q: current_state.next_quiz(); user_answer を next_quiz() から呼ぶようにしたのでエラーを回避できたけど、本当にこれでいいのか？
                if is_correct.is_none() {
                    msg.channel_id.say(&ctx.http, "不正な状態です。").await;
                    return;
                }
                if is_correct.unwrap() {
                    msg.channel_id.say(&ctx.http, "正解です。").await;
                } else {
                    msg.channel_id.say(&ctx.http, "不正解です。").await;
                }
                // let next_state = &mut bot_state.lock().await;
                // println!("next_state{:?}",next_state); 上でlockをとろうとするとここでコードが止まる。下にcurrent_state(=&mut bot_state.lock().await;)がいてライフタイムがあるからロックが取れないのだと思うけど、そういう競合のときってエラーとかで検知できないのか？try_lockはこういうときのためのもの？
                match &current_state.mode {
                    Mode::WaitingUserAnswer(state) => {
                        let current_quiz = &state.questions[state.cursor as usize].clone();
                        let res = msg.channel_id.say(&ctx.http, &current_quiz.content).await;
                        if res.is_err() {
                            println!("不正な状態です")
                        }
                    }
                    Mode::Finish(state) => {
                        let all_q_lens = &state.questions.len();
                        let correct_list = &state.result.len();
                        let txt = format!("{:?}問中{:?}正解です", all_q_lens, correct_list);
                        msg.channel_id.say(&ctx.http, txt).await;
                    }
                    _ => {
                        msg.channel_id.say(&ctx.http, "不正な状態です。").await;
                    }
                }
            }
            Mode::Error => {
                msg.channel_id
                    .say(&ctx.http, "回答待ちではありません")
                    .await;
            }
            _ => {
                println!("fail")
            }
        }
    }
}

#[group]
#[commands(start)]
struct General;

#[command]
async fn start(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let channel_id = msg.channel_id;
    let mut data = ctx.data.write().await;
    let mut state = data
        .get_mut::<AllBotState>()
        .expect("Failed to retrieve map!")
        .lock()
        .await;
    let bot_state = state.states.get(&channel_id);
    if bot_state.is_none() {
        let mut initial_state = BotState { mode: Mode::Init };
        initial_state.initialize_quiz();
        state.states.insert(channel_id, initial_state);
    };

    let bot_state = state.states.get(&channel_id).unwrap();
    msg.channel_id.say(&ctx.http, "Quiz を始めます。").await?;
    let current_state = bot_state;
    match &current_state.mode {
        Mode::WaitingUserAnswer(state) => {
            let cursor = &state.cursor;
            let current_question = &state.questions[*cursor as usize];
            let quiz = &current_question.content;
            msg.channel_id.say(&ctx.http, quiz).await?;
            Ok(())
        }
        _ => {
            msg.channel_id
                .say(&ctx.http, "クイズの初期化に失敗しました。")
                .await?;
            Ok(()) // Q: Error を返したい
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN to be set!");

    let framework = StandardFramework::new()
        .configure(|c| c.case_insensitivity(true))
        .group(&GENERAL_GROUP);

    let initial_state = AllBotState::new();

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<AllBotState>(Arc::new(Mutex::new(initial_state))) // new!
        .await
        .expect("Failed to build client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
