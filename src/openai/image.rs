use async_openai::{
    error::OpenAIError,
    types::{CreateImageRequestArgs, ImageResponseFormat, ImageSize},
    Client,
};
use std::path::PathBuf;

pub async fn generate_request(
    prompt: &str,
    output_directory: &PathBuf,
) -> Result<PathBuf, OpenAIError> {
    let client = Client::new();

    // Create the request.
    let request = CreateImageRequestArgs::default()
        .model(async_openai::types::ImageModel::DallE3)
        .prompt(prompt)
        .response_format(ImageResponseFormat::B64Json)
        .size(ImageSize::S1024x1024)
        .build()?;

    // Send the request to OpenAI's API.
    let response = client.images().create(request).await?;

    // Download and save the image to the current directory.
    let image = response.save(output_directory).await?;

    // Return the path to the image.
    image
        .first()
        .ok_or(OpenAIError::StreamError(
            "Response did not return any images".to_string(),
        ))
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_generate_request() -> Result<(), Box<dyn std::error::Error>> {
        // Load environment variables from a .env file.
        dotenv().ok();

        let prompt = "A beautiful sunset over the ocean.";
        let output_directory = PathBuf::from(".");

        let image = generate_request(prompt, &output_directory).await?;

        assert!(image.exists());

        // Clean up the image file and any directories created.
        std::fs::remove_file(image)?;
        Ok(())
    }
}
