use bytes::Bytes;

use crate::{tag::Tag, Simple, error::CBORError, decode::decode_cbor, CBOREncodable};

use super::{Map, string_util::flanked};

/// A symbolic representation of CBOR data.
#[derive(Clone)]
pub enum CBOR {
    /// Unsigned integer (major type 0).
    Unsigned(u64),
    /// Negative integer (major type 1).
    Negative(i64),
    /// Byte string (major type 2).
    ByteString(Bytes),
    /// UTF-8 string (major type 3).
    Text(String),
    /// Array (major type 4).
    Array(Vec<CBOR>),
    /// Map (major type 5).
    Map(Map),
    /// Tagged value (major type 6).
    Tagged(Tag, Box<CBOR>),
    /// Simple value (major type 7).
    Simple(Simple)
}

/// Affordances for decoding CBOR from binary representation.
impl CBOR {
    /// Decodes the given date into CBOR symbolic representation.
    pub fn from_data(data: &[u8]) -> Result<CBOR, CBORError> {
        decode_cbor(data)
    }

    /// Decodes the given data into CBOR symbolic representation given as a hexadecimal string.
    ///
    /// Panics if the string is not well-formed hexadecimal with no spaces or
    /// other characters.
    pub fn from_hex(hex: &str) -> Result<CBOR, CBORError> {
        let data = hex::decode(hex).unwrap();
        Self::from_data(&data)
    }
}

impl CBOR {
    /// Create a new CBOR value representing a byte string.
    pub fn byte_string<T>(data: T) -> CBOR where T: AsRef<[u8]> {
        CBOR::ByteString(Bytes::copy_from_slice(data.as_ref()))
    }

    /// Create a new CBOR value representing a byte string given as a hexadecimal string.
    pub fn byte_string_hex(hex: &str) -> CBOR {
        Self::byte_string(hex::decode(hex).unwrap())
    }

    /// Extract the CBOR value as a byte string.
    ///
    /// Returns `Some` if the value is a byte string, `None` otherwise.
    pub fn as_byte_string(&self) -> Option<Bytes> {
        match self {
            Self::ByteString(b) => Some(b.clone()),
            _ => None
        }
    }

    /// Extract the CBOR value as a byte string.
    ///
    /// Returns `Ok` if the value is a byte string, `Err` otherwise.
    pub fn expect_byte_string(&self) -> Result<Bytes, CBORError> {
        self.as_byte_string().ok_or(CBORError::WrongType)
    }

    /// Extract the CBOR value as a text string.
    ///
    /// Returns `Some` if the value is a text string, `None` otherwise.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(t) => Some(t),
            _ => None
        }
    }

    /// Extract the CBOR value as a text string.
    ///
    /// Returns `Ok` if the value is a text string, `Err` otherwise.
    pub fn expect_text(&self) -> Result<&str, CBORError> {
        self.as_text().ok_or(CBORError::WrongType)
    }

    /// Extract the CBOR value as an array.
    ///
    /// Returns `Some` if the value is an array, `None` otherwise.
    pub fn as_array(&self) -> Option<&Vec<CBOR>> {
        match self {
            Self::Array(a) => Some(a),
            _ => None
        }
    }

    /// Extract the CBOR value as an array.
    ///
    /// Returns `Ok` if the value is an array, `Err` otherwise.
    pub fn expect_array(&self) -> Result<&Vec<CBOR>, CBORError> {
        self.as_array().ok_or(CBORError::WrongType)
    }

    /// Extract the CBOR value as a map.
    ///
    /// Returns `Some` if the value is a map, `None` otherwise.
    pub fn as_map(&self) -> Option<&Map> {
        match self {
            Self::Map(m) => Some(m),
            _ => None
        }
    }

    /// Extract the CBOR value as a map.
    ///
    /// Returns `Ok` if the value is a map, `Err` otherwise.
    pub fn expect_map(&self) -> Result<&Map, CBORError> {
        self.as_map().ok_or(CBORError::WrongType)
    }

    /// Create a new CBOR value representing a tagged value.
    pub fn tagged_value(tag: impl Into<Tag>, item: impl CBOREncodable) -> CBOR {
        CBOR::Tagged(tag.into(), Box::new(item.cbor()))
    }

    /// Extract the CBOR value as a tagged value.
    ///
    /// Returns `Some` if the value is a tagged value, `None` otherwise.
    pub fn as_tagged_value(&self) -> Option<(&Tag, &CBOR)> {
        match self {
            Self::Tagged(t, v) => Some((t, v)),
            _ => None
        }
    }

    /// Extract the CBOR value as a tagged value.
    ///
    /// Returns `Ok` if the value is a tagged value with the expected tag, `Err`
    /// otherwise.
    pub fn expect_tagged_value(&self, expected_tag: impl Into<Tag>) -> Result<&CBOR, CBORError> {
        match self.as_tagged_value() {
            Some((tag, value)) if tag == &expected_tag.into() => Ok(value),
            _ => Err(CBORError::WrongType)
        }
    }

    /// Extract the CBOR value as a simple value.
    ///
    /// Returns `Some` if the value is a simple value, `None` otherwise.
    pub fn as_simple_value(&self) -> Option<&Simple> {
        match self {
            Self::Simple(s) => Some(s),
            _ => None
        }
    }

    /// Extract the CBOR value as a simple value.
    ///
    /// Returns `Ok` if the value is a simple value, `Err` otherwise.
    pub fn expect_simple_value(&self) -> Result<&Simple, CBORError> {
        self.as_simple_value().ok_or(CBORError::WrongType)
    }
}

/// Associated constants for common CBOR simple values.
impl CBOR {
    /// The CBOR simple value representing `false`.
    pub const FALSE: CBOR = CBOR::Simple(Simple::False);
    /// The CBOR simple value representing `true`.
    pub const TRUE: CBOR = CBOR::Simple(Simple::True);
    /// The CBOR simple value representing `null` (`None`).
    pub const NULL: CBOR = CBOR::Simple(Simple::Null);
}

impl PartialEq for CBOR {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unsigned(l0), Self::Unsigned(r0)) => l0 == r0,
            (Self::Negative(l0), Self::Negative(r0)) => l0 == r0,
            (Self::ByteString(l0), Self::ByteString(r0)) => l0 == r0,
            (Self::Text(l0), Self::Text(r0)) => l0 == r0,
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            (Self::Map(l0), Self::Map(r0)) => l0 == r0,
            (Self::Tagged(l0, l1), Self::Tagged(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Simple(l0), Self::Simple(r0)) => l0 == r0,
            _ => false,
        }
    }
}

fn format_string(s: &str) -> String {
    let mut result = "".to_string();
    for c in s.chars() {
        if c == '"' {
            result.push_str(r#"\""#);
        } else {
            result.push(c);
        }
    }
    flanked(&result, r#"""#, r#"""#)
}

fn format_array(a: &[CBOR]) -> String {
    let s: Vec<String> = a.iter().map(|x| format!("{}", x)).collect();
    flanked(&s.join(", "), "[", "]")
}

fn format_map(m: &Map) -> String {
    let s: Vec<String> = m.iter().map(|x| format!("{}: {}", x.0, x.1)).collect();
    flanked(&s.join(", "), "{", "}")
}

impl std::fmt::Debug for CBOR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsigned(x) => f.debug_tuple("unsigned").field(x).finish(),
            Self::Negative(x) => f.debug_tuple("negative").field(x).finish(),
            Self::ByteString(x) => f.write_fmt(format_args!("bytes({})", hex::encode(x))),
            Self::Text(x) => f.debug_tuple("text").field(x).finish(),
            Self::Array(x) => f.debug_tuple("array").field(x).finish(),
            Self::Map(x) => f.debug_tuple("map").field(x).finish(),
            Self::Tagged(tag, item) => f.write_fmt(format_args!("tagged({}, {:?})", tag, item)),
            Self::Simple(x) => f.write_fmt(format_args!("simple({})", x.name())),
        }
    }
}

impl std::fmt::Display for CBOR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CBOR::Unsigned(x) => format!("{}", x),
            CBOR::Negative(x) => format!("{}", x),
            CBOR::ByteString(x) => format!("h'{}'", hex::encode(x)),
            CBOR::Text(x) => format_string(x),
            CBOR::Array(x) => format_array(x),
            CBOR::Map(x) => format_map(x),
            CBOR::Tagged(tag, item) => format!("{}({})", tag, item),
            CBOR::Simple(x) => format!("{}", x),
        };
        f.write_str(&s)
    }
}
