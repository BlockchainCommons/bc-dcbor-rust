use crate::{hex::{hex_to_bytes, bytes_to_hex}, cbor::{CBOREncode, IntoCBOR, CBOR}, varint::VarIntEncode};

#[derive(Clone)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn new<T>(data: T) -> Bytes where T: AsRef<[u8]> {
        Bytes(data.as_ref().to_owned())
    }

    pub fn from_hex<T>(hex: T) -> Bytes where T: AsRef<str> {
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
        CBOR::BYTES(self.to_owned())
    }
}

impl std::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&bytes_to_hex(&self.0))
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_util::test_encode, cbor::IntoCBOR};

    use super::Bytes;

    #[test]
    fn encode() {
        test_encode(Bytes::new([0x11, 0x22, 0x33]), "43112233");
        test_encode(Bytes::from_hex("c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7"), "5820c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7");
    }

    #[test]
    fn into_cbor() {
        assert_eq!(format!("{:?}", Bytes::from_hex("010203").cbor()), "BYTES(010203)");
    }
}
