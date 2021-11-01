use discord_kuso_quiz_bot::handler::Handler;
use dotenv::dotenv;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN to be set!");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Failed to build client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
