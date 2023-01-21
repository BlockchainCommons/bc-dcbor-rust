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
        let mut buf = self.tag.varint_encode(6);
        buf.extend(self.item.cbor_encode());
        buf
    }
}

impl IntoCBOR for Tagged {
    fn cbor(&self) -> CBOR {
        CBOR::TAGGED(Box::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{tag::Tagged, test_util::test_cbor};

    #[test]
    fn encode() {
        test_cbor(Tagged::new(1, "Hello"), "TAGGED(Tagged { tag: 1, item: STRING(\"Hello\") })", "c16548656c6c6f");
    }
}
