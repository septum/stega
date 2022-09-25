//! Core module that provides the main functionality.

mod carrier;
mod decode;
mod encode;
mod lsb;
mod payload;

use std::string::FromUtf8Error;

pub use carrier::Carrier;

pub use payload::Payload;

pub use encode::encode;

pub use decode::decode;

use thiserror::Error;

/// Errors that can occur while encoding or decoding a carrier
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum SteganographyError {
    /// Carrier capacity is less than necessary
    #[error("Carrier capacity is less than necessary")]
    SmallCarrier,
    /// Carrier has a malformed payload or no payload at all
    #[error("Carrier has a malformed payload or no payload at all")]
    MalformedPayload,
    /// An UTF-8 conversion error ocurred
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] FromUtf8Error),
}
