pub mod string;

use anyhow::Result;
use std::path::Path;

/// A base64-encoded image.
pub struct Base64Image {
    pub image: String,
}

impl Base64Image {
    pub fn to_file(&self, path: &Path) -> Result<()> {
        string::base64_to_png(&self.image, path)?;
        Ok(())
    }
}
