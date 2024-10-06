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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_markdown_template() {
        let template = "This is a template with a {{key}}.";
        let mut values = HashMap::new();
        values.insert("key", "value".to_string());

        let filled_template = fill_markdown_template(template, values);

        assert_eq!(filled_template, "This is a template with a value.");
    }

    #[test]
    fn test_fill_markdown_template_multiple_values() {
        let template = "This is a template with a {{key1}} and a {{key2}}.";
        let mut values = HashMap::new();
        values.insert("key1", "value1".to_string());
        values.insert("key2", "value2".to_string());

        let filled_template = fill_markdown_template(template, values);

        assert_eq!(
            filled_template,
            "This is a template with a value1 and a value2."
        );
    }

    #[test]
    fn test_fill_markdown_template_missing_value() {
        let template = "This is a template with a {{key}}.";
        let values = HashMap::new();

        let filled_template = fill_markdown_template(template, values);

        assert_eq!(filled_template, "This is a template with a {{key}}.");
    }
}
