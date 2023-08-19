use crate::cbor_encodable::CBOREncodable;

use super::{cbor::CBOR, varint::{EncodeVarInt, MajorType}};

/// A CBOR simple value.
#[derive(Clone)]
pub enum Simple {
    /// The boolean value `false`.
    False,
    /// The boolean value `true`.
    True,
    /// The value representing `null` (`None`).
    Null,
    /// A floating point value.
    Float(f64),
}

impl Simple {
    /// Returns the known name of the value, if it has been assigned one.
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }
}

impl CBOREncodable for Simple {
    fn cbor(&self) -> CBOR {
        CBOR::Simple(self.clone())
    }

    fn cbor_data(&self) -> Vec<u8> {
        match self {
            Self::False => 20u8.encode_varint(MajorType::Simple),
            Self::True => 21u8.encode_varint(MajorType::Simple),
            Self::Null => 22u8.encode_varint(MajorType::Simple),
            Self::Float(v) => v.cbor_data(),
        }
    }
}

impl PartialEq for Simple {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::False, Self::False) => true,
            (Self::True, Self::True) => true,
            (Self::Null, Self::Null) => true,
            (Self::Float(v1), Self::Float(v2)) => v1 == v2,
            _ => false,
        }
    }
}

impl std::fmt::Debug for Simple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::False => "false".to_owned(),
            Self::True => "true".to_owned(),
            Self::Null => "null".to_owned(),
            Self::Float(v) => format!("{:?}", v),
        };
        f.write_str(&s)
    }
}

impl std::fmt::Display for Simple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::False => "false".to_owned(),
            Self::True => "true".to_owned(),
            Self::Null => "null".to_owned(),
            Self::Float(v) => format!("{}", v),
        };
        f.write_str(&s)
    }
}
