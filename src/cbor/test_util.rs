use crate::{util::hex::bytes_to_hex, cbor::decode::decode};

use super::cbor::CBOREncodable;

pub fn test_cbor<T>(t: T, expected_debug: &str, expected_display: &str, expected_data: &str) where T: CBOREncodable {
    let cbor = t.cbor();
    assert_eq!(format!("{:?}", cbor), expected_debug);
    assert_eq!(format!("{}", cbor), expected_display);
    let data = cbor.encode_cbor();
    assert_eq!(bytes_to_hex(&data), expected_data);
    let decoded_cbor = decode(&data).unwrap();
    assert_eq!(cbor, decoded_cbor);
}
