use crate::{bytes::Bytes, tag::Tagged, map::CBORMap, value::Value};

#[derive(Debug, Clone)]
pub enum CBOR {
    UINT(u64),
    NINT(i64),
    BYTES(Bytes),
    STRING(String),
    ARRAY(Vec<CBOR>),
    MAP(CBORMap),
    TAGGED(Box<Tagged>),
    VALUE(Value)
}

pub trait IntoCBOR {
    fn cbor(&self) -> CBOR;
}

pub trait CBOREncode {
    fn cbor_encode(&self) -> Vec<u8>;
}

impl IntoCBOR for CBOR {
    fn cbor(&self) -> CBOR {
        self.clone()
    }
}

impl CBOR {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            CBOR::UINT(x) => x.cbor_encode(),
            CBOR::NINT(x) => x.cbor_encode(),
            CBOR::BYTES(x) => x.cbor_encode(),
            CBOR::STRING(x) => x.cbor_encode(),
            CBOR::ARRAY(x) => x.cbor_encode(),
            CBOR::MAP(x) => x.cbor_encode(),
            CBOR::TAGGED(x) => x.cbor_encode(),
            CBOR::VALUE(x) => x.cbor_encode(),
        }
    }
}

impl CBOREncode for CBOR {
    fn cbor_encode(&self) -> Vec<u8> {
        self.encode()
    }
}
