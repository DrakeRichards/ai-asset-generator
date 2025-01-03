use super::{ai_images::cli::GenerationParameters, llm_structured_response::cli::ConfigArgs};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Config {
    pub output_directory: PathBuf,
    pub random_phrase_generator: RandomPhraseGeneratorConfig,
    pub llm_structured_response: LlmStructuredResponseConfig,
    pub ai_images: AiImagesConfig,
    pub markdown_template_filler: MarkdownTemplateFillerConfig,
}

type LlmStructuredResponseConfig = ConfigArgs;

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct RandomPhraseGeneratorConfig {
    pub csv_files: Vec<PathBuf>,
}

type AiImagesConfig = GenerationParameters;

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct MarkdownTemplateFillerConfig {
    pub template_file_path: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_deserialize_config_file() -> Result<()> {
        let config_file_path = PathBuf::from("test/example-config.toml");
        let config = std::fs::read_to_string(config_file_path)?;
        let config: Config = toml::from_str(&config)?;
        println!("{:?}", config);
        Ok(())
    }

    #[test]
    fn test_serialize_default_config() -> Result<()> {
        let config = Config::default();
        let config = toml::to_string(&config)?;
        println!("{}", config);
        Ok(())
    }
}
