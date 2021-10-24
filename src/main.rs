use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
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

impl TypeMapKey for Data {
    type Value = Arc<Mutex<HashMap<u8, String>>>;
  }


#[command]
async fn ping(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
   let mut data = ctx.data.write().await;
   let map = data.get_mut::<Data>().expect("Failed to retrieve map!");
   let content = &msg.content;
   map.lock().await.insert(0,  content.to_string()); // to_string() でコピーできる
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

    let mut client = Client::builder(&token)
      .event_handler(Handler)
      .framework(framework)
      .type_map_insert::<Data>(Arc::new(Mutex::new(HashMap::new()))) // new!
      .await
      .expect("Failed to build client");

    if let Err(why) = client.start().await {
      println!("Client error: {:?}", why);
    }
}
