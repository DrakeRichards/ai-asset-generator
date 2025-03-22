use ai_images::cli::GenerationParameters;
use anyhow::{Error, Result};
use ex::fs;
pub use llm_structured_response::{
    CliConfigArgs, LlmProviderConfig, Prompt, StructuredOutputFormat,
};
use minijinja::Environment;
use random_phrase_generator::RandomphraseGenerator;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, from_str};
use std::path::{Path, PathBuf};
use tokio::runtime::Runtime;

#[derive(Debug, Deserialize, Default, Serialize, PartialEq)]
pub struct AssetConfig {
    pub output_directory: PathBuf,
    pub random_phrase_generator: RandomPhraseGeneratorConfig,
    pub llm_structured_response: LlmStructuredResponseConfig,
    pub ai_images: AiImagesConfig,
    pub markdown_template_filler: MarkdownTemplateFillerConfig,
}

type LlmStructuredResponseConfig = CliConfigArgs;

#[derive(Debug, Deserialize, Default, Serialize, PartialEq)]
pub struct RandomPhraseGeneratorConfig {
    pub csv_files: Vec<PathBuf>,
}

type AiImagesConfig = GenerationParameters;

#[derive(Debug, Deserialize, Default, Serialize, PartialEq)]
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
    fn generate_structured_response(&self, initial_prompt: &str) -> Result<String> {
        // Define the schema for the structured response
        let schema_text: String =
            fs::read_to_string(&self.llm_structured_response.json_schema_file)?;
        let schema: Value = from_str(&schema_text)?;
        let schema: StructuredOutputFormat = serde_json::from_value(schema)?;

        // Create the prompt object
        let prompt = Prompt {
            system: self.llm_structured_response.system_prompt.clone(),
            initial: initial_prompt.to_string(),
        };

        // Generate a default LLM configuration for the provider, if not provided
        let config = match self.llm_structured_response.provider_config {
            Some(ref config) => config.clone(),
            None => LlmProviderConfig::default_for_provider(&self.llm_structured_response.provider),
        };

        // Send the initial prompt to the LLM API to get a structured response
        let llm_structured_response = self
            .llm_structured_response
            .provider
            .request_structured_response(&config, schema, &prompt)?;

        Ok(llm_structured_response)
    }

    /// Generate an image based on the structured response
    fn generate_image(&self, prompt_from_response: &str) -> Result<PathBuf> {
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
        let rt = Runtime::new()?;
        let image = rt.block_on(async {
            let image_params = image_params.clone();
            tokio::spawn(async move { provider.generate_image(image_params).await }).await?
        })?;
        Ok(image)
    }

    /// Fill the markdown template with the image and the structured response
    fn fill_template(&self, structured_response: &Map<String, Value>) -> Result<String> {
        let template = fs::read_to_string(&self.markdown_template_filler.template_file_path)?;
        let mut env = Environment::new();
        env.add_template("template", &template)?;
        let templ = env.get_template("template")?;
        let rendered = templ.render(structured_response)?;
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
    pub fn from_config_file_and_prompt(config_file: &Path, prompt: Option<&str>) -> Result<Asset> {
        let config = AssetConfig::from_toml_file(config_file)?;
        let asset = Asset::from_config(&config, prompt)?;
        Ok(asset)
    }

    pub fn from_config(config: &AssetConfig, user_prompt: Option<&str>) -> Result<Asset> {
        let initial_prompt = match user_prompt {
            Some(prompt) => prompt.to_string(),
            _ => config.generate_random_phrase()?,
        };

        let llm_structured_response = config.generate_structured_response(&initial_prompt)?;
        let llm_structured_response: Value = serde_json::from_str(&llm_structured_response)?;

        // Generate an image based on the structured response and save it
        let image_prompt: &Value = llm_structured_response
            .get("image_prompt")
            .unwrap_or(&Value::Null);
        let image_path: Option<PathBuf> = match image_prompt {
            Value::String(prompt) => Some(config.generate_image(prompt)?),
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
        let markdown: Result<String> = config.fill_template(&llm_structured_response);

        // If the markdown template is not filled, print an error message and save the structured response to a file
        let markdown = match markdown {
            Ok(markdown) => markdown,
            Err(e) => {
                eprintln!("Error filling the markdown template: {}", e);
                let structured_response_file_path = config
                    .output_directory
                    .join(format!("{}.json", chrono::Utc::now().timestamp()));
                // Convert the structured response to a JSON string
                let llm_structured_response =
                    serde_json::to_string_pretty(&llm_structured_response)?;
                // Save the structured response to a file
                fs::write(&structured_response_file_path, llm_structured_response)?;
                return Err(Error::msg(format!(
                    "The markdown template was not filled. The structured response was saved to {:?}",
                    structured_response_file_path
                )));
            }
        };

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
    use serial_test::serial;
    use tempfile::{TempDir, tempdir};

    #[cfg(test)]
    mod default_config {
        use super::*;

        fn generate_default_config_file(dir: &TempDir) -> Result<PathBuf> {
            let config_file_path = dir.path().join("test-config.toml");
            let config = toml::to_string(&AssetConfig::default())?;
            fs::write(&config_file_path, &config)?;
            Ok(config_file_path)
        }

        #[test]
        fn test_deserialize_default_config_file() -> Result<()> {
            let dir = tempdir()?;
            let config_file = generate_default_config_file(&dir)?;
            let config = fs::read_to_string(&config_file)?;
            let config: AssetConfig = toml::from_str(&config)?;
            assert_eq!(config, AssetConfig::default());
            // Clean up
            drop(config_file);
            dir.close()?;
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
        use llm_structured_response::LlmProviders;

        const CONFIG_FILE_PATH: &str = "test-ollama-config.toml";
        const JSON_SCHEMA_FILE_NAME: &str = "test-ollama-schema.json";

        /// Generate a temporary JSON schema file used for testing
        fn generate_schema_file(dir: &TempDir) -> Result<PathBuf> {
            let schema = r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "http://example.com/example.schema.json",
    "name": "Example",
    "description": "An example schema in JSON",
    "schema": {
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
    }
}"#;
            let schema_file_path = dir.path().join(JSON_SCHEMA_FILE_NAME);
            fs::write(&schema_file_path, schema)?;
            Ok(schema_file_path)
        }

        /// Generate a temporary configuration file for testing
        fn generate_default_config() -> AssetConfig {
            let mut config = AssetConfig::default();
            config.llm_structured_response.provider = LlmProviders::Ollama;
            config.llm_structured_response.provider_config = Some(
                LlmProviderConfig::default_for_provider(&config.llm_structured_response.provider),
            );
            config.llm_structured_response.system_prompt = "System prompt".to_string();
            config.llm_structured_response.json_schema_file = PathBuf::from(JSON_SCHEMA_FILE_NAME);
            config
        }

        fn generate_default_config_file(
            dir: &TempDir,
            json_schema_file_path: &Path,
        ) -> Result<PathBuf> {
            let mut config = generate_default_config();
            config.llm_structured_response.json_schema_file = json_schema_file_path.to_path_buf();
            config.output_directory = dir.path().to_path_buf();
            let config = toml::to_string(&config)?;
            let config_file_path = dir.path().join(CONFIG_FILE_PATH);
            fs::write(&config_file_path, &config)?;
            Ok(config_file_path)
        }

        #[test]
        #[serial(ollama, local_server)]
        fn test_ollama_config() -> Result<()> {
            let dir = tempdir()?;
            let schema_file_path = generate_schema_file(&dir)?;
            let mut config = generate_default_config();
            config.llm_structured_response.json_schema_file = schema_file_path.clone();
            config.output_directory = dir.path().to_path_buf();
            let prompt: &str = "Prompt";
            // Creating the asset will probably fail since the markdown template doesn't exist.
            // That's expected though: all we care about is whether we can get a structured response.
            let asset = Asset::from_config(&config, Some(prompt));
            // Clean up
            fs::remove_file(schema_file_path)?;
            dir.close()?;
            // Check that the error is a "file not found" error.
            match asset {
                Err(e) => {
                    assert!(
                        e.to_string()
                            .contains("The markdown template was not filled.")
                    );
                }
                _ => return Err(Error::msg("Expected an error.")),
            }
            Ok(())
        }

        #[test]
        #[serial(ollama, local_server)]
        fn test_ollama_config_from_file() -> Result<()> {
            let dir = tempdir()?;
            let schema_file_path = generate_schema_file(&dir)?;
            let config_file_path = generate_default_config_file(&dir, &schema_file_path)?;
            let prompt = "Prompt";
            // Creating the asset will probably fail since the markdown template doesn't exist.
            // That's expected though: all we care about is whether we can get a structured response.
            let asset = Asset::from_config_file_and_prompt(&config_file_path, Some(prompt));
            // Clean up
            fs::remove_file(schema_file_path)?;
            fs::remove_file(config_file_path)?;
            dir.close()?;
            // Check that the error is a "file not found" error.
            match asset {
                Err(e) => {
                    dbg!(e.to_string());
                    assert!(
                        e.to_string()
                            .contains("The markdown template was not filled.")
                    );
                }
                _ => return Err(Error::msg("Expected an error.")),
            }
            Ok(())
        }
    }
}
