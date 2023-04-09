use dotenvy::dotenv;
use output_design_by_chatgpt::{communicate_with_chatgpt, save_files};
use std::{
  env,
  fs::{create_dir_all, remove_dir_all},
};

#[tokio::main]
async fn main() {
  let dir = "output/default";
  dotenv().ok();
  let api_key = env::var("CHATGPT_API_KEY").expect("CHATGPT_API_KEY must be set int .env file");

  remove_dir_all(dir).expect("Failed to remove directory.");
  create_dir_all(dir).expect("Failed to create directory.");
  communicate_with_chatgpt(api_key, dir, |content| {
    save_files(content, dir);
  })
  .await;
}
