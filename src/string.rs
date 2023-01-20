use crate::{cbor::CBOREncode, varint::VarIntEncode};

impl CBOREncode for &str {
    fn cbor_encode(&self) -> Vec<u8> {
        let mut buf = self.len().varint_encode(3);
        for byte in self.bytes() {
            buf.push(byte);
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::test_encode;

    #[test]
    fn encode_string() {
        test_encode("Hello", "6548656c6c6f");
    }
}
