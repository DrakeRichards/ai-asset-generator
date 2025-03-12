use ai_asset_generator::Asset;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    let args = AssetCli::parse();
    let asset =
        Asset::from_config_file_and_prompt(&args.config_file, args.prompt.as_deref()).await?;
    // Print the paths to the generated asset as a JSON string
    println!("{}", serde_json::to_string(&asset)?);
    Ok(())
}

/// Generate an asset based on the configuration file
#[derive(Parser)]
#[command(about)]
struct AssetCli {
    /// Path to the configuration file
    config_file: PathBuf,

    /// Optional prompt to generate the asset
    prompt: Option<String>,
}
