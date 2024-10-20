use anyhow::{Error, Result};
use clap::Parser;
use dotenvy::dotenv;
use rpg_asset_generator::{generate_asset, Cli};

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

    // Generate the asset and save it to a file.
    let return_string = generate_asset(
        cli.asset_type,
        cli.prompt,
        cli.output_directory,
        cli.image_provider,
        cli.what_if,
        cli.json,
    )
    .await?;

    println!("{}", return_string);

    Ok(())
}
