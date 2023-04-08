use dotenvy::dotenv;
use output_design_by_chatgpt::{communicate_with_chatgpt, save_files};
use std::env;

#[tokio::main]
async fn main() {
  let message = env::args().nth(1).expect("You must input message.");
  let dir = "output/default";
  dotenv().ok();
  let api_key = env::var("CHATGPT_API_KEY").expect("CHATGPT_API_KEY must be set int .env file");

  let content = communicate_with_chatgpt(api_key, message).await;
  save_files(content, dir);
}
