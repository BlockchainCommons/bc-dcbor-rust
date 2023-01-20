
pub trait CBOREncode {
    fn cbor_encode(&self) -> Vec<u8>;
}

pub trait CBORAppend {
    fn cbor_append(&self, buf: &mut Vec<u8>);
}

impl<T> CBORAppend for T where T: CBOREncode {
    fn cbor_append(&self, buf: &mut Vec<u8>) {
        buf.extend(self.cbor_encode());
    }
}
