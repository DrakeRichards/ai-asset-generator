use super::{request, LlmProvider, LlmProviderConfig};
use anyhow::{Error, Result};
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
    },
    Client,
};
use async_trait::async_trait;
use dotenvy::dotenv;

/// An LLM provider that uses OpenAI's API.
#[derive(Debug, Default)]
pub struct OpenAiProvider {
    pub config: LlmProviderConfig,
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    async fn request_structured_response(
        &self,
        schema: request::Schema,
        prompt: request::Prompt,
    ) -> Result<String> {
        // Load environment variables from a .env file.
        dotenv().ok();

        // Check if the OpenAI API key is set.
        if std::env::var("OPENAI_API_KEY").is_err() {
            return Err(Error::msg("OPENAI_API_KEY environment variable not set"));
        }

        let client = Client::new();

        // Set the response format to JSON schema.
        let response_format = ResponseFormat::JsonSchema {
            json_schema: ResponseFormatJsonSchema {
                description: schema.description,
                name: schema.name,
                schema: Some(schema.schema),
                strict: Some(true),
            },
        };

        // Create the request.
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.config.model)
            .messages([
                ChatCompletionRequestSystemMessage::from(prompt.system).into(),
                ChatCompletionRequestUserMessage::from(prompt.initial).into(),
            ])
            .response_format(response_format)
            .build()?;

        // Send the request to OpenAI's API.
        let response = client.chat().create(request).await?;

        // Get the response content.
        for choice in response.choices {
            if let Some(content) = choice.message.content {
                return Ok(content);
            }
        }

        Err(Error::msg("No response content found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_request_structured_response() -> Result<()> {
        let provider = OpenAiProvider::default();
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
    }
}
            "#,
        )?;
        let prompt = request::Prompt {
            system: "Please provide your name.".to_string(),
            initial: "My name is Alice.".to_string(),
        };
        let response = provider.request_structured_response(schema, prompt).await?;
        // Check if the response is valid JSON.
        let _: serde_json::Value = serde_json::from_str(&response)?;
        Ok(())
    }
}
