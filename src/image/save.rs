use std::{ffi::OsStr, path::Path};

use image::{self, ImageFormat, RgbImage};

use super::ImageError;

/// Saves an RGB image in the provided path
pub fn save_image(rgb_image: &RgbImage, path: &Path) -> Result<(), ImageError> {
    if matches!(path.extension().and_then(OsStr::to_str), Some("png")) {
        rgb_image.save_with_format(path, ImageFormat::Png)?;
        Ok(())
    } else {
        Err(ImageError::InvalidExtension)
    }
}

#[cfg(test)]
mod tests {
    use image::{self, RgbImage};
    use tempfile::Builder;

    use super::*;

    #[test]
    fn verify_save_image() {
        let tmpfile = Builder::new().suffix(".png").tempfile().unwrap();
        let rgb_image = RgbImage::new(5, 5);

        let result = save_image(&rgb_image, tmpfile.path());
        assert!(result.is_ok());
    }
}
