//! Resulting schemas for assets that can be generated.
//! Each schema contains the full definition of the asset.

mod building;
mod character;

use crate::image_generation::ImageProviders;
use crate::json::{get_schema_description, get_schema_title};
use crate::text_generation::openai::request_structured_response;
use anyhow::Result;
use clap::ValueEnum;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// The type of asset to generate.
#[derive(ValueEnum, Clone)]
pub enum AssetType {
    Character,
    Building,
}

impl AssetType {
    pub async fn generate_asset_markdown(
        &self,
        prompt: Option<String>,
        output_directory: &Path,
        image_provider: ImageProviders,
    ) -> Result<String> {
        match self {
            AssetType::Character => {
                character::Character::generate(prompt, output_directory, image_provider).await
            }
            AssetType::Building => {
                building::Building::generate(prompt, output_directory, image_provider).await
            }
        }
    }
}

/// A trait for assets that can be generated.
trait Asset {
    /// The JSON schema for the asset.
    const JSON_SCHEMA: &'static str;

    /// The system prompt for the asset.
    const SYSTEM_PROMPT: &'static str;

    /// The Markdown template for the asset.
    const MARKDOWN_TEMPLATE: &'static str;

    /// Generate the initial prompt for the asset.
    fn generate_initial_prompt() -> Result<String>;

    /// Add an image from an ImageProvider to the response.
    /// The image prompt is from the response property "imagePrompt", if it exists.
    async fn generate_image(
        response: &str,
        output_directory: &Path,
        image_provider: ImageProviders,
    ) -> Result<String> {
        // Get the image prompt from the response.
        // If the response does not contain an image prompt, return the response as-is.
        let image_prompt: String = match crate::json::get_string_value(response, "imagePrompt") {
            Ok(image_prompt) => image_prompt,
            Err(_) => return Ok(response.to_string()),
        };

        // Generate the image from OpenAI's API and get the path to the generated image.
        let image_path: PathBuf = image_provider
            .generate_image(&image_prompt, output_directory)
            .await?;

        // Add the image file name to the response.
        let modified_response = crate::json::add_string_property(
            response,
            "imageFileName",
            image_path
                .file_name()
                .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))?
                .to_str()
                .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))?,
        )?;

        Ok(modified_response)
    }

    /// Perform any post-processing on the response.
    fn post_process_response(response: &str) -> Result<String> {
        // Do nothing by default.
        Ok(response.to_string())
    }

    /// Fill the Markdown template with the response.
    /// The template contains placeholders in the form of "{{key}}" that will be replaced with the corresponding value from the response.
    fn fill_markdown_template(template: &str, response: HashMap<&str, String>) -> String {
        let mut markdown = template.to_string();
        for (key, value) in response {
            markdown = markdown.replace(&format!("{{{{{}}}}}", key), &value);
        }
        markdown
    }

    /// Generate the asset.
    async fn generate(
        prompt: Option<String>,
        output_directory: &Path,
        image_provider: ImageProviders,
    ) -> Result<String> {
        // Get the schema name and description.
        let schema_name: String = get_schema_title(Self::JSON_SCHEMA)?;
        let schema_description: Option<String> = get_schema_description(Self::JSON_SCHEMA);

        // Generate the initial prompt for the asset.
        let initial_prompt = match prompt {
            Some(prompt) => prompt,
            None => Self::generate_initial_prompt()?,
        };

        // Generate the response from OpenAI's API.
        let mut response: String = request_structured_response(
            &schema_name,
            schema_description,
            Self::JSON_SCHEMA,
            initial_prompt,
            Self::SYSTEM_PROMPT.to_string(),
        )
        .await?;

        // Add an image from DALL-E to the response.
        response = Self::generate_image(&response, output_directory, image_provider).await?;

        // Do some post-processing on the response.
        response = Self::post_process_response(&response)?;

        // Generate the Markdown file from the response.
        let response: HashMap<&str, String> = serde_json::from_str(&response)?;
        Ok(Self::fill_markdown_template(
            Self::MARKDOWN_TEMPLATE,
            response,
        ))
    }
}
