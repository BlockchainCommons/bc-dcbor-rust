use crate::{CBORTaggedDecodable, CBORTaggedEncodable};

/// # Tagged CBOR Encoding and Decoding Support
///
/// This module provides the `CBORTaggedCodable` trait, which serves as a
/// convenience marker for types that can be both encoded to and decoded from
/// tagged CBOR values.
///
/// The trait is automatically implemented for any type that implements both
/// `CBORTaggedEncodable` and `CBORTaggedDecodable`.
/// A trait for types that can be both encoded to and decoded from CBOR with a
/// specific tag.
///
/// This trait is automatically implemented for any type that implements both
/// `CBORTaggedEncodable` and `CBORTaggedDecodable`. It serves as a convenience
/// marker to indicate full tagged CBOR serialization support.
///
/// ## Example
///
/// ```
/// use dcbor::prelude::*;
///
/// // Define a Date type
/// #[derive(Debug, Clone, PartialEq)]
/// struct Date(f64); // Timestamp as seconds since epoch
///
/// // Implement CBORTagged
/// impl CBORTagged for Date {
///     fn cbor_tags() -> Vec<Tag> {
///         vec![Tag::with_value(1)] // Standard date tag
///     }
/// }
///
/// // Implement encoding to tagged CBOR
/// impl CBORTaggedEncodable for Date {
///     fn untagged_cbor(&self) -> CBOR { self.0.into() }
/// }
///
/// // Implement decoding from tagged CBOR
/// impl CBORTaggedDecodable for Date {
///     fn from_untagged_cbor(cbor: CBOR) -> dcbor::Result<Self> {
///         let timestamp: f64 = cbor.try_into()?;
///         Ok(Date(timestamp))
///     }
/// }
///
/// // Implement TryFrom<CBOR> (required by CBORTaggedDecodable)
/// impl TryFrom<CBOR> for Date {
///     type Error = dcbor::Error;
///
///     fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
///         Self::from_tagged_cbor(cbor)
///     }
/// }
///
/// // The CBORTaggedCodable trait is automatically implemented
/// // Create a date and demonstrate round-trip conversion
/// let original = Date(1609459200.0);
/// let cbor = original.tagged_cbor();
/// let roundtrip: Date = cbor.try_into().unwrap();
/// assert_eq!(original, roundtrip);
/// ```
pub trait CBORTaggedCodable {}

impl<T> CBORTaggedCodable for T where
    T: CBORTaggedEncodable + CBORTaggedDecodable
{
}
