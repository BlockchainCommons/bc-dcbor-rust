use crate::util::string_util::flanked;

use super::{bytes::Bytes, map::CBORMap, tagged::Tagged, value::Value};

#[derive(Debug, Clone)]
pub enum CBOR {
    Uint(u64),
    Nint(i64),
    Bytes(Bytes),
    String(String),
    Array(Vec<CBOR>),
    Map(CBORMap),
    Tagged(Box<Tagged>),
    Value(Value)
}

pub trait AsCBOR {
    fn as_cbor(&self) -> CBOR;
}

pub trait IntoCBOR {
    fn into_cbor(self) -> CBOR;
}

pub trait CBOREncode {
    fn cbor_encode(&self) -> Vec<u8>;
}

impl AsCBOR for CBOR {
    fn as_cbor(&self) -> CBOR {
        self.clone()
    }
}

impl IntoCBOR for CBOR {
    fn into_cbor(self) -> CBOR {
        self
    }
}

impl CBOR {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            CBOR::Uint(x) => x.cbor_encode(),
            CBOR::Nint(x) => x.cbor_encode(),
            CBOR::Bytes(x) => x.cbor_encode(),
            CBOR::String(x) => x.cbor_encode(),
            CBOR::Array(x) => x.cbor_encode(),
            CBOR::Map(x) => x.cbor_encode(),
            CBOR::Tagged(x) => x.cbor_encode(),
            CBOR::Value(x) => x.cbor_encode(),
        }
    }
}

impl CBOREncode for CBOR {
    fn cbor_encode(&self) -> Vec<u8> {
        self.encode()
    }
}

impl PartialEq for CBOR {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Uint(l0), Self::Uint(r0)) => l0 == r0,
            (Self::Nint(l0), Self::Nint(r0)) => l0 == r0,
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
    let s: Vec<String> = m.values().map(|x| format!("{}: {}", x.0, x.1)).collect();
    flanked(&s.join(", "), "{", "}")
}

impl std::fmt::Display for CBOR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CBOR::Uint(x) => format!("{}", x),
            CBOR::Nint(x) => format!("{}", x),
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
