use dotenvy::dotenv;
use output_design_by_chatgpt::{communicate_with_chatgpt, config::Config, save_files};
use std::{
  env,
  fs::{create_dir_all, remove_dir_all},
};

#[tokio::main]
async fn main() {
  dotenv().ok();
  let config = Config::generate();
  let api_key = env::var("CHATGPT_API_KEY").expect("CHATGPT_API_KEY must be set int .env file");

  remove_dir_all(&config.dir).expect("Failed to remove directory.");
  create_dir_all(&config.dir).expect("Failed to create directory.");
  communicate_with_chatgpt(api_key, &config.dir, |content| {
    save_files(content, &config.dir);
  })
  .await;
}
