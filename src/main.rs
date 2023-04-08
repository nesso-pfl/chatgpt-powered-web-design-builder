use chatgpt::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::Path;

#[tokio::main]
async fn main() {
  let message = env::args().nth(1).expect("You must input message.");
  let dir = "default";
  dotenv().ok();
  let api_key = env::var("CHATGPT_API_KEY").expect("CHATGPT_API_KEY must be set int .env file");
  let mut client =
ChatGPT::new(api_key).expect("Failed to create client. You may input invalid API key.").new_conversation_directed(r"私は user で、あなたは assistant です。
私はHTML, CSS, JavaScript を利用して、 Web ページそのものや Web ページのパーツのレイアウトの実装を行いたいです。
私はどのような Web ページや Web ページのパーツを作りたいかをあなたに尋ねるので、あなたは私の要望に沿った Web ページや Web ページのパーツを作成し、私に提供してください。
作成した Web ページや Web ページのパーツは、必ず HTML と CSS と JavaScript で実装してください。
出力する HTML のコードブロックは、必ず「```html」で始まり「```」で終わってください。
出力する CSS のコードブロックは、必ず「```css」で始まり「```」で終わってください。
出力する JavaScript のコードブロックは、必ず「```js」で始まり「```」で終わってください。
但し、 JavaScript は使用する必要がなければ使用しなくても構いません。
可能な限り JavaScript を使わずに実装してください。
");
  let response = client
    .send_message(message)
    .await
    .expect("Failed to send message. Confirm your network connection.");

  println!("{}", &response.message().content);
  create_dir(dir).unwrap_or_else(|err| {
    if err.kind() == std::io::ErrorKind::AlreadyExists {
      return;
    }
    panic!("Failed to create directory.");
  });
  save_html(&response.message().content, dir);
  save_css(&response.message().content, dir);
  save_js(&response.message().content, dir);
}

fn save_html(content: &str, dir: &str) {
  let html_content = content
    .split("```")
    .find(|str| str.starts_with("html"))
    .expect("Failed to get html.")
    .strip_prefix("html")
    .expect("Failed to get html.");

  let mut file =
    File::create(Path::new(dir).join("index.html")).expect("Failed to create html file.");
  write!(file, "{}", html_content).unwrap_or_else(|_| panic!("Failed to save html."));
}

fn save_css(content: &str, dir: &str) {
  let css_content = content
    .split("```")
    .find(|str| str.starts_with("css"))
    .expect("Failed to get css.")
    .strip_prefix("css")
    .expect("Failed to get css.");

  let mut file =
    File::create(Path::new(dir).join("index.css")).expect("Failed to create css file.");
  write!(file, "{}", css_content).unwrap_or_else(|_| panic!("Failed to save css."));
}

fn save_js(content: &str, dir: &str) {
  let js_content = content
    .split("```")
    .find(|str| str.starts_with("js"))
    .and_then(|str| str.strip_prefix("js"));

  if let Some(js_content) = js_content {
    let mut file =
      File::create(Path::new(dir).join("index.js")).expect("Failed to create js file.");
    write!(file, "{}", js_content).unwrap_or_else(|_| panic!("Failed to save js."));
  }
}
