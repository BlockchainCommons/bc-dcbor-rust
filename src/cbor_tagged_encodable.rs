use crate::{tag::Tag, CBOR, CBOREncodable};

/// A type that can be encoded to CBOR with a specific tag.
///
/// Typically types that implement this trait will only provide the `CBOR_TAG`
/// associated constant and implement the `untagged_cbor` function.
pub trait CBORTaggedEncodable: CBOREncodable {
    const CBOR_TAG: Tag;

    fn untagged_cbor(&self) -> CBOR;

    fn tagged_cbor(&self) -> CBOR {
        CBOR::Tagged(Self::CBOR_TAG, Box::new(self.untagged_cbor()))
    }

    /// This override specifies that the default CBOR encoding will be tagged.
    fn cbor(&self) -> CBOR {
        self.tagged_cbor()
    }

    /// This override specifies that the default CBOR encoding will be tagged.
    fn cbor_data(&self) -> Vec<u8> {
        self.tagged_cbor().cbor_data()
    }
}
