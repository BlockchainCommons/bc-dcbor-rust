use crate::{util::hex::bytes_to_hex, cbor::decode::decode_cbor};

use super::cbor::CBOREncodable;

pub fn test_cbor<T>(t: T, expected_debug: &str, expected_display: &str, expected_data: &str) where T: CBOREncodable {
    let cbor = t.as_cbor();
    assert_eq!(format!("{:?}", cbor), expected_debug);
    assert_eq!(format!("{}", cbor), expected_display);
    let data = cbor.encode();
    assert_eq!(bytes_to_hex(&data), expected_data);
    let decoded_cbor = decode_cbor(&data).unwrap();
    assert_eq!(cbor, decoded_cbor);
}
