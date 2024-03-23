import_stdlib!();

use anyhow::bail;

use crate::{CBOR, CBOREncodable, CBORDecodable, CBORError, CBORCodable, CBORCase};

use super::varint::{EncodeVarInt, MajorType};

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
        CBORCase::Simple(self.clone()).into()
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

impl From<Simple> for CBOR {
    fn from(value: Simple) -> Self {
        value.cbor()
    }
}

impl CBORDecodable for Simple {
    fn from_cbor(cbor: &CBOR) -> anyhow::Result<Self> {
        match cbor.case() {
            CBORCase::Simple(simple) => Ok(simple.clone()),
            _ => bail!(CBORError::WrongType),
        }
    }
}

impl TryFrom<CBOR> for Simple {
    type Error = anyhow::Error;

    fn try_from(value: CBOR) -> Result<Self, Self::Error> {
        Self::from_cbor(&value)
    }
}

impl CBORCodable for Simple { }

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

impl fmt::Debug for Simple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::False => "false".to_owned(),
            Self::True => "true".to_owned(),
            Self::Null => "null".to_owned(),
            Self::Float(v) => format!("{:?}", v),
        };
        f.write_str(&s)
    }
}

impl fmt::Display for Simple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::False => "false".to_owned(),
            Self::True => "true".to_owned(),
            Self::Null => "null".to_owned(),
            Self::Float(v) => format!("{:?}", v),
        };
        f.write_str(&s)
    }
}
