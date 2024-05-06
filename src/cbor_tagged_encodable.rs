import_stdlib!();

use crate::{CBOR, CBORTagged, CBORCase};

/// A type that can be encoded to CBOR with a specific tag.
///
/// Typically types that implement this trait will only provide the `CBOR_TAG`
/// associated constant and implement the `untagged_cbor` function.
pub trait CBORTaggedEncodable: CBORTagged {
    /// Returns the untagged CBOR encoding of this instance.
    fn untagged_cbor(&self) -> CBOR;

    /// Returns the tagged CBOR encoding of this instance.
    fn tagged_cbor(&self) -> CBOR {
        CBORCase::Tagged(Self::cbor_tags()[0].clone(), self.untagged_cbor()).into()
    }

    /// Returns the tagged value in CBOR binary representation.
    fn tagged_cbor_data(&self) -> Vec<u8> {
        self.tagged_cbor().cbor_data()
    }
}
