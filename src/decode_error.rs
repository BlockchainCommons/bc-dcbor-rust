use std::str::Utf8Error;

use crate::tag::Tag;


/// An error encountered while decoding CBOR.
#[derive(Debug)]
pub enum DecodeError {
    /// Early end of data.
    Underrun,

    /// Unsupported value in CBOR header.
    UnsupportedHeaderValue(u8),

    /// An integer was encoded in non-canonical form.
    NonCanonicalInt,

    /// A floating point value was encoded in non-canonical form.
    NonCanonicalFloat,

    /// An invalidly-encoded UTF-8 string was encountered.
    InvalidString(Utf8Error),

    /// The decoded CBOR had extra data at the end.
    UnusedData(usize),

    /// The decoded CBOR map has keys that are not in canonical order.
    MisorderedMapKey,

    /// The decoded CBOR map has a duplicate key.
    DuplicateMapKey,

    /// The decoded integer could not be represented in the specified integer type.
    IntegerOutOfRange,

    /// The decoded value was not the expected type.
    WrongType,

    /// The decoded value did not have the expected tag.
    ///
    /// The case includes the expected tag and encountered tag as associated data.
    WrongTag(Tag, Tag),

    /// Invalid CBOR format. Frequently thrown by libraries depending on this one.
    InvalidFormat,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DecodeError::Underrun => format!("early end of data"),
            DecodeError::UnsupportedHeaderValue(v) => format!("unsupported value in header ({})", v),
            DecodeError::NonCanonicalInt => format!("non-canonical int format"),
            DecodeError::NonCanonicalFloat => format!("non-canonical float format"),
            DecodeError::InvalidString(err) => format!("invalid string format: {:?}", err),
            DecodeError::UnusedData(len) => format!("unused data past end: {:?} bytes", len),
            DecodeError::MisorderedMapKey => format!("mis-ordered map key"),
            DecodeError::DuplicateMapKey => format!("duplicate map key"),
            DecodeError::IntegerOutOfRange => format!("integer out of range"),
            DecodeError::WrongType => format!("wrong type"),
            DecodeError::WrongTag(expected, encountered) => format!("wrong tag, expected: {:?}, encountered: {:?}", expected, encountered),
            DecodeError::InvalidFormat => format!("invalid CBOR format"),
        };
        f.write_str(&s)
    }
}

impl std::error::Error for DecodeError {
}

impl PartialEq for DecodeError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UnsupportedHeaderValue(l0), Self::UnsupportedHeaderValue(r0)) => l0 == r0,
            (Self::InvalidString(l0), Self::InvalidString(r0)) => l0 == r0,
            (Self::UnusedData(l0), Self::UnusedData(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
