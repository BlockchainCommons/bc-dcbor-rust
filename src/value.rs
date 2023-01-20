use crate::cbor::CBOREncode;

impl CBOREncode for bool {
    fn cbor_encode(&self) -> Vec<u8> {
        let x = match self {
            false => 0xf4,
            true => 0xf5
        };
        vec![x]
    }
}
