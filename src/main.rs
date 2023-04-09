use dotenvy::dotenv;
use output_design_by_chatgpt::{communicate_with_chatgpt, config::Config, save_files};
use std::{env, fs::create_dir_all};

#[tokio::main]
async fn main() {
  dotenv().ok();
  let config = Config::generate();
  let api_key = env::var("CHATGPT_API_KEY").expect("CHATGPT_API_KEY must be set int .env file");

  create_dir_all(config.output_dir()).unwrap_or_else(|err| {
    if err.kind() == std::io::ErrorKind::AlreadyExists {
      return;
    }
    panic!("Failed to create directory.");
  });
  communicate_with_chatgpt(api_key, &config, |content| {
    save_files(content, &format!("output/{}", &config.chat_id));
  })
  .await;
}
