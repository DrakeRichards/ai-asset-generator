use super::provider::Provider;
use crate::ImageParams;
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Generate images using an image generation API.
#[derive(Parser)]
pub struct Commands {
    /// Specify how to load image generation parameters.
    #[command(subcommand)]
    pub parameter_source: ParameterSource,

    /// The prompt to send to the image generation API.
    #[arg(short, long)]
    pub prompt: Option<String>,
}

/// Specify how to load image generation parameters.
#[derive(Subcommand, Debug)]
pub enum ParameterSource {
    /// Load parameters from a TOML file.
    Toml(TomlArgs),
    /// Load parameters from command-line arguments.
    Args(GenerationParameters),
}

/// Load parameters from a TOML file.
#[derive(Args, Deserialize, Debug, Default, Serialize)]
pub struct TomlArgs {
    /// The path to the TOML file containing image generation parameters.
    #[arg(short, long)]
    pub file: PathBuf,
}

/// Generate images using an image generation API.
#[derive(Args, Deserialize, Debug, Default, Serialize)]
#[group(required = false, multiple = true)]
pub struct GenerationParameters {
    /// The image generation provider to use.
    #[clap(flatten)]
    pub provider: Provider,

    /// Parameters for the image generation request.
    #[clap(flatten)]
    pub params: ImageParams,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::fs;
    use std::path::PathBuf;
    use toml::from_str;

    const TEST_CONFIG_PATH: &str = "test-config.toml";

    /// Generate a default CommandArgs struct and convert it to a TOML string, then save it to a file.
    fn generate_toml_config() -> Result<PathBuf> {
        let config_args = GenerationParameters::default();
        let toml_string = toml::to_string(&config_args)?;
        let toml_file = PathBuf::from(TEST_CONFIG_PATH);
        fs::write(&toml_file, toml_string)?;
        Ok(toml_file)
    }

    #[test]
    fn test_read_from_toml_file() -> Result<()> {
        let toml_file = generate_toml_config()?;
        let config_args = {
            let toml_content = fs::read_to_string(&toml_file)?;
            from_str::<GenerationParameters>(&toml_content)?
        };
        let expected_args = GenerationParameters::default();
        // We can't compare the two structs directly they don't implement PartialEq.
        // Instead, compare the serialized versions of the structs.
        assert_eq!(
            toml::to_string(&config_args)?,
            toml::to_string(&expected_args)?
        );
        Ok(())
    }
}
