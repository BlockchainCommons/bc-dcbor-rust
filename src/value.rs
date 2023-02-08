use crate::cbor_encodable::CBOREncodable;

use super::{cbor::CBOR, varint::{EncodeVarInt, MajorType}};


/// A CBOR simple value.
#[derive(Clone)]
pub struct Value(pub u64);

impl Value {
    /// Creates a new CBOR "simple" value.
    pub fn new<T>(v: T) -> Value where T: IntoValue {
        v.into_value()
    }

    /// Returns the known name of the value, if it has been assigned one.
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }

    /// Returns the raw value.
    pub fn raw_value(&self) -> u64 {
        self.0
    }
}

impl CBOREncodable for Value {
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
            false => CBOR::Simple(Value::new(20)),
            true => CBOR::Simple(Value::new(21)),
        }
    }

    fn cbor_data(&self) -> Vec<u8> {
        match self {
            false => Value::new(20).cbor_data(),
            true => Value::new(21).cbor_data()
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::fmt::Debug for Value {
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

impl std::fmt::Display for Value {
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

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for u64 {
    fn into_value(self) -> Value {
        Value(self)
    }
}

impl IntoValue for i32 {
    fn into_value(self) -> Value {
        Value(self as u64)
    }
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}
