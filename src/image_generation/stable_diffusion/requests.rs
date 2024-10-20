use anyhow::Result;
use base64::Engine;
use serde_json::Value;
use std::path::Path;

#[derive(Debug, serde::Serialize)]
pub enum RequestBody {
    Txt2Img(Txt2Img),
}

#[derive(Debug, serde::Serialize)]
pub struct Txt2Img {
    pub prompt: String,
    pub negative_prompt: String,
    pub steps: u32,
    pub batch_size: u32,
    pub width: u32,
    pub height: u32,
    pub sampler_name: String,
    pub cfg_scale: u32,
}

pub async fn is_up(base_url: &str) -> Result<bool> {
    let response = reqwest::get(base_url).await?;
    Ok(response.status().is_success())
}

/// Send a GET request to `/sdapi/v1/options` to get the current configuration
async fn get_config(base_url: &str) -> Result<Value> {
    let endpoint = "/sdapi/v1/options";
    let url = format!("{}{}", base_url, endpoint);
    let response = reqwest::get(url).await?;
    let config = response.text().await?;
    let config: Value = serde_json::from_str(&config)?;
    Ok(config)
}

/// Get the name of the currently loaded checkpoint
pub async fn get_model_name(base_url: &str) -> Result<String> {
    let config = get_config(base_url).await?;
    let model_name = config["sd_model_checkpoint"]
        .as_str()
        .ok_or(anyhow::anyhow!("Unable to get currently loaded model."))?;
    Ok(model_name.to_string())
}

/// Send a POST request to `/sdapi/v1/txt2img` to start a new image generation task.
/// The response contains the images in base64 encoding.
pub async fn post_txt2img(base_url: &str, request_body: &RequestBody) -> Result<Vec<String>> {
    let endpoint = "/sdapi/v1/txt2img";
    let url = format!("{}{}", base_url, endpoint);
    let body = match request_body {
        RequestBody::Txt2Img(txt2img) => serde_json::to_string(txt2img)?,
    };
    let client = reqwest::Client::new();
    let response = client.post(url).body(body).send().await?;
    let response: Value = serde_json::from_str(&response.text().await?)?;
    let images: Vec<String> = response["images"]
        .as_array()
        .ok_or(anyhow::anyhow!("Unable to get images."))?
        .iter()
        .map(|image| {
            image
                .as_str()
                .ok_or(anyhow::anyhow!("Unable to get image."))
                .map(|image| image.to_string())
        })
        .collect::<Result<Vec<String>>>()?;
    Ok(images)
}

/// Convert a base64-encoded image to a PNG file which is saved to file_path.
pub fn base64_to_png(image: &str, file_path: &Path) -> Result<()> {
    // Check that file_path is not a directory.
    if file_path.is_dir() {
        return Err(anyhow::anyhow!("file_path must be a file path."));
    }
    let image = base64::prelude::BASE64_STANDARD.decode(image)?;
    std::fs::write(file_path, image)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[tokio::test]
    async fn test_is_up() {
        let base_url = "http://localhost:7860";
        let is_up = is_up(base_url).await.unwrap();
        assert!(is_up);
    }

    #[tokio::test]
    async fn test_get_model_name() {
        let base_url = "http://localhost:7860";
        let model_name = get_model_name(base_url).await.unwrap();
        // Do we get a model name?
        dbg!(&model_name);
        assert!(!model_name.is_empty());
    }

    #[tokio::test]
    async fn test_post_txt2img() {
        let base_url = "http://localhost:7860";
        let request_body = RequestBody::Txt2Img(Txt2Img {
            prompt: "A cat".to_string(),
            negative_prompt: "".to_string(),
            steps: 6,
            batch_size: 1,
            width: 1024,
            height: 1024,
            sampler_name: "UniPC".to_string(),
            cfg_scale: 2,
        });
        let images = post_txt2img(base_url, &request_body).await.unwrap();
        // Assert that we get one image.
        assert_eq!(images.len(), 1);
        // Assert that the image is of size 1024x1024.
        let image = base64::prelude::BASE64_STANDARD.decode(&images[0]).unwrap();
        let image = image::load_from_memory(&image).unwrap();
        assert_eq!(image.width(), 1024);
        assert_eq!(image.height(), 1024);
    }

    #[test]
    fn test_base64_to_png() {
        let image = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAABjElEQVR42mNk".to_string();
        let output_directory = Path::new(".");
        let output_path = output_directory.join("test_base64_to_png.png");
        base64_to_png(&image, &output_path).unwrap();
        assert!(output_path.exists());
        std::fs::remove_file(output_path).unwrap();
    }

    #[tokio::test]
    async fn test_generate_image() -> Result<()> {
        // Set the URL for the local Stable Diffusion instance.
        std::env::set_var("STABLE_DIFFUSION_URL", "http://localhost:7860");
        let prompt = "A cat";
        let output_path = Path::new("./test_generate_image.png");
        let image_path = post_txt2img(
            "http://localhost:7860",
            &RequestBody::Txt2Img(Txt2Img {
                prompt: prompt.to_string(),
                negative_prompt: "".to_string(),
                steps: 6,
                batch_size: 1,
                width: 1024,
                height: 1024,
                sampler_name: "UniPC".to_string(),
                cfg_scale: 2,
            }),
        )
        .await?;
        base64_to_png(&image_path[0], output_path)?;
        assert!(output_path.exists());
        std::fs::remove_file(output_path)?;
        Ok(())
    }
}
