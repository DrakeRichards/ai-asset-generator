use clap::Args;
use serde::{Deserialize, Serialize};

/// Configuration for the LLM provider.
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

impl Default for LlmProviderConfig {
    fn default() -> Self {
        Self {
            model: "gpt-4o".to_string(),
            url: None,
            port: None,
        }
    }
}
