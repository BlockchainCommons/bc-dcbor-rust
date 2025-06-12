import_stdlib!();

use crate::Tag;

/// # CBOR Tagged Value Support
///
/// CBOR allows values to be "tagged" with semantic information using tag
/// numbers. The dCBOR library provides a set of traits for working with tagged
/// values in a type-safe manner.
///
/// Tags in CBOR provide additional context about how a value should be
/// interpreted. For example, tag 1 is used for dates, indicating the value is a
/// timestamp.
///
/// This trait system allows Rust types to define their associated CBOR tags
/// and provide serialization/deserialization logic specifically for tagged
/// values. A trait for types that have an associated CBOR tag.
///
/// In CBOR, tags provide semantic information about how to interpret data
/// items. This trait defines which CBOR tag(s) are associated with a particular
/// Rust type.
///
/// Implementing this trait is a prerequisite for implementing
/// `CBORTaggedEncodable` and `CBORTaggedDecodable`.
///
/// ## Multiple Tags for Backward Compatibility
///
/// The `cbor_tags()` method returns a vector of tags, enabling support for
/// backward compatibility with older tag versions:
///
/// - **When encoding**: Only the first tag in the vector is used for
///   serialization
/// - **When decoding**: Any of the tags in the vector will be accepted
///
/// This design solves several real-world problems:
///
/// 1. **IANA Registration Simplification**: If you initially choose a tag in
///    the Specification Required range (24-32767) and later want to move to the
///    simpler First Come First Served range (32768+), you can migrate while
///    maintaining compatibility with existing data.
///
/// 2. **Protocol Evolution**: As your protocol evolves, you can introduce new
///    preferred tags while still supporting data encoded with older tags.
///
/// 3. **Versioning**: Different tags can represent different versions of your
///    data format while sharing the same Rust type for handling.
///
/// ## Example: Single Tag
///
/// ```
/// use dcbor::prelude::*;
///
/// // Define a Date struct
/// #[derive(Debug, Clone, PartialEq)]
/// struct Date(f64); // Storing timestamp as seconds since epoch
///
/// // Associate with CBOR tag 1 (standard date tag)
/// impl CBORTagged for Date {
///     fn cbor_tags() -> Vec<Tag> { vec![Tag::with_value(1)] }
/// }
///
/// // Now the Date type has an association with CBOR tag 1
/// let tags = Date::cbor_tags();
/// assert_eq!(tags[0].value(), 1);
/// ```
///
/// ## Example: Multiple Tags for Backward Compatibility
///
/// ```
/// use dcbor::prelude::*;
///
/// // Seed data structure for cryptographic operations
/// #[derive(Debug, Clone, PartialEq)]
/// struct Seed {
///     bytes: [u8; 16], // Example seed data
/// }
///
/// // Associate with two tags:
/// // - Tag 40300 (First Come First Served IANA range, used for encoding)
/// // - Tag 300 (legacy tag in the Specification Required range, still accepted for decoding)
/// impl CBORTagged for Seed {
///     fn cbor_tags() -> Vec<Tag> {
///         vec![
///             // Primary tag (used for all new encodings)
///             Tag::with_value(40300), // Higher range (32768+) only needs First Come First Served registration
///             
///             // Legacy tag (accepted when decoding old data)
///             Tag::with_value(300),   // Originally chosen tag in Specification Required range (24-32767)
///         ]
///     }
/// }
///
/// // New data is encoded with tag 40300
/// let seed = Seed { bytes: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] };
///
/// // But when decoding, the system can still read data tagged with either
/// // 40300 or 300, ensuring backward compatibility with existing data
/// ```
pub trait CBORTagged {
    /// Returns the CBOR tags associated with this type.
    ///
    /// This method should return a vector of tags in order of preference:
    ///
    /// - The first tag in the vector is the "preferred" tag and will be used
    ///   when encoding values of this type via
    ///   `CBORTaggedEncodable::tagged_cbor()`.
    ///
    /// - All tags in the vector are considered equivalent for decoding. When
    ///   `CBORTaggedDecodable::from_tagged_cbor()` is called, any tag in this
    ///   vector will be accepted as valid for this type.
    ///
    /// This design enables backward compatibility: you can introduce a new tag
    /// (placed first in the vector) while still supporting older tags for
    /// decoding.
    ///
    /// For standard CBOR tags, you can use predefined tag constants from the
    /// `tags` module, or create custom tags with `Tag::with_value()`.
    fn cbor_tags() -> Vec<Tag>;
}
