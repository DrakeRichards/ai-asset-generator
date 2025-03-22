use ai_asset_generator::{Asset, AssetConfig, LlmProviderConfig};
use anyhow::Error;
use anyhow::Result;
use ex::fs;
use serial_test::serial;
use std::path::{Path, PathBuf};
use tempfile::{TempDir, tempdir};

#[cfg(test)]
mod default_config {
    use super::*;

    fn generate_default_config_file(dir: &TempDir) -> Result<PathBuf> {
        let config_file_path = dir.path().join("test-config.toml");
        let config = toml::to_string(&AssetConfig::default())?;
        fs::write(&config_file_path, &config)?;
        Ok(config_file_path)
    }

    #[test]
    fn test_deserialize_default_config_file() -> Result<()> {
        let dir = tempdir()?;
        let config_file = generate_default_config_file(&dir)?;
        let config = fs::read_to_string(&config_file)?;
        let config: AssetConfig = toml::from_str(&config)?;
        assert_eq!(config, AssetConfig::default());
        // Clean up
        drop(config_file);
        dir.close()?;
        Ok(())
    }

    #[test]
    fn test_serialize_default_config() -> Result<()> {
        let config = AssetConfig::default();
        let config = toml::to_string(&config)?;
        dbg!("{}", config);
        Ok(())
    }
}

#[cfg(test)]
mod ollama_config {
    use super::*;
    use llm_structured_response::LlmProviders;

    const CONFIG_FILE_PATH: &str = "test-ollama-config.toml";
    const JSON_SCHEMA_FILE_NAME: &str = "test-ollama-schema.json";

    /// Generate a temporary JSON schema file used for testing
    fn generate_schema_file(dir: &TempDir) -> Result<PathBuf> {
        let schema = r#"{
"$schema": "http://json-schema.org/draft-07/schema#",
"$id": "http://example.com/example.schema.json",
"name": "Example",
"description": "An example schema in JSON",
"schema": {
    "type": "object",
    "properties": {
        "name": {
            "description": "Name of the animal",
            "type": "string"
        },
        "activity": {
            "description": "Activity of the animal",
            "type": "string"
        }
    }
}
}"#;
        let schema_file_path = dir.path().join(JSON_SCHEMA_FILE_NAME);
        fs::write(&schema_file_path, schema)?;
        Ok(schema_file_path)
    }

    /// Generate a temporary configuration file for testing
    fn generate_default_config() -> AssetConfig {
        let mut config = AssetConfig::default();
        config.llm_structured_response.provider = LlmProviders::Ollama;
        config.llm_structured_response.provider_config = Some(
            LlmProviderConfig::default_for_provider(&config.llm_structured_response.provider),
        );
        config.llm_structured_response.system_prompt = "System prompt".to_string();
        config.llm_structured_response.json_schema_file = PathBuf::from(JSON_SCHEMA_FILE_NAME);
        config
    }

    fn generate_default_config_file(
        dir: &TempDir,
        json_schema_file_path: &Path,
    ) -> Result<PathBuf> {
        let mut config = generate_default_config();
        config.llm_structured_response.json_schema_file = json_schema_file_path.to_path_buf();
        config.output_directory = dir.path().to_path_buf();
        let config = toml::to_string(&config)?;
        let config_file_path = dir.path().join(CONFIG_FILE_PATH);
        fs::write(&config_file_path, &config)?;
        Ok(config_file_path)
    }

    #[test]
    #[serial(ollama, local_server)]
    fn test_ollama_config() -> Result<()> {
        let dir = tempdir()?;
        let schema_file_path = generate_schema_file(&dir)?;
        let mut config = generate_default_config();
        config.llm_structured_response.json_schema_file = schema_file_path.clone();
        config.output_directory = dir.path().to_path_buf();
        let prompt: &str = "Prompt";
        // Creating the asset will probably fail since the markdown template doesn't exist.
        // That's expected though: all we care about is whether we can get a structured response.
        let asset = Asset::from_config(&config, Some(prompt));
        // Clean up
        fs::remove_file(schema_file_path)?;
        dir.close()?;
        // Check that the error is a "file not found" error.
        match asset {
            Err(e) => {
                assert!(
                    e.to_string()
                        .contains("The markdown template was not filled.")
                );
            }
            _ => return Err(Error::msg("Expected an error.")),
        }
        Ok(())
    }

    #[test]
    #[serial(ollama, local_server)]
    fn test_ollama_config_from_file() -> Result<()> {
        let dir = tempdir()?;
        let schema_file_path = generate_schema_file(&dir)?;
        let config_file_path = generate_default_config_file(&dir, &schema_file_path)?;
        let prompt = "Prompt";
        // Creating the asset will probably fail since the markdown template doesn't exist.
        // That's expected though: all we care about is whether we can get a structured response.
        let asset = Asset::from_config_file_and_prompt(&config_file_path, Some(prompt));
        // Clean up
        fs::remove_file(schema_file_path)?;
        fs::remove_file(config_file_path)?;
        dir.close()?;
        // Check that the error is a "file not found" error.
        match asset {
            Err(e) => {
                dbg!(e.to_string());
                assert!(
                    e.to_string()
                        .contains("The markdown template was not filled.")
                );
            }
            _ => return Err(Error::msg("Expected an error.")),
        }
        Ok(())
    }
}
