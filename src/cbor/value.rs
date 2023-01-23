use super::{cbor::{CBOREncode, IntoCBOR, CBOR}, varint::{VarIntEncode, MajorType}};

#[derive(Clone)]
pub struct Value(u64);

impl Value {
    pub fn new(v: u64) -> Value {
        Value(v)
    }
}

impl CBOREncode for Value {
    fn cbor_encode(&self) -> Vec<u8> {
        self.0.varint_encode(MajorType::Value)
    }
}

impl IntoCBOR for Value {
    fn cbor(&self) -> CBOR {
        CBOR::Value(self.clone())
    }
}

impl IntoCBOR for bool {
    fn cbor(&self) -> CBOR {
        match self {
            false => CBOR::Value(Value::new(20)),
            true => CBOR::Value(Value::new(21)),
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
    use crate::cbor::{test_util::test_cbor, cbor::IntoCBOR};

    use super::Value;

    #[test]
    fn encode() {
        test_cbor(false, "Value(false)", "f4");
        test_cbor(true, "Value(true)", "f5");
        test_cbor(Value::new(100), "Value(100)", "f864");
    }

    #[test]
    fn format() {
        assert_eq!(format!("{}", Value::new(20).cbor()), "false");
        assert_eq!(format!("{}", Value::new(21).cbor()), "true");
        assert_eq!(format!("{}", Value::new(100).cbor()), "simple(100)");
    }
}
