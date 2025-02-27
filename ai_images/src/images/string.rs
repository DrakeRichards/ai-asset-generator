//! Functions to handle the conversion of images from and to base64 encoding.
use anyhow::Result;
use base64::Engine;
use std::path::Path;

/// Convert a base64-encoded image to a PNG file which is saved to file_path.
pub fn base64_to_png(image: &str, file_path: &Path) -> Result<()> {
    // Check that file_path is not a directory.
    if file_path.is_dir() {
        return Err(anyhow::anyhow!("file_path must be a file path."));
    }
    let image = base64::prelude::BASE64_STANDARD.decode(image)?;
    std::fs::write(file_path, image)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_to_png() -> Result<()> {
        let image = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAABjElEQVR42mNk".to_string();
        let output_directory = Path::new(".");
        let output_path = output_directory.join("test_base64_to_png.png");
        base64_to_png(&image, &output_path)?;
        assert!(output_path.exists());
        std::fs::remove_file(output_path)?;
        Ok(())
    }
}
