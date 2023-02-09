use crate::cbor_encodable::CBOREncodable;

use super::{cbor::CBOR, varint::{EncodeVarInt, MajorType}};


/// A CBOR simple value.
#[derive(Clone)]
pub struct Simple(u64);

impl Simple {
    /// Creates a new CBOR simple value.
    pub fn new<T>(v: T) -> Simple where T: Into<Simple> {
        v.into()
    }

    /// Returns the known name of the value, if it has been assigned one.
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }

    /// Returns the wrapped value.
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Creates a new CBOR simple value from the provided integer.
    ///
    /// Can be used to initialize const expressions.
    pub const fn new_const(v: u64) -> Simple {
        Simple(v)
    }
}

impl CBOREncodable for Simple {
    fn cbor(&self) -> CBOR {
        CBOR::Simple(self.clone())
    }

    fn cbor_data(&self) -> Vec<u8> {
        self.0.encode_varint(MajorType::Simple)
    }
}

impl CBOREncodable for bool {
    fn cbor(&self) -> CBOR {
        match self {
            false => CBOR::Simple(Simple::new(20)),
            true => CBOR::Simple(Simple::new(21)),
        }
    }

    fn cbor_data(&self) -> Vec<u8> {
        match self {
            false => Simple::new(20).cbor_data(),
            true => Simple::new(21).cbor_data()
        }
    }
}

impl PartialEq for Simple {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::fmt::Debug for Simple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.0 {
            20 => "false".to_owned(),
            21 => "true".to_owned(),
            22 => "null".to_owned(),
            _ => format!("{:?}", self.0),
        };
        f.write_str(&s)
    }
}

impl std::fmt::Display for Simple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.0 {
            20 => "false".to_owned(),
            21 => "true".to_owned(),
            22 => "null".to_owned(),
            _ => format!("simple({:?})", self.0),
        };
        f.write_str(&s)
    }
}

impl From<u64> for Simple {
    fn from(value: u64) -> Self {
        Simple(value)
    }
}

impl From<i32> for Simple {
    fn from(value: i32) -> Self {
        Simple(value as u64)
    }
}
