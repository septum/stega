//! A simple library to conceal and reveal UTF-8 encoded data within PNG images.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::cargo)]

mod image;
mod steganography;

pub use crate::image::{ImageError, open_image, save_image};

pub use crate::steganography::{Carrier, Payload, SteganographyError, decode, encode};
