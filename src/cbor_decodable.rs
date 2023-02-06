use crate::{CBOR, decode_error::DecodeError, decode_cbor};

/// A type that can be decoded from CBOR.
pub trait CBORDecodable {
    /// Creates an instance of this type from CBOR symbolic representation.
    fn from_cbor(cbor: &CBOR) -> Result<Box<Self>, DecodeError>;

    /// Creates an instance of this type from encoded CBOR binary data.
    fn from_cbor_data(cbor_data: &[u8]) -> Result<Box<Self>, DecodeError> {
        let cbor = decode_cbor(cbor_data)?;
        Self::from_cbor(&cbor)
    }
}
