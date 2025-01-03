use clap::Args;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A prompt to generate an image from.
#[derive(Args, Deserialize, Debug, Serialize, Clone)]
pub struct Prompt {
    pub base: String,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub negative: Option<String>,
}

impl Default for Prompt {
    fn default() -> Self {
        Self {
            base: "A beautiful sunset over the ocean.".to_string(),
            prefix: None,
            suffix: None,
            negative: None,
        }
    }
}

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut prompt = self.base.clone();
        if let Some(prefix) = &self.prefix {
            prompt = format!("{} {}", prefix, prompt);
        }
        if let Some(suffix) = &self.suffix {
            prompt = format!("{} {}", prompt, suffix);
        }
        write!(f, "{}", prompt)
    }
}

impl std::convert::From<String> for Prompt {
    fn from(prompt: String) -> Self {
        Self {
            base: prompt,
            prefix: None,
            suffix: None,
            negative: None,
        }
    }
}
