use crate::{CBOREncodable, CBORDecodable};

/// A type that can be encoded to or decoded from CBOR.
pub trait CBORCodable: CBOREncodable + CBORDecodable { }
