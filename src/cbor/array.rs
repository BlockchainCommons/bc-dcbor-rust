use super::{cbor::{EncodeCBOR, AsCBOR, CBOR, IntoCBOR}, varint::{EncodeVarInt, MajorType}};

impl<T> EncodeCBOR for Vec<T> where T: EncodeCBOR {
    fn encode_cbor(&self) -> Vec<u8> {
        let mut buf = self.len().encode_varint(MajorType::Array);
        for item in self {
            buf.extend(item.encode_cbor());
        }
        buf
    }
}

impl<T> AsCBOR for Vec<T> where T: AsCBOR {
    fn as_cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.as_cbor()).collect())
    }
}

impl<T> IntoCBOR for Vec<T> where T: IntoCBOR {
    fn into_cbor(self) -> CBOR {
        CBOR::Array(self.into_iter().map(|x| x.into_cbor()).collect())
    }
}

impl AsCBOR for Vec<Box<dyn AsCBOR>> {
    fn as_cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.as_cbor()).collect())
    }
}

impl<T, const N: usize> AsCBOR for [T; N] where T: AsCBOR {
    fn as_cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.as_cbor()).collect())
    }
}

impl<T, const N: usize> IntoCBOR for [T; N] where T: IntoCBOR {
    fn into_cbor(self) -> CBOR {
        CBOR::Array(self.into_iter().map(|x| x.into_cbor()).collect())
    }
}

impl<T, const N: usize> AsCBOR for &[T; N] where T: AsCBOR {
    fn as_cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.as_cbor()).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::cbor::{test_util::test_cbor, cbor::AsCBOR};

    #[test]
    fn encode() {
        test_cbor(vec![1, 2, 3], "Array([Uint(1), Uint(2), Uint(3)])", "[1, 2, 3]", "83010203");
        test_cbor([1, 2, 3], "Array([Uint(1), Uint(2), Uint(3)])", "[1, 2, 3]", "83010203");
        test_cbor(&[1, -2, 3], "Array([Uint(1), Nint(-2), Uint(3)])", "[1, -2, 3]", "83012103");
    }
}
