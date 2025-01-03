pub mod cli;
mod images;
mod params;
pub mod providers;

use anyhow::{Error, Result};
pub use params::{ImageParams, Prompt};
pub use providers::ImageProviders;

impl cli::Provider {
    pub fn to_image_provider(&self) -> Result<ImageProviders> {
        match self.name {
            cli::ImageProviders::OpenAi => Ok(ImageProviders::OpenAi(providers::OpenAiProvider)),
            cli::ImageProviders::StableDiffusion => {
                // If the Stable Diffusion provider is selected, check that the URL is provided.
                if let Some(url) = &self.config.url {
                    if url.is_empty() {
                        return Err(Error::msg(
                            "The URL for the Stable Diffusion provider must be provided.",
                        ));
                    }
                    Ok(providers::ImageProviders::StableDiffusion(
                        providers::StableDiffusionXLProvider {
                            url: url.to_string(),
                        },
                    ))
                } else {
                    Err(Error::msg(
                        "The URL for the Stable Diffusion provider must be provided.",
                    ))
                }
            }
        }
    }
}
