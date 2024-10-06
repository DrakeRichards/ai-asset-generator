//! Resulting schemas for assets that can be generated.
//! Each schema contains the full definition of the asset.

mod building;
mod character;
mod markdown;
mod random_prompts;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum, Serialize, Deserialize)]
pub enum AssetType {
    Character,
    Location,
}

trait Asset {
    const JSON_SCHEMA: &'static str;
    const SYSTEM_PROMPT: &'static str;
}

impl AssetType {
    /// The schema for the asset as a JSON string.
    pub fn schema(&self) -> &str {
        match self {
            AssetType::Character => character::Character::JSON_SCHEMA,
            AssetType::Location => building::Building::JSON_SCHEMA,
        }
    }

    /// The system prompt to use when generating the asset.
    pub fn system_prompt(&self) -> &str {
        match self {
            AssetType::Character => character::Character::SYSTEM_PROMPT,
            AssetType::Location => building::Building::SYSTEM_PROMPT,
        }
    }

    /// Generate an initial prompt for the asset.
    pub fn generate_initial_prompt(&self) -> Result<String, std::io::Error> {
        match self {
            AssetType::Character => random_prompts::character::generate_initial_prompt(),
            AssetType::Location => random_prompts::building::generate_initial_prompt(),
        }
    }

    // Add an image from DALL-E to the response.
    // The image prompt is from the response property "dallePrompt", if it exists.
    async fn generate_image(
        response: &str,
        output_directory: PathBuf,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let image_prompt: Option<String> =
            crate::json::get_string_value(&response, "dallePrompt").ok();

        // If the response does not contain an image prompt, return the response as is.
        if image_prompt.is_none() {
            return Ok(response.to_string());
        }

        let image_prompt: String = image_prompt.expect("Image prompt should exist.");
        dbg!(&image_prompt);

        let image_path: PathBuf =
            crate::openai::image::generate_request(&image_prompt, output_directory).await?;

        let modified_response = crate::json::add_string_property(
            &response,
            "imageFileName",
            image_path
                .file_name()
                .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))?
                .to_str()
                .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))?,
        )?;

        Ok(modified_response)
    }

    /// Generate an asset using OpenAI's API.
    pub async fn generate_asset(
        &self,
        prompt: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Generate the response from OpenAI's API.
        let mut response: String = crate::openai::text::generate_request(self, prompt).await?;

        // Add an image from DALL-E to the response.
        let output_directory: PathBuf = PathBuf::from(".");
        response = AssetType::generate_image(&response, output_directory).await?;

        // Do some post-processing on the response.
        match self {
            AssetType::Character => {
                // Add the first name to the response schema.
                let first_name: String = character::get_first_name(&response)?;
                response = crate::json::add_string_property(&response, "firstName", &first_name)?;
            }
            AssetType::Location => {}
        }

        Ok(response)
    }

    /// Convert the response from OpenAI's API to a Markdown string.
    pub fn to_markdown(&self, response: &str) -> serde_json::Result<String> {
        match self {
            AssetType::Character => {
                let character: HashMap<&str, String> = match serde_json::from_str(response) {
                    Ok(character) => character,
                    Err(e) => return Err(e),
                };
                Ok(markdown::fill_markdown_template(
                    markdown::CHARACTER_TEMPLATE,
                    character,
                ))
            }

            AssetType::Location => {
                let location: HashMap<&str, String> = match serde_json::from_str(response) {
                    Ok(location) => location,
                    Err(e) => return Err(e),
                };
                Ok(markdown::fill_markdown_template(
                    markdown::BUILDING_TEMPLATE,
                    location,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::expect_used)]
    use super::*;

    #[test]
    fn test_asset_type_schema() {
        let character_schema: &str = character::Character::JSON_SCHEMA;
        let location_schema: &str = building::Building::JSON_SCHEMA;

        assert!(!character_schema.is_empty());
        assert!(!location_schema.is_empty());
    }

    #[test]
    fn test_asset_type_system_prompt() {
        let character_prompt: &str = AssetType::Character.system_prompt();
        let location_prompt: &str = AssetType::Location.system_prompt();

        assert!(!character_prompt.is_empty());
        assert!(!location_prompt.is_empty());
    }
}
