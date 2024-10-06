use super::Asset;

pub struct Building;

impl Asset for Building {
    const JSON_SCHEMA: &'static str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/rpg-generation-assets/buildings/waterdeep-building.schema.json"
    ));
    const SYSTEM_PROMPT: &'static str = "You are a game master creating a new building for a Dungeons & Dragons RPG campaign set in the city of Waterdeep. Your descriptions should be concise but detailed. Use descriptive prose, but don't be overly verbose: keep each of your descriptions between 1-3 sentences. Ensure that the details you generate are appropriate for a fantasy setting.";
}
