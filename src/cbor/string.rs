use super::{cbor::{EncodeCBOR, AsCBOR, CBOR, IntoCBOR}, varint::{EncodeVarInt, MajorType}};

impl EncodeCBOR for &str {
    fn encode_cbor(&self) -> Vec<u8> {
        let mut buf = self.len().encode_varint(MajorType::String);
        for byte in self.bytes() {
            buf.push(byte);
        }
        buf
    }
}

impl EncodeCBOR for String {
    fn encode_cbor(&self) -> Vec<u8> {
        self.as_str().encode_cbor()
    }
}

impl AsCBOR for &str {
    fn as_cbor(&self) -> CBOR {
        CBOR::String(self.to_string())
    }
}

impl IntoCBOR for String {
    fn into_cbor(self) -> CBOR {
        CBOR::String(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::cbor::{test_util::test_cbor, cbor::AsCBOR};

    #[test]
    fn encode() {
        test_cbor("Hello", r#"String("Hello")"#, r#""Hello""#, "6548656c6c6f");
    }

    #[test]
    fn format() {
        assert_eq!(format!("{}", "Hello".as_cbor()), r#""Hello""#);
    }
}
