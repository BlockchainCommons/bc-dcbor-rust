use crate::cbor::{CBOREncode, IntoCBOR, CBOR};

impl CBOREncode for bool {
    fn cbor_encode(&self) -> Vec<u8> {
        let x = match self {
            false => 0xf4,
            true => 0xf5
        };
        vec![x]
    }
}

impl IntoCBOR for bool {
    fn cbor(&self) -> CBOR {
        match self {
            false => CBOR::VALUE(20),
            true => CBOR::VALUE(21),
        }
    }
}
