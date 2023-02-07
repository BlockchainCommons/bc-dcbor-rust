use crate::{cbor_encodable::CBOREncodable, CBORDecodable, decode_error::DecodeError, CBORCodable};

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
    fn from_cbor(cbor: &CBOR) -> Result<Box<Self>, crate::decode_error::DecodeError> {
        match cbor {
            CBOR::Text(s) => Ok(Box::new(s.clone())),
            _ => Err(DecodeError::WrongType),
        }
    }
}

impl CBORCodable for String { }
