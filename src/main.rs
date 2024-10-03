use std::time::Duration;

use clap::Parser;
use dotenvy::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use rpg_asset_generator::{generate_asset, Cli};

#[tokio::main]
async fn main() {
    // Load environment variables from a .env file.
    dotenv().ok();

    // Parse command-line arguments.
    let cli = Cli::parse();

    // Create a progress bar.
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Generating asset...");
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"]),
    );

    // Generate the asset.
    let asset = generate_asset(cli.asset_type, cli.prompt).await;

    // Finish the progress bar.
    spinner.finish_and_clear();

    // Print the asset or an error message.
    match asset {
        Ok(asset) => {
            println!("{}", asset);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
