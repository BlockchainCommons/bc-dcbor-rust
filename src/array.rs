use crate::{cbor::{CBOREncode, IntoCBOR, CBOR}, varint::VarIntEncode};

impl<T> CBOREncode for Vec<T> where T: CBOREncode {
    fn cbor_encode(&self) -> Vec<u8> {
        let mut buf = self.len().varint_encode(4);
        for item in self {
            buf.extend(item.cbor_encode());
        }
        buf
    }
}

impl<T> IntoCBOR for Vec<T> where T: IntoCBOR {
    fn cbor(&self) -> CBOR {
        CBOR::ARRAY(self.iter().map(|x| x.cbor()).collect())
    }
}

impl<T, const N: usize> IntoCBOR for [T; N] where T: IntoCBOR {
    fn cbor(&self) -> CBOR {
        CBOR::ARRAY(self.iter().map(|x| x.cbor()).collect())
    }
}

impl<T, const N: usize> IntoCBOR for &[T; N] where T: IntoCBOR {
    fn cbor(&self) -> CBOR {
        CBOR::ARRAY(self.iter().map(|x| x.cbor()).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_util::test_cbor, cbor::IntoCBOR};

    #[test]
    fn encode() {
        test_cbor(vec![1, 2, 3], "ARRAY([UINT(1), UINT(2), UINT(3)])", "83010203");
        test_cbor([1, 2, 3], "ARRAY([UINT(1), UINT(2), UINT(3)])", "83010203");
        test_cbor(&[1, -2, 3], "ARRAY([UINT(1), NINT(-2), UINT(3)])", "83012103");
    }

    #[test]
    fn format() {
        assert_eq!(format!("{}", [1, 2, 3].cbor()), "[1, 2, 3]");
    }
}
