use bytes::Bytes;

use crate::{tag::Tag, Simple, error::CBORError, decode::decode_cbor, CBOREncodable};

use super::{Map, string_util::flanked};

#[cfg(feature = "multithreaded")]
use std::sync::Arc as RefCounted;

#[cfg(not(feature = "multithreaded"))]
use std::rc::Rc as RefCounted;

/// A symbolic representation of CBOR data.
#[derive(Clone)]
pub struct CBOR(RefCounted<CBORCase>);

impl CBOR {
    pub fn case(&self) -> &CBORCase {
        &self.0
    }
}

impl From<CBORCase> for CBOR {
    fn from(case: CBORCase) -> Self {
        Self(RefCounted::new(case))
    }
}

#[derive(Clone)]
pub enum CBORCase {
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
        CBORCase::ByteString(Bytes::copy_from_slice(data.as_ref())).into()
    }

    /// Create a new CBOR value representing a byte string given as a hexadecimal string.
    pub fn byte_string_hex(hex: &str) -> CBOR {
        Self::byte_string(hex::decode(hex).unwrap())
    }

    /// Extract the CBOR value as a byte string.
    ///
    /// Returns `Some` if the value is a byte string, `None` otherwise.
    pub fn as_byte_string(&self) -> Option<Bytes> {
        match self.case() {
            CBORCase::ByteString(b) => Some(b.clone()),
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
        match self.case() {
            CBORCase::Text(t) => Some(t),
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
        match self.case() {
            CBORCase::Array(a) => Some(a),
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
        match self.case() {
            CBORCase::Map(m) => Some(m),
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
        CBORCase::Tagged(tag.into(), Box::new(item.cbor())).into()
    }

    /// Extract the CBOR value as a tagged value.
    ///
    /// Returns `Some` if the value is a tagged value, `None` otherwise.
    pub fn as_tagged_value(&self) -> Option<(&Tag, &CBOR)> {
        match self.case() {
            CBORCase::Tagged(t, v) => Some((t, v)),
            _ => None
        }
    }

    /// Extract the CBOR value as a tagged value.
    ///
    /// Returns `Ok` if the value is a tagged value with the expected tag, `Err`
    /// otherwise.
    pub fn expect_tagged_value(&self, expected_tag: impl Into<Tag>) -> Result<&CBOR, CBORError> {
        match self.as_tagged_value() {
            Some((tag, value)) => {
                let expected_tag = expected_tag.into();
                if tag == &expected_tag {
                    Ok(value)
                } else {
                    Err(CBORError::WrongTag(expected_tag, tag.to_owned()))
                }
            },
            _ => Err(CBORError::WrongType)
        }
    }

    /// Extract the CBOR value as a simple value.
    ///
    /// Returns `Some` if the value is a simple value, `None` otherwise.
    pub fn as_simple_value(&self) -> Option<&Simple> {
        match self.case() {
            CBORCase::Simple(s) => Some(s),
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
    pub fn r#false() -> Self {
        CBORCase::Simple(Simple::False).into()
    }

    /// The CBOR simple value representing `true`.
    pub fn r#true() -> Self {
        CBORCase::Simple(Simple::True).into()
    }

    /// The CBOR simple value representing `null` (`None`).
    pub fn null() -> Self {
        CBORCase::Simple(Simple::Null).into()
    }
}

impl PartialEq for CBOR {
    fn eq(&self, other: &Self) -> bool {
        match (self.case(), other.case()) {
            (CBORCase::Unsigned(l0), CBORCase::Unsigned(r0)) => l0 == r0,
            (CBORCase::Negative(l0), CBORCase::Negative(r0)) => l0 == r0,
            (CBORCase::ByteString(l0), CBORCase::ByteString(r0)) => l0 == r0,
            (CBORCase::Text(l0), CBORCase::Text(r0)) => l0 == r0,
            (CBORCase::Array(l0), CBORCase::Array(r0)) => l0 == r0,
            (CBORCase::Map(l0), CBORCase::Map(r0)) => l0 == r0,
            (CBORCase::Tagged(l0, l1), CBORCase::Tagged(r0, r1)) => l0 == r0 && l1 == r1,
            (CBORCase::Simple(l0), CBORCase::Simple(r0)) => l0 == r0,
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
        match self.case() {
            CBORCase::Unsigned(x) => f.debug_tuple("unsigned").field(x).finish(),
            CBORCase::Negative(x) => f.debug_tuple("negative").field(x).finish(),
            CBORCase::ByteString(x) => f.write_fmt(format_args!("bytes({})", hex::encode(x))),
            CBORCase::Text(x) => f.debug_tuple("text").field(x).finish(),
            CBORCase::Array(x) => f.debug_tuple("array").field(x).finish(),
            CBORCase::Map(x) => f.debug_tuple("map").field(x).finish(),
            CBORCase::Tagged(tag, item) => f.write_fmt(format_args!("tagged({}, {:?})", tag, item)),
            CBORCase::Simple(x) => f.write_fmt(format_args!("simple({})", x.name())),
        }
    }
}

impl std::fmt::Display for CBOR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.case() {
            CBORCase::Unsigned(x) => format!("{}", x),
            CBORCase::Negative(x) => format!("{}", x),
            CBORCase::ByteString(x) => format!("h'{}'", hex::encode(x)),
            CBORCase::Text(x) => format_string(x),
            CBORCase::Array(x) => format_array(x),
            CBORCase::Map(x) => format_map(x),
            CBORCase::Tagged(tag, item) => format!("{}({})", tag, item),
            CBORCase::Simple(x) => format!("{}", x),
        };
        f.write_str(&s)
    }
}
