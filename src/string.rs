use crate::{cbor::CBOREncode, varint::VarIntEncode, cbor::{IntoCBOR, CBOR}};

impl CBOREncode for &str {
    fn cbor_encode(&self) -> Vec<u8> {
        let mut buf = self.len().varint_encode(3);
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
    use crate::{test_util::test_encode, cbor::IntoCBOR};

    #[test]
    fn encode() {
        test_encode("Hello", "6548656c6c6f");
    }

    #[test]
    fn into_cbor() {
        assert_eq!(format!("{:?}", "Hello".cbor()), "STRING(\"Hello\")");
    }
}
