//! Generate images with a local Stable Diffusion instance.

mod api;
mod provider;

use super::{Base64Image, ImageParams, ImageProvider};
pub use provider::StableDiffusionXLProvider;
