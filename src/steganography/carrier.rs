use std::slice::ChunksExact;

use image::RgbImage;

use super::{SteganographyError, lsb};

const MIN_CARRIER_CAPACITY: usize = 27;

/// Image that conceals or will conceal a payload
pub struct Carrier(RgbImage);

impl Carrier {
    /// Create a new carrier by wrapping an RGB image
    pub fn new(rgb_image: RgbImage) -> Result<Self, SteganographyError> {
        if rgb_image.len() >= MIN_CARRIER_CAPACITY {
            Ok(Self(rgb_image))
        } else {
            Err(SteganographyError::SmallCarrier)
        }
    }

    /// Get the available space in a carrier
    pub fn capacity(&self) -> usize {
        self.0.len()
    }

    /// Get an iterator of each color for every pixel
    pub fn subpixels(&mut self) -> impl Iterator<Item = &mut u8> + ExactSizeIterator {
        self.0.iter_mut()
    }

    /// Get and iterator of all the byte-worth chunks from a payload
    pub fn payload_chunks(&self) -> ChunksExact<'_, u8> {
        self.0.chunks_exact(lsb::BITS_PER_BYTE)
    }

    /// Get the contained RGB image
    pub fn unwrap(self) -> RgbImage {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHUNKS_IN_MIN_CAPACITY_CARRIER: usize = 3;

    #[test]
    fn create_new_carrier() {
        let rgb_image = RgbImage::new(3, 3);
        let result = Carrier::new(rgb_image);
        assert!(result.is_ok());
    }

    #[test]
    fn try_to_create_new_carrier() {
        let rgb_image = RgbImage::new(3, 2);
        let result = Carrier::new(rgb_image);
        assert!(result.is_err());

        let error = result.err().unwrap();
        assert!(matches!(error, SteganographyError::SmallCarrier));
    }

    #[test]
    fn verify_capacity() {
        let rgb_image = RgbImage::new(3, 3);
        let result = Carrier::new(rgb_image);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.capacity(), MIN_CARRIER_CAPACITY);
    }

    #[test]
    fn verify_chunks_size() {
        let rgb_image = RgbImage::new(3, 3);
        let result = Carrier::new(rgb_image);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(
            result.payload_chunks().len(),
            CHUNKS_IN_MIN_CAPACITY_CARRIER
        );
    }
}
