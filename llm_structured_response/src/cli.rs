use super::providers::{LlmProviderConfig, LlmProviders};
use clap::{Args, Parser};
use serde::{Deserialize, Serialize};

/// Send a request to an LLM provider and return a structured JSON response.
/// Currently only supports OpenAI's GPT-4 model, since that's the only model that supports JSON schema responses that I know of.
#[derive(Parser, Debug, Deserialize)]
pub struct Cli {
    /// Load options from a configuration TOML file.
    #[command(flatten)]
    pub config_file: Option<ConfigFileArgs>,

    /// Manually specify configuration options.
    #[command(flatten)]
    pub config: Option<ConfigArgs>,
}

#[derive(Args, Debug, Deserialize)]
#[group(required = true)]
pub struct ConfigFileArgs {
    /// Load options from a configuration TOML file.
    #[arg(short, long)]
    pub config_file_path: std::path::PathBuf,
}

#[derive(Args, Debug, Deserialize, Clone, Default, Serialize, PartialEq)]
#[group(required = true)]
pub struct ConfigArgs {
    /// The LLM provider to use.
    #[arg(short, long)]
    pub provider: LlmProviders,

    /// Configuration for the LLM provider.
    /// These are options common to most providers. Your provider might not need all of them.
    #[command(flatten)]
    pub provider_config: LlmProviderConfig,

    /// The JSON schema file to use.
    #[arg(long)]
    pub json_schema_file: std::path::PathBuf,

    /// The initial prompt to use.
    #[arg(long)]
    pub initial_prompt: String,

    /// The system prompt to use.
    #[arg(long)]
    pub system_prompt: String,
}
