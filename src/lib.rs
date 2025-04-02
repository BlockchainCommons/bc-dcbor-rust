#![doc(html_root_url = "https://docs.rs/dcbor/0.16.5")]
#![warn(rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]

//! # dCBOR: Deterministic CBOR Codec
//!
//! `dcbor` is a reference implementation of Deterministic CBOR. The current
//! specification of the norms and practices guiding the creation of this
//! implementation are currently found in this IETF Internet Draft:
//! [draft-mcnally-deterministic-cbor](https://datatracker.ietf.org/doc/draft-mcnally-deterministic-cbor/).
//!
//! # Getting Started
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! dcbor = "0.16.5"
//! ```
//!
//! # Features
//!
//! ## Multi-threaded
//!
//! The `multithreaded` feature is available but not enabled by default. It uses
//! `Arc` for reference counting instead of `Rc`. To enable it, add the
//! following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.dcbor]
//! version = "0.16.5"
//! features = ["multithreaded"]
//! ```
//!
//! ## `no_std`
//!
//! The `dcbor` library is `no_std` compatible. To use it in a `no_std`
//! environment, disable the default features in your `Cargo.toml` and enable
//! the `no_std` feature:
//!
//! ```toml
//! [dependencies.dcbor]
//! version = "0.16.5"
//! default-features = false
//! features = ["no_std"]
//! ```
//!
//! # Usage
//!
//! Encode an array of integers as CBOR.
//!
//! ```
//! # fn main() {
//! # {
//! use dcbor::prelude::*;
//! let array = [1000, 2000, 3000];
//! let cbor: CBOR = array.into();
//! assert_eq!(cbor.hex(), "831903e81907d0190bb8");
//! # }
//! # }
//! ```
//!
//! Decode CBOR binary back to an array of integers.
//!
//! ```
//! # fn main() {
//! # {
//! use dcbor::prelude::*;
//! let data = hex_literal::hex!("831903e81907d0190bb8");
//! let cbor = CBOR::try_from_data(&data).unwrap();
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

#[macro_use]
mod stdlib;

mod cbor;
pub use cbor::*;

mod byte_string;
pub use byte_string::ByteString;

mod bool_value;

mod float;

mod array;

mod error;
pub use error::CBORError;

mod date;
pub use date::Date;

mod diag;
mod dump;

mod tags_store;
pub use tags_store::{CBORSummarizer, TagsStore, TagsStoreTrait};

mod tag;
pub use tag::{Tag, TagValue};

mod tags;
pub use tags::*;

mod cbor_codable;
pub use cbor_codable::{CBORCodable, CBORDecodable, CBOREncodable};

mod cbor_tagged;
pub use cbor_tagged::CBORTagged;

mod cbor_tagged_encodable;
pub use cbor_tagged_encodable::CBORTaggedEncodable;
mod cbor_tagged_decodable;
pub use cbor_tagged_decodable::CBORTaggedDecodable;
mod cbor_tagged_codable;
pub use cbor_tagged_codable::CBORTaggedCodable;

mod decode;

mod int;

mod map;
pub use map::{Map, MapIter};

mod string;

mod string_util;

mod simple;
pub use simple::Simple;

mod exact;
mod varint;
use exact::ExactFrom;

pub mod prelude;

// Re-export standard library types used in our public API
#[doc(hidden)]
pub use stdlib::public_exports::*;
