use super::StableDiffusionXLProvider;
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StableDiffusionModel {
    title: String,
    model_name: String,
    filename: String,
    #[serde(rename = "type")]
    model_type: String,
    sha256: Option<String>,
    hash: Option<String>,
    config: Option<String>,
}

impl StableDiffusionXLProvider {
    /// Get the name of the currently loaded checkpoint
    pub async fn get_model_name(&self) -> Result<String> {
        let config_response = self.get_config().await?;
        let model_name = config_response["sd_model_checkpoint"]
            .as_str()
            // If the response doesn't contain a model name, return an error.
            .ok_or({
                let response = config_response.to_string();
                Error::msg(format!(
                    "Config response did not contain the model name. Response: {:?}",
                    response
                ))
            })?;
        Ok(model_name.to_string())
    }

    /// Get a list of available models
    pub async fn get_models(&self) -> Result<Vec<StableDiffusionModel>> {
        let endpoint = "/sdapi/v1/sd-models";
        let url = format!("{}{}", self.get_url(), endpoint);
        let response = reqwest::get(url).await?;
        let models = response.text().await?;
        let models: Vec<StableDiffusionModel> = serde_json::from_str(&models)?;
        Ok(models)
    }

    /// Set the model to use for generating images
    pub async fn set_model(&self, model_name: &str) -> Result<()> {
        let endpoint = "/sdapi/v1/options";
        let url = format!("{}{}", self.get_url(), endpoint);
        let body = serde_json::json!({
            "sd_model_checkpoint": model_name,
        });
        let response = reqwest::Client::new().post(url).json(&body).send().await?;

        // Check that the response has a status code of 200.
        if !response.status().is_success() {
            return Err(Error::msg("Failed to update model."));
        }

        // Check that the loaded model matches the requested model.
        let loaded_model = self.get_model_name().await?;
        if loaded_model != model_name {
            return Err(Error::msg("Failed to update model."));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[tokio::test]
    async fn test_get_model_name() {
        let provider = StableDiffusionXLProvider::default();
        let model_name = provider.get_model_name().await.unwrap();
        println!("Model name: {}", model_name);
        assert!(!model_name.is_empty());
    }

    #[tokio::test]
    async fn test_get_models() -> Result<()> {
        let provider = StableDiffusionXLProvider::default();
        let models = provider.get_models().await?;
        println!("Models: {:?}", models);
        assert!(!models.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_model() -> Result<()> {
        let provider = StableDiffusionXLProvider::default();
        let models = provider.get_models().await?;
        let model_name = &models[0].model_name;
        provider.set_model(model_name).await
    }
}
