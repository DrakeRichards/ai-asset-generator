use std::error::Error;

use crate::assets::{Asset, AssetType};
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
    },
    Client,
};
use serde_json::Value;

/// Get a property's value from a JSON object.
/// The property must be a string.
fn get_property_value(schema_text: &str, property: &str) -> Option<String> {
    let schema: Value = serde_json::from_str(schema_text).ok()?;
    schema.get(property)?.as_str().map(|s| s.to_string())
}

pub async fn generate_request(
    asset_type: AssetType,
    initial_prompt: String,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    // Get the name (title) and description from the schema.
    // If either is missing, return an error and print the schema.
    let description: Option<String> = get_property_value(asset_type.schema_string(), "description");
    let name: String = get_property_value(asset_type.schema_string(), "title")
        .ok_or("Schema is missing a title")?;
    let schema: Option<Value> = serde_json::from_str(asset_type.schema_string()).ok();

    let response_format = ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema {
            description,
            name,
            schema,
            strict: Some(true),
        },
    };

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-4o-2024-08-06")
        .messages([
            ChatCompletionRequestSystemMessage::from(
                "You are a game master creating a new RPG asset. The asset is a character, item, or location. If I provide you with additional information, fill in the blanks of what I don't provide. If I do not provide you with any additional prompt, generate a completely random asset based on the schema you are given.",
            ).into(),
            ChatCompletionRequestUserMessage::from(initial_prompt).into(),
        ])
        .response_format(response_format)
        .build()?;

    let response = client.chat().create(request).await?;

    for choice in response.choices {
        if let Some(content) = choice.message.content {
            return Ok(content);
        }
    }

    Err("No response content found".into())
}
