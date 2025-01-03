use super::{ImageParams, ImageProvider};
use anyhow::{Error, Result};
use async_openai::{
    error::OpenAIError,
    types::{CreateImageRequestArgs, ImageResponseFormat, ImageSize},
    Client,
};
use async_trait::async_trait;
use clap::Args;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// An image provider that generates images using OpenAI's DALL-E 3 API. Requires that you set the `OPENAI_API_KEY` environment variable, or have it in a `.env` file.
#[derive(Args, Deserialize, Debug, Serialize)]
pub struct OpenAiProvider;

impl Default for OpenAiProvider {
    fn default() -> Self {
        OpenAiProvider
    }
}

#[async_trait]
impl ImageProvider for OpenAiProvider {
    async fn text_to_image(&self, params: ImageParams) -> Result<PathBuf> {
        // Load environment variables from a .env file.
        dotenv()?;

        // Create a new OpenAI client.
        let client = Client::new();

        // Check that the OpenAI API key is set.
        if std::env::var("OPENAI_API_KEY").is_err() {
            return Err(Error::msg("OpenAI API key not set"));
        }

        // Standardize the image size.
        let size = to_openai_size(&params.width, &params.height);

        // Create the request.
        let request = CreateImageRequestArgs::default()
            .model(async_openai::types::ImageModel::DallE3)
            .prompt(params.prompt.to_string())
            .response_format(ImageResponseFormat::B64Json)
            .size(size)
            .build()?;

        // Send the request to OpenAI's API.
        let response = client.images().create(request).await?;

        // Download and save the image to the current directory.
        let image: Vec<PathBuf> = response.save(&params.output_directory).await?;

        // Rename the image file to the current timestamp.
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let new_image = params.output_directory.join(format!("{}.png", timestamp));
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

fn to_openai_size(width: &u32, height: &u32) -> ImageSize {
    match (width, height) {
        (1024, 1024) => ImageSize::S1024x1024,
        (1024, 1792) => ImageSize::S1024x1792,
        (1792, 1024) => ImageSize::S1792x1024,
        (256, 256) => ImageSize::S256x256,
        (512, 512) => ImageSize::S512x512,
        _ => ImageSize::S1024x1024,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_request() -> Result<()> {
        let params = ImageParams::default();
        let provider = OpenAiProvider;
        let image = provider.text_to_image(params).await?;
        assert!(image.exists());
        // Clean up the image file and any directories created.
        std::fs::remove_file(image)?;
        Ok(())
    }
}
