use super::StableDiffusionXLProvider;
use anyhow::Result;
use reqwest::get;

impl StableDiffusionXLProvider {
    /// Send a GET request to `/sdapi/v1/status` to check if the local Stable Diffusion instance is up.
    pub async fn is_up(&self) -> Result<bool> {
        let response = get(&self.get_url()).await?;
        Ok(response.status().is_success())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[tokio::test]
    async fn test_is_up() {
        let provider = StableDiffusionXLProvider::default();
        let is_up = provider.is_up().await.unwrap();
        assert!(is_up);
    }
}
