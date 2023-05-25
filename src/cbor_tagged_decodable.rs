use std::rc::Rc;

use crate::{CBOR, CBORDecodable, cbor_error::CBORError, CBORTagged};

/// A type that can be decoded from CBOR with a specific tag.
///
/// Typically types that implement this trait will only provide the `CBOR_TAG`
/// associated constant and implement the `from_untagged_cbor` function.
pub trait CBORTaggedDecodable: CBORDecodable + CBORTagged {
    /// Creates an instance of this type by decoding it from untagged CBOR.
    fn from_untagged_cbor(cbor: &CBOR) -> Result<Rc<Self>, CBORError>;

    /// Creates an instance of this type by decoding it from tagged CBOR.
    fn from_tagged_cbor(cbor: &CBOR) -> Result<Rc<Self>, CBORError> {
        match cbor {
            CBOR::Tagged(tag, item) => {
                if *tag == Self::CBOR_TAG {
                    Self::from_untagged_cbor(item)
                } else {
                    Err(CBORError::WrongTag(Self::CBOR_TAG, tag.clone()))
                }
            },
            _ => Err(CBORError::WrongType)
        }
    }

    /// Creates an instance of this type by decoding it from binary encoded tagged CBOR.
    fn from_tagged_cbor_data(data: &[u8]) -> Result<Rc<Self>, CBORError> {
        Self::from_tagged_cbor(&CBOR::from_data(data)?)
    }

    /// Creates an instance of this type by decoding it from binary encoded untagged CBOR.
    fn from_untagged_cbor_data(data: &[u8]) -> Result<Rc<Self>, CBORError> {
        Self::from_untagged_cbor(&CBOR::from_data(data)?)
    }
}
