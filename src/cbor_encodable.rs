use crate::{CBOR, Tagged};

/// A type that can be encoded as CBOR.
pub trait CBOREncodable {
    /// Returns the value in CBOR symbolic representation.
    fn cbor(&self) -> CBOR;
    /// Returns the value in CBOR binary representation.
    fn cbor_data(&self) -> Vec<u8>;
}

impl CBOREncodable for CBOR {
    fn cbor(&self) -> CBOR {
        self.clone()
    }

    fn cbor_data(&self) -> Vec<u8> {
        match self {
            CBOR::UInt(x) => x.cbor_data(),
            CBOR::NInt(x) => {
                assert!(x < &0);
                x.cbor_data()
            },
            CBOR::Bytes(x) => x.cbor_data(),
            CBOR::String(x) => x.cbor_data(),
            CBOR::Array(x) => x.cbor_data(),
            CBOR::Map(x) => x.cbor_data(),
            CBOR::Tagged(tag, item) => {
                let x = Tagged::new(tag.clone(), *item.clone());
                x.cbor_data()
            },
            CBOR::Value(x) => x.cbor_data(),
        }
    }
}
