//! Generate images for RPG assets.

mod openai;
mod stable_diffusion;
use anyhow::Result;
use clap::ValueEnum;
use std::path::{Path, PathBuf};

/// Different image generation providers.
#[derive(ValueEnum, Clone)]
pub enum ImageProviders {
    OpenAi,
    StableDiffusion,
}

/// Defines an image generation provider.
pub trait ImageProvider {
    /// Generate an image and return the file path where it is saved.
    async fn generate_image(prompt: &str, output_path: &Path) -> Result<PathBuf>;
}

impl ImageProviders {
    /// Generate an image using the specified provider.
    pub async fn generate_image(&self, prompt: &str, output_path: &Path) -> Result<PathBuf> {
        match self {
            ImageProviders::OpenAi => {
                openai::OpenAiProvider::generate_image(prompt, output_path).await
            }
            ImageProviders::StableDiffusion => {
                stable_diffusion::StableDiffusionXLProvider::generate_image(prompt, output_path)
                    .await
            }
        }
    }
}
