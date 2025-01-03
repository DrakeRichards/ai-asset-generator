pub mod config;
pub mod model;
pub mod queue;
pub mod status;
pub mod txt2img;

use super::{Base64Image, ImageParams, StableDiffusionXLProvider};
use clap::Subcommand;
use serde::Serialize;

/// Types of requests that can be sent to the Stable Diffusion API to generate images.
/// Right now only supports Txt2Img requests. Other potential requests include Img2Img and Control.
#[derive(Debug, Serialize, Subcommand)]
pub enum RequestBody {
    Txt2Img(txt2img::Txt2ImgRequestBody),
}
