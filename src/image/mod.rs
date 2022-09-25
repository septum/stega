//! Abstraction module to simplify the file operations on a PNG image.

mod open;
mod save;

pub use open::open_image;
pub use save::save_image;

use thiserror::Error;

/// Errors that can occur while opening or saving an image
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ImageError {
    /// File has an invalid extension
    #[error("File has an invalid extension")]
    InvalidExtension,
    /// An IO error occurred
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    /// An image error occurred
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
}
