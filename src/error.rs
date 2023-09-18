use std::str::Utf8Error;
use crate::tag::Tag;
use thiserror::Error;

/// An error encountered while decoding or parsing CBOR.
#[derive(Debug, Error)]
pub enum CBORError {
    #[error("early end of CBOR data")]
    Underrun,

    #[error("ensupported value in CBOR header")]
    UnsupportedHeaderValue(u8),

    #[error("a CBOR numeric value was encoded in non-canonical form")]
    NonCanonicalNumeric,

    #[error("an invalid CBOR simple value was encountered")]
    InvalidSimpleValue,

    #[error("an invalidly-encoded UTF-8 string was encountered in the CBOR ({0:?})")]
    InvalidString(Utf8Error),

    #[error("the decoded CBOR had {0} extra bytes at the end")]
    UnusedData(usize),

    #[error("the decoded CBOR map has keys that are not in canonical order")]
    MisorderedMapKey,

    #[error("the decoded CBOR map has a duplicate key")]
    DuplicateMapKey,

    #[error("missing CBOR map key")]
    MissingMapKey,

    #[error("the CBOR numeric value could not be represented in the specified numeric type")]
    OutOfRange,

    #[error("the decoded CBOR value was not the expected type")]
    WrongType,

    #[error("expected CBOR tag {0}, but got {1}")]
    WrongTag(Tag, Tag),
}

impl From<Utf8Error> for CBORError {
    fn from(err: Utf8Error) -> Self {
        CBORError::InvalidString(err)
    }
}
