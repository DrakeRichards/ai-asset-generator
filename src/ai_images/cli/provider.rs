use clap::{Args, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Args, Deserialize, Serialize, Debug)]
pub struct Provider {
    /// The name of the provider to use.
    #[clap(short = 'n', long = "provider-name")]
    pub name: ImageProviders,

    /// The configuration options needed by some providers.
    #[clap(flatten)]
    pub config: ProviderConfig,
}

impl Default for Provider {
    fn default() -> Self {
        Provider {
            name: ImageProviders::OpenAi,
            config: ProviderConfig { url: None },
        }
    }
}

#[derive(ValueEnum, Clone, Debug, Deserialize, Serialize)]
pub enum ImageProviders {
    OpenAi,
    StableDiffusion,
}

#[derive(Args, Deserialize, Serialize, Debug)]
pub struct ProviderConfig {
    /// The URL of the provider's API.
    #[clap(long)]
    pub url: Option<String>,
}
