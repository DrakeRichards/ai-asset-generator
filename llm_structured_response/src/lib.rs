//! Given an input schema, system prompt, and user prompt, this module sends a request to an LLM provider and returns a structured response in JSON format.
//!
//! # Example
//!
//! ```rust
//! use llm_structured_response::{LlmProviderConfig, LlmProviders, Prompt};
//! use serde_json::{from_str, Value};
//! use jsonschema::validator_for;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let provider = LlmProviders::OpenAi;
//!    let config = LlmProviderConfig {
//!        model: "gpt-4o".to_string(),
//!        ..Default::default()
//!    };
//!    let schema: Value = from_str(
//!        r#"
//!{
//!    "name": "Student",
//!    "description": "A student in college.",
//!    "schema": {
//!        "type": "object",
//!        "properties": {
//!            "name": {
//!                "type": "string"
//!            },
//!            "age": {
//!                "type": "integer"
//!            },
//!            "is_student": {
//!                "type": "boolean"
//!            }
//!        },
//!        "required": ["name", "age", "is_student"],
//!        "additionalProperties": false
//!    }
//!}
//!    "#,
//!    )?;
//!    let prompt = Prompt {
//!        system: "You are an AI assistant that generates random students.".to_string(),
//!        initial: "Generate a random student using the provided JSON schema.".to_string(),
//!    };
//!    let response = provider.request_structured_response(&config, &schema, &prompt)?;
//!    assert!(!response.is_empty());
//!    // Check that the response validates against the schema.
//!    let validator = validator_for(&schema)?;
//!    let response_json: Value = from_str(&response)?;
//!    assert!(validator.is_valid(&response_json));
//!    Ok(())
//! }
//! ```

#![deny(unused_crate_dependencies)]

mod cli;
mod providers;
mod request;

pub use cli::CliConfigArgs;
pub use providers::{LlmProviderConfig, LlmProviders};
pub use request::Prompt;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use serde_json::{Value, from_str};

    #[test]
    fn test_ollama() -> Result<()> {
        let provider = LlmProviders::Ollama;
        let config = LlmProviderConfig {
            model: "llama3.1".to_string(),
            ..Default::default()
        };
        let schema = from_str(
            r#"{
            "description": "A student in college.",
            "name": "Student",
            "type": "object",
            "properties": {
                "name": {
                    "type": "string"
                },
                "age": {
                    "type": "integer"
                },
                "major": {
                    "type": "string"
                }
            },
            "required": ["name", "age", "major"]
        }"#,
        )?;
        let prompt = Prompt {
            system: "You are an AI assistant that generates random students.".to_string(),
            initial: "Generate a random student using the provided JSON schema.".to_string(),
        };
        let response = provider.request_structured_response(&config, &schema, &prompt)?;
        assert!(!response.is_empty());
        // Check that the response validates against the schema.
        let validator = jsonschema::validator_for(&schema)?;
        let response_json: Value = from_str(&response)?;
        assert!(validator.is_valid(&response_json));
        Ok(())
    }
}
