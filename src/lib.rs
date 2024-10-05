#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

mod assets;
mod openai;
use assets::AssetType;
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

/// Generate an asset using OpenAI's API and generate a Markdown file for it.
///
/// The openai_async crate is used to send the request to OpenAI's API. This crate requires that the `OPENAI_API_KEY` environment variable be set.
pub async fn generate_asset(
    asset_type: AssetType,
    prompt: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    // Check if the environment variable is set.
    if std::env::var("OPENAI_API_KEY").is_err() {
        return Err("The OPENAI_API_KEY environment variable is not set.".into());
    }

    // Generate a semi-random initial prompt.
    let initial_prompt: String = match prompt {
        Some(prompt) => prompt,
        None => asset_type.generate_initial_prompt()?,
    };

    // Send the request to OpenAI's API.
    let response = openai::generate_request(&asset_type, initial_prompt).await?;

    // Generate a Markdown file for the asset.
    let markdown: String = asset_type.to_markdown(&response)?;

    Ok(markdown)
}
