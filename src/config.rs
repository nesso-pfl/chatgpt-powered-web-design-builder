use chatgpt::prelude::ChatGPTEngine;
use dotenvy::dotenv;
use std::env;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, Clone, EnumString)]
pub enum Engine {
  #[strum(serialize = "gpt-3.5-turbo")]
  GPT35Turbo,
  #[strum(serialize = "gpt-3.5-turbo-0301")]
  Gpt35turbo0301,
  #[strum(serialize = "gpt-4")]
  GPT4,
  #[strum(serialize = "gpt-4-32k")]
  GPT4_32k,
  #[strum(serialize = "gpt-4-0314")]
  GPT4_0314,
  #[strum(serialize = "gpt-4-32k-0314")]
  Gpt4_32k0314,
}

impl From<Engine> for ChatGPTEngine {
  fn from(value: Engine) -> Self {
    match value {
      Engine::GPT35Turbo => ChatGPTEngine::Gpt35Turbo,
      Engine::Gpt35turbo0301 => ChatGPTEngine::Gpt35Turbo_0301,
      Engine::GPT4 => ChatGPTEngine::Gpt4,
      Engine::GPT4_32k => ChatGPTEngine::Gpt4_32k,
      Engine::GPT4_0314 => ChatGPTEngine::Gpt4_0314,
      Engine::Gpt4_32k0314 => ChatGPTEngine::Gpt4_32k_0314,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Config {
  pub chrome: bool,
  pub chat_id: String,
  pub engine: Engine,
  pub parse_only: bool,
}

impl Default for Config {
  fn default() -> Self {
    dotenv().ok();
    Self {
      chrome: env::var("CHROME").map_or(false, |var| var.parse::<bool>().unwrap_or(false)),
      chat_id: env::var("CHAT_ID").unwrap_or("default".to_string()),
      engine: env::var("ENGINE").map_or(Engine::GPT35Turbo, |var| {
        Engine::from_str(&var).unwrap_or(Engine::GPT35Turbo)
      }),
      parse_only: false,
    }
  }
}

impl Config {
  pub fn generate() -> Self {
    Self::default()
  }

  pub fn output_dir(&self) -> String {
    format!("output/{}", &self.chat_id)
  }
}
