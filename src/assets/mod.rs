//! Resulting schemas for assets that can be generated.
//! Each schema contains the full definition of the asset.

mod building;
mod character;
mod markdown;
mod random_prompts;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

    /// Generate an asset using OpenAI's API.
    pub async fn generate_asset(
        &self,
        prompt: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let response: Result<String, Box<dyn std::error::Error>> =
            crate::openai::generate_request(self, prompt).await;
        // Do some post-processing on the response.
        match self {
            AssetType::Character => match response {
                Ok(response) => {
                    // Add the first name to the response schema.
                    let first_name: String = character::get_first_name(&response)?;
                    let modified_response: String =
                        crate::json::add_string_property(&response, "firstName", &first_name)?;
                    Ok(modified_response)
                }
                Err(e) => Err(e),
            },
            AssetType::Location => match response {
                Ok(response) => Ok(response),
                Err(e) => Err(e),
            },
        }
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
