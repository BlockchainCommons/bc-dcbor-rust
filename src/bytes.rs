use crate::{cbor::CBOREncode, varint::VarIntEncode, cbor::{IntoCBOR, CBOR}, hex::hex_to_bytes};

#[derive(Clone, Debug)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn new(data: &[u8]) -> Bytes {
        Bytes(data.to_owned())
    }

    pub fn from_hex(hex: &str) -> Bytes {
        Bytes(hex_to_bytes(hex))
    }
}

impl CBOREncode for Bytes {
    fn cbor_encode(&self) -> Vec<u8> {
        let a = &self.0;
        let mut buf = a.len().varint_encode(2);
        for b in a {
            buf.push(*b);
        }
        buf
    }
}

impl IntoCBOR for Bytes {
    fn cbor(&self) -> CBOR {
        CBOR::BYTES(self.0.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_util::test_encode, hex::hex_to_bytes};
    use super::Bytes;

    #[test]
    fn encode_bytestring() {
        test_encode(Bytes::new(&[0x11, 0x22, 0x33]), "43112233");
        test_encode(Bytes::new(&hex_to_bytes("c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7")), "5820c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7");
    }
}
