use crate::{cbor_encodable::CBOREncodable, tag::Tag};

use super::{cbor::CBOR, varint::{MajorType, EncodeVarInt}};


/// A CBOR tagged value.
#[derive(Debug, Clone)]
pub struct Tagged {
    tag: Tag,
    item: CBOR,
}

impl Tagged {
    /// Creates a new tagged value.
    pub fn new<T, I>(value: T, item: I) -> Tagged where T: Into<Tag>, I: CBOREncodable {
        Tagged { tag: value.into(), item: item.cbor() }
    }

    /// Returns the known name of the tag, if it has been assigned one.
    pub fn name(&self) -> String {
        format!("{}", self.tag())
    }

    /// Returns the tag.
    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    /// Returns the CBOR item that was tagged.
    pub fn item(&self) -> &CBOR {
        &self.item
    }
}

impl CBOREncodable for Tagged {
    fn cbor(&self) -> CBOR {
        let tag = self.tag().clone();
        let item = Box::new(self.item().clone());
        CBOR::Tagged(tag, item)
    }

    fn cbor_data(&self) -> Vec<u8> {
        let mut buf = self.tag.value().encode_varint(MajorType::Tagged);
        buf.extend(self.item.cbor_data());
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
