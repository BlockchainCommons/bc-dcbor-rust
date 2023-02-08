//! `dcbor` is a [CBOR](https://cbor.io) codec that focuses on writing and
//! parsing "deterministic" CBOR per [§4.2 of
//! RFC-8949](https://www.rfc-editor.org/rfc/rfc8949.html#name-deterministically-encoded-c).
//! It does not support parts of the spec forbidden by deterministic CBOR (such
//! as indefinite length arrays and maps). It also does not currently support
//! encoding or decoding floating point values. It is strict in both what it
//! writes and reads: in particular it will throw decoding errors if
//! variable-length integers are not encoded in their minimal form, or CBOR map
//! keys are not in lexicographic order, or there is extra data past the end of
//! the decoded CBOR item.

mod array;

pub mod decode_error;

mod bytes;
pub use bytes::Data;

mod date;
pub use date::Date;

pub mod diag;
pub mod dump;

mod known_tags;
pub use known_tags::{name_for_tag, KnownTagsDict};

pub mod tag;
pub use tag::{Tag, IntoTag};

mod cbor;
pub use cbor::*;

mod cbor_encodable;
pub use cbor_encodable::CBOREncodable;
mod cbor_decodable;
pub use cbor_decodable::CBORDecodable;
mod cbor_codable;
pub use cbor_codable::CBORCodable;

mod cbor_tagged_encodable;
pub use cbor_tagged_encodable::CBORTaggedEncodable;
mod cbor_tagged_decodable;
pub use cbor_tagged_decodable::CBORTaggedDecodable;
mod cbor_tagged_codable;
pub use cbor_tagged_codable::CBORTaggedCodable;

mod decode;

mod hex;
pub use hex::{hex_to_bytes, bytes_to_hex};

mod int;

mod map;
pub use map::{Map, Iter};

mod string;

mod string_util;

mod tagged;
pub use tagged::Tagged;

mod simple;
pub use simple::Simple;

mod varint;

#[cfg(test)]
mod test_encode;

#[cfg(test)]
mod test_format;
