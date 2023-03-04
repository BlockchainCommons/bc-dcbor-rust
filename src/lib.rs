#![doc(html_root_url = "https://docs.rs/dcbor/0.3.1")]
#![warn(rust_2018_idioms)]

//! # dCBOR: Deterministic CBOR Codec
//!
//! `dcbor` is a [CBOR](https://cbor.io) codec that focuses on writing and
//! parsing "deterministic" CBOR per [§4.2 of
//! RFC-8949](https://www.rfc-editor.org/rfc/rfc8949.html#name-deterministically-encoded-c).
//! It does not support parts of the spec forbidden by deterministic CBOR (such
//! as indefinite length arrays and maps). It is strict in both what it
//! writes and reads: in particular it will throw decoding errors if
//! variable-length integers are not encoded in their minimal form, or CBOR map
//! keys are not in lexicographic order, or there is extra data past the end of
//! the decoded CBOR item.
//!
//! # Getting Started
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! dcbor = "0.3.1"
//! ```
//!
//! # Usage
//!
//! Encode an array of strings as CBOR.
//!
//! ```
//! # fn main() {
//! # {
//! use dcbor::*;
//! let array = [1000, 2000, 3000];
//! let cbor = array.cbor();
//! assert_eq!(cbor.hex(), "831903e81907d0190bb8");
//! # }
//! # }
//! ```
//!
//! Decode CBOR binary back to an array of strings.
//!
//! ```
//! # fn main() {
//! # {
//! use dcbor::*;
//! let data = hex::hex_to_data("831903e81907d0190bb8");
//! let cbor: CBOR = data.try_into().unwrap();
//! assert_eq!(cbor.diagnostic(), "[1000, 2000, 3000]");
//! let array: Vec::<u32> = cbor.try_into().unwrap();
//! assert_eq!(format!("{:?}", array), "[1000, 2000, 3000]");
//! # }
//! # }
//! ```
//!
//! See the unit tests For further examples, including encoding and decoding
//! arrays with heterogenous elements, maps, and user-defined types with custom
//! CBOR tags.

mod cbor;
pub use cbor::*;

mod bool_value;
pub use bool_value::*;

mod float;

mod array;

mod decode_error;
pub use decode_error::DecodeError;

mod bytes;
pub use bytes::Bytes;

mod date;
pub use date::Date;

mod diag;
mod dump;

mod known_tags;
pub use known_tags::{KnownTags, KnownTagsDict};

mod tag;
pub use tag::Tag;

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

pub mod hex;

mod int;

mod map;
pub use map::{Map, MapIter};

mod string;

mod string_util;

mod tagged;
pub use tagged::Tagged;

mod simple;
pub use simple::Simple;

mod varint;
