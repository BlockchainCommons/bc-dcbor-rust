use crate::{tag::Tag, CBOR, CBOREncodable};

/// A type that can be encoded to CBOR with a specific tag.
///
/// Typically types that implement this trait will only provide the `CBOR_TAG`
/// associated constant and implement the `untagged_cbor` function.
pub trait CBORTaggedEncodable: CBOREncodable {
    /// The CBOR tag assocated with this type.
    const CBOR_TAG: Tag;

    /// Returns the untagged CBOR encoding of this instance.
    fn untagged_cbor(&self) -> CBOR;

    /// Returns the tagged CBOR encoding of this instance.
    fn tagged_cbor(&self) -> CBOR {
        CBOR::Tagged(Self::CBOR_TAG, Box::new(self.untagged_cbor()))
    }
}
