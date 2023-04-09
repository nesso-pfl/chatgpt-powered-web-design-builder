use crate::config::Config;
use chatgpt::prelude::*;
use std::{
  io::{stdin, stdout, Write},
  path::Path,
  process,
};

pub async fn communicate_with_chatgpt<F: Fn(String)>(
  api_key: String,
  config: &Config,
  on_received: F,
) {
  let direction_message = include_str!("./direction_message.txt");

  let mut conversation = ChatGPT::new_with_config(
    api_key,
    ModelConfigurationBuilder::default()
      .engine(config.clone().engine)
      .build()
      .unwrap(),
  )
  .expect("Failed to create client. You may input invalid API key.")
  .new_conversation_directed(direction_message);

  loop {
    println!("Request to ChatGPT: ");
    let _ = stdout().flush();
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read line.");

    if buf.trim().is_empty() {
      println!("exit with 0");
      process::exit(0);
    } else {
      let response = conversation
        .send_message(&buf)
        .await
        .expect("Failed to send message. Confirm your network connection.");
      let json_path = Path::new(&config.output_dir()).join("history.json");
      conversation
        .save_history_json(&json_path)
        .await
        .expect("Failed to save history.");
      on_received(response.message().clone().content);
      println!(
        "Saved in {}. You can send another request.",
        json_path.display()
      );
    }
  }
}
