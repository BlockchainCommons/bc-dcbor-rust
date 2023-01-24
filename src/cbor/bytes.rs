use crate::util::hex::{hex_to_bytes, bytes_to_hex};

use super::{cbor::{EncodeCBOR, AsCBOR, CBOR, IntoCBOR}, varint::{EncodeVarInt, MajorType}};

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

impl EncodeCBOR for Bytes {
    fn encode_cbor(&self) -> Vec<u8> {
        let a = &self.0;
        let mut buf = a.len().encode_varint(MajorType::Bytes);
        for b in a {
            buf.push(*b);
        }
        buf
    }
}

impl AsCBOR for Bytes {
    fn as_cbor(&self) -> CBOR {
        CBOR::Bytes(self.to_owned())
    }
}

impl IntoCBOR for Bytes {
    fn into_cbor(self) -> CBOR {
        CBOR::Bytes(self)
    }
}

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&bytes_to_hex(&self.0))
    }
}

impl std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("h'")?;
        f.write_str(&bytes_to_hex(&self.0))?;
        f.write_str("'")
    }
}

#[cfg(test)]
mod tests {
    use crate::cbor::{test_util::test_cbor, cbor::IntoCBOR};

    use super::Bytes;

    #[test]
    fn encode() {
        // test_cbor(Bytes::new([0x11, 0x22, 0x33]), "Bytes(112233)", "43112233");
        test_cbor(
            Bytes::from_hex("c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7"),
            "Bytes(c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7)",
            "h'c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7'",
            "5820c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7"
        );
    }
}
