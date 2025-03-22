use crate::LlmProviders;
use clap::Args;
use serde::{Deserialize, Serialize};

/// Configuration for the LLM provider.
/// These are options common to most providers. Your provider might not need all of them.
#[derive(Debug, Clone, Args, Deserialize, PartialEq, Serialize)]
pub struct LlmProviderConfig {
    /// The model to use.
    #[arg(long, default_value = "gpt-4o")]
    pub model: String,
    /// The URL of the API.
    #[arg(long)]
    pub url: Option<String>,
    /// The port of the API.
    #[arg(long)]
    pub port: Option<u16>,
}

impl LlmProviderConfig {
    /// Create a default configuration for the LLM provider.
    pub fn default_for_provider(provider: &LlmProviders) -> Self {
        match provider {
            LlmProviders::OpenAi => Self {
                model: "gpt-4o".to_string(),
                url: None,
                port: None,
            },
            LlmProviders::Ollama => Self {
                model: "llama3.1:latest".to_string(),
                url: Some("http://127.0.0.1".to_string()),
                port: Some(11434),
            },
            LlmProviders::XAI => Self {
                model: "grok-2-latest".to_string(),
                url: None,
                port: None,
            },
        }
    }
}
