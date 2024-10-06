use crate::assets::AssetType;
use crate::json::{clean_schema, get_string_value, to_slug};

use async_openai::{
    types::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
    },
    Client,
};
use serde_json::Value;
use std::error::Error;

pub async fn generate_request(
    asset_type: &AssetType,
    initial_prompt: String,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    // Get the name (title) and description from the schema.
    // The schema might not have a description.
    let description: Option<String> = get_string_value(asset_type.schema(), "description").ok();
    let name: String = to_slug(&get_string_value(asset_type.schema(), "title")?);

    // Clean the schema for use with OpenAI's API.
    let cleaned_schema: String = clean_schema(asset_type.schema())?;
    let schema: Option<Value> = serde_json::from_str(cleaned_schema.as_str())?;

    // Set the response format to JSON schema.
    let response_format = ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema {
            description,
            name,
            schema,
            strict: Some(true),
        },
    };

    // Create the request.
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-2024-08-06")
        .messages([
            ChatCompletionRequestSystemMessage::from(asset_type.system_prompt()).into(),
            ChatCompletionRequestUserMessage::from(initial_prompt).into(),
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

    Err("No response content found".into())
}
