use super::{cbor::{CBOREncodable, CBOR}, varint::{EncodeVarInt, MajorType}};

impl<T> CBOREncodable for Vec<T> where T: CBOREncodable {
    fn cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.cbor()).collect())
    }

    fn encode_cbor(&self) -> Vec<u8> {
        let mut buf = self.len().encode_varint(MajorType::Array);
        for item in self {
            buf.extend(item.encode_cbor());
        }
        buf
    }
}

impl CBOREncodable for Vec<Box<dyn CBOREncodable>> {
    fn cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.cbor()).collect())
    }

    fn encode_cbor(&self) -> Vec<u8> {
        let mut buf = self.len().encode_varint(MajorType::Array);
        for item in self {
            buf.extend(item.encode_cbor());
        }
        buf
    }
}

impl<T, const N: usize> CBOREncodable for [T; N] where T: CBOREncodable {
    fn cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.cbor()).collect())
    }

    fn encode_cbor(&self) -> Vec<u8> {
        let mut buf = self.len().encode_varint(MajorType::Array);
        for item in self {
            buf.extend(item.encode_cbor());
        }
        buf
    }
}

impl<T, const N: usize> CBOREncodable for &[T; N] where T: CBOREncodable {
    fn cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.cbor()).collect())
    }

    fn encode_cbor(&self) -> Vec<u8> {
        let mut buf = self.len().encode_varint(MajorType::Array);
        for item in *self {
            buf.extend(item.encode_cbor());
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::cbor::test_util::test_cbor;

    #[test]
    fn encode() {
        test_cbor(vec![1, 2, 3], "Array([UInt(1), UInt(2), UInt(3)])", "[1, 2, 3]", "83010203");
        test_cbor([1, 2, 3], "Array([UInt(1), UInt(2), UInt(3)])", "[1, 2, 3]", "83010203");
        test_cbor(&[1, -2, 3], "Array([UInt(1), NInt(-2), UInt(3)])", "[1, -2, 3]", "83012103");
    }
}
