use crate::{cbor::{CBOREncode, IntoCBOR}, hex::bytes_to_hex, map::CBORMap};

pub fn test_encode<T: CBOREncode>(t: T, expected: &str) {
    assert_eq!(bytes_to_hex(&t.cbor_encode()), expected);
}

// pub fn test_encode_array<T: CBOREncode>(t: &[T], expected: &str) {
//     assert_eq!(bytes_to_hex(&t.cbor_encode()), expected);
// }

// pub fn test_encode_heterogenous_array(t: &[Box<dyn CBOREncode>], expected: &str) {
//     assert_eq!(bytes_to_hex(&t.cbor_encode()), expected);
// }

pub fn test_encode_map(t: &CBORMap, expected: &str) {
    assert_eq!(bytes_to_hex(&t.cbor_encode()), expected);
}

pub fn test_cbor<T>(t: T, expected_cbor: &str, expected_data: &str) where T: IntoCBOR {
    let cbor = t.cbor();
    assert_eq!(format!("{:?}", cbor), expected_cbor);
    let data = cbor.encode();
    assert_eq!(bytes_to_hex(data), expected_data);
}
