use clap::Parser;
use dotenvy::dotenv;
use rpg_asset_generator::{generate_asset, Cli};

#[tokio::main]
async fn main() {
    // Load environment variables from a .env file.
    dotenv().ok();

    // Parse command-line arguments.
    let cli = Cli::parse();

    // Generate the asset.
    let asset = generate_asset(cli.asset_type, cli.prompt).await;

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
