//! Functions to manipulate JSON strings.

use serde_json::{Result, Value};

/// Get a property's value from a JSON string.
/// The property must be a string.
pub fn get_property_value(schema_text: &str, property: &str) -> Option<String> {
    let schema: Value = serde_json::from_str(schema_text).ok()?;
    schema.get(property)?.as_str().map(|s| s.to_string())
}

/// Recursively remove any "default" properties from a JSON string.
/// This is necessary because the OpenAI API does not accept "default" properties.
fn remove_defaults(schema_text: &str) -> Result<String> {
    let mut schema: Value = serde_json::from_str(schema_text)?;

    // Recursively remove `default` properties
    fn recurse_remove_defaults(value: &mut Value) {
        match value {
            // If it's an object, potentially modify fields
            Value::Object(map) => {
                // Remove all "default" fields
                let keys_to_remove: Vec<String> = map
                    .iter()
                    .filter(|(k, _)| k == &"default")
                    .map(|(k, _)| k.clone())
                    .collect();

                for key in keys_to_remove {
                    map.remove(&key);
                }

                // Recursively process remaining fields
                for (_, v) in map.iter_mut() {
                    recurse_remove_defaults(v);
                }
            }
            // If it's an array, handle each element recursively
            Value::Array(arr) => {
                for v in arr {
                    recurse_remove_defaults(v);
                }
            }
            // Other types don't need modification
            _ => {}
        }
    }

    // Call the recursive function
    recurse_remove_defaults(&mut schema);

    // Serialize the modified JSON back into a string
    let cleaned_json = serde_json::to_string_pretty(&schema)?;

    Ok(cleaned_json)
}

/// Add a property for "additionalProperties" to a JSON string and set it to false.
/// "additionalProperties" is required by the OpenAI API. See <https://platform.openai.com/docs/guides/structured-outputs/additionalproperties-false-must-always-be-set-in-objects>.
fn add_additional_properties(schema_text: &str) -> Result<String> {
    let mut schema: Value = serde_json::from_str(schema_text)?;
    // Check if "additionalProperties" is already set.
    if schema.get("additionalProperties").is_some() {
        return Ok(schema.to_string());
    }
    schema.as_object_mut().map(|properties| {
        properties.insert("additionalProperties".to_string(), Value::Bool(false))
    });
    Ok(schema.to_string())
}

// Get a list of keys contained in a property.
fn get_property_keys(schema_text: &str, property_key: &str) -> Option<Vec<String>> {
    let schema: Value = serde_json::from_str(schema_text).ok()?;
    schema
        .get(property_key)?
        .as_object()
        .map(|properties| properties.keys().cloned().collect())
}

/// Add all properties to the root "required" array.
fn add_required_properties(schema_text: &str) -> Result<String> {
    let mut schema: Value = serde_json::from_str(schema_text)?;

    // Get a list of all properties.
    let all_properties: Vec<String> =
        get_property_keys(schema_text, "properties").expect("No properties found.");

    // Add the "required" array if it doesn't exist.
    if schema.get("required").is_none() {
        schema
            .as_object_mut()
            .map(|properties| properties.insert("required".to_string(), Value::Array(Vec::new())));
    }

    // What properties are already in the "required" array?
    let required: Vec<String> = schema
        .get("required")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();

    // What properties are missing from the "required" array?
    let missing: Vec<String> = all_properties
        .iter()
        .filter(|p| !required.contains(p))
        .cloned()
        .collect();

    // Add the missing properties to the "required" array.
    schema
        .get_mut("required")
        .unwrap()
        .as_array_mut()
        .unwrap()
        .extend(missing.iter().map(|p| Value::String(p.clone())));

    Ok(schema.to_string())
}

/// Clean up a schema for use with OpenAI's API.
pub fn clean_schema(schema_text: &str) -> Result<String> {
    let mut schema = remove_defaults(schema_text)?;
    schema = add_additional_properties(schema.as_str())?;
    schema = add_required_properties(schema.as_str())?;
    Ok(schema)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_property_value() {
        let schema = r#"{"title": "Character", "description": "A character in an RPG."}"#;
        assert_eq!(
            get_property_value(schema, "title"),
            Some("Character".to_string())
        );
        assert_eq!(
            get_property_value(schema, "description"),
            Some("A character in an RPG.".to_string())
        );
        assert_eq!(get_property_value(schema, "unknown"), None);
    }

    #[test]
    fn test_remove_defaults() {
        let schema = r#"{"title": "Character", "description": "A character in an RPG.", "properties": {"name": {"type": "string", "default": ""}}}"#;
        let expected = r#"{"title": "Character", "description": "A character in an RPG.", "properties": {"name": {"type": "string"}}}"#;
        let result: String = remove_defaults(schema).unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(result.as_str())
                .unwrap()
                .as_object(),
            serde_json::from_str::<Value>(expected).unwrap().as_object()
        );
    }

    #[test]
    fn test_nested_default_removal() {
        let schema = r#"{"title": "Character", "description": "A character in an RPG.", "properties": {"name": {"type": "string", "default": ""}, "stats": {"type": "object", "properties": {"strength": {"type": "number", "default": 0}}}}}"#;
        let expected = r#"{"title": "Character", "description": "A character in an RPG.", "properties": {"name": {"type": "string"}, "stats": {"type": "object", "properties": {"strength": {"type": "number"}}}}}"#;
        let result: String = remove_defaults(schema).unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(result.as_str())
                .unwrap()
                .as_object(),
            serde_json::from_str::<Value>(expected).unwrap().as_object()
        );
    }

    #[test]
    fn test_add_additional_properties() {
        let schema = r#"{"title": "Character", "description": "A character in an RPG."}"#;
        let expected = r#"{"title":"Character","description":"A character in an RPG.","additionalProperties":false}"#;
        let result: String = add_additional_properties(schema).unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(result.as_str())
                .unwrap()
                .as_object(),
            serde_json::from_str::<Value>(expected).unwrap().as_object()
        );
    }

    #[test]
    fn test_clean_schema() {
        let schema = r#"{"title": "Character", "type": "object", "description": "A character in an RPG.", "properties": {"name": {"type": "string", "default": ""}}}"#;
        let expected = r#"{"title":"Character", "type": "object", "description":"A character in an RPG.", "properties": {"name": {"type": "string"}}, "additionalProperties": false, "required": ["name"]}"#;
        let expected: Value = serde_json::from_str(expected).unwrap();
        let expected = expected.as_object().unwrap();

        let cleaned_schema = clean_schema(schema).unwrap();
        let result: Value = serde_json::from_str(cleaned_schema.as_str()).unwrap();
        let result = result.as_object().unwrap();
        assert_eq!(expected, result);
    }
}
