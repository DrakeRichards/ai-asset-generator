use crate::{providers::provider_config::LlmProviderConfig, request::Prompt};
use anyhow::{Error, Result};
use clap::ValueEnum;
use dotenvy::dotenv;
use llm::{
    builder::{LLMBackend, LLMBuilder},
    chat::{ChatMessage, StructuredOutputFormat},
    error::LLMError,
};
use serde::{Deserialize, Serialize};
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
        schema: StructuredOutputFormat,
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

        // Build the base URL based on the URL and port, if provided.
        let base_url = config.url.clone().map(|url| {
            let port: Option<u16> = config.port;
            match port {
                Some(port) => format!("{}:{}", url, port),
                None => url,
            }
        });

        // Build the LLM instance.
        let llm = match backend {
            LLMBackend::OpenAI => LLMBuilder::new()
                .backend(backend)
                .model(config.model.clone())
                .api_key(api_key)
                .stream(false)
                .system(prompt.system.clone())
                .schema(schema)
                .build()?,
            LLMBackend::Ollama => LLMBuilder::new()
                .backend(backend)
                .model(config.model.clone())
                .base_url(base_url.ok_or(Error::msg("Missing base URL"))?)
                .stream(false)
                .system(prompt.system.clone())
                .schema(schema)
                .build()?,
            LLMBackend::XAI => LLMBuilder::new()
                .backend(backend)
                .model(config.model.clone())
                .api_key(api_key)
                .stream(false)
                .system(prompt.system.clone())
                .schema(schema)
                .build()?,
            _ => return Err(Error::msg("Backend not supported")),
        };

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
