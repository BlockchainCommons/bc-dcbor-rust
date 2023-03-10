use crate::{CBOR, cbor_error::CBORError};

/// A type that can be decoded from CBOR.
pub trait CBORDecodable {
    /// Creates an instance of this type from CBOR symbolic representation.
    fn from_cbor(cbor: &CBOR) -> Result<Box<Self>, CBORError>;

    /// Creates an instance of this type from encoded CBOR binary data.
    fn from_cbor_data(cbor_data: &[u8]) -> Result<Box<Self>, CBORError> {
        Self::from_cbor(&CBOR::from_data(cbor_data)?)
    }
}
