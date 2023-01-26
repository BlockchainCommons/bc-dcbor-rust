use super::{cbor::{CBOREncodable, CBOR}, varint::{EncodeVarInt, MajorType}, hex_to_bytes, bytes_to_hex};


/// A CBOR byte string.
#[derive(Clone)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    /// Creates a new CBOR byte string from the provided data.
    pub fn new<T>(data: T) -> Bytes where T: AsRef<[u8]> {
        Bytes(data.as_ref().to_owned())
    }

    /// Creates a new CBOR byte string from the provided hexadecimal string.
    ///
    /// Panics if the string is not well-formed, lower case hex with no spaces or
    /// other characters.
    pub fn from_hex<T>(hex: T) -> Bytes where T: AsRef<str> {
        Bytes(hex_to_bytes(hex))
    }

    /// The wrapped data.
    pub fn data(&self) -> &Vec<u8> {
        &self.0
    }
}

impl CBOREncodable for Bytes {
    fn cbor(&self) -> CBOR {
        CBOR::Bytes(self.to_owned())
    }

    fn encode_cbor(&self) -> Vec<u8> {
        let a = &self.0;
        let mut buf = a.len().encode_varint(MajorType::Bytes);
        for b in a {
            buf.push(*b);
        }
        buf
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
