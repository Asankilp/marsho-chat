use crate::{
    configs::config::MarshoConfig,
    models::{client::OpenAIClient, context::MarshoContext, message::BaseMessage},
};
use anyhow::{Context, Result};
use serde_json::Value;
use futures_util::Stream;

pub struct MarshoHandler {
    config: MarshoConfig,
    model_config: Value,
    client: OpenAIClient,
}

impl MarshoHandler {
    pub fn new(config: MarshoConfig, model_config: Value) -> Self {
        let client = OpenAIClient::new(config.base_url.clone(), config.api_key.clone());
        Self {
            config,
            model_config,
            client,
        }
    }

    pub async fn handle(
        &mut self,
        input: String,
        context: MarshoContext,
    ) -> impl Stream<Item = Value> {
        let mut message = vec![BaseMessage::system(self.config.system_prompt.to_string())];
        message.extend(context.get().iter().cloned());
        message.extend(vec![BaseMessage::user(input.to_string())]);
        self.client.make_chat_stream(&mut self.model_config, message).await
    }

    pub async fn models(&mut self) -> Result<()> {
        let models = self.client.get_models().await?;
        for model_name in models.data {
            println!("{}", model_name.id);
        }
        Ok(())
    }
}
