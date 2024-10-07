mod initial_prompt;
use super::Asset;
use initial_prompt::generate_initial_prompt;

pub struct Building;

impl Asset for Building {
    const JSON_SCHEMA: &'static str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/rpg-generation-assets/buildings/waterdeep-building.schema.json"
    ));

    const SYSTEM_PROMPT: &'static str = "You are a game master creating a new building for a Dungeons & Dragons RPG campaign set in the city of Waterdeep. Your descriptions should be concise but detailed. Use descriptive prose, but don't be overly verbose: keep each of your descriptions between 1-3 sentences. Ensure that the details you generate are appropriate for a fantasy setting.";

    const MARKDOWN_TEMPLATE: &'static str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/rpg-generation-assets/buildings/building.md"
    ));

    fn generate_initial_prompt() -> Result<String, Box<dyn std::error::Error>> {
        generate_initial_prompt().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}
