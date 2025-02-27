//! Generate images for RPG assets.

mod openai;
mod stable_diffusion;

use super::images::Base64Image;
use super::params::ImageParams;
use anyhow::Result;
use async_trait::async_trait;
use clap::Subcommand;
pub use openai::OpenAiProvider;
use serde::{Deserialize, Serialize};
pub use stable_diffusion::StableDiffusionXLProvider;
use std::path::PathBuf;

/// Different image generation providers.
#[derive(Subcommand, Deserialize, Debug, Serialize)]
pub enum ImageProviders {
    OpenAi(openai::OpenAiProvider),
    StableDiffusion(stable_diffusion::StableDiffusionXLProvider),
}

/// Defines an image generation provider.
#[async_trait]
pub trait ImageProvider {
    /// Generate an image and return the file path where it is saved.
    async fn text_to_image(&self, params: ImageParams) -> Result<PathBuf>;
}

impl ImageProviders {
    /// Generate an image using the specified provider.
    pub async fn generate_image(&self, params: ImageParams) -> Result<PathBuf> {
        match self {
            ImageProviders::OpenAi(provider) => provider.text_to_image(params).await,
            ImageProviders::StableDiffusion(provider) => {
                let image = provider.queue_txt2img(&params).await?;
                // Image filename is the current timestamp.
                let image_filename: String = format!("{}.png", chrono::Utc::now().timestamp());
                let image_path = params.output_directory.join(image_filename);
                image.to_file(&image_path)?;
                Ok(image_path)
            }
        }
    }
}

impl Default for ImageProviders {
    fn default() -> Self {
        ImageProviders::OpenAi(openai::OpenAiProvider)
    }
}
