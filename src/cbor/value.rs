use super::{cbor::{CBOREncodable, CBOR}, varint::{EncodeVarInt, MajorType}};

#[derive(Clone)]
pub struct Value(u64);

impl Value {
    pub fn new(v: u64) -> Value {
        Value(v)
    }
}

impl CBOREncodable for Value {
    fn as_cbor(&self) -> CBOR {
        CBOR::Value(self.clone())
    }

    fn encode_cbor(&self) -> Vec<u8> {
        self.0.encode_varint(MajorType::Value)
    }
}

impl CBOREncodable for bool {
    fn as_cbor(&self) -> CBOR {
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

#[cfg(test)]
mod tests {
    use crate::cbor::test_util::test_cbor;

    use super::Value;

    #[test]
    fn encode() {
        test_cbor(false, "Value(false)", "false", "f4");
        test_cbor(true, "Value(true)", "true", "f5");
        test_cbor(Value::new(100), "Value(100)", "simple(100)", "f864");
    }
}
