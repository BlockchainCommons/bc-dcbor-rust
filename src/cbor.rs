use crate::tag::Tag;

use super::{bytes::Data, Value, Tagged, Map, string_util::flanked};

/// A symbolic representation of CBOR data.
#[derive(Clone)]
pub enum CBOR {
    /// Unsigned integer (major type 0).
    UInt(u64),
    /// Negative integer (major type 1).
    NInt(i64),
    /// Byte string (major type 2).
    Bytes(Data),
    /// UTF-8 string (major type 3).
    String(String),
    /// Array (major type 4).
    Array(Vec<CBOR>),
    /// Map (major type 5).
    Map(Map),
    /// Tagged value (major type 6).
    Tagged(Tag, Box<CBOR>),
    /// Simple value (majory type 7).
    Value(Value)
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
            (Self::Tagged(l0, l1), Self::Tagged(r0, r1)) => l0 == r0 && l1 == r1,
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

fn format_map(m: &Map) -> String {
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
            Self::Tagged(tag, item) => f.write_fmt(format_args!("Tagged({}, {:?})", tag, item)),
            Self::Value(x) => f.write_fmt(format_args!("Value({})", x.name())),
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
            CBOR::Tagged(tag, item) => format!("{}", Tagged::new(tag.clone(), *item.clone())),
            CBOR::Value(x) => format!("{}", x),
        };
        f.write_str(&s)
    }
}
