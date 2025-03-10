use super::{request, LlmProvider, LlmProviderConfig};
use anyhow::{Error, Result};
use async_trait::async_trait;
use ollama_rs::{
    generation::{
        chat::{request::ChatMessageRequest, ChatMessage},
        parameters::FormatType,
    },
    Ollama,
};
use serde::{Deserialize, Serialize};

/// An LLM provider that uses Ollama's API.
#[derive(Debug, Default)]
pub struct OllamaProvider {
    pub config: LlmProviderConfig,
}

impl OllamaProvider {
    /// Get the version of the Ollama API.
    pub async fn get_version(&self) -> Result<String> {
        let endpoint = format!(
            "{}:{}/api/version",
            self.config.url.as_ref().unwrap(),
            self.config.port.unwrap()
        );
        let response = reqwest::get(&endpoint).await?.text().await?;
        Ok(response)
    }

    /// Get the list of locally available models.
    pub async fn get_models(&self) -> Result<OllamaModels> {
        let endpoint = format!(
            "{}:{}/api/tags",
            self.config.url.as_ref().unwrap(),
            self.config.port.unwrap()
        );
        let response = reqwest::get(&endpoint).await?.text().await?;
        dbg!(response.clone());
        let models: OllamaModels = serde_json::from_str(&response)?;
        Ok(models)
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    async fn request_structured_response(
        &self,
        schema: &request::Schema,
        prompt: &request::Prompt,
    ) -> Result<String> {
        // Check that the URL and port are set.
        if self.config.url.is_none() || self.config.port.is_none() {
            return Err(Error::msg("URL and port must be set"));
        }

        let format = FormatType::StructuredJsonValue(schema.json.clone());

        let messages: Vec<ChatMessage> = vec![
            ChatMessage::system(prompt.system.clone()),
            ChatMessage::user(prompt.initial.clone()),
        ];

        let request = ChatMessageRequest::new(self.config.model.clone(), messages).format(format);

        let url = self
            .config
            .url
            .clone()
            .expect("Should have already checked that the URL was populated.");
        let port = self
            .config
            .port
            .expect("Should have already checked that the port was populated.");
        // Initialize the connection.
        let ollama = Ollama::new(url, port);

        let response = ollama.send_chat_messages(request).await?;

        Ok(response.message.content)
    }
}

/// A model for the Ollama provider.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OllamaModel {
    pub name: String,
    pub modified_at: Option<String>,
    pub size: u64,
    pub digest: Option<String>,
    pub details: OllamaModelDetails,
}

/// Details for the Ollama model.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OllamaModelDetails {
    pub format: Option<String>,
    pub family: Option<String>,
    #[serde(default)]
    pub families: Vec<String>,
    pub parameter_size: Option<String>,
    pub quantization_level: Option<String>,
}

/// A list of available models.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OllamaModels {
    pub models: Vec<OllamaModel>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_request_structured_response() -> Result<()> {
        let provider = OllamaProvider {
            config: LlmProviderConfig {
                url: Some("http://localhost".to_string()),
                port: Some(11434),
                model: "llama3.1".to_string(),
            },
        };

        let schema = request::Schema::from_json_string(
            r#"
            {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "http://example.com/example.schema.json",
    "title": "Example",
    "description": "An example schema in JSON",
    "type": "object",
    "properties": {
        "name": {
            "description": "Name of the person",
            "type": "string"
        },
        "age": {
            "description": "Age of the person",
            "type": "integer"
        },
        "isStudent": {
            "description": "Is the person a student?",
            "type": "boolean"
        },
        "courses": {
            "description": "Courses the person is taking",
            "type": "array",
            "items": {
                "type": "string"
            }
        }
    },
    "required": ["name", "age", "isStudent", "courses"]
}
            "#,
        )?;

        let prompt = request::Prompt {
            initial: "Generate a random student using the provided schema. Return as JSON."
                .to_string(),
            system: "You are an AI assistant. Return the response as JSON.".to_string(),
        };

        let response = provider
            .request_structured_response(&schema, &prompt)
            .await?;
        // Check that the response is not empty.
        assert!(!response.is_empty());
        // Check if the response is valid JSON.
        assert!(&schema.validate(&serde_json::from_str(&response)?));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_version() -> Result<()> {
        let provider = OllamaProvider {
            config: LlmProviderConfig {
                url: Some("http://localhost".to_string()),
                port: Some(11434),
                model: "llama3.1".to_string(),
            },
        };

        let version = provider.get_version().await?;
        dbg!(&version);
        assert!(!version.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_models() -> Result<()> {
        let provider = OllamaProvider {
            config: LlmProviderConfig {
                url: Some("http://localhost".to_string()),
                port: Some(11434),
                model: "llama3.1".to_string(),
            },
        };

        let models = provider.get_models().await?;
        dbg!(&models);
        assert!(!models.models.is_empty());
        Ok(())
    }
}
