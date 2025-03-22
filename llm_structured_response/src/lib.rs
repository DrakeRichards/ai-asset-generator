//! Given an input schema, system prompt, and user prompt, this module sends a request to an LLM provider and returns a structured response in JSON format.
//!
//! # Example
//!
//! ```rust
//! use anyhow::Result;
//! use llm_structured_response::{LlmProviderConfig, LlmProviders, Prompt, StructuredOutputFormat};
//! use serde_json::from_str;
//!
//! let provider = LlmProviders::OpenAi;
//! let config = LlmProviderConfig::default_for_provider(&provider);
//! let schema: StructuredOutputFormat = from_str(
//!     r#"
//! {
//!     "name": "Student",
//!     "schema": {
//!         "type": "object",
//!         "properties": {
//!             "name": {
//!                 "type": "string"
//!             },
//!             "age": {
//!                 "type": "integer"
//!             },
//!             "major": {
//!                 "type": "string"
//!             }
//!         },
//!         "required": ["name", "age", "major"]
//!     }
//! }
//! "#,
//! ).unwrap();
//! let prompt = Prompt {
//!     system: "You are an AI assistant that generates random students.".to_string(),
//!     initial: "Generate a random student using the provided JSON schema.".to_string(),
//! };
//! let response = provider.request_structured_response(&config, schema, &prompt).unwrap();
//! assert!(!response.is_empty());
//! // Check that the response validates against the schema.
//!
//! #[derive(Debug, serde::Deserialize)]
//! struct Student {
//!     pub name: String,
//!     pub age: u8,
//!     pub major: String,
//! }
//! let response_json: Student = from_str(&response).unwrap();
//! assert!(!response_json.name.is_empty());
//! assert!(response_json.age > 0);
//! assert!(!response_json.major.is_empty());
//! ```

#![deny(unused_crate_dependencies)]

mod cli;
mod providers;
mod request;

pub use cli::CliConfigArgs;
pub use llm::chat::StructuredOutputFormat;
pub use providers::{LlmProviderConfig, LlmProviders};
pub use request::Prompt;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use serde_json::from_str;

    #[test]
    fn test_openai() -> Result<()> {
        let provider = LlmProviders::OpenAi;
        let config = LlmProviderConfig::default_for_provider(&provider);
        let schema: StructuredOutputFormat = from_str(
            r#"
    {
        "name": "Student",
        "schema": {
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
        }
    }
    "#,
        )?;
        let prompt = Prompt {
            system: "You are an AI assistant that generates random students.".to_string(),
            initial: "Generate a random student using the provided JSON schema.".to_string(),
        };
        let response = provider.request_structured_response(&config, schema, &prompt)?;
        assert!(!response.is_empty());
        // Check that the response validates against the schema.

        #[derive(Debug, serde::Deserialize)]
        struct Student {
            pub name: String,
            pub age: u8,
            pub major: String,
        }
        let response_json: Student = from_str(&response)?;
        assert!(!response_json.name.is_empty());
        assert!(response_json.age > 0);
        assert!(!response_json.major.is_empty());
        Ok(())
    }
}
