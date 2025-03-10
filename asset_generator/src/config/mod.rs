#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use ai_images::cli::GenerationParameters;
use anyhow::{Error, Result};
use llm_structured_response::cli::ConfigArgs;
use llm_structured_response::request::{Prompt, Schema};
use minijinja::Environment;
use random_phrase_generator::RandomphraseGenerator;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct AssetConfig {
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

impl AssetConfig {
    pub fn from_toml_file(config_file: &Path) -> Result<Self> {
        let config = fs::read_to_string(config_file)?;
        let config: AssetConfig = toml::from_str(&config)?;
        Ok(config)
    }

    /// Generate the initial prompt if not provided by the user
    fn generate_random_phrase(&self) -> Result<String> {
        let random_phrase_generator: RandomphraseGenerator =
            RandomphraseGenerator::from_csv_files(&self.random_phrase_generator.csv_files)?;
        let random_phrase = random_phrase_generator.generate_random_phrase();
        Ok(random_phrase)
    }

    /// Send the initial prompt to the LLM API to get a structured response
    async fn generate_structured_response(&self, initial_prompt: &str) -> Result<String> {
        // Define the schema for the structured response
        let schema_text: String =
            fs::read_to_string(&self.llm_structured_response.json_schema_file)?;
        let schema: Schema = Schema::from_json_string(&schema_text)?;

        // Create the prompt object
        let prompt = Prompt {
            system: self.llm_structured_response.system_prompt.clone(),
            initial: initial_prompt.to_string(),
        };

        // Send the initial prompt to the LLM API to get a structured response
        let llm_structured_response = self
            .llm_structured_response
            .provider
            .request_structured_response(
                self.llm_structured_response.provider_config.clone(),
                &schema,
                &prompt,
            )
            .await?;

        Ok(llm_structured_response)
    }

    /// Generate an image based on the structured response
    async fn generate_image(&self, prompt_from_response: &str) -> Result<PathBuf> {
        // Initialize the provider.
        let provider = self.ai_images.provider.to_image_provider()?;
        // Set up the prompt.
        let image_prompt: ai_images::Prompt = ai_images::Prompt {
            base: prompt_from_response.to_string(),
            prefix: self.ai_images.params.prompt.prefix.clone(),
            suffix: self.ai_images.params.prompt.suffix.clone(),
            negative: self.ai_images.params.prompt.negative.clone(),
        };
        // Merge the new image prompt into the existing image params.
        // We need to do this since the configuration TOML file can define prefixes, suffixes, etc. which cannot be passed from the command line, and which are not part of the structured response.
        let mut image_params: ai_images::ImageParams = self.ai_images.params.clone();
        image_params.prompt = image_prompt;
        // Generate the image.
        let image = provider.generate_image(image_params).await?;
        Ok(image)
    }

    /// Fill the markdown template with the image and the structured response
    fn fill_template(&self, structured_response: Map<String, Value>) -> Result<String> {
        let template = fs::read_to_string(&self.markdown_template_filler.template_file_path)?;
        let mut env = Environment::new();
        env.add_template("template", &template)?;
        let templ = env.get_template("template")?;
        let rendered = templ.render(&structured_response)?;
        Ok(rendered)
    }
}

/// The asset to be generated
#[derive(Debug, Serialize)]
pub struct Asset {
    pub markdown: PathBuf,
    pub image: Option<PathBuf>,
}

impl Asset {
    pub async fn from_config_file_and_prompt(
        config_file: &Path,
        prompt: Option<&str>,
    ) -> Result<Asset> {
        let config = AssetConfig::from_toml_file(config_file)?;
        let asset = Asset::from_config(&config, prompt).await?;
        Ok(asset)
    }

    async fn from_config(config: &AssetConfig, user_prompt: Option<&str>) -> Result<Asset> {
        let initial_prompt = match user_prompt {
            Some(prompt) => prompt.to_string(),
            _ => config.generate_random_phrase()?,
        };

        let llm_structured_response = config.generate_structured_response(&initial_prompt).await?;
        let llm_structured_response: Value = serde_json::from_str(&llm_structured_response)?;

        // Generate an image based on the structured response and save it
        let image_prompt: &Value = llm_structured_response
            .get("image_prompt")
            .unwrap_or(&Value::Null);
        let image_path: Option<PathBuf> = match image_prompt {
            Value::String(prompt) => Some(config.generate_image(prompt).await?),
            _ => None,
        };
        // Strip the image path to the filename
        let image_filename: Option<String> = match &image_path {
            Some(image_path) => image_path
                .file_name()
                .ok_or(Error::msg("Unable to get the image filename."))?
                .to_string_lossy()
                .to_string()
                .parse()
                .ok(),
            _ => None,
        };
        // Add the image name to the structured response
        let mut llm_structured_response: Map<String, Value> = llm_structured_response
            .as_object()
            .ok_or(Error::msg("Unable to convert response to an object."))?
            .clone();
        if let Some(image_filename) = image_filename {
            llm_structured_response
                .insert("image_file_name".to_string(), Value::String(image_filename));
        }

        // Fill the markdown template with the image and the structured response
        let markdown: String = config.fill_template(llm_structured_response)?;

        // If output_dir is empty, save the markdown to the current directory
        let output_dir: PathBuf = if config.output_directory == PathBuf::new() {
            PathBuf::from(".")
        } else {
            config.output_directory.clone()
        };

        // Create the output directory if it does not exist
        if !output_dir.exists() {
            fs::create_dir_all(&output_dir)?;
        }

        // Save the markdown to a file. The filename is the current unix timestamp
        let markdown_file_path = output_dir.join(format!("{}.md", chrono::Utc::now().timestamp()));
        fs::write(&markdown_file_path, &markdown)?;

        // Return the markdown and the image path
        Ok(Asset {
            markdown: markdown_file_path,
            image: image_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[cfg(test)]
    mod default_config {
        use super::*;

        fn generate_default_config_file() -> Result<PathBuf> {
            const CONFIG_FILE_PATH: &str = "test-config.toml";
            let config = AssetConfig::default();
            let config = toml::to_string(&config)?;
            let config_file_path = PathBuf::from(CONFIG_FILE_PATH);
            fs::write(&config_file_path, &config)?;
            Ok(config_file_path)
        }

        #[test]
        fn test_deserialize_config_file() -> Result<()> {
            let config_file_path = generate_default_config_file()?;
            let config = std::fs::read_to_string(&config_file_path)?;
            let config: AssetConfig = toml::from_str(&config)?;
            dbg!("{:?}", config);
            // Clean up
            fs::remove_file(config_file_path)?;
            Ok(())
        }

        #[test]
        fn test_serialize_default_config() -> Result<()> {
            let config = AssetConfig::default();
            let config = toml::to_string(&config)?;
            dbg!("{}", config);
            Ok(())
        }
    }

    #[cfg(test)]
    mod ollama_config {
        use super::*;

        const CONFIG_FILE_PATH: &str = "test-ollama-config.toml";
        const JSON_SCHEMA_FILE_PATH: &str = "test-ollama-schema.json";

        fn generate_schema_file() -> Result<PathBuf> {
            let schema = r#"{
                "$schema": "http://json-schema.org/draft-07/schema#",
                "$id": "http://example.com/example.schema.json",
                "title": "Example",
                "description": "An example schema in JSON",
                "type": "object",
                "properties": {
                    "name": {
                        "description": "Name of the animal",
                        "type": "string"
                    },
                    "activity": {
                        "description": "Activity of the animal",
                        "type": "string"
                    }
                }
            }"#;
            let schema_file_path = PathBuf::from(JSON_SCHEMA_FILE_PATH);
            fs::write(&schema_file_path, schema)?;
            Ok(schema_file_path)
        }

        fn generate_default_config() -> AssetConfig {
            let mut config = AssetConfig::default();
            config.llm_structured_response.provider =
                llm_structured_response::providers::LlmProviders::Ollama;
            config.llm_structured_response.provider_config.url =
                Some("http://localhost".to_string());
            config.llm_structured_response.provider_config.port = Some(11434);
            config.llm_structured_response.system_prompt = "System prompt".to_string();
            config.llm_structured_response.json_schema_file = PathBuf::from(JSON_SCHEMA_FILE_PATH);
            config.llm_structured_response.provider_config.model = "llama3.1".to_string();
            config
        }

        fn generate_default_config_file() -> Result<PathBuf> {
            let config = generate_default_config();
            let config = toml::to_string(&config)?;
            let config_file_path = PathBuf::from(CONFIG_FILE_PATH);
            fs::write(&config_file_path, &config)?;
            Ok(config_file_path)
        }

        #[tokio::test]
        async fn test_ollama_config() -> Result<()> {
            let prompt = "Prompt";
            let config = generate_default_config();
            let schema = generate_schema_file()?;
            let asset = Asset::from_config(&config, Some(prompt)).await?;
            dbg!("{:?}", asset);
            // Clean up
            fs::remove_file(schema)?;
            Ok(())
        }

        #[tokio::test]
        async fn test_ollama_config_from_file() -> Result<()> {
            let config_file_path = generate_default_config_file()?;
            let prompt = "Prompt";
            let asset = Asset::from_config_file_and_prompt(&config_file_path, Some(prompt)).await?;
            dbg!("{:?}", asset);
            Ok(())
        }
    }
}
