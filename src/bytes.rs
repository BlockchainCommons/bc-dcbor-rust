use bytes::Bytes;

use crate::CBOR;

impl From<Bytes> for CBOR {
    fn from(value: Bytes) -> Self {
        CBOR::ByteString(value)
    }
}

impl From<&Bytes> for CBOR {
    fn from(value: &Bytes) -> Self {
        CBOR::ByteString(value.clone())
    }
}

impl From<CBOR> for Bytes {
    fn from(value: CBOR) -> Self {
        match value {
            CBOR::ByteString(b) => b,
            _ => panic!("Cannot convert {:?} to Bytes", value)
        }
    }
}
