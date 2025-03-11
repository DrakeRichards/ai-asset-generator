use super::prompt::Prompt;
use clap::Args;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Parameters for the image generation request.
#[derive(Args, Deserialize, Debug, Serialize, Clone, PartialEq)]
pub struct ImageParams {
    /// The prompt to generate the image from.
    #[clap(short, long)]
    pub prompt: Prompt,

    /// The output directory to save the image to.
    #[clap(short = 'o', long, default_value = ".")]
    pub output_directory: PathBuf,

    /// The model to use. Not supported by all providers.
    #[clap(long)]
    pub model: Option<String>,

    /// The width of the image to generate. Defaults to 1024.
    #[clap(long, default_value = "1024")]
    pub width: u32,

    /// The height of the image to generate. Defaults to 1024.
    #[clap(long, default_value = "1024")]
    pub height: u32,

    /// How many steps to generate for.
    #[clap(long, default_value = "15")]
    pub steps: u32,

    /// The sampler name to use. Not supported by all providers.
    #[clap(long, default_value = "UniPC")]
    pub sampler_name: String,

    /// The CFG scale to use. Not supported by all providers.
    #[clap(long, default_value = "2")]
    pub cfg_scale: u32,
}

impl Default for ImageParams {
    fn default() -> Self {
        Self {
            prompt: Prompt::default(),
            output_directory: PathBuf::from("."),
            model: None,
            width: 1024,
            height: 1024,
            steps: 15,
            sampler_name: "UniPC".to_string(),
            cfg_scale: 2,
        }
    }
}
