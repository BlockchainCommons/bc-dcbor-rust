use crate::{tag::Tag, CBOR, CBOREncodable};

/// A type that can be encoded to CBOR with a specific tag.
///
/// Typically types that implement this trait will only provide the `CBOR_TAG`
/// associated constant and implement the `untagged_cbor` function.
pub trait CBORTaggedEncodable: CBOREncodable {
    fn tag() -> Tag;

    fn untagged_cbor(&self) -> CBOR;

    fn tagged_cbor(&self) -> CBOR {
        CBOR::Tagged(Self::tag(), Box::new(self.untagged_cbor()))
    }
}
