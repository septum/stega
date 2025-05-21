use super::{SteganographyError, carrier::Carrier, lsb, payload::Payload};

/// Encodes a payload into a carrier by overwriting in place its least significant bits
pub fn encode(payload: &Payload, carrier: &mut Carrier) -> Result<(), SteganographyError> {
    if carrier.capacity() >= payload.length() {
        carrier
            .subpixels()
            .zip(payload.bits())
            .for_each(|(subpixel, bit)| lsb::encode(subpixel, bit));
        Ok(())
    } else {
        Err(SteganographyError::SmallCarrier)
    }
}

#[cfg(test)]
mod tests {
    use image::RgbImage;

    use super::*;

    const HASH_CHAR: char = '#';
    const SHEBANG_STR: &str = "#!";

    const HASH_CHAR_CARRIER: [u8; 27] = [
        0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
    ];

    #[test]
    fn encode_char_payload() {
        let rgb_image = RgbImage::from_vec(3, 3, HASH_CHAR_CARRIER.into()).unwrap();
        let mut carrier = Carrier::new(rgb_image).unwrap();
        let payload = Payload::new(&HASH_CHAR.to_string());

        let result = encode(&payload, &mut carrier);
        assert!(result.is_ok());

        let result = carrier
            .subpixels()
            .zip(&HASH_CHAR_CARRIER)
            .all(|(a, b)| a == b);
        assert!(result);
    }

    #[test]
    fn try_to_encode_into_small_carrier() {
        let rgb_image = RgbImage::new(3, 3);
        let mut carrier = Carrier::new(rgb_image).unwrap();
        let payload = Payload::new(&SHEBANG_STR.to_string());

        let result = encode(&payload, &mut carrier);
        assert!(result.is_err());

        let error = result.err().unwrap();
        assert!(matches!(error, SteganographyError::SmallCarrier));
    }
}
