use crate::{cbor_encodable::CBOREncodable, CBORDecodable, cbor_error::CBORError, CBORCodable};

use super::{cbor::CBOR, varint::{EncodeVarInt, MajorType}};


/// A CBOR byte string.
#[derive(Clone)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    /// Creates a new CBOR byte string from the provided data.
    pub fn from_data<T>(data: T) -> Bytes where T: AsRef<[u8]> {
        Bytes(data.as_ref().to_owned())
    }

    /// Creates a new CBOR byte string from the provided hexadecimal string.
    ///
    /// Panics if the string is not well-formed hexadecimal with no spaces or
    /// other characters.
    pub fn from_hex<T>(hex: T) -> Bytes where T: AsRef<str> {
        Bytes(hex::decode(hex.as_ref()).unwrap())
    }

    /// The wrapped data.
    pub fn data(&self) -> &Vec<u8> {
        &self.0
    }

    /// The wrapped data as a hexadecimal string.
    pub fn hex(&self) -> String {
        hex::encode(&self.0)
    }

    /// The length of the wrapped data in bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the wrapped data is empty, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl CBOREncodable for Bytes {
    fn cbor(&self) -> CBOR {
        CBOR::Bytes(self.to_owned())
    }

    fn cbor_data(&self) -> Vec<u8> {
        let a = &self.0;
        let mut buf = a.len().encode_varint(MajorType::Bytes);
        for b in a {
            buf.push(*b);
        }
        buf
    }
}

impl CBORDecodable for Bytes {
    fn from_cbor(cbor: &CBOR) -> Result<Box<Self>, crate::cbor_error::CBORError> {
        match cbor {
            CBOR::Bytes(data) => Ok(Box::new(data.clone())),
            _ => Err(CBORError::WrongType),
        }
    }
}

impl CBORCodable for Bytes { }

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&hex::encode(&self.0))
    }
}

impl std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("h'")?;
        f.write_str(&hex::encode(&self.0))?;
        f.write_str("'")
    }
}

impl From<&Bytes> for CBOR {
    fn from(value: &Bytes) -> Self {
        value.cbor()
    }
}

impl From<Bytes> for CBOR {
    fn from(value: Bytes) -> Self {
        value.cbor()
    }
}

impl From<CBOR> for Bytes {
    fn from(value: CBOR) -> Self {
        *Self::from_cbor(&value).unwrap()
    }
}

impl TryFrom<&CBOR> for Bytes {
    type Error = CBORError;

    fn try_from(value: &CBOR) -> Result<Self, Self::Error> {
        match value {
            CBOR::Bytes(bytes) => Ok(bytes.clone()),
            _ => Err(CBORError::WrongType),
        }
    }
}

impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Bytes::from_data(value)
    }
}

impl<const L: usize> From<&[u8; L]> for Bytes {
    fn from(value: &[u8; L]) -> Self {
        Bytes::from_data(value)
    }
}
