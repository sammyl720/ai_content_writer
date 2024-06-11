use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client as OpenAIClient,
};

use crate::errors::ApiError;

pub trait Agent {
    fn name(&self) -> String;
    fn client(&self) -> OpenAIClient<OpenAIConfig>;
    fn system_message(&self) -> String;

    // to be given a default implementation later
    async fn prompt(&self, input: &str, data: String) -> Result<String, ApiError> {
        let input = format!(
            "{input}
            
            Provided context:
            {}
            ",
            serde_json::to_string_pretty(&data)?
        );

        let res = self
            .client()
            .chat()
            .create(
                CreateChatCompletionRequestArgs::default()
                    .model("gpt-40")
                    .messages(vec![
                        ChatCompletionRequestMessage::System(
                            ChatCompletionRequestSystemMessageArgs::default()
                                .content(&self.system_message())
                                .build()?,
                        ),
                        ChatCompletionRequestMessage::User(
                            ChatCompletionRequestUserMessageArgs::default()
                                .content(input)
                                .build()?,
                        ),
                    ])
                    .build()?,
            )
            .await
            .map(|res| {
                // we extract the first one
                res.choices[0].message.content.clone().unwrap()
            })?;

        println!("Retrieved result from prompt: {res}");

        Ok(res)
    }
}
