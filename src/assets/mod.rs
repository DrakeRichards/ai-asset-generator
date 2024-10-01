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

/// Assets can be serialized to JSON. They also follow a specific schema.
pub trait Schema {
    /// Get the schema for the asset as a JSON string.
    fn schema(&self) -> &str;
}

impl Schema for AssetType {
    fn schema(&self) -> &str {
        match self {
            AssetType::Character => CHARACTER_SCHEMA,
            AssetType::Location => LOCATION_SCHEMA,
        }
    }
}
