use crate::tag::Tag;
use crate::Simple;

use super::{bytes::Data, Tagged, Map, string_util::flanked};

/// A symbolic representation of CBOR data.
#[derive(Clone)]
pub enum CBOR {
    /// Unsigned integer (major type 0).
    Unsigned(u64),
    /// Negative integer (major type 1).
    Negative(i64),
    /// Byte string (major type 2).
    Bytes(Data),
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

impl CBOR {
    pub const FALSE: CBOR = CBOR::Simple(Simple(20));
    pub const TRUE: CBOR = CBOR::Simple(Simple(21));
    pub const NULL: CBOR = CBOR::Simple(Simple(22));
}

impl PartialEq for CBOR {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unsigned(l0), Self::Unsigned(r0)) => l0 == r0,
            (Self::Negative(l0), Self::Negative(r0)) => l0 == r0,
            (Self::Bytes(l0), Self::Bytes(r0)) => l0 == r0,
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
            Self::Unsigned(x) => f.debug_tuple("unsigned").field(x).finish(),
            Self::Negative(x) => f.debug_tuple("negative").field(x).finish(),
            Self::Bytes(x) => f.debug_tuple("bytes").field(x).finish(),
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
            CBOR::Bytes(x) => format!("{}", x),
            CBOR::Text(x) => format_string(x),
            CBOR::Array(x) => format_array(x),
            CBOR::Map(x) => format_map(x),
            CBOR::Tagged(tag, item) => format!("{}", Tagged::new(tag.clone(), *item.clone())),
            CBOR::Simple(x) => format!("{}", x),
        };
        f.write_str(&s)
    }
}
