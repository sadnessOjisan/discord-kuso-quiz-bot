use std::sync::Arc;

use discord_kuso_quiz_bot::{bot::BotState, handler::Handler};
use dotenv::dotenv;
use serenity::{futures::lock::Mutex, prelude::*};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN to be set!");

    let mut client = Client::builder(&token)
        .event_handler(Handler::default())
        .type_map_insert::<BotState>(Arc::new(Mutex::new(BotState::default())))
        .await
        .expect("Failed to build client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
