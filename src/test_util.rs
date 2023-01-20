use crate::{cbor::CBOREncode, hex::bytes_to_hex};

pub fn test_encode<T: CBOREncode>(t: T, expected: &str) {
    assert_eq!(bytes_to_hex(&t.cbor_encode()), expected);
}

pub fn test_encode_array<T: CBOREncode>(t: &[T], expected: &str) {
    assert_eq!(bytes_to_hex(&t.cbor_encode()), expected);
}

pub fn test_encode_heterogenous_array(t: &Vec<Box<dyn CBOREncode>>, expected: &str) {
    assert_eq!(bytes_to_hex(&t.cbor_encode()), expected);
}
