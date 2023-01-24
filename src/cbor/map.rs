use std::collections::BTreeMap;

use super::{cbor::{EncodeCBOR, AsCBOR, CBOR, IntoCBOR}, varint::{EncodeVarInt, MajorType}};

pub type CBORMap = BTreeMap<Vec<u8>, (CBOR, CBOR)>;

impl EncodeCBOR for CBORMap {
    fn encode_cbor(&self) -> Vec<u8> {
        let pairs: Vec<(Vec<u8>, Vec<u8>)> = self.iter().map(|x| (x.0.to_owned(), x.1.1.encode_cbor())).collect();
        let mut buf = pairs.len().encode_varint(MajorType::Map);
        for pair in pairs {
            buf.extend(pair.0);
            buf.extend(pair.1);
        }
        buf
    }
}

impl AsCBOR for CBORMap {
    fn as_cbor(&self) -> CBOR {
        CBOR::Map(self.clone())
    }
}

impl IntoCBOR for CBORMap {
    fn into_cbor(self) -> CBOR {
        CBOR::Map(self)
    }
}

pub trait CBORMapInsert {
    fn cbor_insert(&mut self, k: CBOR, v: CBOR);
    fn cbor_insert_into<K, V>(&mut self, k: K, v: V) where K: AsCBOR, V: AsCBOR;
}

impl CBORMapInsert for CBORMap {
    fn cbor_insert(&mut self, k: CBOR, v: CBOR) {
        self.insert(k.encode_cbor(), (k, v));
    }

    fn cbor_insert_into<K, V>(&mut self, k: K, v: V) where K: AsCBOR, V: AsCBOR {
        self.cbor_insert(k.as_cbor(), v.as_cbor());
    }
}

#[cfg(test)]
mod tests {
    use crate::cbor::test_util::test_cbor;

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
            r#"{10: 1, 100: 2, -1: 3, "z": 4, "aa": 5, [100]: 6, [-1]: 7, false: 8}"#,
            "a80a011864022003617a046261610581186406812007f408");
    }
}
