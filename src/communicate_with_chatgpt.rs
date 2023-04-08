use chatgpt::prelude::*;

pub async fn communicate_with_chatgpt(api_key: String, message: String) -> String {
  let mut client =
ChatGPT::new(api_key).expect("Failed to create client. You may input invalid API key.").new_conversation_directed(r"私は user で、あなたは assistant です。
私はHTML, CSS, JavaScript を利用して、 Web ページそのものや Web ページのパーツのレイアウトの実装を行いたいです。
私はどのような Web ページや Web ページのパーツを作りたいかをあなたに尋ねるので、あなたは私の要望に沿った Web ページや Web ページのパーツを作成し、私に提供してください。
作成した Web ページや Web ページのパーツは、必ず HTML と CSS と JavaScript で実装してください。
出力する HTML のコードブロックは、必ず「```html」で始まり「```」で終わってください。
出力する HTML のファイル名は「index.html」としてください。
出力する CSS のコードブロックは、必ず「```css」で始まり「```」で終わってください。
出力する CSS のファイル名は「index.css」としてください。
出力する JavaScript のコードブロックは、必ず「```js」で始まり「```」で終わってください。
出力する JavaScript のファイル名は「index.js」としてください。
但し、 JavaScript は使用する必要がなければ使用しなくても構いません。
可能な限り JavaScript を使わずに実装してください。
出力する HTML は、必ず出力する CSS と出力する JavaScript とを読み込むようにしてください。
出力する JavaScript がない場合、出力する HTML は JavaScript を読み込まないでください。
");
  let response = client
    .send_message(message)
    .await
    .expect("Failed to send message. Confirm your network connection.");

  response.message().content.clone()
}
