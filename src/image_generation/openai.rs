use super::ImageProvider;
use anyhow::Result;
use async_openai::{
    error::OpenAIError,
    types::{CreateImageRequestArgs, ImageResponseFormat, ImageSize},
    Client,
};
use std::path::{Path, PathBuf};

pub struct OpenAiProvider;

impl ImageProvider for OpenAiProvider {
    async fn generate_image(prompt: &str, output_directory: &Path) -> Result<PathBuf> {
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
        let image: Vec<PathBuf> = response.save(&output_directory).await?;

        // Rename the image file to the current timestamp.
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let new_image = output_directory.join(format!("{}.png", timestamp));
        std::fs::rename(&image[0], &new_image)?;

        // Return the path to the image.
        let image: Option<PathBuf> = Some(new_image);
        match image {
            Some(image) => Ok(image),
            None => Err(anyhow::Error::new(OpenAIError::StreamError(
                "Response did not return any images".to_string(),
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_generate_request() -> Result<()> {
        // Load environment variables from a .env file.
        dotenv().ok();

        let prompt = "A beautiful sunset over the ocean.";
        let output_directory = PathBuf::from(".");

        let image = OpenAiProvider::generate_image(prompt, &output_directory).await?;

        assert!(image.exists());

        // Clean up the image file and any directories created.
        std::fs::remove_file(image)?;
        Ok(())
    }
}
