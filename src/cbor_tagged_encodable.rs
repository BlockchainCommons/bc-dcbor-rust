
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
        CBOR::Tagged(Self::CBOR_TAG, Box::new(self.untagged_cbor()))
    }

    /// Returns the tagged value in CBOR binary representation.
    fn tagged_cbor_data(&self) -> Vec<u8> {
        self.tagged_cbor().cbor_data()
    }
}
