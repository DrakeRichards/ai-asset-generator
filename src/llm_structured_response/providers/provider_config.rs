use clap::Args;
use serde::{Deserialize, Serialize};

/// Configuration for the LLM provider.
#[derive(Debug, Clone, Args, Deserialize, PartialEq, Serialize)]
pub struct LlmProviderConfig {
    /// The model to use.
    #[arg(long, default_value = "gpt-4o-2024-08-06")]
    pub model: String,
}

impl Default for LlmProviderConfig {
    fn default() -> Self {
        Self {
            model: "gpt-4o-2024-08-06".to_string(),
        }
    }
}
