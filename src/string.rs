use crate::{cbor_encodable::CBOREncodable, CBORDecodable, error::Error, CBORCodable};

use super::{cbor::CBOR, varint::{EncodeVarInt, MajorType}};

impl CBOREncodable for &str {
    fn cbor(&self) -> CBOR {
        CBOR::Text(self.to_string())
    }

    fn cbor_data(&self) -> Vec<u8> {
        let mut buf = self.len().encode_varint(MajorType::Text);
        for byte in self.bytes() {
            buf.push(byte);
        }
        buf
    }
}

impl CBOREncodable for String {
    fn cbor(&self) -> CBOR {
        CBOR::Text(self.clone())
    }

    fn cbor_data(&self) -> Vec<u8> {
        self.as_str().cbor_data()
    }
}

impl CBORDecodable for String {
    fn from_cbor(cbor: &CBOR) -> Result<Self, Error> {
        match cbor {
            CBOR::Text(s) => Ok(s.clone()),
            _ => Err(Error::WrongType),
        }
    }
}

impl CBORCodable for String { }

impl From<&str> for CBOR {
    fn from(value: &str) -> Self {
        value.cbor()
    }
}

impl From<String> for CBOR {
    fn from(value: String) -> Self {
        value.cbor()
    }
}

impl From<CBOR> for String {
    fn from(value: CBOR) -> Self {
        Self::from_cbor(&value).unwrap()
    }
}

impl TryFrom<&CBOR> for String {
    type Error = Error;

    fn try_from(value: &CBOR) -> Result<Self, Self::Error> {
        match value {
            CBOR::Text(s) => Ok(s.clone()),
            _ => Err(Error::WrongType),
        }
    }
}
