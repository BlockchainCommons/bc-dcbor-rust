use crate::{cbor::CBOREncode, varint::VarIntEncode};

impl CBOREncode for &str {
    fn cbor_encode(&self) -> Vec<u8> {
        let mut data: Vec<u8> = self.bytes().collect();
        let mut buf = data.len().varint_encode(3);
        buf.append(&mut data);
        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::{hex::bytes_to_hex, cbor::CBOREncode, bytestring::ByteString};

    fn test_encode<T: CBOREncode>(t: T, expected: &str) {
        assert_eq!(bytes_to_hex(&t.cbor_encode()), expected);
    }

    #[test]
    fn encode_bytestring() {
        test_encode("Hello", "43112233");
    }
}
