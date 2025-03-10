//! JSON utilities for working with OpenAI's API.

use serde::de::Error;
use serde_json::{Result, Value};

/// Clean up a schema for use with OpenAI's API.
pub fn clean_schema(schema: Value) -> Result<Value> {
    let mut cleaned_schema = remove_defaults(schema)?;
    cleaned_schema = add_additional_properties(cleaned_schema)?;
    cleaned_schema = add_required_properties(cleaned_schema)?;
    Ok(cleaned_schema)
}

/// Recursively remove any "default" properties from a JSON Value.
/// This is necessary because the OpenAI API does not accept "default" properties.
fn remove_defaults(schema: Value) -> Result<Value> {
    let mut cleaned_schema = schema.clone();

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
    recurse_remove_defaults(&mut cleaned_schema);

    Ok(cleaned_schema)
}

/// Add a property for "additionalProperties" to a JSON Value and set it to false.
/// "additionalProperties" is required by the OpenAI API. See <https://platform.openai.com/docs/guides/structured-outputs/additionalproperties-false-must-always-be-set-in-objects>.
fn add_additional_properties(schema: Value) -> Result<Value> {
    let mut schema = schema;
    // Check if "additionalProperties" is already set.
    if schema.get("additionalProperties").is_some() {
        return Ok(schema);
    }
    schema.as_object_mut().map(|properties| {
        properties.insert("additionalProperties".to_string(), Value::Bool(false))
    });
    Ok(schema)
}

/// Add all properties to the root "required" array.
fn add_required_properties(schema: Value) -> Result<Value> {
    let mut schema = schema;

    // Get a list of all properties.
    let all_properties: Vec<String> = get_property_keys(&schema, "properties")
        .ok_or_else(|| Error::missing_field("properties"))?;

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

    Ok(schema)
}

/// Get a list of keys contained in a property.
fn get_property_keys(schema: &Value, property_key: &str) -> Option<Vec<String>> {
    schema
        .get(property_key)?
        .as_object()
        .map(|properties| properties.keys().cloned().collect())
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

/// Get a property's value from a JSON string.
/// The property must be a string.
pub fn get_string_value(schema: &Value, property: &'static str) -> Result<String> {
    schema
        .get(property)
        .ok_or(Error::missing_field(property))?
        .as_str()
        .map(|s| s.to_string())
        .ok_or(Error::custom("failed to convert property to a string"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_property_value() -> Result<()> {
        let schema = r#"{"title": "Character", "description": "A character in an RPG."}"#;
        let schema: Value = serde_json::from_str(schema)?;
        let title = get_string_value(&schema, "title")?;
        let description = get_string_value(&schema, "description")?;
        assert_eq!(title, "Character".to_string());
        assert_eq!(description, "A character in an RPG.".to_string());
        Ok(())
    }

    #[test]
    fn test_remove_defaults() -> Result<()> {
        let schema = r#"{"title": "Character", "description": "A character in an RPG.", "properties": {"name": {"type": "string", "default": ""}}}"#;
        let schema: Value = serde_json::from_str(schema)?;
        let expected = r#"{"title": "Character", "description": "A character in an RPG.", "properties": {"name": {"type": "string"}}}"#;
        let expected: Value = serde_json::from_str(expected)?;
        let result = remove_defaults(schema)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_nested_default_removal() -> Result<()> {
        let schema = r#"{"title": "Character", "description": "A character in an RPG.", "properties": {"name": {"type": "string", "default": ""}, "stats": {"type": "object", "properties": {"strength": {"type": "number", "default": 0}}}}}"#;
        let schema: Value = serde_json::from_str(schema)?;
        let expected = r#"{"title": "Character", "description": "A character in an RPG.", "properties": {"name": {"type": "string"}, "stats": {"type": "object", "properties": {"strength": {"type": "number"}}}}}"#;
        let expected: Value = serde_json::from_str(expected)?;
        let result = remove_defaults(schema)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_add_additional_properties() -> Result<()> {
        let schema = r#"{"title": "Character", "description": "A character in an RPG."}"#;
        let schema: Value = serde_json::from_str(schema)?;
        let expected = r#"{"title":"Character","description":"A character in an RPG.","additionalProperties":false}"#;
        let expected: Value = serde_json::from_str(expected)?;
        let result = add_additional_properties(schema)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_add_required_properties() -> Result<()> {
        let schema = r#"{"title": "Character", "description": "A character in an RPG.", "properties": {"name": {"type": "string"}}}"#;
        let schema: Value = serde_json::from_str(schema)?;
        let expected = r#"{"title":"Character","description":"A character in an RPG.","properties":{"name":{"type":"string"}},"required":["name"]}"#;
        let expected: Value = serde_json::from_str(expected)?;
        let result = add_required_properties(schema)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_clean_schema() -> Result<()> {
        let schema = r#"{"title": "Character", "type": "object", "description": "A character in an RPG.", "properties": {"name": {"type": "string", "default": ""}}}"#;
        let schema: Value = serde_json::from_str(schema)?;
        let expected = r#"{"title":"Character", "type": "object", "description":"A character in an RPG.", "properties": {"name": {"type": "string"}}, "additionalProperties": false, "required": ["name"]}"#;
        let expected: Value = serde_json::from_str(expected)?;

        let result = clean_schema(schema)?;
        assert_eq!(expected, result);
        Ok(())
    }
}
