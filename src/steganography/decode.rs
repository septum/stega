use super::{SteganographyError, carrier::Carrier, lsb, payload::Payload};

/// Reveal UTF-8 encoded data within a carrier by reading its least significant bits
pub fn decode(carrier: &Carrier) -> Result<String, SteganographyError> {
    let mut payload_bytes = carrier.payload_chunks().map(lsb::decode);

    if payload_bytes.next().filter(Payload::is_stx).is_none() {
        return Err(SteganographyError::MalformedPayload);
    }

    let text_bytes_length_limit = payload_bytes.len();
    let text_bytes: Vec<u8> = payload_bytes
        .map_while(|byte| Payload::not_etx(&byte).then_some(byte))
        .collect();

    if text_bytes.len() < text_bytes_length_limit {
        Ok(String::from_utf8(text_bytes)?)
    } else {
        Err(SteganographyError::MalformedPayload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbImage;

    const HASH_CHAR: char = '#';
    const CRAB_EMOJI: char = '🦀';

    const HASH_CHAR_CARRIER: [u8; 27] = [
        0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
    ];
    const CRAB_EMOJI_CARRIER: [u8; 54] = [
        0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1,
        1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0,
    ];
    const HASH_CHAR_TRAILING_CARRIER: [u8; 36] = [
        0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];
    const HASH_CHAR_NO_STX_CARRIER: [u8; 27] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
    ];
    const HASH_CHAR_NO_ETX_CARRIER: [u8; 27] = [
        0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const INVALID_UTF8_CARRIER: [u8; 54] = [
        0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0,
    ];

    #[test]
    fn decode_char_payload() {
        let rgb_image = RgbImage::from_vec(3, 3, HASH_CHAR_CARRIER.into()).unwrap();
        let carrier = Carrier::new(rgb_image).unwrap();

        let result = decode(&carrier);
        assert!(result.is_ok());

        let payload = result.unwrap();
        assert_eq!(payload, HASH_CHAR.to_string());
    }

    #[test]
    fn decode_emoji_payload() {
        let rgb_image = RgbImage::from_vec(6, 3, CRAB_EMOJI_CARRIER.into()).unwrap();
        let carrier = Carrier::new(rgb_image).unwrap();

        let result = decode(&carrier);
        assert!(result.is_ok());

        let payload = result.unwrap();
        assert_eq!(payload, CRAB_EMOJI.to_string());
    }

    #[test]
    fn decode_trailing_carrier_payload() {
        let rgb_image = RgbImage::from_vec(3, 3, HASH_CHAR_TRAILING_CARRIER.into()).unwrap();
        let carrier = Carrier::new(rgb_image).unwrap();

        let result = decode(&carrier);
        assert!(result.is_ok());

        let payload = result.unwrap();
        assert_eq!(payload, HASH_CHAR.to_string());
    }

    #[test]
    fn try_to_decode_no_stx_carrier() {
        let rgb_image = RgbImage::from_vec(3, 3, HASH_CHAR_NO_STX_CARRIER.into()).unwrap();
        let carrier = Carrier::new(rgb_image).unwrap();

        let result = decode(&carrier);
        assert!(result.is_err());

        let error = result.err().unwrap();
        assert!(matches!(error, SteganographyError::MalformedPayload));
    }

    #[test]
    fn try_to_decode_no_etx_carrier() {
        let rgb_image = RgbImage::from_vec(3, 3, HASH_CHAR_NO_ETX_CARRIER.into()).unwrap();
        let carrier = Carrier::new(rgb_image).unwrap();

        let result = decode(&carrier);
        assert!(result.is_err());

        let error = result.err().unwrap();
        assert!(matches!(error, SteganographyError::MalformedPayload));
    }

    #[test]
    fn try_to_decode_invalid_utf8_carrier() {
        let rgb_image = RgbImage::from_vec(6, 3, INVALID_UTF8_CARRIER.into()).unwrap();
        let carrier = Carrier::new(rgb_image).unwrap();

        let result = decode(&carrier);
        assert!(result.is_err());

        let error = result.err().unwrap();
        assert!(matches!(error, SteganographyError::Utf8Error(_)));
    }
}
