use bytes::Bytes;

use crate::{CBOR, CBORError};

/// A type that can be decoded from CBOR.
pub trait CBORDecodable: 'static {
    /// Creates an instance of this type from CBOR symbolic representation.
    fn from_cbor(cbor: &CBOR) -> anyhow::Result<Self> where Self: Sized;

    /// Creates an instance of this type from encoded CBOR binary data.
    fn from_cbor_data(cbor_data: &[u8]) -> anyhow::Result<Self> where Self: Sized {
        Self::from_cbor(&CBOR::from_data(cbor_data)?)
    }
}

impl CBORDecodable for CBOR {
    fn from_cbor(cbor: &CBOR) -> anyhow::Result<Self> {
        Ok(cbor.clone())
    }
}

impl CBORDecodable for Bytes {
    fn from_cbor(cbor: &CBOR) -> anyhow::Result<Self> {
        match cbor {
            CBOR::ByteString(b) => Ok(b.clone()),
            _ => Err(CBORError::WrongType)?
        }
    }
}
