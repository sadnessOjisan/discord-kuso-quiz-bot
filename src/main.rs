use dotenv::dotenv;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use std::collections::HashMap;
use std::env;

#[group]
struct General;

struct Handler {
    manager: QuizManager,
}

#[derive(Debug, Clone)]
struct Quiz {
    id: i8,
    content: String,
    answer: String,
}
#[derive(Debug)]
enum BotState {
    Initialized,
    IntentingQuestion,
    WaitingAnswer,
}

#[derive(Debug)]
struct QuizManager {
    quizs: Vec<Quiz>,
    currentQuiz: Option<Quiz>,
    result: HashMap<i8, bool>, // {[quizId]: bool}
    cursor: Option<i8>,
    state: BotState,
}

impl QuizManager {
    fn init(&mut self) {
        let data = vec![
            Quiz {
                id: 1,
                content: "test?".to_string(),
                answer: "test".to_string(),
            },
            Quiz {
                id: 2,
                content: "test2?".to_string(),
                answer: "test2".to_string(),
            },
        ];
        self.quizs = data;
        self.cursor = Some(0);
        let quiz = self.quizs[self.cursor.unwrap() as usize].clone(); // cloneが無理やりごまかした感がある
        self.currentQuiz = Some(quiz);
        self.state = BotState::Initialized;
    }

    fn setNextQuestion(&mut self) {
        // 組み込まれたやり方で Some + Some をやる方法ってないんだっけ？
        self.cursor = self.cursor.map(|v| v+1); 
        let quiz = self.quizs[self.cursor.unwrap()as usize].clone(); // cloneが無理やりごまかした感がある
        self.currentQuiz = Some(quiz);
        self.state = BotState::WaitingAnswer;
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&mut self, ctx: Context, msg: Message) {
        // 質問開始
        match &self.manager.state {
            BotState::Initialized => {
                if msg.content == "q!" {
                    println!("{:?}", &self.manager.currentQuiz.as_ref().unwrap());
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Quiz を始めます").await {
                        println!("Error sending message: {:?}", why);
                    }

                    // MEMO: ここの as_ref がなぜ必要か調べる
                    if let Err(why) = msg
                        .channel_id
                        .say(
                            &ctx.http,
                            &self.manager.currentQuiz.as_ref().unwrap().content,
                        )
                        .await
                    {
                        println!("Error sending message: {:?}", why);
                    }

                    &self.manager.setNextQuestion();
                }
            }
            BotState::IntentingQuestion => {}
            BotState::WaitingAnswer => {
                let userAnswer = msg.content;
                let currentQuestion = &self.manager.currentQuiz;
                let currentQuestionAnswer = currentQuestion.as_ref().unwrap().answer;
                &self.manager.result.insert(currentQuestion.unwrap().id, userAnswer == currentQuestionAnswer);
            }
        };
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    let mut manager = QuizManager {
        quizs: vec![],
        currentQuiz: None,
        result: HashMap::new(),
        cursor: None,
        state: BotState::Initialized,
    };

    manager.init();

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler { manager: manager })
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
