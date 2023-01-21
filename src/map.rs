use std::collections::BTreeMap;

use crate::{cbor::{CBOREncode, CBOR, IntoCBOR}, varint::VarIntEncode};

pub type CBORMap = BTreeMap<Vec<u8>, (CBOR, CBOR)>;

impl CBOREncode for CBORMap {
    fn cbor_encode(&self) -> Vec<u8> {
        let pairs: Vec<(Vec<u8>, Vec<u8>)> = self.iter().map(|x| (x.0.to_owned(), x.1.1.cbor_encode())).collect();
        let mut buf = pairs.len().varint_encode(5);
        for pair in pairs {
            buf.extend(pair.0);
            buf.extend(pair.1);
        }
        buf
    }
}

impl IntoCBOR for CBORMap {
    fn cbor(&self) -> CBOR {
        CBOR::MAP(self.clone())
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
    use crate::test_util::test_cbor;

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
        test_cbor(m,
            "MAP({[10]: (UINT(10), UINT(1)), [24, 100]: (UINT(100), UINT(2)), [32]: (NINT(-1), UINT(3)), [97, 122]: (STRING(\"z\"), UINT(4)), [98, 97, 97]: (STRING(\"aa\"), UINT(5)), [129, 24, 100]: (ARRAY([UINT(100)]), UINT(6)), [129, 32]: (ARRAY([NINT(-1)]), UINT(7)), [244]: (VALUE(false), UINT(8))})",
            "a80a011864022003617a046261610581186406812007f408");
    }
}
