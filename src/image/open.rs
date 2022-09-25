use std::{ffi::OsStr, fs::File, io::BufReader, path::Path};

use image::{self, ImageFormat, RgbImage};

use super::ImageError;

/// Opens an PNG image file as an RGB image
pub fn open_image(path: &Path) -> Result<RgbImage, ImageError> {
    if matches!(path.extension().and_then(OsStr::to_str), Some("png")) {
        let file = File::open(path)?;
        let image = image::load(BufReader::new(file), ImageFormat::Png)?;
        Ok(image.into_rgb8())
    } else {
        Err(ImageError::InvalidExtension)
    }
}

#[cfg(test)]
mod tests {
    use image::{self, EncodableLayout, RgbImage};
    use tempfile::Builder;

    use super::*;

    #[test]
    fn verify_open_image() {
        let tmpfile = Builder::new().suffix(".png").tempfile().unwrap();
        let rgb_image = RgbImage::new(5, 5);

        image::save_buffer(
            tmpfile.path(),
            rgb_image.as_bytes(),
            5,
            5,
            image::ColorType::Rgb8,
        )
        .unwrap();

        let result = open_image(tmpfile.path());
        assert!(result.is_ok());
    }
}
