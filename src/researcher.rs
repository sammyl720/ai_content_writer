use async_openai::{config::OpenAIConfig, Client as OpenAIClient};
use axum::http::HeaderMap;
use serde_json::Value;

use crate::{agent::Agent, common::create_openai_client, errors::ApiError};

#[derive(Clone)]
pub struct Researcher {
    http_client: reqwest::Client,
    system: Option<String>,
    openai_client: OpenAIClient<OpenAIConfig>,
}

impl Researcher {
    pub fn new(openai_api_key: String, serper_api_key: String) -> Self {
        let openai_client = create_openai_client(openai_api_key);

        let mut headers = HeaderMap::new();
        headers.insert("X-API-KEY", serper_api_key.parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            http_client,
            system: None,
            openai_client,
        }
    }

    pub async fn prepare_data(&self, prompt: &str) -> Result<String, ApiError> {
        let json = serde_json::json!({
            "q": prompt
        });

        let res = self
            .http_client
            .post("https://google.serper.dev/search")
            .json(&json)
            .send()
            .await
            .unwrap();

        let json = res.json::<Value>().await?;
        Ok(serde_json::to_string_pretty(&json)?)
    }
}

impl Agent for Researcher {
    fn name(&self) -> String {
        "Researcher".to_string()
    }

    fn client(&self) -> OpenAIClient<OpenAIConfig> {
        self.openai_client.clone()
    }

    fn system_message(&self) -> String {
        if let Some(message) = &self.system {
            message.to_owned()
        } else {
            "You are an agent.

        You will receive a question that may be quite short or does not have much context.
        Your job is to research the Internet and to return with a high-quality summary to the user, assisted by the provided context.
        The provided context will be in JSON format and contains data about the initial Google results for the website or query.

        Be concise.

        Question:
        ".to_string()
        }
    }
}
