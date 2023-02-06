use crate::{CBORTaggedEncodable, CBORTaggedDecodable};

/// A type that can be encoded to or from CBOR with a specific tag.
pub trait CBORTaggedCodable: CBORTaggedEncodable + CBORTaggedDecodable { }
