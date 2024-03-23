use bytes::Bytes;

use crate::{CBOR, CBORError, CBORCase};

/// A type that can be decoded from CBOR.
pub trait CBORDecodable: TryFrom<CBOR> + 'static {
    /// Creates an instance of this type from CBOR symbolic representation.
    fn from_cbor(cbor: &CBOR) -> anyhow::Result<Self> where Self: Sized;

    /// Creates an instance of this type from encoded CBOR binary data.
    fn from_cbor_data(cbor_data: &[u8]) -> anyhow::Result<Self> where Self: Sized {
        Self::from_cbor(&CBOR::from_data(cbor_data).map_err(anyhow::Error::msg)?)
    }
}

impl CBORDecodable for CBOR {
    fn from_cbor(cbor: &CBOR) -> anyhow::Result<Self> {
        Ok(cbor.clone())
    }
}

impl CBORDecodable for Bytes {
    fn from_cbor(cbor: &CBOR) -> anyhow::Result<Self> {
        match cbor.case() {
            CBORCase::ByteString(b) => Ok(b.clone()),
            _ => Err(anyhow::Error::msg(CBORError::WrongType))?
        }
    }
}
