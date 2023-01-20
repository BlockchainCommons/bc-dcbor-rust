use std::collections::HashMap;

use crate::bytes::Bytes;

#[derive(Debug)]
pub enum CBOR {
    UINT(u64),
    NINT(i128),
    BYTES(Vec<u8>),
    STRING(String),
    ARRAY(Vec<CBOR>),
    MAP(HashMap<Vec<u8>, (CBOR, CBOR)>),
    TAG(Box<(u64, CBOR)>),
    VALUE(u64)
}

pub trait IntoCBOR {
    fn cbor(&self) -> CBOR;
}
