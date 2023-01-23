use super::{cbor::{CBOREncode, AsCBOR, CBOR, IntoCBOR}, varint::{VarIntEncode, MajorType}};

impl CBOREncode for &str {
    fn cbor_encode(&self) -> Vec<u8> {
        let mut buf = self.len().varint_encode(MajorType::String);
        for byte in self.bytes() {
            buf.push(byte);
        }
        buf
    }
}

impl CBOREncode for String {
    fn cbor_encode(&self) -> Vec<u8> {
        self.as_str().cbor_encode()
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
        test_cbor("Hello", r#"String("Hello")"#, "6548656c6c6f");
    }

    #[test]
    fn format() {
        assert_eq!(format!("{}", "Hello".as_cbor()), r#""Hello""#);
    }
}
