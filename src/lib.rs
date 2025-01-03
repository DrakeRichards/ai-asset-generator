#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

mod ai_images;
mod config;
mod llm_structured_response;
mod random_phrase_generator;

use anyhow::{Error, Result};
pub use config::Config;
use llm_structured_response::request::{Prompt, Schema};
use minijinja::Environment;
use random_phrase_generator::RandomphraseGenerator;
use serde::Serialize;
use serde_json::{Map, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

impl Config {
    pub fn from_toml_file(config_file: &Path) -> Result<Self> {
        let config = fs::read_to_string(config_file)?;
        let config: Config = toml::from_str(&config)?;
        Ok(config)
    }

    pub async fn generate_asset(&self, user_prompt: Option<&str>) -> Result<Asset> {
        let initial_prompt = match user_prompt {
            Some(prompt) => prompt.to_string(),
            None => self.generate_random_phrase()?,
        };

        let llm_structured_response = self.generate_structured_response(&initial_prompt).await?;
        let llm_structured_response: Value = serde_json::from_str(&llm_structured_response)?;

        // Generate an image based on the structured response and save it
        let image_prompt = llm_structured_response
            .get("image_prompt")
            .unwrap_or(&Value::Null);
        let image_path = match image_prompt {
            Value::String(prompt) => Some(self.generate_image(prompt).await?),
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
            None => None,
        };
        // Add the image name to the structured response
        let mut llm_structured_response = llm_structured_response
            .as_object()
            .ok_or(Error::msg("Unable to convert response to an object."))?
            .clone();
        if let Some(image_filename) = image_filename {
            llm_structured_response
                .insert("image_file_name".to_string(), Value::String(image_filename));
        }

        // Fill the markdown template with the image and the structured response
        let markdown = self.fill_template(llm_structured_response)?;

        // If output_dir is empty, save the markdown to the current directory
        let output_dir = if self.output_directory == PathBuf::new() {
            PathBuf::from(".")
        } else {
            self.output_directory.clone()
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

    /// Generate the initial prompt if not provided by the user
    fn generate_random_phrase(&self) -> Result<String> {
        let csv_files: Vec<&Path> = self
            .random_phrase_generator
            .csv_files
            .iter()
            .map(|path| path.as_path())
            .collect();
        let random_phrase_generator: RandomphraseGenerator =
            RandomphraseGenerator::from_csv_files(csv_files)?;
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
                schema,
                prompt,
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
