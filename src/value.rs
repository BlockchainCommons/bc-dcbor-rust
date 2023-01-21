use crate::{cbor::{CBOREncode, IntoCBOR, CBOR}, varint::VarIntEncode};

#[derive(Clone)]
pub struct Value(u64);

impl Value {
    pub fn new(v: u64) -> Value {
        Value(v)
    }
}

impl CBOREncode for Value {
    fn cbor_encode(&self) -> Vec<u8> {
        self.0.varint_encode(7)
    }
}

impl IntoCBOR for Value {
    fn cbor(&self) -> CBOR {
        CBOR::VALUE(self.clone())
    }
}

impl IntoCBOR for bool {
    fn cbor(&self) -> CBOR {
        match self {
            false => CBOR::VALUE(Value::new(20)),
            true => CBOR::VALUE(Value::new(21)),
        }
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

#[cfg(test)]
mod tests {
    use crate::test_util::test_cbor;

    use super::Value;

    #[test]
    fn encode() {
        test_cbor(false, "VALUE(false)", "f4");
        test_cbor(true, "VALUE(true)", "f5");
        test_cbor(Value::new(100), "VALUE(100)", "f864");
    }
}
