#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

mod assets;
mod openai;
mod random_prompts;

use crate::assets::AssetType;
use crate::random_prompts::RandomPrompt;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Cli {
    /// The type of asset to generate.
    #[arg(short = 't', long = "type", value_enum)]
    pub asset_type: assets::AssetType,
    /// Use a specific prompt instead of generating a random one.
    #[arg(short, long)]
    pub prompt: Option<String>,
}

pub async fn generate_asset(
    asset_type: AssetType,
    prompt: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    // Generate a semi-random initial prompt.
    let initial_prompt: String = match prompt {
        Some(prompt) => prompt,
        None => asset_type.generate_initial_prompt()?,
    };

    // Send the request to OpenAI's API.
    let response = openai::generate_request(asset_type, initial_prompt).await?;

    Ok(response)
}
