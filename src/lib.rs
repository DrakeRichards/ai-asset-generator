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
/// If the what-if flag is set, print the asset to stdout instead of saving to a file.
/// If the JSON flag is set, return the JSON schema for the asset instead of the filled Markdown template.
/// Otherwise, returns the path to the saved file.
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
                Ok(asset)
            } else {
                let mut output_file = save_asset_to_file(&asset, output_directory)?
                    .as_os_str()
                    .to_string_lossy()
                    .to_string();
                // If output_file starts with "\\?\", remove it.
                if output_file.starts_with(r#"\\?\"#) {
                    output_file = output_file[4..].to_string();
                }
                Ok(output_file)
            }
        }
        Err(e) => Err(e),
    }
}

/// Save the asset to a file. The file name is the text of the first H1 heading in the asset. If no H1 heading is found, the file name is "asset.md". Returns the path to the saved file.
pub fn save_asset_to_file(asset: &str, output_directory: PathBuf) -> Result<PathBuf> {
    let file_name: String =
        crate::markdown::get_first_h1_heading(asset).unwrap_or_else(|| "asset".to_string());
    let file_path: PathBuf = Path::new(&output_directory).join(format!("{}.md", file_name));
    let _ = std::fs::write(&file_path, asset);
    // Resolve the file path into an absolute path.
    let file_path: PathBuf = file_path.canonicalize()?;
    Ok(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_generate_asset() {
        // Set the environment variable for the OpenAI API key.
        env::set_var("OPENAI_API_KEY", "test");

        let asset_type = AssetType::Character;
        let prompt = Some("A character with a sword.".to_string());
        let output_directory = Some(".".to_string());
        let image_provider = Some(ImageProviders::OpenAi);
        let what_if = false;
        let as_json = false;

        let asset = generate_asset(
            asset_type,
            prompt,
            output_directory,
            image_provider,
            what_if,
            as_json,
        )
        .await;
        assert!(asset.is_ok());
    }

    #[test]
    fn test_save_asset_to_file() -> Result<()> {
        let asset = "# Title\n\nThis is the asset.";
        let output_directory = PathBuf::from(".");
        let file_path = save_asset_to_file(asset, output_directory)?;
        assert!(file_path.exists());
        Ok(())
    }
}
