use chatgpt::prelude::*;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    let message = env::args().nth(1).expect("You must input message.");
    dotenv().ok();
    let api_key = env::var("CHATGPT_API_KEY").expect("CHATGPT_API_KEY must be set int .env file");
    let client =
        ChatGPT::new(api_key).expect("Failed to create client. You may input invalid API key.");
    let response = client
        .send_message(message)
        .await
        .expect("Failed to send message. Confirm your network connection.");

    println!("{}", response.message().content);
}
