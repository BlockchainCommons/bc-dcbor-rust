use crate::cbor_encodable::CBOREncodable;

use super::{cbor::CBOR, varint::{EncodeVarInt, MajorType}};

/// A CBOR simple value.
#[derive(Clone)]
pub enum Simple {
    /// A numeric value.
    Value(u64),
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
    /// Creates a new CBOR simple value.
    pub fn new<T>(v: T) -> Simple where T: Into<Simple> {
        v.into()
    }

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
            Self::Value(v) => v.encode_varint(MajorType::Simple),
            Self::False => Simple::new(20).cbor_data(),
            Self::True => Simple::new(21).cbor_data(),
            Self::Null => Simple::new(22).cbor_data(),
            Self::Float(v) => v.cbor_data(),
        }
    }
}

impl PartialEq for Simple {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(v1), Self::Value(v2)) => v1 == v2,
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
            Self::Value(v) => format!("{:?}", v),
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
            Self::Value(v) => format!("simple({})", v),
            Self::False => "false".to_owned(),
            Self::True => "true".to_owned(),
            Self::Null => "null".to_owned(),
            Self::Float(v) => format!("{}", v),
        };
        f.write_str(&s)
    }
}

impl From<u64> for Simple {
    fn from(value: u64) -> Self {
        Simple::Value(value)
    }
}

impl From<i32> for Simple {
    fn from(value: i32) -> Self {
        Simple::Value(value as u64)
    }
}
