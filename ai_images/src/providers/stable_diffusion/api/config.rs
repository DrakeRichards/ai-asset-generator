use super::StableDiffusionXLProvider;
use anyhow::Result;
use serde_json::Value;

impl StableDiffusionXLProvider {
    /// Send a GET request to `/sdapi/v1/options` to get the current configuration
    pub async fn get_config(&self) -> Result<Value> {
        let endpoint = "/sdapi/v1/options";
        let url = format!("{}{}", self.get_url(), endpoint);
        let response = reqwest::get(&url).await?;
        // If the response is not successful, return an error.
        if !response.status().is_success() {
            return {
                let response = response.text().await?;
                Err(anyhow::anyhow!(
                    "Failed to get config. Request URL: {:?}, Response: {:?}",
                    &url,
                    response
                ))
            };
        }
        let config = response.json().await?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[tokio::test]
    async fn test_get_config() {
        let provider = StableDiffusionXLProvider::default();
        // Will fail if the local Stable Diffusion instance is not available.
        let config = provider.get_config().await.unwrap();
        // Do we get a config?
        dbg!(&config);
    }
}
