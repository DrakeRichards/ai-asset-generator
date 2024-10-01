mod assets;
mod openai;
mod random_prompts;
mod weighted_random;

use assets::AssetType;
use clap::Parser;
use dotenvy::dotenv;
use random_prompts::RandomPrompt;

/// Use OpenAI's ChatGPT to generate a random RPG asset, such as a character, item, or location.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Cli {
    /// The type of asset to generate.
    #[arg(short = 't', long = "type", value_enum)]
    asset_type: AssetType,
    /// Use a specific prompt instead of generating a random one.
    #[arg(short, long)]
    prompt: Option<String>,
}

#[tokio::main]
async fn main() {
    // Load environment variables from a .env file.
    dotenv().ok();

    // Parse command-line arguments.
    let cli = Cli::parse();

    // Generate a semi-random initial prompt.
    let asset_type: AssetType = cli.asset_type;
    let initial_prompt: String = match cli.prompt {
        Some(prompt) => prompt,
        None => asset_type.generate_initial_prompt(),
    };

    // Send the request to OpenAI's API.
    let response = openai::generate_request(asset_type, initial_prompt);

    // Handle the response.
    match response.await {
        Ok(response) => {
            // Print the response.
            println!("{}", response);
        }
        Err(e) => {
            // Print the error.
            eprintln!("{}", e);
        }
    }
}
