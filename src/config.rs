use dotenvy::dotenv;
use std::env;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, Clone, EnumString)]
pub enum Engine {
  #[strum(serialize = "ChatGPT3.5", serialize = "3.5")]
  GPT3_5,
  #[strum(serialize = "ChatGPT4", serialize = "4")]
  GPT4,
}
#[derive(Debug, Clone)]
pub struct Config {
  pub chrome: bool,
  pub dir: String,
  pub engine: Engine,
  pub parse_only: bool,
}

impl Default for Config {
  fn default() -> Self {
    dotenv().ok();
    Self {
      chrome: env::var("CHROME")
        .map(|var| var.parse::<bool>().unwrap_or(false))
        .unwrap_or(false),
      dir: env::var("CHAT_ID").unwrap_or("default".to_string()),
      engine: env::var("ENGINE")
        .map(|var| Engine::from_str(&var).unwrap_or(Engine::GPT3_5))
        .unwrap_or(Engine::GPT3_5),
      parse_only: false,
    }
  }
}

impl Config {
  pub fn generate() -> Self {
    Self::default()
  }
}
