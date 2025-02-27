use super::json::{clean_schema, get_string_value};
use anyhow::Result;
use serde_json::Value;
use std::{fs::File, io::Read};

pub struct Schema {
    pub name: String,
    pub description: Option<String>,
    pub schema: Value,
}

impl Schema {
    pub fn from_json_string(schema_text: &str) -> Result<Self> {
        let schema: Value = serde_json::from_str(schema_text)?;
        let schema = clean_schema(schema)?;
        let name = get_string_value(&schema, "title")?;
        let description = get_string_value(&schema, "description").ok();

        Ok(Self {
            name,
            description,
            schema,
        })
    }

    pub fn from_file(mut file: &File) -> Result<Self> {
        let mut schema_text = String::new();
        file.read_to_string(&mut schema_text)?;
        Self::from_json_string(&schema_text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_schema_from_json_string() -> Result<()> {
        let schema_text = r#"{
            "title": "Test Schema",
            "description": "A test schema",
            "type": "object",
            "properties": {
                "test": {
                    "description": "A test property",
                    "type": "string"
                }
            }
        }"#;
        let schema = Schema::from_json_string(schema_text)?;
        assert_eq!(schema.name, "Test Schema");
        assert_eq!(schema.description, Some("A test schema".to_string()));
        Ok(())
    }

    #[test]
    fn test_schema_from_file() -> Result<()> {
        let file = File::open("test/example.schema.json")?;
        let schema = Schema::from_file(&file)?;
        assert_eq!(schema.name, "Example");
        assert_eq!(
            schema.description,
            Some("An example schema in JSON".to_string())
        );
        Ok(())
    }
}
