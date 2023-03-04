use crate::{cbor_encodable::CBOREncodable, CBORDecodable, CBORCodable, decode_error::DecodeError};

use super::{cbor::CBOR, varint::{EncodeVarInt, MajorType}};

impl<T> CBOREncodable for Vec<T> where T: CBOREncodable {
    fn cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.cbor()).collect())
    }

    fn cbor_data(&self) -> Vec<u8> {
        let mut buf = self.len().encode_varint(MajorType::Array);
        for item in self {
            buf.extend(item.cbor_data());
        }
        buf
    }
}

impl<T> TryFrom<CBOR> for Vec<T> where T: CBORDecodable {
    type Error = DecodeError;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        match cbor {
            CBOR::Array(cbor_array) => {
                let mut result = vec![];
                for cbor in cbor_array {
                    let element = T::from_cbor(&cbor)?;
                    result.push(*element);
                }
                Ok(result)
            },
            _ => Err(DecodeError::WrongType)
        }
    }
}

impl<T> CBORDecodable for Vec<T> where T: CBORDecodable {
    fn from_cbor(cbor: &CBOR) -> Result<Box<Self>, crate::decode_error::DecodeError> {
        match cbor {
            CBOR::Array(cbor_array) => {
                let mut result = vec![];
                for cbor in cbor_array {
                    let element = T::from_cbor(cbor)?;
                    result.push(*element);
                }
                Ok(Box::new(result))
            },
            _ => Err(DecodeError::WrongType)
        }
    }
}

impl<T> CBORCodable for Vec<T> where T: CBORCodable { }

impl CBOREncodable for Vec<Box<dyn CBOREncodable>> {
    fn cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.cbor()).collect())
    }

    fn cbor_data(&self) -> Vec<u8> {
        let mut buf = self.len().encode_varint(MajorType::Array);
        for item in self {
            buf.extend(item.cbor_data());
        }
        buf
    }
}

impl<T, const N: usize> CBOREncodable for [T; N] where T: CBOREncodable {
    fn cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.cbor()).collect())
    }

    fn cbor_data(&self) -> Vec<u8> {
        let mut buf = self.len().encode_varint(MajorType::Array);
        for item in self {
            buf.extend(item.cbor_data());
        }
        buf
    }
}

impl<T, const N: usize> CBOREncodable for &[T; N] where T: CBOREncodable {
    fn cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.cbor()).collect())
    }

    fn cbor_data(&self) -> Vec<u8> {
        let mut buf = self.len().encode_varint(MajorType::Array);
        for item in *self {
            buf.extend(item.cbor_data());
        }
        buf
    }
}
