use std::{collections::HashMap, sync::Arc};

use discord_kuso_quiz_bot::{bot::BotState, handler::Handler};
use dotenv::dotenv;
use serenity::{futures::lock::Mutex, prelude::*};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN to be set!");

    let initial_state = BotState {
        channel_sender_pair: HashMap::new(),
    };

    let mut client = Client::builder(&token)
        .event_handler(Handler {
            channel_sender_pair: HashMap::new(),
        })
        .type_map_insert::<BotState>(Arc::new(Mutex::new(initial_state)))
        .await
        .expect("Failed to build client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
