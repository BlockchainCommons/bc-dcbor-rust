
use crate::{CBOR, CBORDecodable, error::Error, CBORTagged};

/// A type that can be decoded from CBOR with a specific tag.
///
/// Typically types that implement this trait will only provide the `CBOR_TAG`
/// associated constant and implement the `from_untagged_cbor` function.
pub trait CBORTaggedDecodable: CBORDecodable + CBORTagged {
    /// Creates an instance of this type by decoding it from untagged CBOR.
    fn from_untagged_cbor(cbor: &CBOR) -> Result<Self, Error> where Self: Sized;

    /// Creates an instance of this type by decoding it from tagged CBOR.
    fn from_tagged_cbor(cbor: &CBOR) -> Result<Self, Error> where Self: Sized {
        match cbor {
            CBOR::Tagged(tag, item) => {
                if *tag == Self::CBOR_TAG {
                    Self::from_untagged_cbor(item)
                } else {
                    Err(Error::WrongTag(Self::CBOR_TAG, tag.clone()))
                }
            },
            _ => Err(Error::WrongType)
        }
    }

    /// Creates an instance of this type by decoding it from binary encoded tagged CBOR.
    fn from_tagged_cbor_data(data: &[u8]) -> Result<Self, Error> where Self: Sized {
        Self::from_tagged_cbor(&CBOR::from_data(data)?)
    }

    /// Creates an instance of this type by decoding it from binary encoded untagged CBOR.
    fn from_untagged_cbor_data(data: &[u8]) -> Result<Self, Error> where Self: Sized {
        Self::from_untagged_cbor(&CBOR::from_data(data)?)
    }
}
