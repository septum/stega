use super::lsb;

const START_OF_TEXT: char = '\u{02}';
const END_OF_TEXT: char = '\u{03}';

/// UTF-8 encoded data to be concealed in a carrier
pub struct Payload(String);

impl Payload {
    /// Create a new carrier from a string slice
    pub fn new(data: &str) -> Self {
        Payload(format!("{START_OF_TEXT}{data}{END_OF_TEXT}"))
    }

    /// Check if a byte is the start of text delimiter
    pub fn is_stx(byte: &u8) -> bool {
        *byte == START_OF_TEXT as u8
    }

    /// Check if a byte is not the end of text delimiter
    pub fn not_etx(byte: &u8) -> bool {
        *byte != END_OF_TEXT as u8
    }

    /// Get its total number of bits
    pub fn length(&self) -> usize {
        self.0.len() * lsb::BITS_PER_BYTE
    }

    /// Get an iterator of the payload bits
    pub fn bits(&self) -> impl Iterator<Item = bool> + '_ {
        self.0.as_bytes().iter().flat_map(lsb::byte_to_bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HASH_CHAR: char = '#';
    const HASH_CHAR_PAYLOAD_LENGTH: usize = 24;
    const HASH_CHAR_PAYLOAD_BITS: [bool; HASH_CHAR_PAYLOAD_LENGTH] = [
        false, false, false, false, false, false, true, false, false, false, true, false, false,
        false, true, true, false, false, false, false, false, false, true, true,
    ];

    #[test]
    fn create_new_payload() {
        let payload = Payload::new(&HASH_CHAR.to_string());

        let result = payload.length();
        assert_eq!(result, HASH_CHAR_PAYLOAD_LENGTH);

        let result = payload
            .bits()
            .zip(HASH_CHAR_PAYLOAD_BITS)
            .all(|(a, b)| a == b);
        assert!(result);
    }

    #[test]
    fn verify_delimiters() {
        let byte = START_OF_TEXT as u8;
        assert!(Payload::is_stx(&byte));
        assert!(Payload::not_etx(&byte));

        let byte = END_OF_TEXT as u8;
        assert!(!Payload::is_stx(&byte));
        assert!(!Payload::not_etx(&byte));
    }
}
