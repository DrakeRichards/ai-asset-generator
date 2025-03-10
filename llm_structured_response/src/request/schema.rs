use super::json::{clean_schema, get_string_value};
use anyhow::Result;
use serde_json::Value;
use std::{fs::File, io::Read};

pub struct Schema {
    pub name: String,
    pub description: Option<String>,
    pub json: Value,
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
            json: schema,
        })
    }

    pub fn from_file(mut file: &File) -> Result<Self> {
        let mut schema_text = String::new();
        file.read_to_string(&mut schema_text)?;
        Self::from_json_string(&schema_text)
    }

    pub fn validate(&self, json: &Value) -> bool {
        if let Ok(validator) = jsonschema::validator_for(&self.json) {
            return validator.is_valid(json);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::{fs::File, io::Write};

    const TEST_SCHEMA: &str = r#"{
        "title": "Test Schema",
        "description": "A test schema in JSON",
        "type": "object",
        "properties": {
            "test": {
                "description": "A test property",
                "type": "string"
            }
        }
    }"#;

    const TEST_SCHEMA_PATH: &str = "test.schema.json";

    fn generate_test_schema_file() -> Result<PathBuf> {
        let path = PathBuf::from(TEST_SCHEMA_PATH);
        let mut file = File::create(&path)?;
        file.write_all(TEST_SCHEMA.as_bytes())?;
        Ok(path)
    }

    #[test]
    fn test_schema_from_json_string() -> Result<()> {
        let schema = Schema::from_json_string(TEST_SCHEMA)?;
        assert_eq!(schema.name, "Test Schema");
        assert_eq!(
            schema.description,
            Some("A test schema in JSON".to_string())
        );
        Ok(())
    }

    #[test]
    fn test_schema_from_file() -> Result<()> {
        let file_path = generate_test_schema_file()?;
        let file = File::open(&file_path)?;
        let schema = Schema::from_file(&file)?;
        assert_eq!(schema.name, "Test Schema");
        assert_eq!(
            schema.description,
            Some("A test schema in JSON".to_string())
        );
        std::fs::remove_file(TEST_SCHEMA_PATH)?;
        Ok(())
    }
}
