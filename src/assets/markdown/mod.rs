//! Parse an asset into a Markdown file.

use std::collections::HashMap;

pub const CHARACTER_TEMPLATE: &str = include_str!("templates/character.md");
pub const BUILDING_TEMPLATE: &str = include_str!("templates/building.md");

/// Replace the values in a Markdown template with the provided values.
/// Values to replace in the template are surrounded by double curly braces.
/// When it finds a value to replace, it looks for the key in the provided values.
/// If the key is found, it replaces the value in the template with the value from the provided values.
/// If the key is not found, it replaces the value in the template with an empty string.
pub fn fill_markdown_template(template: &str, values: HashMap<&str, String>) -> String {
    let mut filled_template = template.to_string();

    for (key, value) in values {
        let key = format!("{{{{{}}}}}", key);
        filled_template = filled_template.replace(&key, &value);
    }

    filled_template
}
