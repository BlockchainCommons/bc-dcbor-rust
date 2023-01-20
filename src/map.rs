use std::collections::HashMap;

use crate::{cbor::{CBOREncode, CBOR, IntoCBOR}, varint::VarIntEncode};

pub type CBORMap = HashMap<Vec<u8>, (CBOR, CBOR)>;

impl CBOREncode for CBORMap {
    fn cbor_encode(&self) -> Vec<u8> {
        let mut pairs: Vec<(Vec<u8>, Vec<u8>)> = self.iter().map(|x| (x.0.to_owned(), x.1.1.cbor_encode())).collect();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        let mut buf = pairs.len().varint_encode(5);
        for pair in pairs {
            buf.extend(pair.0);
            buf.extend(pair.1);
        }
        buf
    }
}

trait CBORMapInsert {
    fn cbor_insert<K, V>(&mut self, k: K, v: V) where K: IntoCBOR, V: IntoCBOR;
}

impl CBORMapInsert for CBORMap {
    fn cbor_insert<K, V>(&mut self, k: K, v: V) where K: IntoCBOR, V: IntoCBOR {
        let k = k.cbor();
        let v = v.cbor();
        self.insert(k.cbor_encode(), (k, v));
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_util::test_encode_map, cbor::IntoCBOR};

    use super::{CBORMapInsert, CBORMap};

    #[test]
    fn encode() {
        let mut m = CBORMap::new();
        m.cbor_insert(-1, 3);
        m.cbor_insert(vec![-1], 7);
        m.cbor_insert("z", 4);
        m.cbor_insert(10, 1);
        m.cbor_insert(false, 8);
        m.cbor_insert(100, 2);
        m.cbor_insert("aa", 5);
        m.cbor_insert(vec![100], 6);
        test_encode_map(&m, "a80a011864022003617a046261610581186406812007f408");
    }

    #[test]
    fn into_cbor() {
        assert_eq!(format!("{:?}", [1.cbor(), (-2).cbor(), 3.cbor()].cbor()), "ARRAY([UINT(1), NINT(-2), UINT(3)])");
        assert_eq!(format!("{:?}", [1.cbor(), (-2).cbor(), 3.cbor(), "Hello".cbor()].cbor()), "ARRAY([UINT(1), NINT(-2), UINT(3), STRING(\"Hello\")])");
    }
}
