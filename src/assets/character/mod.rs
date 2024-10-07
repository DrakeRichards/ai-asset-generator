mod initial_prompt;

use super::Asset;
use initial_prompt::generate_initial_prompt;

pub struct Character;

impl Asset for Character {
    const JSON_SCHEMA: &'static str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/rpg-generation-assets/characters/character.schema.json"
    ));
    const SYSTEM_PROMPT: &'static str = "You are a game master creating a new character for a Dungeons & Dragons RPG campaign set in the city of Waterdeep. Your descriptions should be concise but detailed. Use descriptive prose, but don't be overly verbose: keep each of your descriptions between 1-3 sentences. Ensure that the details you generate are appropriate for a fantasy setting.";
    const MARKDOWN_TEMPLATE: &'static str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/rpg-generation-assets/characters/character.md"
    ));

    fn generate_initial_prompt() -> Result<String, Box<dyn std::error::Error>> {
        generate_initial_prompt().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    fn post_process_response(response: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Add the first name to the response schema.
        let first_name: String = get_first_name(response)?;
        let response = crate::json::add_string_property(response, "firstName", &first_name)?;
        Ok(response)
    }
}

/// Get the first name of a character by splitting the "name" property on the first space.
fn get_first_name(response: &str) -> serde_json::Result<String> {
    // We can use expect here because we know that the "name" property is required in the schema.
    #![allow(clippy::expect_used)]
    let name = crate::json::get_string_value(response, "name")?;
    let first_name: String = name
        .split_whitespace()
        .next()
        .expect("'name' property should not be empty.")
        .to_string();
    Ok(first_name)
}
