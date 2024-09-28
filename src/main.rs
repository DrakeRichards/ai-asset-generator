mod assets;
mod random_prompts;
mod weighted_random;

use assets::AssetType;
use clap::Parser;
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

fn main() {
    let cli = Cli::parse();
    let asset_type: AssetType = cli.asset_type;
    let initial_prompt: String = match cli.prompt {
        Some(prompt) => prompt,
        None => asset_type.generate_initial_prompt(),
    };
    println!("Initial prompt: {}", initial_prompt);
    //let asset = generate_asset(asset_type, initial_prompt);
    //println!("{}", asset.to_json_string().unwrap());
}
