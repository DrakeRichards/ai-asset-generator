use super::{api, Base64Image, ImageParams, ImageProvider};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use clap::Args;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A provider for generating images with a local Stable Diffusion instance.
#[derive(Args, Deserialize, Debug, Serialize)]
pub struct StableDiffusionXLProvider {
    /// The URL for the local Stable Diffusion instance.
    pub url: String,
}

impl Default for StableDiffusionXLProvider {
    fn default() -> Self {
        Self {
            url: "http://localhost:7860".to_string(),
        }
    }
}

#[async_trait]
impl ImageProvider for StableDiffusionXLProvider {
    /// Generate an image using the local Stable Diffusion instance.
    async fn text_to_image(&self, params: ImageParams) -> Result<PathBuf> {
        // Check if the local Stable Diffusion instance is available.
        let is_up: bool = self.is_up().await?;
        if !is_up {
            return Err(anyhow::anyhow!(
                "Local Stable Diffusion instance is not available."
            ));
        }

        // Check what model is currently loaded.
        let model_name: String = self.get_model_name().await?;

        // Check if the currently loaded model matches the requested model.
        if let Some(model) = &params.model {
            if model_name != model.as_str() {
                self.set_model(model).await?;
            }
        }

        // Select the appropriate request body based on the model.
        // If a Flux model is loaded, return an error.
        let request_body = api::txt2img::Txt2ImgRequestBody {
            prompt: params.prompt.to_string(),
            negative_prompt: params.prompt.negative.unwrap_or_default(),
            steps: params.steps,
            batch_size: 1,
            width: params.width,
            height: params.height,
            sampler_name: params.sampler_name,
            cfg_scale: params.cfg_scale,
        };

        // Send the request.
        let images: Vec<Base64Image> = self.post_txt2img(&request_body).await?;

        // Build the output path for the image.
        let timestamp = Utc::now().timestamp().to_string();
        let output_path = params.output_directory.join(format!("{}.png", timestamp));

        // Convert the base64-encoded image to a PNG file.
        images[0].to_file(&output_path)?;
        Ok(output_path)
    }
}

impl StableDiffusionXLProvider {
    /// Get the sanitized URL for the local Stable Diffusion instance.
    /// The URL should not have a trailing slash.
    pub fn get_url(&self) -> String {
        self.url.trim_end_matches('/').to_string()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[tokio::test]
    async fn test_generate_image() -> Result<()> {
        let params = ImageParams::default();
        let provider = StableDiffusionXLProvider::default();
        let image_path = provider.text_to_image(params).await?;
        // Check if the image was generated.
        assert!(image_path.exists());
        // Clean up the test output.
        std::fs::remove_file(image_path)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_prompt_prefix() -> Result<()> {
        let mut params = ImageParams::default();
        params.prompt.prefix = Some("A photograph of".to_string());
        params.prompt.base = "a cat".to_string();
        let provider = StableDiffusionXLProvider::default();
        // Generate the image.
        let image_path = provider.text_to_image(params).await?;
        // Check if the image was generated.
        assert!(image_path.exists());
        // Check the EXIF data for the image to see if the prompt was added.
        let file = std::fs::File::open(&image_path)?;
        let mut bufreader = std::io::BufReader::new(file);
        let exif_data = exif::Reader::new()
            .continue_on_error(true)
            .read_from_container(&mut bufreader)
            .or_else(|e| {
                e.distill_partial_result(|errors| {
                    errors.iter().for_each(|e| eprintln!("Warning: {}", e));
                })
            })?;
        let user_comment = exif_data.get_field(exif::Tag::UserComment, exif::In::PRIMARY);
        match user_comment {
            Some(user_comment) => match user_comment.value {
                exif::Value::Undefined(ref comment, _) => {
                    // The EXIF specification states that the UserComment field is of type "undefined",
                    // which is treated as a byte array. The first 8 bytes are the encoding, and the
                    // rest is the comment. The encoding is ASCII, so we can convert the comment to a
                    // string.
                    let comment = String::from_utf8_lossy(comment);
                    let first_line = comment.lines().next().unwrap();
                    // Remove null characters from the comment.
                    let sanitized_comment: String =
                        first_line.chars().filter(|&c| c != '\0').collect();
                    // Check if the prompt was added to the image.
                    assert_eq!(sanitized_comment, "UNICODEA photograph of a cat");
                }
                _ => panic!("Unable to read EXIF data: {:?}", user_comment.value),
            },
            _ => panic!("User comment not found in EXIF data."),
        }
        // Clean up the test output.
        std::fs::remove_file(image_path)?;
        Ok(())
    }
}
