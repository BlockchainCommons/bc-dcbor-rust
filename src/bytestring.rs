use crate::{cbor::CBOREncode, varint::VarIntEncode};

pub struct ByteString(Vec<u8>);

impl ByteString {
    pub fn new(data: &[u8]) -> ByteString {
        ByteString(data.to_owned())
    }
}

impl CBOREncode for ByteString {
    fn cbor_encode(&self) -> Vec<u8> {
        let a = &self.0;
        let mut buf = a.len().varint_encode(2);
        buf.append(&mut a.to_owned());
        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::{hex::{bytes_to_hex, hex_to_bytes}, cbor::CBOREncode, bytestring::ByteString};

    fn test_encode<T: CBOREncode>(t: T, expected: &str) {
        assert_eq!(bytes_to_hex(&t.cbor_encode()), expected);
    }

    #[test]
    fn encode_bytestring() {
        test_encode(ByteString::new(&[0x11, 0x22, 0x33]), "43112233");
        test_encode(ByteString::new(&hex_to_bytes("c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7")), "5820c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7");
    }
}
