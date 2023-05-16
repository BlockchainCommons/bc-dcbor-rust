use std::rc::Rc;

use crate::{CBOR, CBOREncodable, CBORTagged};

/// A type that can be encoded to CBOR with a specific tag.
///
/// Typically types that implement this trait will only provide the `CBOR_TAG`
/// associated constant and implement the `untagged_cbor` function.
pub trait CBORTaggedEncodable: CBOREncodable + CBORTagged {
    /// Returns the untagged CBOR encoding of this instance.
    fn untagged_cbor(&self) -> CBOR;

    /// Returns the tagged CBOR encoding of this instance.
    fn tagged_cbor(&self) -> CBOR {
        CBOR::Tagged(Self::CBOR_TAG, Rc::new(self.untagged_cbor()))
    }
}
