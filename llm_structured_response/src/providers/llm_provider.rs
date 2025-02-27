//! Generic declarations for LLM providers.
use super::request;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait LlmProvider {
    async fn request_structured_response(
        &self,
        schema: request::Schema,
        prompt: request::Prompt,
    ) -> Result<String>;
}
