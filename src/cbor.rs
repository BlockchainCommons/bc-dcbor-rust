use crate::{bytes::Bytes, tagged::Tagged, map::CBORMap, value::Value, string_util::flanked};

#[derive(Debug, Clone)]
pub enum CBOR {
    UINT(u64),
    NINT(i64),
    BYTES(Bytes),
    STRING(String),
    ARRAY(Vec<CBOR>),
    MAP(CBORMap),
    TAGGED(Box<Tagged>),
    VALUE(Value)
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
            CBOR::UINT(x) => x.cbor_encode(),
            CBOR::NINT(x) => x.cbor_encode(),
            CBOR::BYTES(x) => x.cbor_encode(),
            CBOR::STRING(x) => x.cbor_encode(),
            CBOR::ARRAY(x) => x.cbor_encode(),
            CBOR::MAP(x) => x.cbor_encode(),
            CBOR::TAGGED(x) => x.cbor_encode(),
            CBOR::VALUE(x) => x.cbor_encode(),
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
            result.push_str("\\\"");
        } else {
            result.push(c);
        }
    }
    flanked(&result, "\"", "\"")
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
            CBOR::UINT(x) => format!("{}", x),
            CBOR::NINT(x) => format!("{}", x),
            CBOR::BYTES(x) => format!("{}", x),
            CBOR::STRING(x) => format_string(x),
            CBOR::ARRAY(x) => format_array(x),
            CBOR::MAP(x) => format_map(x),
            CBOR::TAGGED(x) => format!("{}", x),
            CBOR::VALUE(x) => format!("{}", x),
        };
        f.write_str(&s)
    }
}
