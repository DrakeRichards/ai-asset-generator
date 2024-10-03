//! Resulting schemas for assets that can be generated.
//! Each schema contains the full definition of the asset.

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

const CHARACTER_SCHEMA: &str =
    include_str!("../../rpg-generation-assets/characters/character.schema.json");
const LOCATION_SCHEMA: &str =
    include_str!("../../rpg-generation-assets/buildings/waterdeep-building.schema.json");

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum, Serialize, Deserialize)]
pub enum AssetType {
    Character,
    Location,
}

/// Trait for assets that can be generated.
pub trait Asset {
    /// The schema for the asset as a JSON string.
    fn schema(&self) -> &str;
    /// The system prompt to use when generating the asset.
    fn system_prompt(&self) -> &str;
}

impl Asset for AssetType {
    fn schema(&self) -> &str {
        match self {
            AssetType::Character => CHARACTER_SCHEMA,
            AssetType::Location => LOCATION_SCHEMA,
        }
    }
    fn system_prompt(&self) -> &str {
        match self {
            AssetType::Character => "You are a game master creating a new character for a Dungeons & Dragons RPG campaign set in the city of Waterdeep. Your descriptions should be concise but detailed. Use descriptive prose, but don't be overly verbose: keep each of your descriptions between 1-3 sentences. Ensure that the details you generate are appropriate for a fantasy setting.",
            AssetType::Location => "You are a game master creating a new building for a Dungeons & Dragons RPG campaign set in the city of Waterdeep. Your descriptions should be concise but detailed. Use descriptive prose, but don't be overly verbose: keep each of your descriptions between 1-3 sentences. Ensure that the details you generate are appropriate for a fantasy setting.",
        }
    }
}
