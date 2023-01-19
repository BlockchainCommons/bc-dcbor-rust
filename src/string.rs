use crate::{cbor::CBOREncode, varint::VarIntEncode};

impl CBOREncode for str {
    fn cbor_encode(&self) -> Vec<u8> {
        let mut data: Vec<u8> = self.bytes().collect();
        let mut buf = data.len().varint_encode(3);
        buf.append(&mut data);
        buf
    }
}
