use std::path::Path;

use stega::{decode, encode, open_image, save_image, Carrier, Payload};

const PAYLOAD_DATA: &str = "🦀";
const FERRIS_PATH: &str = "tests/files/ferris.png";
const CARRIER_PATH: &str = "tests/files/carrier.png";

#[test]
fn verify_conceal_functionality() {
    let payload = Payload::new(PAYLOAD_DATA);
    let rgb_image = open_image(Path::new(FERRIS_PATH)).unwrap();
    let mut carrier = Carrier::new(rgb_image).unwrap();

    assert!(encode(&payload, &mut carrier).is_ok());
    assert!(save_image(&carrier.unwrap(), Path::new(CARRIER_PATH)).is_ok());
}

#[test]
fn verify_reveal_functionality() {
    let rgb_image = open_image(Path::new(CARRIER_PATH)).unwrap();
    let carrier = Carrier::new(rgb_image).unwrap();
    assert!(decode(&carrier).is_ok());
}
