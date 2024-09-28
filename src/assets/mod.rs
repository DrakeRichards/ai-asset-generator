//! Resulting schemas for assets that can be generated.
//! Each schema contains the full definition of the asset.

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum, Serialize, Deserialize)]
pub enum AssetType {
    Character,
    Location,
}

/// Assets can be serialized to JSON.
pub trait Asset: Serialize {
    /// Convert the asset to a JSON string.
    fn to_json_string(&self) -> Result<String>;
}

impl Asset for AssetType {
    fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(&self)
    }
}
