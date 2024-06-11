use std::env;

use async_openai::{config::OpenAIConfig, Client};

pub fn create_openai_client() -> Client<OpenAIConfig> {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let config = OpenAIConfig::new().with_api_key(api_key);

    Client::with_config(config)
}
