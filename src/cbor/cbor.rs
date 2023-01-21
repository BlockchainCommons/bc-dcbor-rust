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

pub trait IntoCBOR {
    fn cbor(&self) -> CBOR;
}

pub trait CBOREncode {
    fn cbor_encode(&self) -> Vec<u8>;
}

impl IntoCBOR for CBOR {
    fn cbor(&self) -> CBOR {
        self.clone()
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
