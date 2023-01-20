use crate::{cbor::{CBOREncode, CBOR, IntoCBOR}, varint::VarIntEncode};

#[derive(Debug, Clone)]
pub struct Tagged {
    tag: u64,
    item: CBOR,
}

impl Tagged {
    pub fn new<T>(tag: u64, item: T) -> Tagged where T: IntoCBOR {
        Tagged { tag, item: item.cbor() }
    }
}

impl CBOREncode for Tagged {
    fn cbor_encode(&self) -> Vec<u8> {
        let mut buf = self.tag.varint_encode(7);
        buf.extend(self.item.cbor_encode());
        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::{cbor::CBOREncode, hex::bytes_to_hex, tag::Tagged};

    #[test]
    fn encode_tag() {
        let tagged = Tagged::new(1, "Hello");
        let bytes = tagged.cbor_encode();
        println!("{:?}", bytes_to_hex(bytes));
    }
}
