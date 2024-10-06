//! Functions to manipulate JSON strings.

use serde::de::Error;
use serde_json::{Result, Value};

/// Get a property's value from a JSON string.
/// The property must be a string.
pub fn get_string_value(schema_text: &str, property: &'static str) -> Result<String> {
    let schema: Value = serde_json::from_str(schema_text)?;
    schema
        .get(property)
        .ok_or(Error::missing_field(property))?
        .as_str()
        .map(|s| s.to_string())
        .ok_or(Error::custom("failed to convert property to a string"))
}

/// Add a string property to a JSON string.
pub fn add_string_property(
    schema_text: &str,
    property_key: &str,
    property_value: &str,
) -> Result<String> {
    let mut schema: Value = serde_json::from_str(schema_text)?;
    schema.as_object_mut().map(|properties| {
        properties.insert(
            property_key.to_string(),
            Value::String(property_value.to_string()),
        )
    });
    Ok(schema.to_string())
}

/// Convert a string to lowercase and replace spaces with dashes.
pub fn to_slug(s: &str) -> String {
    s.to_lowercase().replace(" ", "-")
}

/// Get the strings contained in an array property from a JSON string.
pub fn get_array_strings(schema: &Value, property_key: &str) -> Result<Vec<String>> {
    let array = schema
        .get(property_key)
        .ok_or_else(|| {
            Error::custom(format!(
                "failed to convert property to an array. No property named '{}' found in schema.",
                property_key
            ))
        })?
        .as_array()
        .ok_or_else(|| {
            Error::custom(format!("expected property {} to be an array", property_key))
        })?;
    let strings: Vec<String> = array
        .iter()
        .map(|v| {
            v.as_str()
                .ok_or(Error::custom("failed to convert array element to string"))
                .map(|s| s.to_string())
        })
        .collect::<Result<Vec<String>>>()?;
    Ok(strings)
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
        get_property_keys(schema_text, "properties").ok_or(Error::missing_field("properties"))?;

    // Add the "required" array if it doesn't exist.
    if schema.get("required").is_none() {
        schema
            .as_object_mut()
            .map(|properties| properties.insert("required".to_string(), Value::Array(Vec::new())));
    }

    // What properties are already in the "required" array?
    let required: Vec<String> = get_array_strings(&schema, "required")?;

    // What properties are missing from the "required" array?
    let missing: Vec<String> = all_properties
        .iter()
        .filter(|p| !required.contains(p))
        .cloned()
        .collect();

    // Add the missing properties to the "required" array.
    schema
        .get_mut("required")
        .ok_or_else(|| Error::missing_field("required"))?
        .as_array_mut()
        .ok_or_else(|| Error::custom("'required' property was not an array"))?
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
    fn test_get_property_value() -> Result<()> {
        let schema = r#"{"title": "Character", "description": "A character in an RPG."}"#;
        let title = get_string_value(schema, "title")?;
        let description = get_string_value(schema, "description")?;
        assert_eq!(title, "Character".to_string());
        assert_eq!(description, "A character in an RPG.".to_string());
        Ok(())
    }

    #[test]
    fn test_remove_defaults() -> Result<()> {
        let schema = r#"{"title": "Character", "description": "A character in an RPG.", "properties": {"name": {"type": "string", "default": ""}}}"#;
        let expected = r#"{"title": "Character", "description": "A character in an RPG.", "properties": {"name": {"type": "string"}}}"#;
        let result: String = remove_defaults(schema)?;
        assert_eq!(
            serde_json::from_str::<Value>(result.as_str())?.as_object(),
            serde_json::from_str::<Value>(expected)?.as_object()
        );
        Ok(())
    }

    #[test]
    fn test_nested_default_removal() -> Result<()> {
        let schema = r#"{"title": "Character", "description": "A character in an RPG.", "properties": {"name": {"type": "string", "default": ""}, "stats": {"type": "object", "properties": {"strength": {"type": "number", "default": 0}}}}}"#;
        let expected = r#"{"title": "Character", "description": "A character in an RPG.", "properties": {"name": {"type": "string"}, "stats": {"type": "object", "properties": {"strength": {"type": "number"}}}}}"#;
        let result: String = remove_defaults(schema)?;
        Ok(assert_eq!(
            serde_json::from_str::<Value>(result.as_str())?.as_object(),
            serde_json::from_str::<Value>(expected)?.as_object()
        ))
    }

    #[test]
    fn test_add_additional_properties() -> Result<()> {
        let schema = r#"{"title": "Character", "description": "A character in an RPG."}"#;
        let expected = r#"{"title":"Character","description":"A character in an RPG.","additionalProperties":false}"#;
        let result: String = add_additional_properties(schema)?;
        Ok(assert_eq!(
            serde_json::from_str::<Value>(result.as_str())?.as_object(),
            serde_json::from_str::<Value>(expected)?.as_object()
        ))
    }

    #[test]
    fn test_add_required_properties() -> Result<()> {
        let schema = r#"{"title": "Character", "description": "A character in an RPG.", "properties": {"name": {"type": "string"}}}"#;
        let expected = r#"{"title":"Character","description":"A character in an RPG.","properties":{"name":{"type":"string"}},"required":["name"]}"#;
        let result: String = add_required_properties(schema)?;
        Ok(assert_eq!(
            serde_json::from_str::<Value>(result.as_str())?.as_object(),
            serde_json::from_str::<Value>(expected)?.as_object()
        ))
    }

    #[test]
    fn test_clean_schema() -> Result<()> {
        let schema = r#"{"title": "Character", "type": "object", "description": "A character in an RPG.", "properties": {"name": {"type": "string", "default": ""}}}"#;
        let expected = r#"{"title":"Character", "type": "object", "description":"A character in an RPG.", "properties": {"name": {"type": "string"}}, "additionalProperties": false, "required": ["name"]}"#;
        let expected: Value = serde_json::from_str(expected)?;
        let expected = expected.as_object();

        let cleaned_schema = clean_schema(schema)?;
        let result: Value = serde_json::from_str(cleaned_schema.as_str())?;
        let result = result.as_object();
        Ok(assert_eq!(expected, result))
    }
}
