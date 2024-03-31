import_stdlib!();

use crate::Tag;

/// A type with an associated CBOR tag.
///
/// Implementing this is a prerequisite for implementing `CBORTaggedEncodable`
/// and `CBORTaggedDecodable`.
pub trait CBORTagged {
    /// The CBOR tags assocated with this type. If more than one tag is present,
    /// they are considered equivalent for reading, but only the first one is
    /// used for writing.
    fn cbor_tags() -> Vec<Tag>;
}
