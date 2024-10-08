use anyhow::{Error, Result};
use clap::Parser;
use dotenvy::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use rpg_asset_generator::{generate_asset, Cli};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from a .env file.
    dotenv().ok();

    // Parse command-line arguments.
    let cli: Cli = Cli::parse();

    // Check if the environment variable is set.
    if std::env::var("OPENAI_API_KEY").is_err() {
        return Err(Error::msg(
            "The OPENAI_API_KEY environment variable is not set.",
        ));
    }

    // Create a progress bar.
    let spinner: ProgressBar = ProgressBar::new_spinner();
    spinner.set_message("Generating asset...");
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"]),
    );

    // Generate the asset and save it to a file.
    generate_asset(
        cli.asset_type,
        cli.prompt,
        cli.output_directory,
        cli.image_provider,
    )
    .await?;

    // Finish the progress bar.
    spinner.finish_and_clear();

    Ok(())
}
