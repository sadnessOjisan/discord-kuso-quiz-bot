use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};
use std::collections::HashMap;
use dotenv::dotenv;
use std::env;

#[group]
struct General;

struct Handler;

struct Quiz {
    id: i8,
    content:String,
    answer:String
}

struct Result {
    quizId: i8,
    isCorrect: bool,
}

struct QuizManager {
    quizs: Vec<Quiz>,
    currentQuiz: Quiz,
    result: HashMap<i8, bool> // {[quizId]: bool}
}

impl QuizManager {
    fn init(&mut self){
        self.quizs = vec![
            Quiz {
                id: 1,
                content: "aaa".to_string(),
                answer: "bbb".to_string()
            }
        ]
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
