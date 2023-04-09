use chatgpt::prelude::*;

pub async fn communicate_with_chatgpt(api_key: String, message: String) -> String {
  let direction_message = include_str!("./direction_message.txt");

  let mut client = ChatGPT::new(api_key)
    .expect("Failed to create client. You may input invalid API key.")
    .new_conversation_directed(direction_message);
  let response = client
    .send_message(message)
    .await
    .expect("Failed to send message. Confirm your network connection.");

  response.message().content.clone()
}
