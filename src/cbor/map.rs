use std::collections::BTreeMap;

use super::{cbor::{CBOREncode, IntoCBOR, CBOR}, varint::{VarIntEncode, MajorType}};

pub type CBORMap = BTreeMap<Vec<u8>, (CBOR, CBOR)>;

impl CBOREncode for CBORMap {
    fn cbor_encode(&self) -> Vec<u8> {
        let pairs: Vec<(Vec<u8>, Vec<u8>)> = self.iter().map(|x| (x.0.to_owned(), x.1.1.cbor_encode())).collect();
        let mut buf = pairs.len().varint_encode(MajorType::Map);
        for pair in pairs {
            buf.extend(pair.0);
            buf.extend(pair.1);
        }
        buf
    }
}

impl IntoCBOR for CBORMap {
    fn cbor(&self) -> CBOR {
        CBOR::Map(self.clone())
    }
}

pub trait CBORMapInsert {
    fn cbor_insert(&mut self, k: CBOR, v: CBOR);
    fn cbor_insert_into<K, V>(&mut self, k: K, v: V) where K: IntoCBOR, V: IntoCBOR;
}

impl CBORMapInsert for CBORMap {
    fn cbor_insert(&mut self, k: CBOR, v: CBOR) {
        self.insert(k.cbor_encode(), (k, v));
    }

    fn cbor_insert_into<K, V>(&mut self, k: K, v: V) where K: IntoCBOR, V: IntoCBOR {
        self.cbor_insert(k.cbor(), v.cbor());
    }
}

#[cfg(test)]
mod tests {
    use crate::cbor::{test_util::test_cbor, cbor::IntoCBOR};

    use super::{CBORMapInsert, CBORMap};

    #[test]
    fn encode() {
        let mut m = CBORMap::new();
        m.cbor_insert_into(-1, 3);
        m.cbor_insert_into(vec![-1], 7);
        m.cbor_insert_into("z", 4);
        m.cbor_insert_into(10, 1);
        m.cbor_insert_into(false, 8);
        m.cbor_insert_into(100, 2);
        m.cbor_insert_into("aa", 5);
        m.cbor_insert_into(vec![100], 6);
        test_cbor(m,
            r#"Map({[10]: (Uint(10), Uint(1)), [24, 100]: (Uint(100), Uint(2)), [32]: (Nint(-1), Uint(3)), [97, 122]: (String("z"), Uint(4)), [98, 97, 97]: (String("aa"), Uint(5)), [129, 24, 100]: (Array([Uint(100)]), Uint(6)), [129, 32]: (Array([Nint(-1)]), Uint(7)), [244]: (Value(false), Uint(8))})"#,
            "a80a011864022003617a046261610581186406812007f408");
    }

    #[test]
    fn format() {
        let mut m = CBORMap::new();
        m.cbor_insert_into(-1, 3);
        m.cbor_insert_into(vec![-1], 7);
        m.cbor_insert_into("z", 4);
        assert_eq!(format!("{}", m.cbor()), r#"{-1: 3, "z": 4, [-1]: 7}"#);
    }
}
