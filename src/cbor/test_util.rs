use crate::{util::hex::bytes_to_hex, cbor::decode::cbor_decode};

use super::cbor::IntoCBOR;

pub fn test_cbor<T>(t: T, expected_cbor: &str, expected_data: &str) where T: IntoCBOR {
    let cbor = t.cbor();
    assert_eq!(format!("{:?}", cbor), expected_cbor);
    let data = cbor.encode();
    assert_eq!(bytes_to_hex(&data), expected_data);
    let decoded_cbor = cbor_decode(&data).unwrap();
    assert_eq!(cbor, decoded_cbor);
}
