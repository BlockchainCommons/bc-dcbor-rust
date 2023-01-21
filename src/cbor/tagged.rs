use super::{cbor::{CBOREncode, IntoCBOR, CBOR}, varint::{VarIntEncode, MajorType}};

#[derive(Debug, Clone)]
pub struct Tagged {
    tag: u64,
    item: CBOR,
}

impl Tagged {
    pub fn new<T>(tag: u64, item: T) -> Tagged where T: IntoCBOR {
        Tagged { tag, item: item.cbor() }
    }

    pub fn name(&self) -> String {
        format!("{}", self.tag)
    }
}

impl CBOREncode for Tagged {
    fn cbor_encode(&self) -> Vec<u8> {
        let mut buf = self.tag.varint_encode(MajorType::TAGGED);
        buf.extend(self.item.cbor_encode());
        buf
    }
}

impl IntoCBOR for Tagged {
    fn cbor(&self) -> CBOR {
        CBOR::TAGGED(Box::new(self.clone()))
    }
}

impl std::fmt::Display for Tagged {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}({})", self.name(), self.item))
    }
}

#[cfg(test)]
mod tests {
    use crate::cbor::{test_util::test_cbor, cbor::IntoCBOR};

    use super::Tagged;

    #[test]
    fn encode() {
        test_cbor(Tagged::new(1, "Hello"), r#"TAGGED(Tagged { tag: 1, item: STRING("Hello") })"#, "c16548656c6c6f");
    }

    #[test]
    fn format() {
        assert_eq!(format!("{}", Tagged::new(32, "Hello").cbor()), r#"32("Hello")"#);
    }
}
