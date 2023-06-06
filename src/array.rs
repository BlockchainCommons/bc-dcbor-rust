use std::collections::{VecDeque, HashSet};

use crate::{cbor_encodable::CBOREncodable, CBORDecodable, error::Error};

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

impl<T> TryFrom<CBOR> for Vec<T> where T: CBORDecodable + Clone {
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        match cbor {
            CBOR::Array(cbor_array) => {
                let mut result = Vec::new();
                for cbor in cbor_array {
                    let element = T::from_cbor(&cbor)?;
                    result.push(element.clone());
                }
                Ok(result)
            },
            _ => Err(Error::WrongType)
        }
    }
}

impl<T> From<&[T]> for CBOR where T: CBOREncodable {
    fn from(array: &[T]) -> Self {
        CBOR::Array(array.iter().map(|x| x.cbor()).collect())
    }
}

impl<T> CBORDecodable for Vec<T> where T: CBORDecodable + Clone {
    fn from_cbor(cbor: &CBOR) -> Result<Self, Error> {
        match cbor {
            CBOR::Array(cbor_array) => {
                let mut result = Vec::new();
                for cbor in cbor_array {
                    let element = T::from_cbor(cbor)?;
                    result.push(element.clone());
                }
                Ok(result)
            },
            _ => Err(Error::WrongType)
        }
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

impl<T> CBOREncodable for VecDeque<T> where T: CBOREncodable {
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

impl<T> TryFrom<CBOR> for VecDeque<T> where T: CBORDecodable + Clone {
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        match cbor {
            CBOR::Array(cbor_array) => {
                let mut result = VecDeque::new();
                for cbor in cbor_array {
                    let element = T::from_cbor(&cbor)?;
                    result.push_back(element.clone());
                }
                Ok(result)
            },
            _ => Err(Error::WrongType)
        }
    }
}

impl<T> From<&VecDeque<T>> for CBOR where T: CBOREncodable {
    fn from(array: &VecDeque<T>) -> Self {
        CBOR::Array(array.iter().map(|x| x.cbor()).collect())
    }
}

impl<T> CBOREncodable for HashSet<T> where T: CBOREncodable {
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

impl<T> TryFrom<CBOR> for HashSet<T> where T: CBORDecodable + Eq + std::hash::Hash + Clone {
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        match cbor {
            CBOR::Array(cbor_array) => {
                let mut result = HashSet::new();
                for cbor in cbor_array {
                    let element = T::from_cbor(&cbor)?;
                    result.insert(element.clone());
                }
                Ok(result)
            },
            _ => Err(Error::WrongType)
        }
    }
}
