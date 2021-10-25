use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::CommandError;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult, StandardFramework,
};
use serenity::model::{channel::Message, gateway::Ready};
use serenity::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Error;
use std::sync::Arc;
use std::vec;

struct Handler;

type QuestionID = i8;
#[derive(Debug, Clone)]
struct Question {
    id: QuestionID,
    content: String,
    answer: String,
}
#[derive(Debug, Clone)]
enum Mode {
    Init,
    WaitingUserAnswer(State),
    Error,
}

#[derive(Debug, Clone)]
struct BotState {
    mode: Mode,
}

impl TypeMapKey for BotState {
    type Value = Arc<Mutex<BotState>>;
}

impl BotState {
    fn new() -> BotState {
        BotState { mode: Mode::Init }
    }

    fn initialize_quiz(&mut self) {
        let questions = vec![
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
        let cursor = 0;
        let result = HashSet::new();
        self.mode = Mode::WaitingUserAnswer(State {
            questions,
            cursor,
            result,
        })
    }

    fn next_quiz(&mut self) {
        match &self.mode {
            Mode::WaitingUserAnswer(state) => {
                let next_state = State {
                    cursor: state.cursor + 1,
                    questions: state.questions.clone(), // Q: string は copy できないから clone するは正しいか
                    result: state.result.clone(),
                };
                self.mode = Mode::WaitingUserAnswer(next_state);
            }
            _ => {
                self.mode = Mode::Error;
            }
        }
    }

    fn user_answer(mut self, answer: String) {
        match &self.mode {
            Mode::WaitingUserAnswer(state) => {
                let current_quiz = state.questions[state.cursor as usize].clone();
                let current_q_answer = current_quiz.answer;
                let is_correct = current_q_answer == answer;
                let mut current_result =state.result.clone();
                if is_correct {
                    current_result.insert(current_quiz.id);
                }
                let next_state = State {
                    cursor: state.cursor + 1,
                    questions: state.questions.clone(), // Q: string は copy できないから clone するは正しいか
                    result: current_result,
                };
                self.mode = Mode::WaitingUserAnswer(next_state);
                self.next_quiz()
            }
            _ => {
                self.mode = Mode::Error;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    questions: Vec<Question>,
    result: HashSet<QuestionID>,
    cursor: u8,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot ready with username {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let mut data = ctx.data.write().await;
        let bot_state = data.get_mut::<BotState>().expect("Failed to retrieve map!");
        let mut current_state =  bot_state.lock().await.clone();

        match &current_state.mode {
            Mode::WaitingUserAnswer(state) => {
                let user_answer = msg.content;
                current_state.user_answer(user_answer);
                // Q: current_state.next_quiz(); user_answer を next_quiz() から呼ぶようにしたのでエラーを回避できたけど、本当にこれでいいのか？
                let next_state =  &mut bot_state.lock().await;
                match &next_state.mode {
                    Mode::WaitingUserAnswer(state) => {
                        let current_quiz = &state.questions[state.cursor as usize].clone();
                        msg.channel_id.say(&ctx.http, &current_quiz.content).await;
                    }
                    _ => {
                        msg.channel_id.say(&ctx.http, "不正な状態です。").await;
                    }
                }
            }
            _ => {
                msg.channel_id.say(&ctx.http, "不正な状態です。").await;
            }
        }
    }
}

#[group]
#[commands(start)]
struct General;

#[command]
async fn start(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    let state = data.get_mut::<BotState>().expect("Failed to retrieve map!");
    state.lock().await.initialize_quiz();
    msg.channel_id.say(&ctx.http, "Quiz を始めます。").await?;
    let current_state = &state.lock().await;
    match &current_state.mode {
        Mode::WaitingUserAnswer(state) => {
            let cursor = &state.cursor;
            let current_question = &state.questions[*cursor as usize];
            let quiz = &current_question.content;
            msg.channel_id.say(&ctx.http, quiz).await?;
            Ok(())
        },
        _ => {
            msg.channel_id.say(&ctx.http, "クイズの初期化に失敗しました。").await?;
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

    let initial_state = BotState::new();

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<BotState>(Arc::new(Mutex::new(initial_state))) // new!
        .await
        .expect("Failed to build client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
