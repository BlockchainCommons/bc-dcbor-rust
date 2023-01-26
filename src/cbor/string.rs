use super::{cbor::{CBOREncodable, CBOR}, varint::{EncodeVarInt, MajorType}};

impl CBOREncodable for &str {
    fn cbor(&self) -> CBOR {
        CBOR::String(self.to_string())
    }

    fn encode_cbor(&self) -> Vec<u8> {
        let mut buf = self.len().encode_varint(MajorType::String);
        for byte in self.bytes() {
            buf.push(byte);
        }
        buf
    }
}

impl CBOREncodable for String {
    fn cbor(&self) -> CBOR {
        CBOR::String(self.to_string())
    }

    fn encode_cbor(&self) -> Vec<u8> {
        self.as_str().encode_cbor()
    }
}

#[cfg(test)]
mod tests {
    use crate::cbor::test_util::test_cbor;

    #[test]
    fn encode() {
        test_cbor("Hello", r#"String("Hello")"#, r#""Hello""#, "6548656c6c6f");
    }
}
