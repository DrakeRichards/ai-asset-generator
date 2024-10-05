//! Resulting schemas for assets that can be generated.
//! Each schema contains the full definition of the asset.

mod markdown;
mod random_prompts;
mod schemas;

use clap::ValueEnum;
use markdown::fill_markdown_template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum, Serialize, Deserialize)]
pub enum AssetType {
    Character,
    Location,
}

impl AssetType {
    /// The schema for the asset as a JSON string.
    pub fn schema(&self) -> &str {
        match self {
            AssetType::Character => schemas::CHARACTER_SCHEMA,
            AssetType::Location => schemas::LOCATION_SCHEMA,
        }
    }

    /// The system prompt to use when generating the asset.
    pub fn system_prompt(&self) -> &str {
        match self {
            AssetType::Character => "You are a game master creating a new character for a Dungeons & Dragons RPG campaign set in the city of Waterdeep. Your descriptions should be concise but detailed. Use descriptive prose, but don't be overly verbose: keep each of your descriptions between 1-3 sentences. Ensure that the details you generate are appropriate for a fantasy setting.",
            AssetType::Location => "You are a game master creating a new building for a Dungeons & Dragons RPG campaign set in the city of Waterdeep. Your descriptions should be concise but detailed. Use descriptive prose, but don't be overly verbose: keep each of your descriptions between 1-3 sentences. Ensure that the details you generate are appropriate for a fantasy setting.",
        }
    }

    /// Generate an initial prompt for the asset.
    pub fn generate_initial_prompt(&self) -> Result<String, std::io::Error> {
        match self {
            AssetType::Character => random_prompts::character::generate_initial_prompt(),
            AssetType::Location => random_prompts::building::generate_initial_prompt(),
        }
    }

    pub fn to_markdown(&self, response: &str) -> serde_json::Result<String> {
        match self {
            AssetType::Character => {
                let character: HashMap<&str, String> = match serde_json::from_str(response) {
                    Ok(character) => character,
                    Err(e) => return Err(e),
                };
                Ok(fill_markdown_template(
                    markdown::CHARACTER_TEMPLATE,
                    character,
                ))
            }
            AssetType::Location => {
                let location: HashMap<&str, String> = match serde_json::from_str(response) {
                    Ok(location) => location,
                    Err(e) => return Err(e),
                };
                Ok(fill_markdown_template(
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
        let character_schema: &str = schemas::CHARACTER_SCHEMA;
        let location_schema: &str = schemas::LOCATION_SCHEMA;

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
