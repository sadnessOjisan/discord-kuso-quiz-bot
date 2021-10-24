use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult, StandardFramework,
};
use serenity::model::{
    channel::{Message, Reaction},
    gateway::Ready,
};
use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::{env, vec};

struct Handler;

type QuestionID = i8;

#[derive(Debug)]
struct Question {
    id: QuestionID,
    content: String,
    answer: String,
}

#[derive(Debug)]
enum Mode {
    WaitingUserAnswer,
    Questioning,
}
#[derive(Debug)]
struct State {
    questions: Vec<Question>,
    result: HashMap<QuestionID, bool>,
    cursor: Option<u8>,
    mode: Mode,
}

impl State {
    fn init(&mut self) {
        // & なくてよい？
        self.questions = vec![
            Question {
                id: 1,
                content: "test".to_string(),
                answer: "hoge".to_string(),
            },
            Question {
                id: 2,
                content: "test2".to_string(),
                answer: "hoge2".to_string(),
            },
        ];
        self.cursor = Some(0);
        self.mode = Mode::Questioning
    }

    fn nextQuestion(&mut self) {
        self.cursor = self.cursor.map(|v| v + 1);
        self.mode = Mode::WaitingUserAnswer
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot ready with username {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let mut data = ctx.data.write().await;
        let state = data.get_mut::<State>().expect("Failed to retrieve map!");
        let current_state = &mut state.lock().await;

        match &current_state.mode {
            Mode::Questioning=>{

            }
            Mode::WaitingUserAnswer=>{
                let cursor = &current_state.cursor;
                match cursor {
                    None => {}
                    Some(v) => {
                        let current_q = &current_state.questions[*v as usize];
                        let current_q_answer = &current_q.answer; // どうしてこれは参照がいるのか
                        let user_answer = msg.content;
                        let result = current_q_answer.as_ref() == user_answer;
                        current_state.result.insert(current_q.id, true);
                    }
                }
            }
        }
    }

}

#[group]
#[commands(start)]
struct General;

impl TypeMapKey for State {
    type Value = Arc<Mutex<State>>;
}

#[command]
async fn start(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    let map = data.get_mut::<State>().expect("Failed to retrieve map!");
    map.lock().await.init(); // to_string() でコピーできる
    println!("{:?}", &map.lock().await.cursor);
    msg.channel_id.say(&ctx.http, "Quiz を始めます。").await?;
    map.lock().await.nextQuestion();
    let current_state = &map.lock().await;
    match &current_state.cursor {
        None => {
            msg.channel_id
                .say(&ctx.http, "初期化されていません")
                .await?;
        }
        Some(v) => {
            let current_question = &current_state.questions[*v as usize];
            let txt = &current_question.content;
            println!("{:?}", &current_question);
            msg.channel_id.say(&ctx.http, txt).await?;
        }
    }
    // println!("{:?}", &map.lock().await.cursor);
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN to be set!");

    let framework = StandardFramework::new()
        .configure(|c| c.case_insensitivity(true))
        .group(&GENERAL_GROUP);

    let initial_state = State {
        questions: vec![],
        result: HashMap::new(),
        cursor: None,
        mode: Mode::Questioning,
    };

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<State>(Arc::new(Mutex::new(initial_state))) // new!
        .await
        .expect("Failed to build client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
