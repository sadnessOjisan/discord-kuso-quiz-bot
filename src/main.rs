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

#[derive(Debug)]
struct Question {
    id: u8,
    content: String
}

#[derive(Debug)]
struct State {
    questions:Vec<Question> 
}

impl State {
    fn init(&mut self){
        self.questions = vec![
            Question {
                id: 1,
                content: "test".to_string()
            },
            Question {
                id: 2,
                content: "test2".to_string()
            }
        ]
    }
}

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, _: Context, ready: Ready) {
    println!("Bot ready with username {}", ready.user.name);
  }
}

#[group]
#[commands(ping)]
struct General;

struct Data;

impl TypeMapKey for State {
    type Value = Arc<Mutex<State>>;
  }


#[command]
async fn ping(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
   let mut data = ctx.data.write().await;
   let map = data.get_mut::<State>().expect("Failed to retrieve map!");
   let content = &msg.content;
   map.lock().await.init(); // to_string() でコピーできる
   println!("{:?}", map);
  msg.channel_id.say(&ctx.http, "Pong!").await?;
  
  Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN to be set!");

    let framework = StandardFramework::new()
      .configure(|c| c.case_insensitivity(true))
      .group(&GENERAL_GROUP);

      let initialState = State {
          questions:vec![] 
      };

    let mut client = Client::builder(&token)
      .event_handler(Handler)
      .framework(framework)
      .type_map_insert::<State>(Arc::new(Mutex::new(initialState))) // new!
      .await
      .expect("Failed to build client");

    if let Err(why) = client.start().await {
      println!("Client error: {:?}", why);
    }
}
