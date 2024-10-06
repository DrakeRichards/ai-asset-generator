use async_openai::{
    error::OpenAIError,
    types::{CreateImageRequestArgs, ImageResponseFormat, ImageSize},
    Client,
};
use std::path::PathBuf;

pub async fn generate_request(prompt: String) -> Result<PathBuf, OpenAIError> {
    let client = Client::new();

    // Create the request.
    let request = CreateImageRequestArgs::default()
        .prompt(prompt)
        .response_format(ImageResponseFormat::Url)
        .size(ImageSize::S1024x1024)
        .build()?;

    // Send the request to OpenAI's API.
    let response = client.images().create(request).await?;

    // Download and save the image to the current directory.
    let image = response.save("./").await?;

    // Return the path to the image.
    image
        .first()
        .ok_or(OpenAIError::StreamError(
            "Response did not return any images".to_string(),
        ))
        .cloned()
}
