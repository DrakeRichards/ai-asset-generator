use crate::{providers::provider_config::LlmProviderConfig, request::Prompt};
use anyhow::{Error, Result};
use clap::ValueEnum;
use dotenvy::dotenv;
use llm::{
    builder::{LLMBackend, LLMBuilder},
    chat::ChatMessage,
    error::LLMError,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::runtime::Runtime;

#[derive(Debug, Clone, Deserialize, ValueEnum, PartialEq, Default, Serialize)]
pub enum LlmProviders {
    #[default]
    OpenAi,
    Ollama,
    XAI,
}

impl LlmProviders {
    pub fn request_structured_response(
        &self,
        config: &LlmProviderConfig,
        schema: &Value,
        prompt: &Prompt,
    ) -> Result<String> {
        // Populate the environment variables.
        dotenv().ok();

        // Map the LlmProviders enum to the LLMBackend enum.
        let (backend, api_key) = match self {
            LlmProviders::OpenAi => (LLMBackend::OpenAI, std::env::var("OPENAI_API_KEY")?),
            LlmProviders::Ollama => (
                LLMBackend::Ollama,
                std::env::var("OLLAMA_API_KEY").unwrap_or("".to_string()),
            ),
            LlmProviders::XAI => (LLMBackend::XAI, std::env::var("XAI_API_KEY")?),
        };

        // Build the LLM instance.
        let llm = LLMBuilder::new()
            .backend(backend)
            .model(config.model.clone())
            .api_key(api_key)
            .stream(false)
            .system(prompt.system.clone())
            .schema(schema.clone())
            .build()?;

        // Send the request to the LLM provider.
        let rt = Runtime::new()?;
        let response = rt.block_on(async {
            let initial = prompt.initial.clone();
            tokio::spawn(async move {
                let messages = vec![ChatMessage::user().content(initial).build()];
                llm.chat(&messages)
                    .await?
                    .text()
                    .ok_or(Error::new(LLMError::ProviderError(
                        "Failed to get text response".to_string(),
                    )))
            })
            .await?
        })?;
        Ok(response)
    }
}
