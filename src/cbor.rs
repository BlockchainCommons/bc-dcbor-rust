import_stdlib!();

use bytes::Bytes;
use anyhow::{bail, Result};

use crate::{decode::decode_cbor, error::CBORError, tag::Tag, varint::{EncodeVarInt, MajorType}, Map, Simple};

use super::string_util::flanked;

#[cfg(feature = "multithreaded")]
use sync::Arc as RefCounted;

#[cfg(not(feature = "multithreaded"))]
use rc::Rc as RefCounted;

/// A symbolic representation of CBOR data.
#[derive(Clone)]
pub struct CBOR(RefCounted<CBORCase>);

impl CBOR {
    pub fn as_case(&self) -> &CBORCase {
        &self.0
    }

    pub fn into_case(self) -> CBORCase {
        match RefCounted::try_unwrap(self.0) {
            Ok(b) => b,
            Err(ref_counted) => (*ref_counted).clone(),
        }
    }
}

impl From<CBORCase> for CBOR {
    fn from(case: CBORCase) -> Self {
        Self(RefCounted::new(case))
    }
}

#[derive(Debug, Clone)]
pub enum CBORCase {
    /// Unsigned integer (major type 0).
    Unsigned(u64),
    /// Negative integer (major type 1).
    ///
    /// Actual value is -1 - n
    Negative(u64),
    /// Byte string (major type 2).
    ByteString(Bytes),
    /// UTF-8 string (major type 3).
    Text(String),
    /// Array (major type 4).
    Array(Vec<CBOR>),
    /// Map (major type 5).
    Map(Map),
    /// Tagged value (major type 6).
    Tagged(Tag, CBOR),
    /// Simple value (major type 7).
    Simple(Simple)
}

/// Affordances for decoding CBOR from binary representation.
impl CBOR {
    /// Decodes the given date into CBOR symbolic representation.
    pub fn try_from_data(data: impl AsRef<[u8]>) -> Result<CBOR> {
        decode_cbor(data)
    }

    /// Decodes the given data into CBOR symbolic representation given as a hexadecimal string.
    ///
    /// Panics if the string is not well-formed hexadecimal with no spaces or
    /// other characters.
    pub fn try_from_hex(hex: &str) -> Result<CBOR> {
        let data = hex::decode(hex).unwrap();
        Self::try_from_data(data)
    }

    pub fn to_cbor_data(&self) -> Vec<u8> {
        match self.as_case() {
            CBORCase::Unsigned(x) => x.encode_varint(MajorType::Unsigned),
            CBORCase::Negative(x) => x.encode_varint(MajorType::Negative),
            CBORCase::ByteString(x) => {
                let mut buf = x.len().encode_varint(MajorType::Bytes);
                buf.extend(x);
                buf
            },
            CBORCase::Text(x) => {
                let mut buf = x.len().encode_varint(MajorType::Text);
                for byte in x.bytes() {
                    buf.push(byte);
                }
                buf
            },
            CBORCase::Array(x) => {
                let mut buf = x.len().encode_varint(MajorType::Array);
                for item in x {
                    buf.extend(item.to_cbor_data());
                }
                buf
            },
            CBORCase::Map(x) => x.cbor_data(),
            CBORCase::Tagged(tag, item) => {
                let mut buf = tag.value().encode_varint(MajorType::Tagged);
                buf.extend(item.to_cbor_data());
                buf
            },
            CBORCase::Simple(x) => x.cbor_data(),
        }
    }
}

impl CBOR {
    /// Create a new CBOR value representing a byte string.
    pub fn to_byte_string(data: impl AsRef<[u8]>) -> CBOR {
        CBORCase::ByteString(Bytes::copy_from_slice(data.as_ref())).into()
    }

    /// Create a new CBOR value representing a byte string given as a hexadecimal string.
    ///
    /// Panics if the string is not well-formed hexadecimal.
    pub fn to_byte_string_from_hex(hex: impl AsRef<str>) -> CBOR {
        Self::to_byte_string(hex::decode(hex.as_ref()).unwrap())
    }

    /// Create a new CBOR value representing a tagged value.
    pub fn to_tagged_value(tag: impl Into<Tag>, item: impl Into<CBOR>) -> CBOR {
        CBORCase::Tagged(tag.into(), item.into()).into()
    }
}

impl CBOR {
    /// Extract the CBOR value as a byte string.
    ///
    /// Returns `Ok` if the value is a byte string, `Err` otherwise.
    pub fn try_into_byte_string(self) -> Result<Bytes> {
        match self.into_case() {
            CBORCase::ByteString(b) => Ok(b),
            _ => bail!(CBORError::WrongType)
        }
    }

    pub fn into_byte_string(self) -> Option<Bytes> {
        self.try_into_byte_string().ok()
    }

    /// Extract the CBOR value as a text string.
    ///
    /// Returns `Ok` if the value is a text string, `Err` otherwise.
    pub fn try_into_text(self) -> Result<String> {
        match self.into_case() {
            CBORCase::Text(t) => Ok(t),
            _ => bail!(CBORError::WrongType)
        }
    }

    /// Extract the CBOR value as an array.
    ///
    /// Returns `Ok` if the value is an array, `Err` otherwise.
    pub fn try_into_array(self) -> Result<Vec<CBOR>> {
        match self.into_case() {
            CBORCase::Array(a) => Ok(a),
            _ => bail!(CBORError::WrongType)
        }
    }

    /// Extract the CBOR value as a map.
    ///
    /// Returns `Ok` if the value is a map, `Err` otherwise.
    pub fn try_into_map(self) -> Result<Map> {
        match self.into_case() {
            CBORCase::Map(m) => Ok(m),
            _ => bail!(CBORError::WrongType)
        }
    }

    /// Extract the CBOR value as a tagged value.
    ///
    /// Returns `Ok` if the value is a tagged value, `Err` otherwise.
    pub fn try_into_tagged_value(self) -> Result<(Tag, CBOR)> {
        match self.into_case() {
            CBORCase::Tagged(tag, value) => Ok((tag, value)),
            _ => bail!(CBORError::WrongType)
        }
    }

    /// Extract the CBOR value as an expected tagged value.
    ///
    /// Returns `Ok` if the value is a tagged value with the expected tag, `Err`
    /// otherwise.
    pub fn try_into_expected_tagged_value(self, expected_tag: impl Into<Tag>) -> Result<CBOR> {
        let (tag, value) = self.try_into_tagged_value()?;
        let expected_tag = expected_tag.into();
        if tag == expected_tag {
            Ok(value)
        } else {
            bail!(CBORError::WrongTag(expected_tag, tag))
        }
    }

    /// Extract the CBOR value as a simple value.
    ///
    /// Returns `Ok` if the value is a simple value, `Err` otherwise.
    pub fn try_into_simple_value(self) -> Result<Simple> {
        match self.into_case() {
            CBORCase::Simple(s) => Ok(s),
            _ => bail!(CBORError::WrongType)
        }
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
        match (self.as_case(), other.as_case()) {
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

impl fmt::Debug for CBOR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_case() {
            CBORCase::Unsigned(x) => f.debug_tuple("unsigned").field(x).finish(),
            CBORCase::Negative(x) => f.debug_tuple("negative").field(&(-1 - (*x as i128))).finish(),
            CBORCase::ByteString(x) => f.write_fmt(format_args!("bytes({})", hex::encode(x))),
            CBORCase::Text(x) => f.debug_tuple("text").field(x).finish(),
            CBORCase::Array(x) => f.debug_tuple("array").field(x).finish(),
            CBORCase::Map(x) => f.debug_tuple("map").field(x).finish(),
            CBORCase::Tagged(tag, item) => f.write_fmt(format_args!("tagged({}, {:?})", tag, item)),
            CBORCase::Simple(x) => f.write_fmt(format_args!("simple({})", x.name())),
        }
    }
}

impl fmt::Display for CBOR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self.as_case() {
            CBORCase::Unsigned(x) => format!("{}", x),
            CBORCase::Negative(x) => format!("{}", -1 - (*x as i128)),
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
