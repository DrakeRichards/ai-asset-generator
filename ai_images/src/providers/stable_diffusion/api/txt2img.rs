use crate::images::Base64Image;

use super::StableDiffusionXLProvider;
use anyhow::Result;
use clap::Args;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Args, Deserialize)]
pub struct Txt2ImgRequestBody {
    pub prompt: String,
    pub negative_prompt: String,
    pub steps: u32,
    pub batch_size: u32,
    pub width: u32,
    pub height: u32,
    pub sampler_name: String,
    pub cfg_scale: u32,
}

impl Default for Txt2ImgRequestBody {
    fn default() -> Self {
        Self {
            prompt: "".to_string(),
            negative_prompt: "".to_string(),
            steps: 6,
            batch_size: 1,
            width: 1024,
            height: 1024,
            sampler_name: "Default".to_string(),
            cfg_scale: 2,
        }
    }
}

impl StableDiffusionXLProvider {
    /// Send a POST request to `/sdapi/v1/txt2img` to start a new image generation task.
    /// The response contains the images in base64 encoding.
    pub async fn post_txt2img(&self, request: &Txt2ImgRequestBody) -> Result<Vec<Base64Image>> {
        let endpoint = "/sdapi/v1/txt2img";
        let url = format!("{}{}", self.get_url(), endpoint);
        let body = serde_json::to_string(request)?;
        let client = reqwest::Client::new();
        let response = client.post(url).body(body).send().await?;
        let response: Value = serde_json::from_str(&response.text().await?)?;
        let images: Vec<Base64Image> = response["images"]
            .as_array()
            .ok_or(anyhow::anyhow!("Unable to get images."))?
            .iter()
            .map(|image| {
                image
                    .as_str()
                    .ok_or(anyhow::anyhow!("Unable to get image."))
                    .map(|image| Base64Image {
                        image: image.to_string(),
                    })
            })
            .collect::<Result<Vec<Base64Image>>>()?;
        Ok(images)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;
    use base64::Engine;
    use serial_test::serial;

    #[tokio::test]
    #[serial(stable_diffusion, local_server)]
    async fn test_post_txt2img() {
        let provider = StableDiffusionXLProvider::default();
        let request = Txt2ImgRequestBody {
            prompt: "A cat".to_string(),
            negative_prompt: "".to_string(),
            steps: 6,
            batch_size: 1,
            width: 1024,
            height: 1024,
            sampler_name: "DPM++ 2M".to_string(),
            cfg_scale: 2,
        };
        let received_images = provider.post_txt2img(&request).await.unwrap();
        // Assert that we get one image.
        assert_eq!(received_images.len(), 1);
        // Assert that the image is of size 1024x1024.
        let image = &received_images.first().unwrap().image;
        let image = base64::prelude::BASE64_STANDARD.decode(image).unwrap();
        let image = image::load_from_memory(&image).unwrap();
        assert_eq!(image.width(), 1024);
        assert_eq!(image.height(), 1024);
    }
}
