import_stdlib!();

use core::str::Utf8Error;

use crate::tag::Tag;

/// A comprehensive set of errors that can occur during CBOR encoding and decoding operations,
/// with special focus on enforcing the deterministic encoding rules specified in the dCBOR specification.
///
/// The dCBOR implementation validates all encoded CBOR against the deterministic encoding requirements
/// of RFC 8949 §4.2.1, plus additional constraints defined in the dCBOR application profile.
/// These errors represent all the possible validation failures and decoding issues that can arise.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// // Handling the WrongType error when trying to convert a different type
/// let cbor_string = CBOR::from("hello");
/// let result = cbor_string.try_into_byte_string();
///
/// assert!(result.is_err());
/// if let Err(e) = result {
///     assert!(e.to_string().contains("not the expected type"));
/// }
/// ```
#[derive(Debug, ThisError)]
pub enum Error {
    /// The CBOR data ended prematurely during decoding, before a complete CBOR item could be decoded.
    /// This typically happens when a CBOR item's structure indicates more data than is actually present.
    #[error("early end of CBOR data")]
    Underrun,

    /// An unsupported or invalid value was encountered in a CBOR header byte.
    /// The parameter contains the unsupported header byte value.
    /// This can occur when decoding CBOR that uses unsupported features or is malformed.
    #[error("unsupported value in CBOR header")]
    UnsupportedHeaderValue(u8),

    /// A CBOR numeric value was encoded in a non-canonical form, violating the deterministic
    /// encoding requirement of dCBOR (per Section 2.3 of the dCBOR specification).
    ///
    /// This error is triggered when:
    /// - An integer is not encoded in its shortest possible form
    /// - A floating point value that could be represented as an integer was not reduced
    /// - A NaN value was not encoded in its canonical form (`f97e00`)
    #[error("a CBOR numeric value was encoded in non-canonical form")]
    NonCanonicalNumeric,

    /// An invalid CBOR simple value was encountered during decoding.
    ///
    /// Per Section 2.4 of the dCBOR specification, only `false`, `true`, `null`,
    /// and floating point values are valid simple values in dCBOR. All other
    /// major type 7 values are invalid.
    #[error("an invalid CBOR simple value was encountered")]
    InvalidSimpleValue,

    /// A CBOR text string was not valid UTF-8. The parameter contains the specific UTF-8 error.
    ///
    /// All CBOR text strings (major type 3) must be valid UTF-8 per RFC 8949.
    #[error("an invalidly-encoded UTF-8 string was encountered in the CBOR ({0:?})")]
    InvalidString(str::Utf8Error),

    /// A CBOR text string was not encoded in Unicode Canonical Normalization Form C (NFC).
    ///
    /// Per Section 2.5 of the dCBOR specification, all text strings must be in NFC form,
    /// and decoders must reject any encoded text strings that are not in NFC.
    #[error("a CBOR string was not encoded in Unicode Canonical Normalization Form C")]
    NonCanonicalString,

    /// The decoded CBOR item didn't consume all input data.
    /// The parameter contains the number of unused bytes.
    ///
    /// This error occurs when decoding functions expect exactly one CBOR item
    /// but the input contains additional data after a valid CBOR item.
    #[error("the decoded CBOR had {0} extra bytes at the end")]
    UnusedData(usize),

    /// The keys in a decoded CBOR map were not in canonical lexicographic order of their encoding.
    ///
    /// Per the CDE specification and Section 2.1 of dCBOR, map keys must be in ascending
    /// lexicographic order of their encoded representation for deterministic encoding.
    #[error("the decoded CBOR map has keys that are not in canonical order")]
    MisorderedMapKey,

    /// A decoded CBOR map contains duplicate keys, which is invalid.
    ///
    /// Per Section 2.2 of the dCBOR specification, CBOR maps must not contain
    /// duplicate keys, and decoders must reject encoded maps with duplicate keys.
    #[error("the decoded CBOR map has a duplicate key")]
    DuplicateMapKey,

    /// A requested key was not found in a CBOR map during data extraction.
    #[error("missing CBOR map key")]
    MissingMapKey,

    /// A CBOR numeric value could not be represented in the specified target numeric type.
    ///
    /// This occurs when attempting to convert a CBOR number to a Rust numeric type
    /// that is too small to represent the value without loss of precision.
    #[error("the CBOR numeric value could not be represented in the specified numeric type")]
    OutOfRange,

    /// The CBOR value is not of the expected type for a conversion or operation.
    ///
    /// This occurs when attempting to convert a CBOR value to a type that doesn't
    /// match the actual CBOR item's type (e.g., trying to convert a string to an integer).
    #[error("the decoded CBOR value was not the expected type")]
    WrongType,

    /// The CBOR tagged value had a different tag than expected.
    /// The first parameter is the expected tag, and the second is the actual tag found.
    #[error("expected CBOR tag {0}, but got {1}")]
    WrongTag(Tag, Tag),

    /// Invalid UTF‑8 in a text string.
    #[error("invalid UTF‑8 string: {0}")]
    InvalidUtf8(#[from] Utf8Error),

    /// Invalid ISO 8601 date format.
    #[error("invalid ISO 8601 date string: {0}")]
    InvalidDate(String),

    /// Custom error message.
    #[error("{0}")]
    Custom(String),
}

impl From<&str> for Error {
    /// Converts a string slice into a `CBORError::Custom` variant.
    fn from(error: &str) -> Self {
        Error::Custom(error.to_string())
    }
}

impl From<String> for Error {
    /// Converts a `String` into a `CBORError::Custom` variant.
    fn from(error: String) -> Self {
        Error::Custom(error)
    }
}

pub type Result<T> = StdResult<T, Error>;

#[cfg(feature = "anyhow")]
impl From<anyhow::Error> for Error {
    /// Converts an `anyhow::Error` into a `CBORError::Custom` variant.
    fn from(error: anyhow::Error) -> Self {
        Error::Custom(error.to_string())
    }
}
