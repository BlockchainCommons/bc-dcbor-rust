use crate::{ CBOR, Error, Result, CBORTagged, CBORCase };

/// # Tagged CBOR Decoding Support
///
/// This module provides the `CBORTaggedDecodable` trait, which enables types to
/// be decoded from tagged CBOR values.
///
/// Tagged CBOR values include semantic information about how to interpret the data.
/// This trait allows Rust types to verify that incoming CBOR data has the expected
/// tag(s) and to decode the data appropriately.
/// A trait for types that can be decoded from CBOR with a specific tag.
///
/// This trait extends `CBORTagged` and `TryFrom<CBOR>` to provide methods for
/// decoding tagged CBOR data into Rust types. It handles verification that
/// the CBOR data has the expected tag(s) and provides utilities for both
/// tagged and untagged decoding.
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
/// // Implement decoding from tagged CBOR
/// impl CBORTaggedDecodable for Date {
///     fn from_untagged_cbor(cbor: CBOR) -> dcbor::Result<Self> {
///         // Convert the untagged CBOR to a float
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
/// // Create tagged CBOR data
/// let tagged_cbor = CBOR::to_tagged_value(1, 1609459200.0); // Tag 1 with timestamp
///
/// // Decode using the trait
/// let date = Date::from_tagged_cbor(tagged_cbor.clone()).unwrap();
/// assert_eq!(date.0, 1609459200.0);
///
/// // Or use TryFrom
/// let date2: Date = tagged_cbor.try_into().unwrap();
/// assert_eq!(date, date2);
///
/// // This would fail - wrong tag
/// let wrong_tag = CBOR::to_tagged_value(2, 1609459200.0);
/// assert!(Date::from_tagged_cbor(wrong_tag).is_err());
///
/// // Example of backward compatibility with multiple tags
/// #[derive(Debug, Clone, PartialEq)]
/// struct VersionedData {
///     version: u8,
///     value: String,
/// }
///
/// impl CBORTagged for VersionedData {
///     fn cbor_tags() -> Vec<Tag> {
///         vec![
///             Tag::with_value(201),  // Current version (preferred for encoding)
///             Tag::with_value(200),  // Legacy version (still accepted for decoding)
///         ]
///     }
/// }
///
/// impl CBORTaggedDecodable for VersionedData {
///     fn from_untagged_cbor(cbor: CBOR) -> dcbor::Result<Self> {
///         if let CBORCase::Map(map) = cbor.into_case() {
///             let version: u8 = map.extract("version")?;
///             let value: String = map.extract("value")?;
///             Ok(VersionedData { version, value })
///         } else {
///             return Err(dcbor::Error::WrongType);
///         }
///     }
/// }
///
/// impl TryFrom<CBOR> for VersionedData {
///     type Error = dcbor::Error;
///
///     fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
///         Self::from_tagged_cbor(cbor)
///     }
/// }
///
/// // Legacy data with tag 200 can still be decoded
/// let mut map = Map::new();
/// map.insert("version", 1);
/// map.insert("value", "legacy");
/// let legacy_cbor = CBOR::to_tagged_value(200, map);
///
/// let data = VersionedData::from_tagged_cbor(legacy_cbor).unwrap();
/// assert_eq!(data.version, 1);
/// assert_eq!(data.value, "legacy");
/// ```
pub trait CBORTaggedDecodable: TryFrom<CBOR> + CBORTagged {
    /// Creates an instance of this type by decoding it from untagged CBOR.
    ///
    /// This method defines how to interpret the CBOR content (without considering
    /// the tag) and convert it to the implementing type.
    fn from_untagged_cbor(cbor: CBOR) -> Result<Self> where Self: Sized;

    /// Creates an instance of this type by decoding it from tagged CBOR.
    ///
    /// This method first verifies that the CBOR value has one of the expected tags
    /// (as defined by `cbor_tags()`), then delegates to `from_untagged_cbor()` to
    /// decode the content.
    ///
    /// For backward compatibility, this method accepts any tag from the `cbor_tags()`
    /// vector, not just the first one. This allows new versions of types to still
    /// accept data tagged with older/alternative tag values.
    ///
    /// In most cases, you don't need to override this method.
    fn from_tagged_cbor(cbor: CBOR) -> Result<Self> where Self: Sized {
        match cbor.into_case() {
            CBORCase::Tagged(tag, item) => {
                let cbor_tags = Self::cbor_tags();
                if cbor_tags.iter().any(|t| *t == tag) {
                    Self::from_untagged_cbor(item)
                } else {
                    return Err(Error::WrongTag(cbor_tags[0].clone(), tag));
                }
            }
            _ => {
                return Err(Error::WrongType);
            }
        }
    }

    /// Creates an instance of this type by decoding it from binary encoded tagged CBOR.
    ///
    /// This is a convenience method that first parses the binary data into a CBOR value,
    /// then uses `from_tagged_cbor()` to decode it.
    fn from_tagged_cbor_data(data: impl AsRef<[u8]>) -> Result<Self> where Self: Sized {
        Self::from_tagged_cbor(CBOR::try_from_data(data)?)
    }

    /// Creates an instance of this type by decoding it from binary encoded untagged CBOR.
    ///
    /// This is a convenience method that first parses the binary data into a CBOR value,
    /// then uses `from_untagged_cbor()` to decode it.
    fn from_untagged_cbor_data(data: impl AsRef<[u8]>) -> Result<Self> where Self: Sized {
        Self::from_untagged_cbor(CBOR::try_from_data(data)?)
    }
}
