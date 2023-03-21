use std::str::Utf8Error;

use crate::tag::Tag;


/// An error encountered while decoding or parsing CBOR.
#[derive(Debug)]
pub enum CBORError {
    /// Early end of data.
    Underrun,

    /// Unsupported value in CBOR header.
    UnsupportedHeaderValue(u8),

    /// A numeric value was encoded in non-canonical form.
    NonCanonicalNumeric,

    /// An invalidly-encoded UTF-8 string was encountered.
    InvalidString(Utf8Error),

    /// The decoded CBOR had extra data at the end.
    UnusedData(usize),

    /// The decoded CBOR map has keys that are not in canonical order.
    MisorderedMapKey,

    /// The decoded CBOR map has a duplicate key.
    DuplicateMapKey,

    /// The numeric value could not be represented in the specified numeric type.
    OutOfRange,

    /// The decoded value was not the expected type.
    WrongType,

    /// The decoded value did not have the expected tag.
    ///
    /// The case includes the expected tag and encountered tag as associated data.
    WrongTag(Tag, Tag),

    /// Invalid CBOR format. Frequently returned by libraries depending on this one.
    InvalidFormat,
}

impl std::fmt::Display for CBORError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CBORError::Underrun => format!("early end of data"),
            CBORError::UnsupportedHeaderValue(v) => format!("unsupported value in header ({})", v),
            CBORError::NonCanonicalNumeric => format!("non-canonical numeric value"),
            CBORError::InvalidString(err) => format!("invalid string format: {:?}", err),
            CBORError::UnusedData(len) => format!("unused data past end: {:?} bytes", len),
            CBORError::MisorderedMapKey => format!("mis-ordered map key"),
            CBORError::DuplicateMapKey => format!("duplicate map key"),
            CBORError::OutOfRange => format!("integer out of range"),
            CBORError::WrongType => format!("wrong type"),
            CBORError::WrongTag(expected, encountered) => format!("wrong tag, expected: {:?}, encountered: {:?}", expected, encountered),
            CBORError::InvalidFormat => format!("invalid CBOR format"),
        };
        f.write_str(&s)
    }
}

impl std::error::Error for CBORError {
}

impl PartialEq for CBORError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UnsupportedHeaderValue(l0), Self::UnsupportedHeaderValue(r0)) => l0 == r0,
            (Self::InvalidString(l0), Self::InvalidString(r0)) => l0 == r0,
            (Self::UnusedData(l0), Self::UnusedData(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
