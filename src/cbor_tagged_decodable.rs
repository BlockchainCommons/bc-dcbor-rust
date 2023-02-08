use crate::{CBOR, CBORDecodable, tag::Tag, decode_error::DecodeError, decode_cbor};

/// A type that can be decoded from CBOR with a specific tag.
///
/// Typically types that implement this trait will only provide the `CBOR_TAG`
/// associated constant and implement the `from_untagged_cbor` function.
pub trait CBORTaggedDecodable: CBORDecodable {
    fn tag() -> Tag;

    fn from_untagged_cbor(cbor: &CBOR) -> Result<Box<Self>, DecodeError>;

    /// Creates an instance of this type by decoding it from tagged CBOR.
    fn from_tagged_cbor(cbor: &CBOR) -> Result<Box<Self>, DecodeError> {
        match cbor {
            CBOR::Tagged(tag, item) => {
                if *tag == Self::tag() {
                    return Ok(Self::from_untagged_cbor(item)?);
                } else {
                    return Err(DecodeError::WrongTag(Self::tag(), tag.clone()));
                }
            },
            _ => return Err(DecodeError::WrongType)
        }
    }

    /// Creates an instance of this type by decoding it from binary encoded tagged CBOR.
    fn from_tagged_cbor_data(data: &[u8]) -> Result<Box<Self>, DecodeError> {
        let cbor = decode_cbor(data)?;
        Self::from_tagged_cbor(&cbor)
    }

    /// Creates an instance of this type by decoding it from binary encoded untagged CBOR.
    fn from_untagged_cbor_data(data: &[u8]) -> Result<Box<Self>, DecodeError> {
        let cbor = decode_cbor(data)?;
        Self::from_untagged_cbor(&cbor)
    }
}
