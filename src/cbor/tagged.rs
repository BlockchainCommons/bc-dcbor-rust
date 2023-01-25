use super::{cbor::{EncodeCBOR, AsCBOR, CBOR}, varint::{EncodeVarInt, MajorType}};

#[derive(Debug, Clone)]
pub struct Tagged {
    pub tag: u64,
    pub item: CBOR,
}

impl Tagged {
    pub fn new<T>(tag: u64, item: T) -> Tagged where T: AsCBOR {
        Tagged { tag, item: item.as_cbor() }
    }

    pub fn name(&self) -> String {
        format!("{}", self.tag)
    }
}

impl EncodeCBOR for Tagged {
    fn encode_cbor(&self) -> Vec<u8> {
        let mut buf = self.tag.encode_varint(MajorType::Tagged);
        buf.extend(self.item.encode_cbor());
        buf
    }
}

impl AsCBOR for Tagged {
    fn as_cbor(&self) -> CBOR {
        CBOR::Tagged(Box::new(self.clone()))
    }
}

impl PartialEq for Tagged {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag && self.item == other.item
    }
}

impl std::fmt::Display for Tagged {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}({})", self.name(), self.item))
    }
}

#[cfg(test)]
mod tests {
    use crate::cbor::test_util::test_cbor;

    use super::Tagged;

    #[test]
    fn encode() {
        test_cbor(Tagged::new(1, "Hello"), r#"Tagged(1, String("Hello"))"#, r#"1("Hello")"#, "c16548656c6c6f");
    }
}
