use std::collections::{VecDeque, HashSet};

use crate::{cbor_encodable::CBOREncodable, CBORDecodable, error::CBORError};

use super::{cbor::CBOR, varint::{EncodeVarInt, MajorType}};

use anyhow::bail;

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

impl<T> From<Vec<T>> for CBOR where T: CBOREncodable {
    fn from(vec: Vec<T>) -> Self {
        vec.cbor()
    }
}

impl<T> TryFrom<CBOR> for Vec<T> where T: CBORDecodable + Clone {
    type Error = anyhow::Error;

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
            _ => bail!(CBORError::WrongType)
        }
    }
}

impl<T> From<&[T]> for CBOR where T: CBOREncodable {
    fn from(array: &[T]) -> Self {
        CBOR::Array(array.iter().map(|x| x.cbor()).collect())
    }
}

impl<T> CBORDecodable for Vec<T> where T: CBORDecodable + Clone {
    fn from_cbor(cbor: &CBOR) -> anyhow::Result<Self> {
        match cbor {
            CBOR::Array(cbor_array) => {
                let mut result = Vec::new();
                for cbor in cbor_array {
                    let element = T::from_cbor(cbor)?;
                    result.push(element.clone());
                }
                Ok(result)
            },
            _ => bail!(CBORError::WrongType)
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

impl<T, const N: usize> From<[T; N]> for CBOR where T: CBOREncodable {
    fn from(array: [T; N]) -> Self {
        array.cbor()
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

impl<T> From<VecDeque<T>> for CBOR where T: CBOREncodable {
    fn from(deque: VecDeque<T>) -> Self {
        deque.cbor()
    }
}

impl<T> TryFrom<CBOR> for VecDeque<T> where T: CBORDecodable + Clone {
    type Error = anyhow::Error;

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
            _ => bail!(CBORError::WrongType)
        }
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

impl<T> From<HashSet<T>> for CBOR where T: CBOREncodable {
    fn from(set: HashSet<T>) -> Self {
        set.cbor()
    }
}

impl<T> TryFrom<CBOR> for HashSet<T> where T: CBORDecodable + Eq + std::hash::Hash + Clone {
    type Error = anyhow::Error;

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
            _ => bail!(CBORError::WrongType)
        }
    }
}
