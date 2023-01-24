use std::collections::{BTreeMap, btree_map::Values};

use crate::util::hex::bytes_to_hex;

use super::{cbor::{EncodeCBOR, AsCBOR, CBOR, IntoCBOR}, varint::{EncodeVarInt, MajorType}};

#[derive(Clone)]
struct CBORMapValue {
    key: CBOR,
    value: CBOR,
}

impl CBORMapValue {
    fn new(key: CBOR, value: CBOR) -> CBORMapValue {
        CBORMapValue { key, value }
    }
}

impl PartialEq for CBORMapValue {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

impl std::fmt::Debug for CBORMapValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({:?}, {:?})", self.key, self.value))
    }
}

#[derive(Clone)]
struct CBORMapKey(Vec<u8>);

impl CBORMapKey {
    fn new(key_data: Vec<u8>) -> CBORMapKey {
        CBORMapKey(key_data)
    }
}

impl PartialEq for CBORMapKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for CBORMapKey {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for CBORMapKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for CBORMapKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::fmt::Debug for CBORMapKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("0x{}", bytes_to_hex(&self.0)))
    }
}

pub struct CBORMapIter<'a>(Values<'a, CBORMapKey, CBORMapValue>);

impl<'a> CBORMapIter<'a> {
    fn new(values: Values<'a, CBORMapKey, CBORMapValue>) -> CBORMapIter<'a> {
        CBORMapIter(values)
    }
}

impl<'a> Iterator for CBORMapIter<'a> {
    type Item = (&'a CBOR, &'a CBOR);

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.0.next()?;
        Some((&entry.key, &entry.value))
    }
}

#[derive(Clone)]
pub struct CBORMap(BTreeMap<CBORMapKey, CBORMapValue>);

impl CBORMap {
    pub fn new() -> CBORMap {
        CBORMap(BTreeMap::new())
    }

    pub fn iter<'a>(&'a self) -> CBORMapIter<'a> {
        CBORMapIter::new(self.0.values())
    }
}

impl PartialEq for CBORMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for CBORMap {
    fn assert_receiver_is_total_eq(&self) {}
}

impl EncodeCBOR for CBORMap {
    fn encode_cbor(&self) -> Vec<u8> {
        let pairs: Vec<(Vec<u8>, Vec<u8>)> = self.0.iter().map(|x| (x.0.0.to_owned(), x.1.value.encode_cbor())).collect();
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
        self.0.insert(CBORMapKey::new(k.encode_cbor()), CBORMapValue::new(k, v));
    }

    fn cbor_insert_into<K, V>(&mut self, k: K, v: V) where K: AsCBOR, V: AsCBOR {
        self.cbor_insert(k.as_cbor(), v.as_cbor());
    }
}

impl std::fmt::Debug for CBORMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
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
            r#"Map({0x0a: (UInt(10), UInt(1)), 0x1864: (UInt(100), UInt(2)), 0x20: (NInt(-1), UInt(3)), 0x617a: (String("z"), UInt(4)), 0x626161: (String("aa"), UInt(5)), 0x811864: (Array([UInt(100)]), UInt(6)), 0x8120: (Array([NInt(-1)]), UInt(7)), 0xf4: (Value(false), UInt(8))})"#,
            r#"{10: 1, 100: 2, -1: 3, "z": 4, "aa": 5, [100]: 6, [-1]: 7, false: 8}"#,
            "a80a011864022003617a046261610581186406812007f408");
    }
}
