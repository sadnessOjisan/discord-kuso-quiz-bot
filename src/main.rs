use dotenv::dotenv;
use std::collections::HashMap;
use std::{env, vec};
use std::sync::Arc;
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

struct Handler;

type QuestionID = i8;

#[derive(Debug)]
struct Question {
    id: QuestionID,
    content: String,
    answer: String
}


#[derive(Debug)]
struct State {
    questions:Vec<Question>,
    result: HashMap<QuestionID, bool>,
    cursor: Option<u8>
}

impl State {
    fn init(&mut self){
        self.questions = vec![
            Question {
                id: 1,
                content: "test".to_string(),
                answer: "hoge".to_string()
            },
            Question {
                id: 2,
                content: "test2".to_string(),
                answer: "hoge2".to_string()
            }
        ];
        self.cursor = Some(0)
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
   let txt = &map.lock().await.questions[0].content;
   msg.channel_id.say(&ctx.http, txt).await?;
   
//    match map;
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
          questions:vec![],
          result: HashMap::new(),
          cursor: None
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
