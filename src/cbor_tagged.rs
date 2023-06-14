use crate::Tag;

/// A type with an associated CBOR tag.
///
/// Implementing this is a prerequisite for implementing `CBORTaggedEncodable`
/// and `CBORTaggedDecodable`.
pub trait CBORTagged {
    /// The CBOR tag assocated with this type.
    const CBOR_TAG: Tag;
}
