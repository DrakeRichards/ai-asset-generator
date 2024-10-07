#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

mod assets;
mod json;
mod markdown;
mod openai;
mod weighted_random;

use crate::assets::AssetType;
use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct Cli {
    /// The type of asset to generate.
    #[arg(value_enum, short, long)]
    pub asset_type: AssetType,
    /// The initial prompt to use when generating the asset.
    #[arg(short, long)]
    pub prompt: Option<String>,
    /// The output directory for the generated asset.
    #[arg(short, long)]
    pub output_directory: Option<String>,
}

/// Generate an asset using OpenAI's API and generate a Markdown file for it.
///
/// The openai_async crate is used to send the request to OpenAI's API. This crate requires that the `OPENAI_API_KEY` environment variable be set.
pub async fn generate_asset(
    asset_type: AssetType,
    prompt: Option<String>,
    output_directory: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let output_directory: PathBuf =
        Path::new(&output_directory.unwrap_or_else(|| ".".to_string())).to_path_buf();
    let asset = asset_type
        .generate_asset_markdown(prompt, &output_directory)
        .await;
    match asset {
        Ok(asset) => {
            save_asset_to_file(&asset, output_directory)?;
            Ok(asset)
        }
        Err(e) => Err(e),
    }
}

/// Save the asset to a file. The file name is the text of the first H1 heading in the asset. If no H1 heading is found, the file name is "asset.md".
pub fn save_asset_to_file(asset: &str, output_directory: PathBuf) -> Result<(), std::io::Error> {
    let file_name: String =
        crate::markdown::get_first_h1_heading(asset).unwrap_or_else(|| "asset".to_string());
    let file_path: PathBuf = Path::new(&output_directory).join(format!("{}.md", file_name));
    std::fs::write(file_path, asset)
}
