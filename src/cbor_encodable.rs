use crate::CBOR;

/// A type that can be encoded as CBOR.
pub trait CBOREncodable {
    /// Returns the value in CBOR symbolic representation.
    fn cbor(&self) -> CBOR;
    /// Returns the value in CBOR binary representation.
    fn encode_cbor(&self) -> Vec<u8>;
}

impl CBOREncodable for CBOR {
    fn cbor(&self) -> CBOR {
        self.clone()
    }

    fn encode_cbor(&self) -> Vec<u8> {
        match self {
            CBOR::UInt(x) => x.encode_cbor(),
            CBOR::NInt(x) => {
                assert!(x < &0);
                x.encode_cbor()
            },
            CBOR::Bytes(x) => x.encode_cbor(),
            CBOR::String(x) => x.encode_cbor(),
            CBOR::Array(x) => x.encode_cbor(),
            CBOR::Map(x) => x.encode_cbor(),
            CBOR::Tagged(x) => x.encode_cbor(),
            CBOR::Value(x) => x.encode_cbor(),
        }
    }
}
