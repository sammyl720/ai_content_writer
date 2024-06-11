use async_openai::{config::OpenAIConfig, Client};

pub fn create_openai_client(api_key: String) -> Client<OpenAIConfig> {
    let config = OpenAIConfig::new().with_api_key(api_key);

    Client::with_config(config)
}
