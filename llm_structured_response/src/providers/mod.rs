mod llm_provider;
pub mod openai;
pub mod provider_config;

use super::request;
use clap::ValueEnum;
pub use llm_provider::LlmProvider;
pub use provider_config::LlmProviderConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, ValueEnum, PartialEq, Default, Serialize)]
pub enum LlmProviders {
    #[default]
    OpenAi,
}

impl LlmProviders {
    pub async fn request_structured_response(
        &self,
        config: LlmProviderConfig,
        schema: request::Schema,
        prompt: request::Prompt,
    ) -> anyhow::Result<String> {
        match self {
            LlmProviders::OpenAi => {
                let provider = openai::OpenAiProvider { config };
                provider.request_structured_response(schema, prompt).await
            }
        }
    }
}
