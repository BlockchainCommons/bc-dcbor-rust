use crate::{CBORDecodable, CBOR};

/// A type that can be encoded to or decoded from CBOR.
pub trait CBORCodable { }

impl<T> CBORCodable for T where T: CBORDecodable + Into<CBOR> { }
