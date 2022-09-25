//! A simple library to conceal and reveal UTF-8 encoded data within PNG images.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::cargo)]

mod image;
mod steganography;

pub use crate::image::{open_image, save_image, ImageError};

pub use crate::steganography::{decode, encode, Carrier, Payload, SteganographyError};
