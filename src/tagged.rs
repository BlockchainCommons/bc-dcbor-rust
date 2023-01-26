use super::{cbor::{CBOR, CBOREncodable}, varint::{MajorType, EncodeVarInt}};


/// A CBOR tagged value.
#[derive(Debug, Clone)]
pub struct Tagged {
    tag: u64,
    item: CBOR,
}

impl Tagged {
    /// Creates a new tagged value.
    pub fn new<T>(tag: u64, item: T) -> Tagged where T: CBOREncodable {
        Tagged { tag, item: item.cbor() }
    }

    /// Returns the known name of the tag, if it has been assigned one.
    pub fn name(&self) -> String {
        format!("{}", self.tag())
    }

    /// Returns the tag.
    pub fn tag(&self) -> u64 {
        self.tag
    }

    /// Returns the CBOR item that was tagged.
    pub fn item(&self) -> &CBOR {
        &self.item
    }
}

impl CBOREncodable for Tagged {
    fn cbor(&self) -> CBOR {
        CBOR::Tagged(Box::new(self.clone()))
    }

    fn encode_cbor(&self) -> Vec<u8> {
        let mut buf = self.tag.encode_varint(MajorType::Tagged);
        buf.extend(self.item.encode_cbor());
        buf
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
