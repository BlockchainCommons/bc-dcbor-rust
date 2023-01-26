use crate::util::{string_util::flanked, hex::{bytes_to_hex, hex_to_bytes}};

use super::{map::CBORMap, varint::{EncodeVarInt, MajorType}};

/// A symbolic representation of CBOR data.
#[derive(Clone)]
pub enum CBOR {
    /// Unsigned integer (major type 0).
    UInt(u64),
    /// Negative integer (major type 1).
    NInt(i64),
    /// Byte string (major type 2).
    Bytes(Bytes),
    /// UTF-8 string (major type 3).
    String(String),
    /// Array (major type 4).
    Array(Vec<CBOR>),
    /// Map (major type 5).
    Map(CBORMap),
    /// Tagged value (major type 6).
    Tagged(Box<Tagged>),
    /// Simple value (majory type 7).
    Value(Value)
}

pub trait CBOREncodable {
    /// Returns the value in CBOR symbolic representation.
    fn cbor(&self) -> CBOR;
    /// Returns the value in CBOR binary representation.
    fn encode_cbor(&self) -> Vec<u8>;
}

impl CBOREncodable for CBOR {
    fn cbor(&self) -> CBOR {
        self.clone()
    }

    fn encode_cbor(&self) -> Vec<u8> {
        match self {
            CBOR::UInt(x) => x.encode_cbor(),
            CBOR::NInt(x) => x.encode_cbor(),
            CBOR::Bytes(x) => x.encode_cbor(),
            CBOR::String(x) => x.encode_cbor(),
            CBOR::Array(x) => x.encode_cbor(),
            CBOR::Map(x) => x.encode_cbor(),
            CBOR::Tagged(x) => x.encode_cbor(),
            CBOR::Value(x) => x.encode_cbor(),
        }
    }
}

impl PartialEq for CBOR {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UInt(l0), Self::UInt(r0)) => l0 == r0,
            (Self::NInt(l0), Self::NInt(r0)) => l0 == r0,
            (Self::Bytes(l0), Self::Bytes(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            (Self::Map(l0), Self::Map(r0)) => l0 == r0,
            (Self::Tagged(l0), Self::Tagged(r0)) => l0 == r0,
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
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

fn format_array(a: &Vec<CBOR>) -> String {
    let s: Vec<String> = a.iter().map(|x| format!("{}", x)).collect();
    flanked(&s.join(", "), "[", "]")
}

fn format_map(m: &CBORMap) -> String {
    let s: Vec<String> = m.iter().map(|x| format!("{}: {}", x.0, x.1)).collect();
    flanked(&s.join(", "), "{", "}")
}

impl std::fmt::Debug for CBOR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UInt(x) => f.debug_tuple("UInt").field(x).finish(),
            Self::NInt(x) => f.debug_tuple("NInt").field(x).finish(),
            Self::Bytes(x) => f.debug_tuple("Bytes").field(x).finish(),
            Self::String(x) => f.debug_tuple("String").field(x).finish(),
            Self::Array(x) => f.debug_tuple("Array").field(x).finish(),
            Self::Map(x) => f.debug_tuple("Map").field(x).finish(),
            Self::Tagged(x) => f.write_fmt(format_args!("Tagged({}, {:?})", x.tag, x.item)),
            Self::Value(x) => f.debug_tuple("Value").field(x).finish(),
        }
    }
}

impl std::fmt::Display for CBOR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CBOR::UInt(x) => format!("{}", x),
            CBOR::NInt(x) => format!("{}", x),
            CBOR::Bytes(x) => format!("{}", x),
            CBOR::String(x) => format_string(x),
            CBOR::Array(x) => format_array(x),
            CBOR::Map(x) => format_map(x),
            CBOR::Tagged(x) => format!("{}", x),
            CBOR::Value(x) => format!("{}", x),
        };
        f.write_str(&s)
    }
}

#[derive(Clone)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn new<T>(data: T) -> Bytes where T: AsRef<[u8]> {
        Bytes(data.as_ref().to_owned())
    }

    pub fn from_hex<T>(hex: T) -> Bytes where T: AsRef<str> {
        Bytes(hex_to_bytes(hex))
    }
}

impl CBOREncodable for Bytes {
    fn cbor(&self) -> CBOR {
        CBOR::Bytes(self.to_owned())
    }

    fn encode_cbor(&self) -> Vec<u8> {
        let a = &self.0;
        let mut buf = a.len().encode_varint(MajorType::Bytes);
        for b in a {
            buf.push(*b);
        }
        buf
    }
}

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&bytes_to_hex(&self.0))
    }
}

impl std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("h'")?;
        f.write_str(&bytes_to_hex(&self.0))?;
        f.write_str("'")
    }
}

#[derive(Clone)]
pub struct Value(u64);

impl Value {
    pub fn new(v: u64) -> Value {
        Value(v)
    }
}

impl CBOREncodable for Value {
    fn cbor(&self) -> CBOR {
        CBOR::Value(self.clone())
    }

    fn encode_cbor(&self) -> Vec<u8> {
        self.0.encode_varint(MajorType::Value)
    }
}

impl CBOREncodable for bool {
    fn cbor(&self) -> CBOR {
        match self {
            false => CBOR::Value(Value::new(20)),
            true => CBOR::Value(Value::new(21)),
        }
    }

    fn encode_cbor(&self) -> Vec<u8> {
        match self {
            false => Value::new(20).encode_cbor(),
            true => Value::new(21).encode_cbor()
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
            _ => format!("simple({:?})", self.0),
        };
        f.write_str(&s)
    }
}

#[derive(Debug, Clone)]
pub struct Tagged {
    pub tag: u64,
    pub item: CBOR,
}

impl Tagged {
    pub fn new<T>(tag: u64, item: T) -> Tagged where T: CBOREncodable {
        Tagged { tag, item: item.cbor() }
    }

    pub fn name(&self) -> String {
        format!("{}", self.tag)
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

#[cfg(test)]
mod tests {
    use crate::cbor::test_util::test_cbor;

    use super::{Bytes, Tagged};

    #[test]
    fn encode_bytes() {
        // test_cbor(Bytes::new([0x11, 0x22, 0x33]), "Bytes(112233)", "43112233");
        test_cbor(
            Bytes::from_hex("c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7"),
            "Bytes(c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7)",
            "h'c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7'",
            "5820c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7"
        );
    }

    #[cfg(test)]
    mod tests {
        use crate::cbor::{test_util::test_cbor, cbor::Value};

        #[test]
        fn encode_value() {
            test_cbor(false, "Value(false)", "false", "f4");
            test_cbor(true, "Value(true)", "true", "f5");
            test_cbor(Value::new(100), "Value(100)", "simple(100)", "f864");
        }
    }

    #[test]
    fn encode_tagged() {
        test_cbor(Tagged::new(1, "Hello"), r#"Tagged(1, String("Hello"))"#, r#"1("Hello")"#, "c16548656c6c6f");
    }
    }
