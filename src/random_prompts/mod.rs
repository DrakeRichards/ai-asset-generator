//! Generate a random prompt by picking from weighted lists.

mod building;
mod character;
mod weighted_random;

use crate::assets::AssetType;

pub trait RandomPrompt {
    fn generate_initial_prompt(&self) -> Result<String, std::io::Error>;
}

impl RandomPrompt for AssetType {
    fn generate_initial_prompt(&self) -> Result<String, std::io::Error> {
        match &self {
            AssetType::Character => character::generate_initial_prompt(),
            AssetType::Location => building::generate_initial_prompt(),
        }
    }
}
