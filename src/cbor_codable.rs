use crate::{CBOREncodable, CBORDecodable, CBOR};

/// A type that can be encoded to or decoded from CBOR.
pub trait CBORCodable: CBOREncodable + CBORDecodable { }

impl CBORCodable for CBOR { }
