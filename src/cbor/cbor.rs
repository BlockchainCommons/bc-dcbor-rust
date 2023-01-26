use crate::util::string_util::flanked;

use super::{bytes::Bytes, Value, Tagged, Map};

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
    Map(Map),
    /// Tagged value (major type 6).
    Tagged(Box<Tagged>),
    /// Simple value (majory type 7).
    Value(Value)
}

/// A value that can be encoded as CBOR.
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

#[cfg(test)]
mod tests {
    use crate::cbor::Map;
    use crate::cbor::decode::{decode, Error};
    use crate::cbor::test_util::test_cbor;
    use crate::util::hex::hex_to_bytes;

    use super::{Tagged};

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

    #[test]
    fn encode() {
        let mut m = Map::new();
        m.insert_into(-1, 3);
        m.insert_into(vec![-1], 7);
        m.insert_into("z", 4);
        m.insert_into(10, 1);
        m.insert_into(false, 8);
        m.insert_into(100, 2);
        m.insert_into("aa", 5);
        m.insert_into(vec![100], 6);
        test_cbor(m,
            r#"Map({0x0a: (UInt(10), UInt(1)), 0x1864: (UInt(100), UInt(2)), 0x20: (NInt(-1), UInt(3)), 0x617a: (String("z"), UInt(4)), 0x626161: (String("aa"), UInt(5)), 0x811864: (Array([UInt(100)]), UInt(6)), 0x8120: (Array([NInt(-1)]), UInt(7)), 0xf4: (Value(false), UInt(8))})"#,
            r#"{10: 1, 100: 2, -1: 3, "z": 4, "aa": 5, [100]: 6, [-1]: 7, false: 8}"#,
            "a80a011864022003617a046261610581186406812007f408");
    }

    #[test]
    fn misordered() {
        let cbor = decode(&hex_to_bytes("a2026141016142"));
        assert_eq!(cbor, Err(Error::MisorderedMapKey));
    }
}
