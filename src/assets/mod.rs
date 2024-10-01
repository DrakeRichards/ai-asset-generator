//! Resulting schemas for assets that can be generated.
//! Each schema contains the full definition of the asset.

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
//use serde_json_schema::Schema;

/*
mod character {
    include!(concat!(env!("OUT_DIR"), "/character.rs"));
}

mod building {
    include!(concat!(env!("OUT_DIR"), "/waterdeep-building.rs"));
}
*/

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
pub trait Asset: Serialize {
    /// Get the schema for the asset as a JSON string.
    fn schema_string(&self) -> &str;
    /// Get the schema for the asset as a JSON value.
    fn schema_value(&self) -> Value;
}

impl Asset for AssetType {
    fn schema_string(&self) -> &str {
        match self {
            AssetType::Character => CHARACTER_SCHEMA,
            AssetType::Location => LOCATION_SCHEMA,
        }
    }
    fn schema_value(&self) -> Value {
        match self {
            AssetType::Character => json!(CHARACTER_SCHEMA),
            AssetType::Location => json!(LOCATION_SCHEMA),
        }
    }
}
