#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

mod assets;
mod image_generation;
mod json;
mod markdown;
mod text_generation;
mod weighted_random;

use crate::assets::AssetType;
use crate::image_generation::ImageProviders;
use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    long_about = "Generate an asset using OpenAI's API and save it to a markdown file. The openai_async crate is used to send the request to OpenAI's API. This crate requires that the `OPENAI_API_KEY` environment variable be set."
)]
/// Generate an asset using OpenAI's API and save it to a markdown file.
pub struct Cli {
    /// The type of asset to generate.
    #[clap(subcommand)]
    pub asset_type: AssetType,
    /// The initial prompt to use when generating the asset.
    #[arg(short, long)]
    pub prompt: Option<String>,
    /// The output directory for the generated asset.
    #[arg(short, long)]
    pub output_directory: Option<String>,
    /// The image provider to use when generating the asset.
    #[arg(short, long)]
    pub image_provider: Option<ImageProviders>,
    /// Output the filled markdown template to stdout instead of saving to a file.
    #[arg(short, long, action)]
    pub what_if: bool,
    /// Return the JSON schema for the asset instead of the filled Markdown template.
    #[arg(short, long, action)]
    pub json: bool,
}

/// Generate an asset and save it to a file.
pub async fn generate_asset(
    asset_type: AssetType,
    prompt: Option<String>,
    output_directory: Option<String>,
    image_provider: Option<ImageProviders>,
    what_if: bool,
    as_json: bool,
) -> Result<String> {
    // Convert the output directory to a PathBuf. If no output directory is specified, use the current directory.
    let output_directory: PathBuf =
        Path::new(&output_directory.unwrap_or_else(|| ".".to_string())).to_path_buf();
    // Generate the asset and return the Markdown.
    let asset = asset_type
        .generate_asset_markdown(prompt, &output_directory, image_provider, as_json)
        .await;
    // If the what-if flag is set, print the asset to stdout, then return.
    // Otherwise, save the asset to a file.
    match asset {
        Ok(asset) => {
            if what_if {
                println!("{}", asset);
            } else {
                save_asset_to_file(&asset, output_directory)?;
            }
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
