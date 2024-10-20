//! Generate images with a local Stable Diffusion instance.

mod requests;
use super::ImageProvider;
use anyhow::Result;
use std::path::{Path, PathBuf};

/// A provider for generating images with a local Stable Diffusion instance.
/// The URL for the local instance is specified in the environment variable `STABLE_DIFFUSION_URL`.
pub struct StableDiffusionXLProvider;

impl ImageProvider for StableDiffusionXLProvider {
    async fn generate_image(prompt: &str, output_directory: &Path) -> Result<PathBuf> {
        // Check if the local Stable Diffusion instance is available.
        let base_url = std::env::var("STABLE_DIFFUSION_URL")?;
        let is_up: bool = requests::is_up(&base_url).await?;
        if !is_up {
            return Err(anyhow::anyhow!(
                "Local Stable Diffusion instance is not available."
            ));
        }

        // Check what model is currently loaded.
        let model_name: String = requests::get_model_name(&base_url).await?;

        // Select the appropriate request body based on the model.
        // If a Flux model is loaded, return an error.
        let request_body: requests::RequestBody = if model_name.contains("flux") {
            return Err(anyhow::anyhow!("Flux model is currently loaded."));
        } else if model_name.contains("lightning") {
            requests::RequestBody::Txt2Img(requests::Txt2Img {
                prompt: prompt.to_string(),
                negative_prompt: "".to_string(),
                steps: 6,
                batch_size: 1,
                width: 1024,
                height: 1024,
                sampler_name: "DPM SDE".to_string(),
                cfg_scale: 2,
            })
        } else {
            requests::RequestBody::Txt2Img(requests::Txt2Img {
                prompt: prompt.to_string(),
                negative_prompt: "".to_string(),
                steps: 20,
                batch_size: 1,
                width: 1024,
                height: 1024,
                sampler_name: "UniPC".to_string(),
                cfg_scale: 6,
            })
        };

        // Send the request.
        let images: Vec<String> = requests::post_txt2img(&base_url, &request_body).await?;

        // Build the output path for the image.
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let output_path = output_directory.join(format!("{}.png", timestamp));
        // Convert the base64-encoded image to a PNG file.
        requests::base64_to_png(&images[0], &output_path)?;
        Ok(output_path)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[tokio::test]
    async fn test_generate_image() -> Result<()> {
        // Set the URL for the local Stable Diffusion instance.
        std::env::set_var("STABLE_DIFFUSION_URL", "http://localhost:7860");
        let prompt = "A cat";
        let output_directory = Path::new(".");
        let image_path =
            StableDiffusionXLProvider::generate_image(prompt, output_directory).await?;
        // Check if the image was generated.
        assert!(image_path.exists());
        // Clean up the test output.
        std::fs::remove_file(image_path)?;
        Ok(())
    }
}
