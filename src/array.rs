use crate::{cbor_encode::CBOREncode, varint::VarIntEncode};

impl<T> CBOREncode for &[T] where T: CBOREncode {
    fn cbor_encode(&self) -> Vec<u8> {
        let mut buf = self.len().varint_encode(4);
        for item in *self {
            buf.extend(item.cbor_encode());
        }
        buf
    }
}

impl CBOREncode for &[Box<dyn CBOREncode>] {
    fn cbor_encode(&self) -> Vec<u8> {
        let mut buf = self.len().varint_encode(4);
        for item in *self {
            buf.extend(item.cbor_encode());
        }
        buf
    }
}

impl<T> CBOREncode for Vec<T> where T: CBOREncode {
    fn cbor_encode(&self) -> Vec<u8> {
        let mut buf = self.len().varint_encode(4);
        for item in self {
            buf.extend(item.cbor_encode());
        }
        buf
    }
}

impl CBOREncode for Vec<Box<dyn CBOREncode>> {
    fn cbor_encode(&self) -> Vec<u8> {
        let mut buf = self.len().varint_encode(4);
        for item in self {
            buf.extend(item.cbor_encode());
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_util::{test_encode_array, test_encode_heterogenous_array}, cbor_encode::CBOREncode};

    #[test]
    fn encode_array() {
        test_encode_array(&[1, 2, 3], "83010203");
        test_encode_array(&vec!["Hello", "World"], "826548656c6c6f65576f726c64");

        let mut v: Vec<Box<dyn CBOREncode>> = Vec::new();
        v.push(Box::new(1));
        v.push(Box::new(2));
        v.push(Box::new("Hello"));
        v.push(Box::new("World"));
        v.push(Box::new(vec![10, 20, 30]));
        // [1, 2, "Hello", "World", [10, 20, 30]]
        test_encode_heterogenous_array(&v, "8501026548656c6c6f65576f726c64830a14181e");
    }
}
