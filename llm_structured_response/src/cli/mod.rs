use super::providers::{LlmProviderConfig, LlmProviders};
use clap::{Args, Parser};
use serde::de::Deserializer;
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
    pub config: Option<CliConfigArgs>,
}

#[derive(Args, Debug, Deserialize)]
#[group(required = true)]
pub struct ConfigFileArgs {
    /// Load options from a configuration TOML file.
    #[arg(short, long)]
    pub config_file_path: std::path::PathBuf,
}

#[derive(Args, Debug, Clone, Serialize, PartialEq)]
#[group(required = true)]
pub struct CliConfigArgs {
    /// The LLM provider to use.
    #[arg(short, long)]
    pub provider: LlmProviders,

    /// Configuration for the LLM provider.
    /// These are options common to most providers. Your provider might not need all of them.
    #[command(flatten)]
    pub provider_config: Option<LlmProviderConfig>,

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

impl Default for CliConfigArgs {
    fn default() -> Self {
        CliConfigArgs {
            provider: LlmProviders::default(),
            provider_config: Some(LlmProviderConfig {
                model: "gpt-4o".to_string(),
                url: None,
                port: None,
            }),
            json_schema_file: std::path::PathBuf::default(),
            initial_prompt: String::default(),
            system_prompt: String::default(),
        }
    }
}

impl<'de> Deserialize<'de> for CliConfigArgs {
    // Custom deserializer is needed so that we can populate the provider_config field based on the selected provider.
    // Defaults to OpenAI if no provider is specified.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CliConfigArgsHelper {
            provider: LlmProviders,
            provider_config: Option<LlmProviderConfig>,
            json_schema_file: std::path::PathBuf,
            initial_prompt: String,
            system_prompt: String,
        }

        let helper = CliConfigArgsHelper::deserialize(deserializer)?;
        let provider_config = helper.provider_config.or_else(|| match helper.provider {
            LlmProviders::OpenAi => Some(LlmProviderConfig {
                model: "gpt-4o".to_string(),
                url: None,
                port: None,
            }),
            LlmProviders::Ollama => Some(LlmProviderConfig {
                model: "llama3.1:latest".to_string(),
                url: Some("http://127.0.0.1".to_string()),
                port: Some(11434),
            }),
            LlmProviders::XAI => Some(LlmProviderConfig {
                model: "grok-2-latest".to_string(),
                url: None,
                port: None,
            }),
        });

        Ok(CliConfigArgs {
            provider: helper.provider,
            provider_config,
            json_schema_file: helper.json_schema_file,
            initial_prompt: helper.initial_prompt,
            system_prompt: helper.system_prompt,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that a TOML string can be converted to a `CliConfigArgs` struct.
    /// Useful to help me vizualize the TOML structure.
    #[test]
    fn test_toml_to_config_openai() {
        let toml = r#"
provider = "OpenAi"
json_schema_file = "schema.json"
initial_prompt = "Generate a random student using the provided JSON schema."
system_prompt = "You are an AI assistant that generates random students."
        "#;
        let config: CliConfigArgs = toml::from_str(toml).unwrap();
        assert_eq!(config.provider, LlmProviders::OpenAi);
        assert_eq!(
            config.json_schema_file,
            std::path::PathBuf::from("schema.json")
        );
        assert_eq!(
            config.initial_prompt,
            "Generate a random student using the provided JSON schema."
        );
        assert_eq!(
            config.system_prompt,
            "You are an AI assistant that generates random students."
        );
        // Based on the selected provider, the provider_config should be populated with the proper default values.
        assert_eq!(
            config.provider_config,
            Some(LlmProviderConfig {
                model: "gpt-4o".to_string(),
                url: None,
                port: None,
            })
        );
    }

    /// Test that a TOML string can be converted to a `CliConfigArgs` struct.
    /// Useful to help me vizualize the TOML structure.
    #[test]
    fn test_toml_to_config_ollama() {
        let toml = r#"
provider = "Ollama"
json_schema_file = "schema.json"
initial_prompt = "Generate a random student using the provided JSON schema."
system_prompt = "You are an AI assistant that generates random students."
        "#;
        let config: CliConfigArgs = toml::from_str(toml).unwrap();
        assert_eq!(config.provider, LlmProviders::Ollama);
        assert_eq!(
            config.json_schema_file,
            std::path::PathBuf::from("schema.json")
        );
        assert_eq!(
            config.initial_prompt,
            "Generate a random student using the provided JSON schema."
        );
        assert_eq!(
            config.system_prompt,
            "You are an AI assistant that generates random students."
        );
        // Based on the selected provider, the provider_config should be populated with the proper default values.
        assert_eq!(
            config.provider_config,
            Some(LlmProviderConfig {
                model: "llama3.1:latest".to_string(),
                url: Some("http://127.0.0.1".to_string()),
                port: Some(11434),
            })
        );
    }
}
