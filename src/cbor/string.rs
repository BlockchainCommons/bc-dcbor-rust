use super::{cbor::{CBOREncode, IntoCBOR, CBOR}, varint::{VarIntEncode, MajorType}};

impl CBOREncode for &str {
    fn cbor_encode(&self) -> Vec<u8> {
        let mut buf = self.len().varint_encode(MajorType::STRING);
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

impl IntoCBOR for &str {
    fn cbor(&self) -> CBOR {
        CBOR::STRING(self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::cbor::{test_util::test_cbor, cbor::IntoCBOR};

    #[test]
    fn encode() {
        test_cbor("Hello", r#"STRING("Hello")"#, "6548656c6c6f");
    }

    #[test]
    fn format() {
        assert_eq!(format!("{}", "Hello".cbor()), r#""Hello""#);
    }
}
