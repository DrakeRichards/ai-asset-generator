//! Given an input schema, system prompt, and user prompt, this module sends a request to an LLM provider and returns a structured response in JSON format.
//! Currently only supports OpenAI's GPT-4 model, since that's the only model that supports JSON schema responses that I know of.

pub mod cli;
pub mod providers;
pub mod request;
