use std::path::PathBuf;

use ai_asset_generator::Config;
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = AssetGenerator::parse();
    let config = Config::from_toml_file(&args.config_file)?;
    let asset = config.generate_asset(args.prompt.as_deref()).await?;
    // Print the paths to the generated asset as a JSON string
    println!("{}", serde_json::to_string(&asset)?);
    Ok(())
}

/// Generate an asset based on the configuration file
#[derive(Parser)]
#[command(about)]
struct AssetGenerator {
    /// Path to the configuration file
    #[clap(short, long)]
    config_file: PathBuf,

    /// Optional prompt to generate the asset
    #[clap(short, long)]
    prompt: Option<String>,
}
