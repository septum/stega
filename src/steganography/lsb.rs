const EMPTY_BYTE: u8 = 0b_0000_0000;
const LSB_MASK: u8 = 0b_0000_0001;
const LBS_INDEX: u8 = 0;
const LBS_ZERO_BIT_INDEXING: [u8; BITS_PER_BYTE] = [7, 6, 5, 4, 3, 2, 1, LBS_INDEX];

pub const BITS_PER_BYTE: usize = 8;

pub fn encode(byte: &mut u8, bit: bool) {
    if bit { set_lsb(byte) } else { clear_lsb(byte) }
}

pub fn byte_to_bits(byte: &u8) -> [bool; BITS_PER_BYTE] {
    LBS_ZERO_BIT_INDEXING.map(|index| is_bit_index_set(byte, index))
}

pub fn decode(bytes: &[u8]) -> u8 {
    bytes
        .iter()
        .zip(LBS_ZERO_BIT_INDEXING)
        .fold(EMPTY_BYTE, |byte, (subpixel, index)| {
            if is_bit_index_set(subpixel, LBS_INDEX) {
                byte + index_value(index)
            } else {
                byte
            }
        })
}

fn set_lsb(byte: &mut u8) {
    *byte |= LSB_MASK;
}

fn clear_lsb(byte: &mut u8) {
    *byte &= !LSB_MASK;
}

fn index_value(index: u8) -> u8 {
    LSB_MASK << index
}

fn is_bit_index_set(byte: &u8, index: u8) -> bool {
    *byte & index_value(index) != EMPTY_BYTE
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZERO_BYTE: u8 = 0b_0000_0000;
    const ONE_BYTE: u8 = 0b_0000_0001;
    const TWO_BYTE: u8 = 0b_0000_0010;
    const THREE_BYTE: u8 = 0b_0000_0011;

    const EMPTY_BYTE_BITS: [bool; BITS_PER_BYTE] =
        [false, false, false, false, false, false, false, false];

    const SET_BIT: bool = true;
    const UNSET_BIT: bool = false;

    const HASH_CHAR_BYTE: u8 = 0b_0010_0011;
    const HASH_CHAR_CARRIER_BYTES: [u8; BITS_PER_BYTE] = [
        ZERO_BYTE, ZERO_BYTE, ONE_BYTE, ZERO_BYTE, ZERO_BYTE, ZERO_BYTE, ONE_BYTE, ONE_BYTE,
    ];

    #[test]
    fn encode_bytes() {
        let mut byte = TWO_BYTE;

        encode(&mut byte, SET_BIT);
        assert_eq!(byte, THREE_BYTE);

        encode(&mut byte, UNSET_BIT);
        assert_eq!(byte, TWO_BYTE);
    }

    #[test]
    fn verify_bytes_to_bits() {
        let result = byte_to_bits(&EMPTY_BYTE)
            .into_iter()
            .zip(EMPTY_BYTE_BITS)
            .all(|(a, b)| a == b);
        assert!(result);
    }

    #[test]
    fn verify_decode() {
        assert_eq!(decode(&HASH_CHAR_CARRIER_BYTES), HASH_CHAR_BYTE);
    }
}
