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
use std::collections::linked_list::Cursor;
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
        self.cursor = None;
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
    println!("{:?}", map.lock().await);
    msg.channel_id.say(&ctx.http, "Quiz を始めます。").await?;
    map.lock().await.nextQuestion();
    match &map.lock().await.cursor {
        None => {
            msg.channel_id
                .say(&ctx.http, "初期化されていません")
                .await?;
        }
        Some(v) => {
            let currentQuestion = &map.lock().await.questions[*v as usize];
            let txt = &currentQuestion.content;
            msg.channel_id.say(&ctx.http, txt).await?;
        }
    }
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
